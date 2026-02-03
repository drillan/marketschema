"""Integration tests for BaseAdapter with HTTP client (T074)."""

import httpx
import pytest
import respx

from marketschema.adapters.base import BaseAdapter
from marketschema.http import AsyncHttpClient


class ExchangeAdapter(BaseAdapter):
    """Sample exchange adapter for integration testing."""

    source_name = "exchange"

    async def fetch_ticker(self, symbol: str) -> dict:
        """Fetch ticker data from the exchange API."""
        url = f"https://api.exchange.com/ticker/{symbol}"
        return await self.http_client.get_json(url)


class TestBaseAdapterHttpIntegration:
    """Integration tests for BaseAdapter with HTTP client (T074)."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_adapter_can_fetch_data(self):
        """Adapter should be able to fetch data using http_client."""
        respx.get("https://api.exchange.com/ticker/BTC-USD").mock(
            return_value=httpx.Response(
                200,
                json={
                    "symbol": "BTC-USD",
                    "bid": 50000.0,
                    "ask": 50001.0,
                },
            )
        )

        async with ExchangeAdapter() as adapter:
            result = await adapter.fetch_ticker("BTC-USD")

        assert result["symbol"] == "BTC-USD"
        assert result["bid"] == 50000.0
        assert result["ask"] == 50001.0

    @pytest.mark.asyncio
    @respx.mock
    async def test_adapter_with_custom_client(self):
        """Adapter should work with custom HTTP client."""
        respx.get("https://api.exchange.com/ticker/ETH-USD").mock(
            return_value=httpx.Response(
                200,
                json={
                    "symbol": "ETH-USD",
                    "bid": 3000.0,
                    "ask": 3001.0,
                },
            )
        )

        custom_client = AsyncHttpClient(timeout=60.0)
        async with ExchangeAdapter(http_client=custom_client) as adapter:
            result = await adapter.fetch_ticker("ETH-USD")

        assert result["symbol"] == "ETH-USD"

        # Custom client should still be usable
        async with custom_client:
            pass  # Should not raise

    @pytest.mark.asyncio
    @respx.mock
    async def test_adapter_error_handling(self):
        """Adapter should properly propagate HTTP errors."""
        respx.get("https://api.exchange.com/ticker/INVALID").mock(
            return_value=httpx.Response(404, text="Not Found")
        )

        from marketschema.http import HttpStatusError

        async with ExchangeAdapter() as adapter:
            with pytest.raises(HttpStatusError) as exc_info:
                await adapter.fetch_ticker("INVALID")

        assert exc_info.value.status_code == 404

    @pytest.mark.asyncio
    @respx.mock
    async def test_multiple_requests_reuse_client(self):
        """Multiple requests should reuse the same HTTP client."""
        respx.get("https://api.exchange.com/ticker/BTC-USD").mock(
            return_value=httpx.Response(200, json={"symbol": "BTC-USD"})
        )
        respx.get("https://api.exchange.com/ticker/ETH-USD").mock(
            return_value=httpx.Response(200, json={"symbol": "ETH-USD"})
        )

        async with ExchangeAdapter() as adapter:
            # Store reference to client
            client1 = adapter.http_client

            await adapter.fetch_ticker("BTC-USD")
            await adapter.fetch_ticker("ETH-USD")

            # Should be the same client instance
            assert adapter.http_client is client1
