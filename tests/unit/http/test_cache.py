"""Unit tests for HTTP response cache."""

import asyncio
from datetime import timedelta

import httpx
import pytest
import respx

from marketschema.http import AsyncHttpClient
from marketschema.http.cache import (
    DEFAULT_CACHE_MAX_SIZE,
    DEFAULT_CACHE_TTL_SECONDS,
    ResponseCache,
)


class TestResponseCacheConstructor:
    """Tests for ResponseCache constructor (T057)."""

    def test_default_parameters(self):
        """Constructor should set default parameters."""
        cache = ResponseCache()
        assert cache.max_size == DEFAULT_CACHE_MAX_SIZE
        assert cache.default_ttl == timedelta(seconds=DEFAULT_CACHE_TTL_SECONDS)

    def test_custom_max_size(self):
        """Constructor should accept custom max_size."""
        cache = ResponseCache(max_size=500)
        assert cache.max_size == 500

    def test_custom_default_ttl(self):
        """Constructor should accept custom default_ttl."""
        cache = ResponseCache(default_ttl=timedelta(minutes=10))
        assert cache.default_ttl == timedelta(minutes=10)

    def test_invalid_max_size_raises_error(self):
        """Constructor should raise ValueError for non-positive max_size."""
        import pytest

        with pytest.raises(ValueError, match="max_size must be positive"):
            ResponseCache(max_size=0)

        with pytest.raises(ValueError, match="max_size must be positive"):
            ResponseCache(max_size=-1)

    def test_invalid_default_ttl_raises_error(self):
        """Constructor should raise ValueError for non-positive default_ttl."""
        import pytest

        with pytest.raises(ValueError, match="default_ttl must be positive"):
            ResponseCache(default_ttl=timedelta(seconds=0))

        with pytest.raises(ValueError, match="default_ttl must be positive"):
            ResponseCache(default_ttl=timedelta(seconds=-1))


class TestResponseCacheGetSet:
    """Tests for get() and set() methods (T058)."""

    def test_set_and_get(self):
        """set() and get() should store and retrieve values."""
        cache = ResponseCache()
        cache.set("key1", "value1")
        assert cache.get("key1") == "value1"

    def test_get_missing_key(self):
        """get() should return None for missing keys."""
        cache = ResponseCache()
        assert cache.get("nonexistent") is None

    def test_set_overwrites_existing(self):
        """set() should overwrite existing values."""
        cache = ResponseCache()
        cache.set("key1", "value1")
        cache.set("key1", "value2")
        assert cache.get("key1") == "value2"

    def test_set_with_custom_ttl(self):
        """set() should accept custom TTL."""
        cache = ResponseCache()
        cache.set("key1", "value1", ttl=timedelta(seconds=1))
        assert cache.get("key1") == "value1"


class TestResponseCacheTTLExpiration:
    """Tests for TTL expiration (T059)."""

    @pytest.mark.asyncio
    async def test_expired_entry_returns_none(self):
        """get() should return None for expired entries."""
        cache = ResponseCache(default_ttl=timedelta(milliseconds=50))
        cache.set("key1", "value1")

        # Should be accessible immediately
        assert cache.get("key1") == "value1"

        # Wait for expiration
        await asyncio.sleep(0.1)

        # Should be expired
        assert cache.get("key1") is None

    @pytest.mark.asyncio
    async def test_custom_ttl_per_entry(self):
        """set() should respect per-entry TTL."""
        cache = ResponseCache(default_ttl=timedelta(seconds=10))

        # Set entry with short TTL
        cache.set("short", "value1", ttl=timedelta(milliseconds=50))
        # Set entry with default TTL
        cache.set("long", "value2")

        # Both should be accessible
        assert cache.get("short") == "value1"
        assert cache.get("long") == "value2"

        # Wait for short entry to expire
        await asyncio.sleep(0.1)

        # Short should be expired, long should still be valid
        assert cache.get("short") is None
        assert cache.get("long") == "value2"


class TestResponseCacheLRUEviction:
    """Tests for LRU eviction (T060)."""

    def test_evicts_oldest_when_full(self):
        """Cache should evict oldest entries when max_size is reached."""
        cache = ResponseCache(max_size=3)

        cache.set("key1", "value1")
        cache.set("key2", "value2")
        cache.set("key3", "value3")

        # All should be accessible
        assert cache.get("key1") == "value1"
        assert cache.get("key2") == "value2"
        assert cache.get("key3") == "value3"

        # Adding a new entry should evict the oldest
        cache.set("key4", "value4")

        # key1 should be evicted (oldest)
        assert cache.get("key1") is None
        assert cache.get("key2") == "value2"
        assert cache.get("key3") == "value3"
        assert cache.get("key4") == "value4"

    def test_access_refreshes_entry_order(self):
        """Accessing an entry should refresh its position in LRU order."""
        cache = ResponseCache(max_size=3)

        cache.set("key1", "value1")
        cache.set("key2", "value2")
        cache.set("key3", "value3")

        # Access key1 to make it "recently used"
        cache.get("key1")

        # Add new entry - should evict key2 (now oldest)
        cache.set("key4", "value4")

        # key1 should still be there (recently accessed)
        assert cache.get("key1") == "value1"
        # key2 should be evicted
        assert cache.get("key2") is None
        assert cache.get("key3") == "value3"
        assert cache.get("key4") == "value4"


class TestResponseCacheDeleteClear:
    """Tests for delete() and clear() methods (T061)."""

    def test_delete_existing_key(self):
        """delete() should remove existing entry."""
        cache = ResponseCache()
        cache.set("key1", "value1")
        cache.delete("key1")
        assert cache.get("key1") is None

    def test_delete_nonexistent_key(self):
        """delete() should not raise for nonexistent key."""
        cache = ResponseCache()
        cache.delete("nonexistent")  # Should not raise

    def test_clear_removes_all_entries(self):
        """clear() should remove all entries."""
        cache = ResponseCache()
        cache.set("key1", "value1")
        cache.set("key2", "value2")
        cache.set("key3", "value3")

        cache.clear()

        assert cache.get("key1") is None
        assert cache.get("key2") is None
        assert cache.get("key3") is None


class TestResponseCacheClientIntegration:
    """Tests for AsyncHttpClient with cache integration (T062)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_cache_hit_returns_cached_response(self):
        """Client should return cached response on cache hit."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            return httpx.Response(200, json={"call": call_count})

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        cache = ResponseCache()
        async with AsyncHttpClient(cache=cache) as client:
            # First request
            result1 = await client.get_json("https://api.example.com/data")
            # Second request (should hit cache)
            result2 = await client.get_json("https://api.example.com/data")

        # Only one actual HTTP call should be made
        assert call_count == 1
        # Both results should be the same (cached)
        assert result1["call"] == result2["call"]

    @pytest.mark.asyncio
    @respx.mock
    async def test_different_urls_not_cached_together(self):
        """Client should not return cached response for different URLs."""
        respx.get("https://api.example.com/data1").mock(
            return_value=httpx.Response(200, json={"endpoint": "data1"})
        )
        respx.get("https://api.example.com/data2").mock(
            return_value=httpx.Response(200, json={"endpoint": "data2"})
        )

        cache = ResponseCache()
        async with AsyncHttpClient(cache=cache) as client:
            result1 = await client.get_json("https://api.example.com/data1")
            result2 = await client.get_json("https://api.example.com/data2")

        assert result1["endpoint"] == "data1"
        assert result2["endpoint"] == "data2"

    @pytest.mark.asyncio
    @respx.mock
    async def test_different_params_not_cached_together(self):
        """Client should use different cache keys for different query params."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            return httpx.Response(200, json={"call": call_count})

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        cache = ResponseCache()
        async with AsyncHttpClient(cache=cache) as client:
            await client.get_json("https://api.example.com/data", params={"page": 1})
            await client.get_json("https://api.example.com/data", params={"page": 2})

        # Both should be separate requests
        assert call_count == 2

    @pytest.mark.asyncio
    @respx.mock
    async def test_error_response_not_cached(self):
        """Client should not cache error responses."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            if call_count == 1:
                return httpx.Response(500, text="Server Error")
            return httpx.Response(200, json={"result": "ok"})

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        cache = ResponseCache()
        async with AsyncHttpClient(cache=cache) as client:
            from marketschema.http import HttpStatusError

            # First request should fail
            with pytest.raises(HttpStatusError):
                await client.get_json("https://api.example.com/data")

            # Second request should succeed (not cached error)
            result = await client.get_json("https://api.example.com/data")

        assert result == {"result": "ok"}
        assert call_count == 2
