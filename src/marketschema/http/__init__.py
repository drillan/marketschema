"""HTTP client layer for marketschema adapters.

This module provides an async HTTP client with:
- Connection pooling
- Configurable timeouts
- Clean error handling
- Retry with exponential backoff (Phase 2)
- Rate limiting (Phase 2)
- Response caching (Phase 3)

Example:
    >>> async with AsyncHttpClient() as client:
    ...     data = await client.get_json("https://api.example.com/ticker")
"""

from marketschema.http.cache import ResponseCache
from marketschema.http.client import AsyncHttpClient
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)
from marketschema.http.middleware import RateLimitMiddleware, RetryMiddleware

__all__ = [
    # Client
    "AsyncHttpClient",
    # Exceptions
    "HttpError",
    "HttpTimeoutError",
    "HttpConnectionError",
    "HttpStatusError",
    "HttpRateLimitError",
    # Middleware
    "RetryMiddleware",
    "RateLimitMiddleware",
    # Cache
    "ResponseCache",
]
