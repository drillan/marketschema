"""Unit tests for HTTP exceptions."""

import httpx
import pytest
import respx

from marketschema.exceptions import MarketSchemaError
from marketschema.http import (
    AsyncHttpClient,
    HttpConnectionError,
    HttpError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)


class TestHttpError:
    """Tests for HttpError base exception (T021)."""

    def test_inheritance(self):
        """HttpError should inherit from MarketSchemaError."""
        error = HttpError("test error")
        assert isinstance(error, MarketSchemaError)

    def test_message_attribute(self):
        """HttpError should have message attribute."""
        error = HttpError("test error")
        assert error.message == "test error"

    def test_url_attribute_default(self):
        """HttpError url attribute should default to None."""
        error = HttpError("test error")
        assert error.url is None

    def test_url_attribute_set(self):
        """HttpError url attribute should be settable."""
        error = HttpError("test error", url="https://example.com")
        assert error.url == "https://example.com"

    def test_str_without_url(self):
        """HttpError str should show message without url."""
        error = HttpError("test error")
        assert str(error) == "test error"

    def test_str_with_url(self):
        """HttpError str should show message with url."""
        error = HttpError("test error", url="https://example.com")
        assert str(error) == "test error (url=https://example.com)"


class TestHttpTimeoutError:
    """Tests for HttpTimeoutError (T022)."""

    def test_inheritance(self):
        """HttpTimeoutError should inherit from HttpError."""
        error = HttpTimeoutError("timeout")
        assert isinstance(error, HttpError)
        assert isinstance(error, MarketSchemaError)

    def test_attributes(self):
        """HttpTimeoutError should have correct attributes."""
        error = HttpTimeoutError("timeout", url="https://example.com")
        assert error.message == "timeout"
        assert error.url == "https://example.com"


class TestHttpConnectionError:
    """Tests for HttpConnectionError (T023)."""

    def test_inheritance(self):
        """HttpConnectionError should inherit from HttpError."""
        error = HttpConnectionError("connection failed")
        assert isinstance(error, HttpError)
        assert isinstance(error, MarketSchemaError)

    def test_attributes(self):
        """HttpConnectionError should have correct attributes."""
        error = HttpConnectionError("connection failed", url="https://example.com")
        assert error.message == "connection failed"
        assert error.url == "https://example.com"


class TestHttpStatusError:
    """Tests for HttpStatusError (T024)."""

    def test_inheritance(self):
        """HttpStatusError should inherit from HttpError."""
        error = HttpStatusError("not found", status_code=404)
        assert isinstance(error, HttpError)
        assert isinstance(error, MarketSchemaError)

    def test_status_code_attribute(self):
        """HttpStatusError should have status_code attribute."""
        error = HttpStatusError("not found", status_code=404)
        assert error.status_code == 404

    def test_response_body_attribute(self):
        """HttpStatusError should have response_body attribute."""
        error = HttpStatusError(
            "not found",
            status_code=404,
            response_body='{"error": "not found"}',
        )
        assert error.response_body == '{"error": "not found"}'

    def test_response_body_default(self):
        """HttpStatusError response_body should default to None."""
        error = HttpStatusError("not found", status_code=404)
        assert error.response_body is None

    def test_str_representation(self):
        """HttpStatusError str should include status code."""
        error = HttpStatusError("not found", status_code=404, url="https://example.com")
        assert "404" in str(error)
        assert "https://example.com" in str(error)


class TestHttpRateLimitError:
    """Tests for HttpRateLimitError (T025)."""

    def test_inheritance(self):
        """HttpRateLimitError should inherit from HttpStatusError."""
        error = HttpRateLimitError("rate limited")
        assert isinstance(error, HttpStatusError)
        assert isinstance(error, HttpError)
        assert isinstance(error, MarketSchemaError)

    def test_status_code_is_429(self):
        """HttpRateLimitError status_code should be 429."""
        error = HttpRateLimitError("rate limited")
        assert error.status_code == 429

    def test_retry_after_attribute(self):
        """HttpRateLimitError should have retry_after attribute."""
        error = HttpRateLimitError("rate limited", retry_after=60.0)
        assert error.retry_after == 60.0

    def test_retry_after_default(self):
        """HttpRateLimitError retry_after should default to None."""
        error = HttpRateLimitError("rate limited")
        assert error.retry_after is None

    def test_str_with_retry_after(self):
        """HttpRateLimitError str should include retry_after if present."""
        error = HttpRateLimitError("rate limited", retry_after=60.0)
        assert "60.0" in str(error)


class TestExceptionChaining:
    """Tests for exception chaining with __cause__ (T026)."""

    def test_http_error_preserves_cause(self):
        """HttpError should preserve original exception via __cause__."""
        original = ValueError("original error")
        try:
            try:
                raise original
            except ValueError as e:
                raise HttpError("wrapped error") from e
        except HttpError as e:
            assert e.__cause__ is original

    def test_timeout_error_preserves_cause(self):
        """HttpTimeoutError should preserve original exception."""
        original = TimeoutError("original timeout")
        try:
            try:
                raise original
            except TimeoutError as e:
                raise HttpTimeoutError("wrapped timeout") from e
        except HttpTimeoutError as e:
            assert e.__cause__ is original

    def test_connection_error_preserves_cause(self):
        """HttpConnectionError should preserve original exception."""
        original = ConnectionError("original connection error")
        try:
            try:
                raise original
            except ConnectionError as e:
                raise HttpConnectionError("wrapped connection error") from e
        except HttpConnectionError as e:
            assert e.__cause__ is original


class TestClientRaisesCorrectExceptions:
    """Tests for client raising correct exceptions (T027)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_timeout_error_raised(self):
        """Client should raise HttpTimeoutError on timeout."""
        respx.get("https://api.example.com/data").mock(
            side_effect=httpx.TimeoutException("timeout")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpTimeoutError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.url == "https://api.example.com/data"
        assert exc_info.value.__cause__ is not None

    @pytest.mark.asyncio
    @respx.mock
    async def test_connection_error_raised(self):
        """Client should raise HttpConnectionError on connection failure."""
        respx.get("https://api.example.com/data").mock(
            side_effect=httpx.ConnectError("connection failed")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpConnectionError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.url == "https://api.example.com/data"
        assert exc_info.value.__cause__ is not None

    @pytest.mark.asyncio
    @respx.mock
    async def test_status_error_raised_on_404(self):
        """Client should raise HttpStatusError on 404."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(404, text="Not Found")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpStatusError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 404
        assert exc_info.value.response_body == "Not Found"

    @pytest.mark.asyncio
    @respx.mock
    async def test_status_error_raised_on_500(self):
        """Client should raise HttpStatusError on 500."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(500, text="Internal Server Error")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpStatusError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 500

    @pytest.mark.asyncio
    @respx.mock
    async def test_rate_limit_error_raised_on_429(self):
        """Client should raise HttpRateLimitError on 429."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(
                429,
                text="Too Many Requests",
                headers={"Retry-After": "60"},
            )
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpRateLimitError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 429
        assert exc_info.value.retry_after == 60.0

    @pytest.mark.asyncio
    @respx.mock
    async def test_rate_limit_error_without_retry_after(self):
        """Client should handle 429 without Retry-After header."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(429, text="Too Many Requests")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpRateLimitError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 429
        assert exc_info.value.retry_after is None
