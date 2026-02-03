"""HTTP-specific exceptions for marketschema.

All HTTP errors preserve the original exception via __cause__ for debugging.
"""

from marketschema.exceptions import MarketSchemaError


class HttpError(MarketSchemaError):
    """Base for all HTTP errors.

    Attributes:
        message: Error description.
        url: The URL that caused the error (if available).
    """

    def __init__(self, message: str, url: str | None = None) -> None:
        """Initialize the HTTP error.

        Args:
            message: Error description.
            url: The URL that caused the error.
        """
        super().__init__(message)
        self.message = message
        self.url = url

    def __str__(self) -> str:
        if self.url:
            return f"{self.message} (url={self.url})"
        return self.message


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
    ) -> None:
        """Initialize the HTTP status error.

        Args:
            message: Error description.
            status_code: The HTTP status code.
            url: The URL that caused the error.
            response_body: The response body.
        """
        super().__init__(message, url)
        self.status_code = status_code
        self.response_body = response_body

    def __str__(self) -> str:
        base = f"{self.message} (status_code={self.status_code})"
        if self.url:
            base += f" (url={self.url})"
        return base


HTTP_STATUS_RATE_LIMIT = 429


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
    ) -> None:
        """Initialize the rate limit error.

        Args:
            message: Error description.
            url: The URL that caused the error.
            response_body: The response body.
            retry_after: Seconds to wait before retrying.
        """
        super().__init__(message, HTTP_STATUS_RATE_LIMIT, url, response_body)
        self.retry_after = retry_after

    def __str__(self) -> str:
        base = super().__str__()
        if self.retry_after is not None:
            base += f" (retry_after={self.retry_after})"
        return base


__all__ = [
    "HttpError",
    "HttpTimeoutError",
    "HttpConnectionError",
    "HttpStatusError",
    "HttpRateLimitError",
    "HTTP_STATUS_RATE_LIMIT",
]
