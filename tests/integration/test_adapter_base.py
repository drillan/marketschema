"""Integration tests for BaseAdapter."""

from typing import Any

import pytest

from marketschema.adapters import BaseAdapter, ModelMapping, Transforms
from marketschema.exceptions import AdapterError
from marketschema.models import (
    Quote,
    Side,
    Trade,
)


class SampleExchangeAdapter(BaseAdapter):
    """Sample adapter for testing BaseAdapter functionality."""

    source_name = "sample_exchange"

    def get_quote_mapping(self) -> list[ModelMapping]:
        """Map sample exchange quote format to Quote model."""
        return [
            ModelMapping("symbol", "ticker"),
            ModelMapping(
                "timestamp",
                "time",
                transform=self.transforms.unix_timestamp_ms,
            ),
            ModelMapping("bid", "best_bid.price", transform=self.transforms.to_float),
            ModelMapping("ask", "best_ask.price", transform=self.transforms.to_float),
            ModelMapping(
                "bid_size", "best_bid.size", transform=self.transforms.to_float
            ),
            ModelMapping(
                "ask_size", "best_ask.size", transform=self.transforms.to_float
            ),
        ]

    def get_trade_mapping(self) -> list[ModelMapping]:
        """Map sample exchange trade format to Trade model."""
        return [
            ModelMapping("symbol", "s"),
            ModelMapping("timestamp", "T", transform=self.transforms.unix_timestamp_ms),
            ModelMapping("price", "p", transform=self.transforms.to_float),
            ModelMapping("size", "q", transform=self.transforms.to_float),
            ModelMapping("side", "m", transform=self._map_side),
        ]

    def parse_quote(self, raw_data: dict[str, Any]) -> Quote:
        """Parse raw quote data into Quote model."""
        return self._apply_mapping(raw_data, self.get_quote_mapping(), Quote)

    def parse_trade(self, raw_data: dict[str, Any]) -> Trade:
        """Parse raw trade data into Trade model."""
        return self._apply_mapping(raw_data, self.get_trade_mapping(), Trade)

    @staticmethod
    def _map_side(is_maker: bool) -> str:
        """Map maker flag to side (maker=sell, taker=buy)."""
        return "sell" if is_maker else "buy"


class TestBaseAdapterInit:
    """Test BaseAdapter initialization."""

    def test_adapter_requires_source_name(self) -> None:
        """Adapter without source_name raises AdapterError."""

        class NoSourceAdapter(BaseAdapter):
            pass

        with pytest.raises(AdapterError, match="must define source_name"):
            NoSourceAdapter()

    def test_adapter_with_source_name_initializes(self) -> None:
        """Adapter with source_name initializes successfully."""
        adapter = SampleExchangeAdapter()
        assert adapter.source_name == "sample_exchange"


class TestApplyMapping:
    """Test _apply_mapping functionality."""

    def test_parse_quote_with_nested_fields(self) -> None:
        """Adapter parses nested fields correctly."""
        adapter = SampleExchangeAdapter()

        raw_data = {
            "ticker": "BTC/USDT",
            "time": 1769990400000,  # 2026-02-02T00:00:00Z
            "best_bid": {"price": "50000.00", "size": "1.5"},
            "best_ask": {"price": "50001.00", "size": "2.0"},
        }

        quote = adapter.parse_quote(raw_data)

        assert quote.symbol.root == "BTC/USDT"
        assert quote.bid.root == 50000.00
        assert quote.ask.root == 50001.00
        assert quote.bid_size is not None
        assert quote.bid_size.root == 1.5
        assert quote.ask_size is not None
        assert quote.ask_size.root == 2.0

    def test_parse_trade_with_transforms(self) -> None:
        """Adapter applies transforms correctly."""
        adapter = SampleExchangeAdapter()

        raw_data = {
            "s": "ETH/USDT",
            "T": 1769990400000,  # 2026-02-02T00:00:00Z
            "p": "3000.50",
            "q": "10.0",
            "m": False,  # taker = buy
        }

        trade = adapter.parse_trade(raw_data)

        assert trade.symbol.root == "ETH/USDT"
        assert trade.price.root == 3000.50
        assert trade.size.root == 10.0
        assert trade.side == Side.buy


class TestModelMapping:
    """Test ModelMapping functionality."""

    def test_apply_simple_mapping(self) -> None:
        """Simple mapping extracts value correctly."""
        mapping = ModelMapping("target", "source")
        result = mapping.apply({"source": "value"})
        assert result == "value"

    def test_apply_nested_mapping(self) -> None:
        """Nested mapping extracts value correctly."""
        mapping = ModelMapping("target", "level1.level2.value")
        result = mapping.apply({"level1": {"level2": {"value": 42}}})
        assert result == 42

    def test_apply_with_transform(self) -> None:
        """Mapping applies transform function."""
        mapping = ModelMapping("target", "source", transform=Transforms.to_float)
        result = mapping.apply({"source": "123.45"})
        assert result == 123.45

    def test_apply_with_default(self) -> None:
        """Mapping returns default when source is missing."""
        mapping = ModelMapping("target", "missing", default="default_value")
        result = mapping.apply({"other": "value"})
        assert result == "default_value"

    def test_apply_returns_none_when_missing_no_default(self) -> None:
        """Mapping returns None when source is missing and no default."""
        mapping = ModelMapping("target", "missing")
        result = mapping.apply({"other": "value"})
        assert result is None


class TestTransformsIntegration:
    """Test Transforms work with adapters."""

    def test_timestamp_conversion_chain(self) -> None:
        """Timestamp conversions work in adapter context."""
        adapter = SampleExchangeAdapter()

        raw_data = {
            "ticker": "BTC/USDT",
            "time": 1769990400000,
            "best_bid": {"price": "50000", "size": "1"},
            "best_ask": {"price": "50001", "size": "1"},
        }

        quote = adapter.parse_quote(raw_data)

        # The timestamp should be converted to ISO format
        timestamp_str = str(quote.timestamp.root)
        assert "2026-02-02" in timestamp_str


class TestEmptyMappings:
    """Test adapters with empty or missing mappings."""

    def test_default_mappings_return_empty_list(self) -> None:
        """Default mapping methods return empty lists."""
        adapter = SampleExchangeAdapter()

        # get_ohlcv_mapping is not overridden, should return []
        assert adapter.get_ohlcv_mapping() == []
        assert adapter.get_orderbook_mapping() == []
        assert adapter.get_instrument_mapping() == []
