"""Unit tests for HTTP middleware."""

import asyncio

import httpx
import pytest
import respx

from marketschema.http import AsyncHttpClient
from marketschema.http.middleware import (
    DEFAULT_BACKOFF_FACTOR,
    DEFAULT_JITTER,
    DEFAULT_MAX_RETRIES,
    RETRYABLE_STATUS_CODES,
    RateLimitMiddleware,
    RetryMiddleware,
)


class TestRetryMiddlewareConstructor:
    """Tests for RetryMiddleware constructor (T035)."""

    def test_default_parameters(self):
        """Constructor should accept default parameters."""
        middleware = RetryMiddleware()
        assert middleware.max_retries == DEFAULT_MAX_RETRIES
        assert middleware.backoff_factor == DEFAULT_BACKOFF_FACTOR
        assert middleware.retry_statuses == RETRYABLE_STATUS_CODES
        assert middleware.jitter == DEFAULT_JITTER

    def test_custom_max_retries(self):
        """Constructor should accept custom max_retries."""
        middleware = RetryMiddleware(max_retries=5)
        assert middleware.max_retries == 5

    def test_custom_backoff_factor(self):
        """Constructor should accept custom backoff_factor."""
        middleware = RetryMiddleware(backoff_factor=1.0)
        assert middleware.backoff_factor == 1.0

    def test_custom_retry_statuses(self):
        """Constructor should accept custom retry_statuses."""
        middleware = RetryMiddleware(retry_statuses={503, 504})
        assert middleware.retry_statuses == {503, 504}

    def test_custom_jitter(self):
        """Constructor should accept custom jitter."""
        middleware = RetryMiddleware(jitter=0.2)
        assert middleware.jitter == 0.2


class TestRetryMiddlewareShouldRetry:
    """Tests for should_retry() method (T036)."""

    def test_should_retry_on_retryable_status(self):
        """should_retry() should return True for retryable status codes."""
        middleware = RetryMiddleware(max_retries=3)

        for status in RETRYABLE_STATUS_CODES:
            assert middleware.should_retry(status, attempt=0) is True
            assert middleware.should_retry(status, attempt=1) is True
            assert middleware.should_retry(status, attempt=2) is True

    def test_should_not_retry_when_max_retries_exceeded(self):
        """should_retry() should return False when max retries exceeded."""
        middleware = RetryMiddleware(max_retries=3)

        for status in RETRYABLE_STATUS_CODES:
            assert middleware.should_retry(status, attempt=3) is False
            assert middleware.should_retry(status, attempt=4) is False

    def test_should_not_retry_on_non_retryable_status(self):
        """should_retry() should return False for non-retryable status codes."""
        middleware = RetryMiddleware(max_retries=3)

        for status in [400, 401, 403, 404]:
            assert middleware.should_retry(status, attempt=0) is False

    def test_should_not_retry_on_success(self):
        """should_retry() should return False for success status codes."""
        middleware = RetryMiddleware(max_retries=3)

        for status in [200, 201, 204]:
            assert middleware.should_retry(status, attempt=0) is False

    def test_custom_retry_statuses(self):
        """should_retry() should respect custom retry_statuses."""
        middleware = RetryMiddleware(retry_statuses={503}, max_retries=3)

        assert middleware.should_retry(503, attempt=0) is True
        assert middleware.should_retry(500, attempt=0) is False
        assert middleware.should_retry(429, attempt=0) is False


class TestRetryMiddlewareGetDelay:
    """Tests for get_delay() exponential backoff (T037)."""

    def test_exponential_backoff(self):
        """get_delay() should implement exponential backoff."""
        middleware = RetryMiddleware(backoff_factor=0.5, jitter=0.0)

        # delay = backoff_factor * (2 ** attempt)
        assert middleware.get_delay(attempt=0) == 0.5  # 0.5 * (2^0) = 0.5
        assert middleware.get_delay(attempt=1) == 1.0  # 0.5 * (2^1) = 1.0
        assert middleware.get_delay(attempt=2) == 2.0  # 0.5 * (2^2) = 2.0
        assert middleware.get_delay(attempt=3) == 4.0  # 0.5 * (2^3) = 4.0

    def test_custom_backoff_factor(self):
        """get_delay() should respect custom backoff_factor."""
        middleware = RetryMiddleware(backoff_factor=1.0, jitter=0.0)

        assert middleware.get_delay(attempt=0) == 1.0  # 1.0 * (2^0) = 1.0
        assert middleware.get_delay(attempt=1) == 2.0  # 1.0 * (2^1) = 2.0
        assert middleware.get_delay(attempt=2) == 4.0  # 1.0 * (2^2) = 4.0


class TestRetryMiddlewareJitter:
    """Tests for jitter randomization (T038)."""

    def test_jitter_adds_randomization(self):
        """get_delay() should add jitter randomization."""
        middleware = RetryMiddleware(backoff_factor=1.0, jitter=0.1)

        # With 10% jitter, delay should be between 0.9 and 1.1 of base
        delays = [middleware.get_delay(attempt=0) for _ in range(100)]

        assert min(delays) >= 0.9  # 1.0 - 0.1 = 0.9
        assert max(delays) <= 1.1  # 1.0 + 0.1 = 1.1
        # Should have some variation
        assert len(set(delays)) > 1

    def test_zero_jitter_no_randomization(self):
        """get_delay() with zero jitter should have no randomization."""
        middleware = RetryMiddleware(backoff_factor=1.0, jitter=0.0)

        delays = [middleware.get_delay(attempt=0) for _ in range(10)]
        assert all(d == 1.0 for d in delays)


class TestRetryMiddlewareIntegration:
    """Tests for AsyncHttpClient with retry middleware integration (T039)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_retry_on_server_error(self):
        """Client should retry on server error and eventually succeed."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            if call_count < 3:
                return httpx.Response(503, text="Service Unavailable")
            return httpx.Response(200, json={"result": "ok"})

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        retry = RetryMiddleware(max_retries=3, backoff_factor=0.01, jitter=0.0)
        async with AsyncHttpClient(retry=retry) as client:
            result = await client.get_json("https://api.example.com/data")

        assert result == {"result": "ok"}
        assert call_count == 3

    @pytest.mark.asyncio
    @respx.mock
    async def test_retry_exhausted_raises_error(self):
        """Client should raise error when retries are exhausted."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(503, text="Service Unavailable")
        )

        retry = RetryMiddleware(max_retries=2, backoff_factor=0.01, jitter=0.0)
        async with AsyncHttpClient(retry=retry) as client:
            from marketschema.http import HttpStatusError

            with pytest.raises(HttpStatusError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 503

    @pytest.mark.asyncio
    @respx.mock
    async def test_no_retry_on_client_error(self):
        """Client should not retry on client error (4xx except 429)."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            return httpx.Response(404, text="Not Found")

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        retry = RetryMiddleware(max_retries=3, backoff_factor=0.01, jitter=0.0)
        async with AsyncHttpClient(retry=retry) as client:
            from marketschema.http import HttpStatusError

            with pytest.raises(HttpStatusError):
                await client.get_json("https://api.example.com/data")

        # Should only call once (no retry on 404)
        assert call_count == 1

    @pytest.mark.asyncio
    @respx.mock
    async def test_retry_on_rate_limit(self):
        """Client should retry on 429 rate limit error."""
        call_count = 0

        def handler(request: httpx.Request) -> httpx.Response:
            nonlocal call_count
            call_count += 1
            if call_count < 2:
                return httpx.Response(429, text="Too Many Requests")
            return httpx.Response(200, json={"result": "ok"})

        respx.get("https://api.example.com/data").mock(side_effect=handler)

        retry = RetryMiddleware(max_retries=3, backoff_factor=0.01, jitter=0.0)
        async with AsyncHttpClient(retry=retry) as client:
            result = await client.get_json("https://api.example.com/data")

        assert result == {"result": "ok"}
        assert call_count == 2


# Tests for RateLimitMiddleware will be added in Phase 6


class TestRateLimitMiddlewareConstructor:
    """Tests for RateLimitMiddleware constructor (T046)."""

    def test_default_parameters(self):
        """Constructor should set default parameters."""
        middleware = RateLimitMiddleware(requests_per_second=10.0)
        assert middleware.requests_per_second == 10.0
        assert middleware.burst_size == 10  # defaults to requests_per_second

    def test_custom_burst_size(self):
        """Constructor should accept custom burst_size."""
        middleware = RateLimitMiddleware(requests_per_second=10.0, burst_size=20)
        assert middleware.requests_per_second == 10.0
        assert middleware.burst_size == 20


class TestRateLimitMiddlewareAcquire:
    """Tests for acquire() blocking behavior (T047)."""

    @pytest.mark.asyncio
    async def test_acquire_succeeds_with_tokens(self):
        """acquire() should succeed when tokens are available."""
        middleware = RateLimitMiddleware(requests_per_second=10.0)

        # Should succeed immediately with tokens available
        start = asyncio.get_event_loop().time()
        await middleware.acquire()
        elapsed = asyncio.get_event_loop().time() - start

        assert elapsed < 0.1  # Should be nearly instant

    @pytest.mark.asyncio
    async def test_acquire_blocks_when_depleted(self):
        """acquire() should block when tokens are depleted."""
        middleware = RateLimitMiddleware(requests_per_second=10.0, burst_size=1)

        # First acquire should succeed
        await middleware.acquire()

        # Second acquire should block until token refill
        start = asyncio.get_event_loop().time()
        await middleware.acquire()
        elapsed = asyncio.get_event_loop().time() - start

        # Should wait approximately 0.1 seconds (1/10 requests per second)
        assert elapsed >= 0.05  # Allow some tolerance


class TestRateLimitMiddlewareTryAcquire:
    """Tests for try_acquire() non-blocking (T048)."""

    def test_try_acquire_succeeds_with_tokens(self):
        """try_acquire() should return True when tokens are available."""
        middleware = RateLimitMiddleware(requests_per_second=10.0)

        assert middleware.try_acquire() is True

    def test_try_acquire_fails_when_depleted(self):
        """try_acquire() should return False when tokens are depleted."""
        middleware = RateLimitMiddleware(requests_per_second=10.0, burst_size=1)

        assert middleware.try_acquire() is True  # First succeeds
        assert middleware.try_acquire() is False  # Second fails (no blocking)


class TestRateLimitMiddlewareBurstSize:
    """Tests for burst size handling (T049)."""

    def test_burst_allows_multiple_requests(self):
        """Burst size should allow multiple immediate requests."""
        middleware = RateLimitMiddleware(requests_per_second=1.0, burst_size=5)

        # Should allow 5 immediate acquisitions
        for _ in range(5):
            assert middleware.try_acquire() is True

        # 6th should fail
        assert middleware.try_acquire() is False

    @pytest.mark.asyncio
    async def test_tokens_refill_over_time(self):
        """Tokens should refill over time."""
        middleware = RateLimitMiddleware(requests_per_second=100.0, burst_size=1)

        # Deplete tokens
        assert middleware.try_acquire() is True
        assert middleware.try_acquire() is False

        # Wait for token refill (0.01 seconds = 1 token at 100 req/sec)
        await asyncio.sleep(0.02)

        # Should have token again
        assert middleware.try_acquire() is True


class TestRateLimitMiddlewareClientIntegration:
    """Tests for AsyncHttpClient with rate limit middleware integration (T050)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_rate_limit_applied_to_requests(self):
        """Client should apply rate limiting to requests."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"result": "ok"})
        )

        # Use slower rate to make timing more reliable
        rate_limit = RateLimitMiddleware(requests_per_second=20.0, burst_size=2)
        async with AsyncHttpClient(rate_limit=rate_limit) as client:
            # First two requests should be immediate (burst)
            await client.get_json("https://api.example.com/data")
            await client.get_json("https://api.example.com/data")

            # Third request should wait for token refill
            start = asyncio.get_event_loop().time()
            await client.get_json("https://api.example.com/data")
            elapsed = asyncio.get_event_loop().time() - start

            # Should have waited for token (approximately 0.05 seconds at 20 req/sec)
            assert elapsed >= 0.02
