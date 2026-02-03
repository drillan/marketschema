"""Unit tests for AsyncHttpClient."""

import httpx
import pytest
import respx

from marketschema.http import AsyncHttpClient


class TestAsyncHttpClientConstructor:
    """Tests for AsyncHttpClient constructor (T008)."""

    def test_default_parameters(self):
        """Constructor should accept default parameters."""
        client = AsyncHttpClient()
        assert client.timeout == 30.0
        assert client.max_connections == 100
        assert client.headers is None

    def test_invalid_timeout_raises_error(self):
        """Constructor should raise ValueError for non-positive timeout."""
        import pytest

        with pytest.raises(ValueError, match="timeout must be positive"):
            AsyncHttpClient(timeout=0)

        with pytest.raises(ValueError, match="timeout must be positive"):
            AsyncHttpClient(timeout=-1)

    def test_invalid_max_connections_raises_error(self):
        """Constructor should raise ValueError for non-positive max_connections."""
        import pytest

        with pytest.raises(ValueError, match="max_connections must be positive"):
            AsyncHttpClient(max_connections=0)

        with pytest.raises(ValueError, match="max_connections must be positive"):
            AsyncHttpClient(max_connections=-1)

    def test_custom_timeout(self):
        """Constructor should accept custom timeout."""
        client = AsyncHttpClient(timeout=60.0)
        assert client.timeout == 60.0

    def test_custom_max_connections(self):
        """Constructor should accept custom max_connections."""
        client = AsyncHttpClient(max_connections=50)
        assert client.max_connections == 50

    def test_custom_headers(self):
        """Constructor should accept custom headers."""
        headers = {"User-Agent": "MyApp/1.0"}
        client = AsyncHttpClient(headers=headers)
        assert client.headers == headers

    def test_all_custom_parameters(self):
        """Constructor should accept all custom parameters."""
        headers = {"Accept": "application/json"}
        client = AsyncHttpClient(
            timeout=45.0,
            max_connections=200,
            headers=headers,
        )
        assert client.timeout == 45.0
        assert client.max_connections == 200
        assert client.headers == headers


class TestAsyncHttpClientGetJson:
    """Tests for get_json() method (T009)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_json_success(self):
        """get_json() should return parsed JSON response."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"key": "value"})
        )

        async with AsyncHttpClient() as client:
            result = await client.get_json("https://api.example.com/data")

        assert result == {"key": "value"}

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_json_invalid_json_raises_http_error(self):
        """get_json() should raise HttpError for invalid JSON with URL context."""
        from marketschema.http import HttpError

        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, text="not valid json")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.url == "https://api.example.com/data"
        assert "Invalid JSON response" in str(exc_info.value)
        assert exc_info.value.__cause__ is not None

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_json_with_params(self):
        """get_json() should pass query parameters."""
        route = respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"result": "ok"})
        )

        async with AsyncHttpClient() as client:
            result = await client.get_json(
                "https://api.example.com/data",
                params={"symbol": "BTC-USD", "limit": 10},
            )

        assert result == {"result": "ok"}
        assert route.called
        request = route.calls[0].request
        assert "symbol=BTC-USD" in str(request.url)
        assert "limit=10" in str(request.url)

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_json_with_headers(self):
        """get_json() should merge custom headers."""
        route = respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={})
        )

        async with AsyncHttpClient(headers={"Default": "Header"}) as client:
            await client.get_json(
                "https://api.example.com/data",
                headers={"Custom": "Header"},
            )

        assert route.called
        request = route.calls[0].request
        assert request.headers.get("Default") == "Header"
        assert request.headers.get("Custom") == "Header"


class TestAsyncHttpClientGetText:
    """Tests for get_text() method (T010)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_text_success(self):
        """get_text() should return text response."""
        respx.get("https://example.com/page").mock(
            return_value=httpx.Response(200, text="<html>Hello</html>")
        )

        async with AsyncHttpClient() as client:
            result = await client.get_text("https://example.com/page")

        assert result == "<html>Hello</html>"

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_text_with_params(self):
        """get_text() should pass query parameters."""
        route = respx.get("https://example.com/page").mock(
            return_value=httpx.Response(200, text="result")
        )

        async with AsyncHttpClient() as client:
            result = await client.get_text(
                "https://example.com/page",
                params={"page": 1},
            )

        assert result == "result"
        assert route.called


class TestAsyncHttpClientGet:
    """Tests for get() method (T011)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_returns_response(self):
        """get() should return raw httpx.Response."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"key": "value"})
        )

        async with AsyncHttpClient() as client:
            response = await client.get("https://api.example.com/data")

        assert isinstance(response, httpx.Response)
        assert response.status_code == 200

    @pytest.mark.asyncio
    @respx.mock
    async def test_get_with_custom_timeout(self):
        """get() should accept custom timeout per request."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={})
        )

        async with AsyncHttpClient(timeout=30.0) as client:
            # Should not raise even with very short timeout mock
            response = await client.get(
                "https://api.example.com/data",
                timeout=60.0,
            )

        assert response.status_code == 200


class TestAsyncHttpClientContextManager:
    """Tests for context manager (T012)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_context_manager_enters_and_exits(self):
        """Context manager should properly enter and exit."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={})
        )

        async with AsyncHttpClient() as client:
            assert client is not None
            await client.get_json("https://api.example.com/data")
        # Should not raise after exit

    @pytest.mark.asyncio
    async def test_context_manager_closes_client(self):
        """Context manager should close client on exit."""
        client = AsyncHttpClient()
        async with client:
            pass
        # After exiting, internal client should be closed
        assert client._client is None or client._client.is_closed

    @pytest.mark.asyncio
    @respx.mock
    async def test_manual_close(self):
        """close() should properly close the client."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={})
        )

        client = AsyncHttpClient()
        await client.get_json("https://api.example.com/data")
        await client.close()

        assert client._client is None or client._client.is_closed
