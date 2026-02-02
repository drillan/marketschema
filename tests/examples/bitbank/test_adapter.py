"""Unit tests for BitbankAdapter."""

from typing import Any

from examples.bitbank.adapter import BitbankAdapter
from marketschema.adapters.registry import AdapterRegistry
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
