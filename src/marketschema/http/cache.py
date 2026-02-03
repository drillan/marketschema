"""HTTP response caching with LRU eviction.

This module provides ResponseCache, an in-memory LRU cache for HTTP responses.
"""

import time
from collections import OrderedDict
from dataclasses import dataclass
from datetime import timedelta
from typing import Any

# Cache constants
DEFAULT_CACHE_MAX_SIZE: int = 1000
DEFAULT_CACHE_TTL_SECONDS: int = 300  # 5 minutes


@dataclass
class CacheEntry:
    """Single cache entry with value and expiration time."""

    value: Any
    expires_at: float


class ResponseCache:
    """LRU cache for HTTP responses.

    Example:
        >>> cache = ResponseCache(max_size=500, default_ttl=timedelta(minutes=1))
        >>> client = AsyncHttpClient(cache=cache)
    """

    def __init__(
        self,
        max_size: int = DEFAULT_CACHE_MAX_SIZE,
        default_ttl: timedelta = timedelta(seconds=DEFAULT_CACHE_TTL_SECONDS),
    ) -> None:
        """Initialize response cache.

        Args:
            max_size: Maximum number of cached entries. Must be positive.
            default_ttl: Default time-to-live for cache entries. Must be positive.

        Raises:
            ValueError: If parameters are out of valid range.
        """
        if max_size <= 0:
            raise ValueError(f"max_size must be positive, got {max_size}")
        if default_ttl.total_seconds() <= 0:
            raise ValueError(f"default_ttl must be positive, got {default_ttl}")

        self.max_size = max_size
        self.default_ttl = default_ttl
        self._cache: OrderedDict[str, CacheEntry] = OrderedDict()

    def get(self, key: str) -> Any | None:
        """Get a value from the cache.

        Args:
            key: The cache key (typically the URL).

        Returns:
            The cached value, or None if not found or expired.
        """
        if key not in self._cache:
            return None

        entry = self._cache[key]

        # Check if expired
        if time.monotonic() > entry.expires_at:
            del self._cache[key]
            return None

        # Move to end (most recently used)
        self._cache.move_to_end(key)
        return entry.value

    def set(
        self,
        key: str,
        value: Any,
        ttl: timedelta | None = None,
    ) -> None:
        """Set a value in the cache.

        Args:
            key: The cache key.
            value: The value to cache.
            ttl: Time-to-live. Defaults to default_ttl.
        """
        # Use provided TTL or default
        actual_ttl = ttl if ttl is not None else self.default_ttl
        expires_at = time.monotonic() + actual_ttl.total_seconds()

        # Remove if already exists (to update position)
        if key in self._cache:
            del self._cache[key]

        # Evict oldest entries if at capacity
        while len(self._cache) >= self.max_size:
            self._cache.popitem(last=False)

        # Add new entry
        self._cache[key] = CacheEntry(value=value, expires_at=expires_at)

    def delete(self, key: str) -> None:
        """Delete a value from the cache.

        Args:
            key: The cache key.
        """
        if key in self._cache:
            del self._cache[key]

    def clear(self) -> None:
        """Clear all cached entries."""
        self._cache.clear()


__all__ = [
    "ResponseCache",
    "CacheEntry",
    "DEFAULT_CACHE_MAX_SIZE",
    "DEFAULT_CACHE_TTL_SECONDS",
]
