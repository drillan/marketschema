"""Async HTTP client for marketschema adapters.

This module provides AsyncHttpClient, the main HTTP client class for adapter
implementations.
"""

from __future__ import annotations

import asyncio
import logging
from types import TracebackType
from typing import TYPE_CHECKING, Any

import httpx

from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)

if TYPE_CHECKING:
    from marketschema.http.cache import ResponseCache
    from marketschema.http.middleware import RateLimitMiddleware, RetryMiddleware

# Constants
DEFAULT_TIMEOUT_SECONDS: float = 30.0
DEFAULT_MAX_CONNECTIONS: int = 100


class AsyncHttpClient:
    """Async HTTP client for adapter implementations.

    Features:
    - Connection pooling
    - Configurable timeouts
    - Clean error handling
    - Retry with exponential backoff (optional)
    - Rate limiting (optional)
    - Response caching (optional)

    Example:
        >>> async with AsyncHttpClient() as client:
        ...     data = await client.get_json("https://api.example.com/ticker")
    """

    def __init__(
        self,
        timeout: float = DEFAULT_TIMEOUT_SECONDS,
        max_connections: int = DEFAULT_MAX_CONNECTIONS,
        headers: dict[str, str] | None = None,
        retry: RetryMiddleware | None = None,
        rate_limit: RateLimitMiddleware | None = None,
        cache: ResponseCache | None = None,
    ) -> None:
        """Initialize the HTTP client.

        Args:
            timeout: Request timeout in seconds. Must be positive.
            max_connections: Maximum number of concurrent connections. Must be positive.
            headers: Default headers for all requests.
            retry: Retry configuration (optional).
            rate_limit: Rate limiting configuration (optional).
            cache: Response cache configuration (optional).

        Raises:
            ValueError: If timeout or max_connections is not positive.
        """
        if timeout <= 0:
            raise ValueError(f"timeout must be positive, got {timeout}")
        if max_connections <= 0:
            raise ValueError(f"max_connections must be positive, got {max_connections}")

        self.timeout = timeout
        self.max_connections = max_connections
        self.headers = headers
        self.retry = retry
        self.rate_limit = rate_limit
        self.cache = cache
        self._client: httpx.AsyncClient | None = None

    def _get_client(self) -> httpx.AsyncClient:
        """Get or create the underlying httpx client.

        Returns:
            The httpx.AsyncClient instance.
        """
        if self._client is None:
            limits = httpx.Limits(max_connections=self.max_connections)
            self._client = httpx.AsyncClient(
                timeout=httpx.Timeout(self.timeout),
                limits=limits,
                headers=self.headers,
            )
        return self._client

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
        # Apply rate limiting before request
        if self.rate_limit is not None:
            await self.rate_limit.acquire()

        # Check cache for response
        cache_key = self._build_cache_key(url, params)
        if self.cache is not None:
            cached: httpx.Response | None = self.cache.get(cache_key)
            if cached is not None:
                return cached

        # Make request with optional retry
        response = await self._make_request_with_retry(url, headers, params, timeout)

        # Cache successful response
        if self.cache is not None and response.is_success:
            self.cache.set(cache_key, response)

        return response

    async def _make_request_with_retry(
        self,
        url: str,
        headers: dict[str, str] | None,
        params: dict[str, str | int | float | bool] | None,
        timeout: float | None,
    ) -> httpx.Response:
        """Make a request with optional retry logic.

        Args:
            url: The URL to request.
            headers: Additional headers.
            params: Query parameters.
            timeout: Request timeout.

        Returns:
            The httpx.Response object.

        Raises:
            HttpTimeoutError: If the request times out.
            HttpConnectionError: If connection fails.
            HttpStatusError: If the response has an error status code.
        """
        last_exception: HttpStatusError | None = None
        attempt = 0
        max_attempts = 1 + (self.retry.max_retries if self.retry else 0)

        while attempt < max_attempts:
            try:
                response = await self._make_single_request(
                    url, headers, params, timeout
                )
                self._raise_for_status(response, url)
                return response
            except HttpStatusError as e:
                last_exception = e
                if self.retry and self.retry.should_retry(e.status_code, attempt):
                    delay = self.retry.get_delay(attempt)
                    await asyncio.sleep(delay)
                    attempt += 1
                else:
                    raise

        # Should not reach here, but if we do, raise the last exception
        if last_exception:
            raise last_exception
        raise HttpError("Request failed", url)

    async def _make_single_request(
        self,
        url: str,
        headers: dict[str, str] | None,
        params: dict[str, str | int | float | bool] | None,
        timeout: float | None,
    ) -> httpx.Response:
        """Make a single HTTP request.

        Args:
            url: The URL to request.
            headers: Additional headers.
            params: Query parameters.
            timeout: Request timeout.

        Returns:
            The httpx.Response object.

        Raises:
            HttpTimeoutError: If the request times out.
            HttpConnectionError: If connection fails.
            HttpError: For other request errors.
        """
        client = self._get_client()

        request_timeout = (
            httpx.Timeout(timeout)
            if timeout is not None
            else httpx.Timeout(self.timeout)
        )

        try:
            return await client.get(
                url,
                headers=headers,
                params=params,
                timeout=request_timeout,
            )
        except httpx.TimeoutException as e:
            raise HttpTimeoutError(f"Request timed out: {e}", url) from e
        except httpx.ConnectError as e:
            raise HttpConnectionError(f"Connection failed: {e}", url) from e
        except httpx.RequestError as e:
            raise HttpError(f"Request error: {e}", url) from e

    def _build_cache_key(
        self,
        url: str,
        params: dict[str, str | int | float | bool] | None,
    ) -> str:
        """Build a cache key from URL and params.

        Args:
            url: The request URL.
            params: Query parameters.

        Returns:
            A string cache key.
        """
        if params:
            sorted_params = sorted(params.items())
            param_str = "&".join(f"{k}={v}" for k, v in sorted_params)
            return f"{url}?{param_str}"
        return url

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
            HttpError: If the response is not valid JSON.
        """
        response = await self.get(url, headers=headers, params=params, timeout=timeout)
        try:
            result: dict[str, Any] = response.json()
            return result
        except ValueError as e:
            raise HttpError(f"Invalid JSON response: {e}", url=url) from e

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
        response = await self.get(url, headers=headers, params=params, timeout=timeout)
        return response.text

    def _raise_for_status(self, response: httpx.Response, url: str) -> None:
        """Raise an exception if the response indicates an error.

        Args:
            response: The httpx response.
            url: The request URL.

        Raises:
            HttpRateLimitError: If rate limited (429).
            HttpStatusError: If the response has an error status code.
        """
        if response.is_success:
            return

        status_code = response.status_code
        response_body = response.text

        if status_code == 429:
            retry_after = self._parse_retry_after(response)
            raise HttpRateLimitError(
                f"Rate limit exceeded: {status_code}",
                url=url,
                response_body=response_body,
                retry_after=retry_after,
            )

        if response.is_client_error or response.is_server_error:
            raise HttpStatusError(
                f"HTTP error: {status_code}",
                status_code=status_code,
                url=url,
                response_body=response_body,
            )

    def _parse_retry_after(self, response: httpx.Response) -> float | None:
        """Parse the Retry-After header value.

        Only numeric (seconds) format is supported. HTTP-date format is logged
        as a warning and returns None.

        Args:
            response: The httpx response.

        Returns:
            The retry-after value in seconds, or None if not present or
            in unsupported HTTP-date format.
        """
        retry_after = response.headers.get("Retry-After")
        if retry_after is None:
            return None

        try:
            return float(retry_after)
        except ValueError:
            # HTTP-date format (e.g., "Wed, 21 Oct 2015 07:28:00 GMT") is not supported
            logger = logging.getLogger(__name__)
            logger.warning(
                "Could not parse Retry-After header as numeric value: %r "
                "(HTTP-date format is not supported)",
                retry_after,
            )
            return None

    async def close(self) -> None:
        """Close the HTTP client and release resources."""
        if self._client is not None:
            await self._client.aclose()
            self._client = None

    async def __aenter__(self) -> AsyncHttpClient:
        """Enter async context manager."""
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        """Exit async context manager and close client."""
        await self.close()


__all__ = [
    "AsyncHttpClient",
    "DEFAULT_TIMEOUT_SECONDS",
    "DEFAULT_MAX_CONNECTIONS",
]
