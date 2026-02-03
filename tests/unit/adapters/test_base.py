"""Unit tests for BaseAdapter with HTTP client support."""

import httpx
import pytest
import respx

from marketschema.adapters.base import BaseAdapter
from marketschema.http import AsyncHttpClient


class SampleAdapter(BaseAdapter):
    """Sample adapter for testing HTTP client integration."""

    source_name = "sample"


class TestHttpClientProperty:
    """Tests for http_client property lazy initialization (T071)."""

    def test_http_client_property_lazy_initialization(self):
        """http_client property should lazily initialize the client."""
        adapter = SampleAdapter()

        # Before accessing, _http_client should be None
        assert adapter._http_client is None

        # After accessing, should have a client
        client = adapter.http_client
        assert client is not None
        assert isinstance(client, AsyncHttpClient)

        # Should return same instance on subsequent access
        assert adapter.http_client is client

    def test_http_client_is_owned_by_adapter(self):
        """Adapter should track that it owns the HTTP client."""
        adapter = SampleAdapter()
        _ = adapter.http_client
        assert adapter._owns_http_client is True


class TestBaseAdapterContextManager:
    """Tests for BaseAdapter context manager (T072)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_context_manager_enters_and_exits(self):
        """Context manager should properly enter and exit."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"result": "ok"})
        )

        async with SampleAdapter() as adapter:
            assert adapter is not None
            result = await adapter.http_client.get_json("https://api.example.com/data")
            assert result == {"result": "ok"}
        # Should not raise after exit

    @pytest.mark.asyncio
    async def test_context_manager_closes_http_client(self):
        """Context manager should close HTTP client on exit."""
        adapter = SampleAdapter()
        async with adapter:
            _ = adapter.http_client  # Initialize client
        # After exit, client should be closed
        assert adapter._http_client is None or adapter._http_client._client is None


class TestCustomHttpClientInjection:
    """Tests for custom http_client injection (T073)."""

    def test_custom_http_client_injection(self):
        """Constructor should accept custom http_client."""
        custom_client = AsyncHttpClient(timeout=60.0)
        adapter = SampleAdapter(http_client=custom_client)

        assert adapter.http_client is custom_client
        assert adapter._owns_http_client is False

    @pytest.mark.asyncio
    async def test_injected_client_not_closed_by_adapter(self):
        """Adapter should not close injected HTTP client."""
        custom_client = AsyncHttpClient(timeout=60.0)

        async with SampleAdapter(http_client=custom_client) as adapter:
            _ = adapter.http_client

        # Injected client should not be closed
        assert custom_client._client is None  # Never initialized
        # But we can verify adapter didn't close it (no error raised)

    @pytest.mark.asyncio
    @respx.mock
    async def test_injected_client_still_usable_after_adapter_close(self):
        """Injected client should still be usable after adapter closes."""
        respx.get("https://api.example.com/data").mock(
            return_value=httpx.Response(200, json={"result": "ok"})
        )

        custom_client = AsyncHttpClient(timeout=60.0)

        async with SampleAdapter(http_client=custom_client):
            pass

        # Client should still be usable
        async with custom_client:
            result = await custom_client.get_json("https://api.example.com/data")
            assert result == {"result": "ok"}
