"""Unit tests for BitbankAdapter."""

from typing import Any

import pytest

from examples.bitbank.adapter import BitbankAdapter
from marketschema.adapters.registry import AdapterRegistry
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV, PriceLevel, Side, Trade


class TestBitbankAdapterInit:
    """Test BitbankAdapter initialization."""

    def test_source_name(self) -> None:
        """BitbankAdapter has correct source_name."""
        adapter = BitbankAdapter()
        assert adapter.source_name == "bitbank"


class TestParseQuote:
    """Test Quote parsing from ticker data."""

    def test_parse_quote_from_ticker(
        self, bitbank_ticker_response: dict[str, Any]
    ) -> None:
        """Parse bitbank ticker to Quote model."""
        adapter = BitbankAdapter()

        quote = adapter.parse_quote(bitbank_ticker_response, symbol="btc_jpy")

        assert quote.symbol.root == "btc_jpy"
        assert quote.bid.root == 9651884.0
        assert quote.ask.root == 9653004.0
        assert quote.timestamp.root.isoformat() == "2025-02-02T00:00:00+00:00"

    def test_parse_quote_converts_string_prices(self) -> None:
        """Quote parsing converts string prices to float."""
        adapter = BitbankAdapter()
        ticker = {
            "sell": "100.50",
            "buy": "99.50",
            "timestamp": 1738454400000,
        }

        quote = adapter.parse_quote(ticker, symbol="xrp_jpy")

        assert quote.bid.root == 99.50
        assert quote.ask.root == 100.50

    def test_parse_quote_with_invalid_price_raises_adapter_error(self) -> None:
        """Quote parsing with invalid price raises AdapterError."""
        adapter = BitbankAdapter()
        invalid_ticker = {
            "sell": "not_a_number",
            "buy": "9651884",
            "timestamp": 1738454400000,
        }

        with pytest.raises(AdapterError):
            adapter.parse_quote(invalid_ticker, symbol="btc_jpy")

    def test_parse_quote_with_missing_field_raises_adapter_error(self) -> None:
        """Quote parsing with missing field raises AdapterError."""
        adapter = BitbankAdapter()
        incomplete_ticker = {
            "sell": "9653004",
            # "buy" is missing
            "timestamp": 1738454400000,
        }

        with pytest.raises(AdapterError):
            adapter.parse_quote(incomplete_ticker, symbol="btc_jpy")


class TestParseTrade:
    """Test Trade parsing from transaction data."""

    def test_parse_trade_from_transaction(
        self, bitbank_transactions_response: dict[str, Any]
    ) -> None:
        """Parse bitbank transaction to Trade model."""
        adapter = BitbankAdapter()
        transaction = bitbank_transactions_response["transactions"][0]

        trade = adapter.parse_trade(transaction, symbol="btc_jpy")

        assert trade.symbol.root == "btc_jpy"
        assert trade.price.root == 9651884.0
        assert trade.size.root == 0.1234
        assert trade.side == Side.buy
        assert trade.timestamp.root.isoformat() == "2025-02-02T00:00:00+00:00"

    def test_parse_trade_sell_side(
        self, bitbank_transactions_response: dict[str, Any]
    ) -> None:
        """Parse sell-side transaction correctly."""
        adapter = BitbankAdapter()
        transaction = bitbank_transactions_response["transactions"][1]

        trade = adapter.parse_trade(transaction, symbol="btc_jpy")

        assert trade.side == Side.sell

    def test_parse_trades_batch(
        self, bitbank_transactions_response: dict[str, Any]
    ) -> None:
        """Parse multiple transactions at once."""
        adapter = BitbankAdapter()
        transactions = bitbank_transactions_response["transactions"]

        trades = adapter.parse_trades(transactions, symbol="btc_jpy")

        assert len(trades) == 2
        assert all(isinstance(t, Trade) for t in trades)


class TestParseOHLCV:
    """Test OHLCV parsing from candlestick data."""

    def test_parse_ohlcv_from_candlestick(
        self, bitbank_candlestick_response: dict[str, Any]
    ) -> None:
        """Parse bitbank candlestick array to OHLCV model."""
        adapter = BitbankAdapter()
        # bitbank candlestick format: [open, high, low, close, volume, timestamp]
        ohlcv_array = bitbank_candlestick_response["candlestick"][0]["ohlcv"][0]

        ohlcv = adapter.parse_ohlcv(ohlcv_array, symbol="btc_jpy")

        assert ohlcv.symbol.root == "btc_jpy"
        assert ohlcv.open.root == 9620000.0
        assert ohlcv.high.root == 9680000.0
        assert ohlcv.low.root == 9600000.0
        assert ohlcv.close.root == 9650000.0
        assert ohlcv.volume.root == 123.456
        assert ohlcv.timestamp.root.isoformat() == "2025-02-02T00:00:00+00:00"

    def test_parse_ohlcv_batch(
        self, bitbank_candlestick_response: dict[str, Any]
    ) -> None:
        """Parse multiple candlesticks at once."""
        adapter = BitbankAdapter()
        ohlcv_arrays = bitbank_candlestick_response["candlestick"][0]["ohlcv"]

        ohlcvs = adapter.parse_ohlcv_batch(ohlcv_arrays, symbol="btc_jpy")

        assert len(ohlcvs) == 2
        assert all(isinstance(o, OHLCV) for o in ohlcvs)

    def test_parse_ohlcv_with_insufficient_array_length_raises_error(self) -> None:
        """OHLCV parsing with insufficient array length raises IndexError."""
        adapter = BitbankAdapter()
        # Only 3 elements, but 6 are required [open, high, low, close, volume, timestamp]
        short_array = ["9620000", "9680000", "9600000"]

        with pytest.raises(IndexError):
            adapter.parse_ohlcv(short_array, symbol="btc_jpy")

    def test_parse_ohlcv_with_invalid_value_raises_adapter_error(self) -> None:
        """OHLCV parsing with invalid value raises AdapterError."""
        adapter = BitbankAdapter()
        invalid_array = [
            "not_a_number",
            "9680000",
            "9600000",
            "9650000",
            "123.456",
            1738454400000,
        ]

        with pytest.raises(AdapterError):
            adapter.parse_ohlcv(invalid_array, symbol="btc_jpy")


class TestParseOrderBook:
    """Test OrderBook parsing from depth data."""

    def test_parse_orderbook_from_depth(
        self, bitbank_depth_response: dict[str, Any]
    ) -> None:
        """Parse bitbank depth to OrderBook model."""
        adapter = BitbankAdapter()

        orderbook = adapter.parse_orderbook(bitbank_depth_response, symbol="btc_jpy")

        assert orderbook.symbol.root == "btc_jpy"
        assert orderbook.timestamp.root.isoformat() == "2025-02-02T00:00:00+00:00"

        # asks should be in ascending price order
        assert len(orderbook.asks) == 3
        assert orderbook.asks[0].price.root == 9653004.0
        assert orderbook.asks[0].size.root == 0.5

        # bids should be in descending price order
        assert len(orderbook.bids) == 3
        assert orderbook.bids[0].price.root == 9651884.0
        assert orderbook.bids[0].size.root == 0.3

    def test_parse_orderbook_price_levels(
        self, bitbank_depth_response: dict[str, Any]
    ) -> None:
        """OrderBook price levels are PriceLevel instances."""
        adapter = BitbankAdapter()

        orderbook = adapter.parse_orderbook(bitbank_depth_response, symbol="btc_jpy")

        assert all(isinstance(level, PriceLevel) for level in orderbook.asks)
        assert all(isinstance(level, PriceLevel) for level in orderbook.bids)

    def test_parse_orderbook_with_empty_asks_and_bids(self) -> None:
        """OrderBook parsing handles empty asks and bids arrays."""
        adapter = BitbankAdapter()
        empty_depth = {"asks": [], "bids": [], "timestamp": 1738454400000}

        orderbook = adapter.parse_orderbook(empty_depth, symbol="btc_jpy")

        assert orderbook.asks == []
        assert orderbook.bids == []

    def test_parse_orderbook_with_missing_field_raises_key_error(self) -> None:
        """OrderBook parsing with missing field raises KeyError."""
        adapter = BitbankAdapter()
        incomplete_depth = {
            "asks": [["9653004", "0.5"]],
            # "bids" is missing
            "timestamp": 1738454400000,
        }

        with pytest.raises(KeyError):
            adapter.parse_orderbook(incomplete_depth, symbol="btc_jpy")


class TestAdapterRegistry:
    """Test adapter registration."""

    def test_adapter_is_registered(self) -> None:
        """BitbankAdapter is registered in AdapterRegistry."""
        # Import triggers registration via @register decorator
        from examples.bitbank.adapter import BitbankAdapter  # noqa: F401

        assert AdapterRegistry.is_registered("bitbank")

    def test_adapter_can_be_retrieved(self) -> None:
        """BitbankAdapter can be retrieved from registry."""
        from examples.bitbank.adapter import BitbankAdapter  # noqa: F401

        adapter = AdapterRegistry.get("bitbank")
        assert isinstance(adapter, BitbankAdapter)
