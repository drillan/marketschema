"""bitbank Public API adapter for marketschema.

This adapter transforms data from bitbank's Public API into marketschema models.

API Documentation:
    https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md
"""

from datetime import datetime
from typing import Any

from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.models import (
    OHLCV,
    OrderBook,
    Price,
    PriceLevel,
    Quote,
    Size,
    Symbol,
    Timestamp,
    Trade,
)

# Index constants for bitbank candlestick array [open, high, low, close, volume, timestamp]
OHLCV_INDEX_OPEN = 0
OHLCV_INDEX_HIGH = 1
OHLCV_INDEX_LOW = 2
OHLCV_INDEX_CLOSE = 3
OHLCV_INDEX_VOLUME = 4
OHLCV_INDEX_TIMESTAMP = 5

# Index constants for bitbank price level array [price, size]
PRICE_LEVEL_INDEX_PRICE = 0
PRICE_LEVEL_INDEX_SIZE = 1


@register
class BitbankAdapter(BaseAdapter):
    """Adapter for bitbank Public API.

    Transforms data from bitbank's Public API endpoints into standardized
    marketschema models.

    Supported endpoints:
        - Ticker: GET /{pair}/ticker → Quote
        - Transactions: GET /{pair}/transactions → Trade
        - Candlestick: GET /{pair}/candlestick/{type}/{yyyymmdd} → OHLCV
        - Depth: GET /{pair}/depth → OrderBook

    Note:
        bitbank API responses do not include symbol information.
        Symbol must be provided as a parameter to parse methods.
    """

    source_name = "bitbank"

    def get_quote_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Quote model.

        bitbank ticker format:
            - sell: Ask price (string)
            - buy: Bid price (string)
            - timestamp: Unix timestamp in milliseconds
        """
        return [
            ModelMapping("bid", "buy", transform=self.transforms.to_float),
            ModelMapping("ask", "sell", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.unix_timestamp_ms
            ),
        ]

    def get_trade_mapping(self) -> list[ModelMapping]:
        """Return field mappings for Trade model.

        bitbank transaction format:
            - price: Trade price (string)
            - amount: Trade size (string)
            - side: "buy" or "sell"
            - executed_at: Unix timestamp in milliseconds
        """
        return [
            ModelMapping("price", "price", transform=self.transforms.to_float),
            ModelMapping("size", "amount", transform=self.transforms.to_float),
            ModelMapping("side", "side", transform=self.transforms.side_from_string),
            ModelMapping(
                "timestamp",
                "executed_at",
                transform=self.transforms.unix_timestamp_ms,
            ),
        ]

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OHLCV model.

        bitbank candlestick is converted to dict before mapping.
        """
        return [
            ModelMapping("open", "open", transform=self.transforms.to_float),
            ModelMapping("high", "high", transform=self.transforms.to_float),
            ModelMapping("low", "low", transform=self.transforms.to_float),
            ModelMapping("close", "close", transform=self.transforms.to_float),
            ModelMapping("volume", "volume", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.unix_timestamp_ms
            ),
        ]

    def parse_quote(self, raw_data: dict[str, Any], *, symbol: str) -> Quote:
        """Parse bitbank ticker data into Quote model.

        Args:
            raw_data: Ticker response from bitbank API (data field content)
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            Quote model instance
        """
        # Add symbol to data for mapping
        data_with_symbol = {**raw_data, "symbol": symbol}
        mappings = self.get_quote_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(data_with_symbol, mappings, Quote)

    def parse_trade(self, raw_data: dict[str, Any], *, symbol: str) -> Trade:
        """Parse bitbank transaction data into Trade model.

        Args:
            raw_data: Single transaction from transactions response
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            Trade model instance
        """
        data_with_symbol = {**raw_data, "symbol": symbol}
        mappings = self.get_trade_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(data_with_symbol, mappings, Trade)

    def parse_trades(
        self, transactions: list[dict[str, Any]], *, symbol: str
    ) -> list[Trade]:
        """Parse multiple bitbank transactions into Trade models.

        Args:
            transactions: List of transactions from transactions response
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            List of Trade model instances
        """
        return [self.parse_trade(tx, symbol=symbol) for tx in transactions]

    def parse_ohlcv(self, raw_data: list[Any], *, symbol: str) -> OHLCV:
        """Parse bitbank candlestick array into OHLCV model.

        Args:
            raw_data: Candlestick array [open, high, low, close, volume, timestamp]
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            OHLCV model instance
        """
        # Convert array to dict for mapping
        ohlcv_dict = {
            "symbol": symbol,
            "open": raw_data[OHLCV_INDEX_OPEN],
            "high": raw_data[OHLCV_INDEX_HIGH],
            "low": raw_data[OHLCV_INDEX_LOW],
            "close": raw_data[OHLCV_INDEX_CLOSE],
            "volume": raw_data[OHLCV_INDEX_VOLUME],
            "timestamp": raw_data[OHLCV_INDEX_TIMESTAMP],
        }
        mappings = self.get_ohlcv_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(ohlcv_dict, mappings, OHLCV)

    def parse_ohlcv_batch(
        self, ohlcv_arrays: list[list[Any]], *, symbol: str
    ) -> list[OHLCV]:
        """Parse multiple bitbank candlestick arrays into OHLCV models.

        Args:
            ohlcv_arrays: List of candlestick arrays
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            List of OHLCV model instances
        """
        return [self.parse_ohlcv(arr, symbol=symbol) for arr in ohlcv_arrays]

    def parse_orderbook(self, raw_data: dict[str, Any], *, symbol: str) -> OrderBook:
        """Parse bitbank depth data into OrderBook model.

        Args:
            raw_data: Depth response from bitbank API (data field content)
            symbol: Trading pair symbol (e.g., "btc_jpy")

        Returns:
            OrderBook model instance
        """
        # Convert price level arrays to PriceLevel objects
        asks = [
            PriceLevel(
                price=Price(self.transforms.to_float(level[PRICE_LEVEL_INDEX_PRICE])),
                size=Size(self.transforms.to_float(level[PRICE_LEVEL_INDEX_SIZE])),
            )
            for level in raw_data["asks"]
        ]
        bids = [
            PriceLevel(
                price=Price(self.transforms.to_float(level[PRICE_LEVEL_INDEX_PRICE])),
                size=Size(self.transforms.to_float(level[PRICE_LEVEL_INDEX_SIZE])),
            )
            for level in raw_data["bids"]
        ]

        timestamp_iso = self.transforms.unix_timestamp_ms(raw_data["timestamp"])
        timestamp_dt = datetime.fromisoformat(timestamp_iso.replace("Z", "+00:00"))

        return OrderBook(
            symbol=Symbol(symbol),
            timestamp=Timestamp(timestamp_dt),
            asks=asks,
            bids=bids,
        )


__all__ = ["BitbankAdapter"]
