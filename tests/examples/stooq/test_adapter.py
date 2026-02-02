"""Unit tests for StooqAdapter."""

import pytest

from examples.stooq.adapter import StooqAdapter
from marketschema.adapters.registry import AdapterRegistry
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV


class TestStooqAdapterInit:
    """Test StooqAdapter initialization."""

    def test_source_name(self) -> None:
        """StooqAdapter has correct source_name."""
        adapter = StooqAdapter()
        assert adapter.source_name == "stooq"


class TestDateToIsoTimestamp:
    """Test date to ISO timestamp conversion."""

    def test_valid_date_converts_to_utc_midnight(self) -> None:
        """Valid date converts to UTC midnight timestamp."""
        result = StooqAdapter._date_to_iso_timestamp("2025-01-15")
        assert result == "2025-01-15T00:00:00Z"

    def test_date_with_leading_zeros(self) -> None:
        """Date with leading zeros converts correctly."""
        result = StooqAdapter._date_to_iso_timestamp("1999-04-06")
        assert result == "1999-04-06T00:00:00Z"

    def test_invalid_date_format_raises_adapter_error(self) -> None:
        """Invalid date format raises AdapterError."""
        with pytest.raises(AdapterError, match="Invalid date format"):
            StooqAdapter._date_to_iso_timestamp("2025/01/15")

    def test_incomplete_date_raises_adapter_error(self) -> None:
        """Incomplete date raises AdapterError."""
        with pytest.raises(AdapterError, match="Invalid date format"):
            StooqAdapter._date_to_iso_timestamp("2025-01")

    def test_non_numeric_date_raises_adapter_error(self) -> None:
        """Non-numeric date parts raise AdapterError."""
        with pytest.raises(AdapterError, match="Invalid date format"):
            StooqAdapter._date_to_iso_timestamp("YYYY-MM-DD")

    def test_wrong_length_parts_raises_adapter_error(self) -> None:
        """Date parts with wrong length raise AdapterError."""
        with pytest.raises(AdapterError, match="Invalid date format"):
            StooqAdapter._date_to_iso_timestamp("25-1-5")


class TestParseCsvRow:
    """Test single CSV row parsing."""

    def test_parse_valid_row(self, stooq_row_valid: list[str]) -> None:
        """Parse valid CSV row to OHLCV model."""
        adapter = StooqAdapter()

        ohlcv = adapter.parse_csv_row(stooq_row_valid, symbol="spy.us")

        assert ohlcv.symbol.root == "spy.us"
        assert ohlcv.open.root == 100.50
        assert ohlcv.high.root == 105.25
        assert ohlcv.low.root == 99.75
        assert ohlcv.close.root == 104.00
        assert ohlcv.volume.root == 1234567.0
        assert ohlcv.timestamp.root.isoformat() == "2025-01-15T00:00:00+00:00"

    def test_parse_row_with_insufficient_columns_raises_error(
        self, stooq_row_insufficient: list[str]
    ) -> None:
        """Parsing row with insufficient columns raises AdapterError."""
        adapter = StooqAdapter()

        with pytest.raises(AdapterError, match="Insufficient columns"):
            adapter.parse_csv_row(stooq_row_insufficient, symbol="spy.us")

    def test_parse_row_with_invalid_price_raises_error(self) -> None:
        """Parsing row with invalid price raises AdapterError."""
        adapter = StooqAdapter()
        invalid_row = [
            "2025-01-15",
            "not_a_number",
            "105.25",
            "99.75",
            "104.00",
            "1234567",
        ]

        with pytest.raises(AdapterError):
            adapter.parse_csv_row(invalid_row, symbol="spy.us")

    def test_parse_row_with_invalid_date_raises_error(self) -> None:
        """Parsing row with invalid date raises AdapterError."""
        adapter = StooqAdapter()
        invalid_row = ["invalid-date", "100.50", "105.25", "99.75", "104.00", "1234567"]

        with pytest.raises(AdapterError, match="Invalid date format"):
            adapter.parse_csv_row(invalid_row, symbol="spy.us")


class TestParseCsv:
    """Test full CSV parsing."""

    def test_parse_csv_with_multiple_rows(self, stooq_csv_content: str) -> None:
        """Parse CSV with multiple data rows."""
        adapter = StooqAdapter()

        ohlcvs = adapter.parse_csv(stooq_csv_content, symbol="spy.us")

        assert len(ohlcvs) == 2
        assert all(isinstance(o, OHLCV) for o in ohlcvs)

        # Check first row
        assert ohlcvs[0].symbol.root == "spy.us"
        assert ohlcvs[0].open.root == 898.471
        assert ohlcvs[0].timestamp.root.isoformat() == "1999-04-06T00:00:00+00:00"

        # Check second row
        assert ohlcvs[1].open.root == 919.49
        assert ohlcvs[1].timestamp.root.isoformat() == "1999-04-07T00:00:00+00:00"

    def test_parse_csv_with_single_row(self, stooq_csv_single_row: str) -> None:
        """Parse CSV with single data row."""
        adapter = StooqAdapter()

        ohlcvs = adapter.parse_csv(stooq_csv_single_row, symbol="aapl.us")

        assert len(ohlcvs) == 1
        assert ohlcvs[0].symbol.root == "aapl.us"

    def test_parse_csv_skips_empty_rows(self) -> None:
        """Parse CSV skips empty rows between data rows."""
        adapter = StooqAdapter()
        csv_with_empty_rows = """Date,Open,High,Low,Close,Volume
2025-01-15,100.50,105.25,99.75,104.00,1234567

2025-01-16,104.00,106.00,103.50,105.50,987654
"""

        ohlcvs = adapter.parse_csv(csv_with_empty_rows, symbol="spy.us")

        assert len(ohlcvs) == 2
        assert ohlcvs[0].timestamp.root.isoformat() == "2025-01-15T00:00:00+00:00"
        assert ohlcvs[1].timestamp.root.isoformat() == "2025-01-16T00:00:00+00:00"

    def test_parse_empty_csv_returns_empty_list(self, stooq_csv_empty: str) -> None:
        """Parse CSV with header only returns empty list."""
        adapter = StooqAdapter()

        ohlcvs = adapter.parse_csv(stooq_csv_empty, symbol="spy.us")

        assert ohlcvs == []

    def test_parse_csv_with_invalid_header_raises_error(
        self, stooq_csv_invalid_header: str
    ) -> None:
        """Parsing CSV with invalid header raises AdapterError."""
        adapter = StooqAdapter()

        with pytest.raises(AdapterError, match="Invalid CSV header"):
            adapter.parse_csv(stooq_csv_invalid_header, symbol="spy.us")

    def test_parse_csv_with_no_content_raises_error(self) -> None:
        """Parsing completely empty CSV raises AdapterError."""
        adapter = StooqAdapter()

        with pytest.raises(AdapterError, match="Empty CSV"):
            adapter.parse_csv("", symbol="spy.us")


class TestAdapterRegistry:
    """Test adapter registration."""

    def test_adapter_is_registered(self) -> None:
        """StooqAdapter is registered in AdapterRegistry."""
        # Import triggers registration via @register decorator
        from examples.stooq.adapter import StooqAdapter  # noqa: F401

        assert AdapterRegistry.is_registered("stooq")

    def test_adapter_can_be_retrieved(self) -> None:
        """StooqAdapter can be retrieved from registry."""
        from examples.stooq.adapter import StooqAdapter  # noqa: F401

        adapter = AdapterRegistry.get("stooq")
        assert isinstance(adapter, StooqAdapter)
