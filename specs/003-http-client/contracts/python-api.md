# Python API Contract: HTTP Client Layer

**Feature**: 003-http-client
**Date**: 2026-02-03

> Note: HTTP クライアントレイヤーは Python ライブラリ内部 API のため、REST/GraphQL スキーマではなく、Python の型シグネチャとしてコントラクトを定義する。

## Module: `marketschema.http`

### AsyncHttpClient

```python
class AsyncHttpClient:
    """Async HTTP client for adapter implementations.

    Features:
    - Connection pooling
    - Configurable timeouts
    - Clean error handling

    Example:
        async with AsyncHttpClient() as client:
            data = await client.get_json("https://api.example.com/ticker")
    """

    def __init__(
        self,
        timeout: float = 30.0,
        max_connections: int = 100,
        headers: dict[str, str] | None = None,
        # Phase 2
        retry: RetryMiddleware | None = None,
        rate_limit: RateLimitMiddleware | None = None,
        # Phase 3
        cache: ResponseCache | None = None,
    ) -> None:
        """Initialize the HTTP client.

        Args:
            timeout: Request timeout in seconds.
            max_connections: Maximum number of concurrent connections.
            headers: Default headers for all requests.
            retry: Retry configuration (Phase 2).
            rate_limit: Rate limiting configuration (Phase 2).
            cache: Response cache configuration (Phase 3).
        """
        ...

    async def get(
        self,
        url: str,
        *,
        headers: dict[str, str] | None = None,
        params: dict[str, str | int | float | bool] | None = None,
        timeout: float | None = None,
    ) -> httpx.Response:
        """Send a GET request and return the raw response.

        Args:
            url: The URL to request.
            headers: Additional headers (merged with defaults).
            params: Query parameters.
            timeout: Override the default timeout.

        Returns:
            The httpx.Response object.

        Raises:
            HttpTimeoutError: If the request times out.
            HttpConnectionError: If connection fails.
            HttpStatusError: If the response has an error status code.
            HttpRateLimitError: If rate limited (429).
        """
        ...

    async def get_json(
        self,
        url: str,
        *,
        headers: dict[str, str] | None = None,
        params: dict[str, str | int | float | bool] | None = None,
        timeout: float | None = None,
    ) -> dict[str, Any]:
        """Send a GET request and return the JSON response.

        Args:
            url: The URL to request.
            headers: Additional headers (merged with defaults).
            params: Query parameters.
            timeout: Override the default timeout.

        Returns:
            The parsed JSON response as a dictionary.

        Raises:
            HttpTimeoutError: If the request times out.
            HttpConnectionError: If connection fails.
            HttpStatusError: If the response has an error status code.
            HttpRateLimitError: If rate limited (429).
            ValueError: If the response is not valid JSON.
        """
        ...

    async def get_text(
        self,
        url: str,
        *,
        headers: dict[str, str] | None = None,
        params: dict[str, str | int | float | bool] | None = None,
        timeout: float | None = None,
    ) -> str:
        """Send a GET request and return the text response.

        Args:
            url: The URL to request.
            headers: Additional headers (merged with defaults).
            params: Query parameters.
            timeout: Override the default timeout.

        Returns:
            The response body as a string.

        Raises:
            HttpTimeoutError: If the request times out.
            HttpConnectionError: If connection fails.
            HttpStatusError: If the response has an error status code.
            HttpRateLimitError: If rate limited (429).
        """
        ...

    async def close(self) -> None:
        """Close the HTTP client and release resources."""
        ...

    async def __aenter__(self) -> "AsyncHttpClient":
        """Enter async context manager."""
        ...

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit async context manager and close client."""
        ...
```

### Exceptions

```python
class HttpError(MarketSchemaError):
    """Base for all HTTP errors.

    Attributes:
        message: Error description.
        url: The URL that caused the error (if available).
    """

    def __init__(self, message: str, url: str | None = None) -> None: ...


class HttpTimeoutError(HttpError):
    """Request timed out."""

    pass


class HttpConnectionError(HttpError):
    """Connection failed."""

    pass


class HttpStatusError(HttpError):
    """HTTP status indicates error (4xx, 5xx).

    Attributes:
        status_code: The HTTP status code.
        response_body: The response body (if available).
    """

    def __init__(
        self,
        message: str,
        status_code: int,
        url: str | None = None,
        response_body: str | None = None,
    ) -> None: ...


class HttpRateLimitError(HttpStatusError):
    """Rate limit exceeded (429).

    Attributes:
        retry_after: Seconds to wait before retrying (from Retry-After header).
    """

    def __init__(
        self,
        message: str,
        url: str | None = None,
        response_body: str | None = None,
        retry_after: float | None = None,
    ) -> None: ...
```

### RetryMiddleware (Phase 2)

```python
class RetryMiddleware:
    """Retry failed requests with exponential backoff.

    Example:
        retry = RetryMiddleware(max_retries=5, backoff_factor=1.0)
        client = AsyncHttpClient(retry=retry)
    """

    def __init__(
        self,
        max_retries: int = 3,
        backoff_factor: float = 0.5,
        retry_statuses: set[int] | None = None,
        jitter: float = 0.1,
    ) -> None:
        """Initialize retry middleware.

        Args:
            max_retries: Maximum number of retry attempts.
            backoff_factor: Multiplier for exponential backoff.
            retry_statuses: Status codes to retry. Defaults to {429, 500, 502, 503, 504}.
            jitter: Random jitter factor (0.0 to 1.0) to add to delays.
        """
        ...

    def should_retry(self, status_code: int, attempt: int) -> bool:
        """Check if the request should be retried.

        Args:
            status_code: The HTTP status code.
            attempt: Current attempt number (0-indexed).

        Returns:
            True if the request should be retried.
        """
        ...

    def get_delay(self, attempt: int) -> float:
        """Calculate the delay before the next retry.

        Args:
            attempt: Current attempt number (0-indexed).

        Returns:
            Delay in seconds.
        """
        ...
```

### RateLimitMiddleware (Phase 2)

```python
class RateLimitMiddleware:
    """Rate limiting using token bucket algorithm.

    Example:
        rate_limit = RateLimitMiddleware(requests_per_second=10.0, burst_size=20)
        client = AsyncHttpClient(rate_limit=rate_limit)
    """

    def __init__(
        self,
        requests_per_second: float,
        burst_size: int | None = None,
    ) -> None:
        """Initialize rate limit middleware.

        Args:
            requests_per_second: Maximum requests per second.
            burst_size: Maximum burst size. Defaults to requests_per_second.
        """
        ...

    async def acquire(self) -> None:
        """Acquire a token, waiting if necessary.

        Blocks until a token is available.
        """
        ...

    def try_acquire(self) -> bool:
        """Try to acquire a token without blocking.

        Returns:
            True if a token was acquired, False otherwise.
        """
        ...
```

### ResponseCache (Phase 3)

```python
class ResponseCache:
    """LRU cache for HTTP responses.

    Example:
        cache = ResponseCache(max_size=500, default_ttl=timedelta(minutes=1))
        client = AsyncHttpClient(cache=cache)
    """

    def __init__(
        self,
        max_size: int = 1000,
        default_ttl: timedelta = timedelta(minutes=5),
    ) -> None:
        """Initialize response cache.

        Args:
            max_size: Maximum number of cached entries.
            default_ttl: Default time-to-live for cache entries.
        """
        ...

    def get(self, key: str) -> Any | None:
        """Get a value from the cache.

        Args:
            key: The cache key (typically the URL).

        Returns:
            The cached value, or None if not found or expired.
        """
        ...

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
        ...

    def delete(self, key: str) -> None:
        """Delete a value from the cache.

        Args:
            key: The cache key.
        """
        ...

    def clear(self) -> None:
        """Clear all cached entries."""
        ...
```

## Module: `marketschema.adapters.base`

### BaseAdapter Extensions

```python
class BaseAdapter:
    """Base adapter with HTTP client support."""

    _http_client: AsyncHttpClient | None = None

    def __init__(
        self,
        http_client: AsyncHttpClient | None = None,
    ) -> None:
        """Initialize the adapter.

        Args:
            http_client: Optional HTTP client. If not provided, one will be
                created lazily when http_client property is accessed.
        """
        ...

    @property
    def http_client(self) -> AsyncHttpClient:
        """Get the HTTP client (lazy initialization).

        Returns:
            The HTTP client instance.
        """
        ...

    async def close(self) -> None:
        """Close the HTTP client if owned by this adapter."""
        ...

    async def __aenter__(self) -> Self:
        """Enter async context manager."""
        ...

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit async context manager and close resources."""
        ...
```

## Type Exports

```python
# marketschema.http.__init__.py
__all__ = [
    # Client
    "AsyncHttpClient",

    # Exceptions
    "HttpError",
    "HttpTimeoutError",
    "HttpConnectionError",
    "HttpStatusError",
    "HttpRateLimitError",

    # Middleware (Phase 2)
    "RetryMiddleware",
    "RateLimitMiddleware",

    # Cache (Phase 3)
    "ResponseCache",
]
```
