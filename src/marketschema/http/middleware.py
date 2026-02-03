"""HTTP middleware for retry and rate limiting.

This module provides middleware components for the HTTP client:
- RetryMiddleware: Retry failed requests with exponential backoff
- RateLimitMiddleware: Rate limiting using token bucket algorithm
"""

import asyncio
import random
import time

# Retry constants
DEFAULT_MAX_RETRIES: int = 3
DEFAULT_BACKOFF_FACTOR: float = 0.5
DEFAULT_JITTER: float = 0.1
RETRYABLE_STATUS_CODES: frozenset[int] = frozenset({429, 500, 502, 503, 504})
NON_RETRYABLE_STATUS_CODES: frozenset[int] = frozenset({400, 401, 403, 404})

# Rate limit constants
TOKENS_PER_REQUEST: float = 1.0


class RetryMiddleware:
    """Retry failed requests with exponential backoff.

    Example:
        >>> retry = RetryMiddleware(max_retries=5, backoff_factor=1.0)
        >>> client = AsyncHttpClient(retry=retry)
    """

    def __init__(
        self,
        max_retries: int = DEFAULT_MAX_RETRIES,
        backoff_factor: float = DEFAULT_BACKOFF_FACTOR,
        retry_statuses: set[int] | None = None,
        jitter: float = DEFAULT_JITTER,
    ) -> None:
        """Initialize retry middleware.

        Args:
            max_retries: Maximum number of retry attempts. Must be non-negative.
            backoff_factor: Multiplier for exponential backoff. Must be positive.
            retry_statuses: Status codes to retry. Defaults to {429, 500, 502, 503, 504}.
            jitter: Random jitter factor (0.0 to 1.0) to add to delays.

        Raises:
            ValueError: If parameters are out of valid range.
        """
        if max_retries < 0:
            raise ValueError(f"max_retries must be non-negative, got {max_retries}")
        if backoff_factor <= 0:
            raise ValueError(f"backoff_factor must be positive, got {backoff_factor}")
        if not (0 <= jitter <= 1):
            raise ValueError(f"jitter must be between 0 and 1, got {jitter}")

        self.max_retries = max_retries
        self.backoff_factor = backoff_factor
        self.retry_statuses = (
            retry_statuses
            if retry_statuses is not None
            else set(RETRYABLE_STATUS_CODES)
        )
        self.jitter = jitter

    def should_retry(self, status_code: int, attempt: int) -> bool:
        """Check if the request should be retried.

        Args:
            status_code: The HTTP status code.
            attempt: Current attempt number (0-indexed).

        Returns:
            True if the request should be retried.
        """
        if attempt >= self.max_retries:
            return False
        return status_code in self.retry_statuses

    def get_delay(self, attempt: int) -> float:
        """Calculate the delay before the next retry.

        Uses exponential backoff: delay = backoff_factor * (2 ** attempt)
        With optional jitter: delay * (1 ± jitter)

        Args:
            attempt: Current attempt number (0-indexed).

        Returns:
            Delay in seconds.
        """
        base_delay: float = self.backoff_factor * (2**attempt)

        if self.jitter > 0:
            # Add random jitter: delay * (1 + random(-jitter, +jitter))
            jitter_factor: float = 1 + random.uniform(-self.jitter, self.jitter)
            return float(base_delay * jitter_factor)

        return float(base_delay)


class RateLimitMiddleware:
    """Rate limiting using token bucket algorithm.

    Example:
        >>> rate_limit = RateLimitMiddleware(requests_per_second=10.0, burst_size=20)
        >>> client = AsyncHttpClient(rate_limit=rate_limit)
    """

    def __init__(
        self,
        requests_per_second: float,
        burst_size: int | None = None,
    ) -> None:
        """Initialize rate limit middleware.

        Args:
            requests_per_second: Maximum requests per second. Must be positive.
            burst_size: Maximum burst size. Defaults to requests_per_second. Must be positive.

        Raises:
            ValueError: If parameters are out of valid range.
        """
        if requests_per_second <= 0:
            raise ValueError(
                f"requests_per_second must be positive, got {requests_per_second}"
            )
        if burst_size is not None and burst_size <= 0:
            raise ValueError(f"burst_size must be positive, got {burst_size}")

        self.requests_per_second = requests_per_second
        self.burst_size = (
            burst_size if burst_size is not None else int(requests_per_second)
        )

        # Token bucket state
        self._tokens = float(self.burst_size)
        self._last_update = time.monotonic()
        self._lock = asyncio.Lock()

    def _refill_tokens(self) -> None:
        """Refill tokens based on elapsed time."""
        now = time.monotonic()
        elapsed = now - self._last_update
        self._last_update = now

        # Add tokens based on elapsed time
        tokens_to_add = elapsed * self.requests_per_second
        self._tokens = min(self._tokens + tokens_to_add, float(self.burst_size))

    async def acquire(self) -> None:
        """Acquire a token, waiting if necessary.

        Blocks until a token is available. This method is thread-safe.
        """
        async with self._lock:
            self._refill_tokens()

            while self._tokens < TOKENS_PER_REQUEST:
                # Calculate wait time until next token
                wait_time = (
                    TOKENS_PER_REQUEST - self._tokens
                ) / self.requests_per_second
                await asyncio.sleep(wait_time)
                self._refill_tokens()

            self._tokens -= TOKENS_PER_REQUEST

    def try_acquire(self) -> bool:
        """Try to acquire a token without blocking.

        Warning:
            This method is NOT thread-safe. When called concurrently from
            multiple tasks, race conditions may occur. For concurrent access,
            use acquire() instead.

        Returns:
            True if a token was acquired, False otherwise.
        """
        self._refill_tokens()

        if self._tokens >= TOKENS_PER_REQUEST:
            self._tokens -= TOKENS_PER_REQUEST
            return True
        return False


__all__ = [
    "RetryMiddleware",
    "RateLimitMiddleware",
    "DEFAULT_MAX_RETRIES",
    "DEFAULT_BACKOFF_FACTOR",
    "DEFAULT_JITTER",
    "RETRYABLE_STATUS_CODES",
    "NON_RETRYABLE_STATUS_CODES",
    "TOKENS_PER_REQUEST",
]
