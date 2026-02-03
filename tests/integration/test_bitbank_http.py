"""Integration tests for BitbankAdapter HTTP client methods.

These tests use respx to mock HTTP responses and verify that the adapter
correctly fetches and parses data from the bitbank API.
"""

from typing import Any

import httpx
import pytest
import respx

from examples.bitbank.adapter import BitbankAdapter
from marketschema.exceptions import AdapterError
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpStatusError,
    HttpTimeoutError,
)
from marketschema.models import OHLCV, OrderBook, Quote, Trade

BITBANK_API_BASE = "https://public.bitbank.cc"


@pytest.fixture
def bitbank_ticker_api_response() -> dict[str, Any]:
    """Fixture for full bitbank ticker API response including success field."""
    return {
        "success": 1,
        "data": {
            "sell": "9653004",
            "buy": "9651884",
            "high": "9800000",
            "low": "9550000",
            "open": "9620000",
            "last": "9652000",
            "vol": "1234.5678",
            "timestamp": 1738454400000,
        },
    }


@pytest.fixture
def bitbank_transactions_api_response() -> dict[str, Any]:
    """Fixture for full bitbank transactions API response including success field."""
    return {
        "success": 1,
        "data": {
            "transactions": [
                {
                    "transaction_id": 12345678,
                    "side": "buy",
                    "price": "9651884",
                    "amount": "0.1234",
                    "executed_at": 1738454400000,
                },
                {
                    "transaction_id": 12345679,
                    "side": "sell",
                    "price": "9652000",
                    "amount": "0.5",
                    "executed_at": 1738454401000,
                },
            ]
        },
    }


@pytest.fixture
def bitbank_candlestick_api_response() -> dict[str, Any]:
    """Fixture for full bitbank candlestick API response including success field."""
    return {
        "success": 1,
        "data": {
            "candlestick": [
                {
                    "type": "1hour",
                    "ohlcv": [
                        [
                            "9620000",
                            "9680000",
                            "9600000",
                            "9650000",
                            "123.456",
                            1738454400000,
                        ],
                        [
                            "9650000",
                            "9700000",
                            "9640000",
                            "9680000",
                            "234.567",
                            1738458000000,
                        ],
                    ],
                }
            ]
        },
    }


@pytest.fixture
def bitbank_depth_api_response() -> dict[str, Any]:
    """Fixture for full bitbank depth API response including success field."""
    return {
        "success": 1,
        "data": {
            "asks": [
                ["9653004", "0.5"],
                ["9653010", "1.0"],
                ["9653100", "2.5"],
            ],
            "bids": [
                ["9651884", "0.3"],
                ["9651800", "1.2"],
                ["9651700", "3.0"],
            ],
            "timestamp": 1738454400000,
        },
    }


class TestFetchTicker:
    """Test fetch_ticker method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_ticker_success(
        self, bitbank_ticker_api_response: dict[str, Any]
    ) -> None:
        """fetch_ticker returns Quote on success."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            return_value=httpx.Response(200, json=bitbank_ticker_api_response)
        )

        async with BitbankAdapter() as adapter:
            quote = await adapter.fetch_ticker("btc_jpy")

        assert isinstance(quote, Quote)
        assert quote.symbol.root == "btc_jpy"
        assert quote.bid.root == 9651884.0
        assert quote.ask.root == 9653004.0


class TestFetchTransactions:
    """Test fetch_transactions method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_transactions_success(
        self, bitbank_transactions_api_response: dict[str, Any]
    ) -> None:
        """fetch_transactions returns list of Trade on success."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/transactions").mock(
            return_value=httpx.Response(200, json=bitbank_transactions_api_response)
        )

        async with BitbankAdapter() as adapter:
            trades = await adapter.fetch_transactions("btc_jpy")

        assert isinstance(trades, list)
        assert len(trades) == 2
        assert all(isinstance(t, Trade) for t in trades)
        assert trades[0].price.root == 9651884.0


class TestFetchCandlestick:
    """Test fetch_candlestick method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_candlestick_success(
        self, bitbank_candlestick_api_response: dict[str, Any]
    ) -> None:
        """fetch_candlestick returns list of OHLCV on success."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/candlestick/1hour/20250202").mock(
            return_value=httpx.Response(200, json=bitbank_candlestick_api_response)
        )

        async with BitbankAdapter() as adapter:
            ohlcvs = await adapter.fetch_candlestick("btc_jpy", "1hour", "20250202")

        assert isinstance(ohlcvs, list)
        assert len(ohlcvs) == 2
        assert all(isinstance(o, OHLCV) for o in ohlcvs)
        assert ohlcvs[0].open.root == 9620000.0

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_candlestick_empty_data(self) -> None:
        """fetch_candlestick returns empty list when no candlestick data."""
        empty_response = {
            "success": 1,
            "data": {"candlestick": []},
        }
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/candlestick/1hour/20250202").mock(
            return_value=httpx.Response(200, json=empty_response)
        )

        async with BitbankAdapter() as adapter:
            ohlcvs = await adapter.fetch_candlestick("btc_jpy", "1hour", "20250202")

        assert ohlcvs == []


class TestFetchDepth:
    """Test fetch_depth method."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_depth_success(
        self, bitbank_depth_api_response: dict[str, Any]
    ) -> None:
        """fetch_depth returns OrderBook on success."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/depth").mock(
            return_value=httpx.Response(200, json=bitbank_depth_api_response)
        )

        async with BitbankAdapter() as adapter:
            orderbook = await adapter.fetch_depth("btc_jpy")

        assert isinstance(orderbook, OrderBook)
        assert orderbook.symbol.root == "btc_jpy"
        assert len(orderbook.asks) == 3
        assert len(orderbook.bids) == 3
        assert orderbook.asks[0].price.root == 9653004.0


class TestFetchErrorHandling:
    """Test error handling in fetch methods."""

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_api_error_raises_adapter_error(self) -> None:
        """fetch_ticker raises AdapterError when API returns success != 1."""
        error_response = {
            "success": 0,
            "data": {"code": 10000, "message": "URL not found"},
        }
        respx.get(f"{BITBANK_API_BASE}/invalid_pair/ticker").mock(
            return_value=httpx.Response(200, json=error_response)
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(AdapterError, match="bitbank API error"):
                await adapter.fetch_ticker("invalid_pair")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_http_error_propagates(self) -> None:
        """fetch_ticker propagates HttpStatusError on HTTP error."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            return_value=httpx.Response(500, text="Internal Server Error")
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(HttpStatusError):
                await adapter.fetch_ticker("btc_jpy")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_http_404_error_propagates(self) -> None:
        """fetch_ticker propagates HttpStatusError on 404."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            return_value=httpx.Response(404, text="Not Found")
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(HttpStatusError):
                await adapter.fetch_ticker("btc_jpy")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_timeout_error_propagates(self) -> None:
        """fetch_ticker propagates HttpTimeoutError on timeout."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            side_effect=httpx.TimeoutException("Connection timeout")
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(HttpTimeoutError):
                await adapter.fetch_ticker("btc_jpy")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_connection_error_propagates(self) -> None:
        """fetch_ticker propagates HttpConnectionError on connection failure."""
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            side_effect=httpx.ConnectError("Connection refused")
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(HttpConnectionError):
                await adapter.fetch_ticker("btc_jpy")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_missing_success_field_raises_adapter_error(self) -> None:
        """fetch_ticker raises AdapterError when success field is missing."""
        response_without_success = {
            "data": {"sell": "9653004", "buy": "9651884", "timestamp": 1738454400000}
        }
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            return_value=httpx.Response(200, json=response_without_success)
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(AdapterError, match="bitbank API error"):
                await adapter.fetch_ticker("btc_jpy")

    @pytest.mark.asyncio
    @respx.mock
    async def test_fetch_with_missing_data_field_raises_adapter_error(self) -> None:
        """fetch_ticker raises AdapterError when data field is missing."""
        response_without_data = {"success": 1}
        respx.get(f"{BITBANK_API_BASE}/btc_jpy/ticker").mock(
            return_value=httpx.Response(200, json=response_without_data)
        )

        async with BitbankAdapter() as adapter:
            with pytest.raises(AdapterError, match="Missing required field"):
                await adapter.fetch_ticker("btc_jpy")
