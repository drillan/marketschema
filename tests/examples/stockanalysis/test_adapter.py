"""Unit tests for StockAnalysisAdapter."""

import pytest

from examples.stockanalysis.adapter import StockAnalysisAdapter
from marketschema.adapters.registry import AdapterRegistry
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV


class TestStockAnalysisAdapterInit:
    """Test StockAnalysisAdapter initialization."""

    def test_source_name(self) -> None:
        """StockAnalysisAdapter has correct source_name."""
        adapter = StockAnalysisAdapter()
        assert adapter.source_name == "stockanalysis"


class TestParseDate:
    """Test date parsing (MMM D, YYYY -> ISO 8601)."""

    def test_valid_date_converts_to_iso(self) -> None:
        """Valid date converts to ISO 8601 UTC midnight."""
        result = StockAnalysisAdapter._parse_date("Feb 2, 2026")
        assert result == "2026-02-02T00:00:00Z"

    def test_single_digit_day(self) -> None:
        """Single digit day is properly zero-padded."""
        result = StockAnalysisAdapter._parse_date("Jan 5, 2026")
        assert result == "2026-01-05T00:00:00Z"

    def test_double_digit_day(self) -> None:
        """Double digit day is properly formatted."""
        result = StockAnalysisAdapter._parse_date("Feb 12, 2026")
        assert result == "2026-02-12T00:00:00Z"

    def test_all_months(self) -> None:
        """All month abbreviations are recognized."""
        months = [
            ("Jan", "01"),
            ("Feb", "02"),
            ("Mar", "03"),
            ("Apr", "04"),
            ("May", "05"),
            ("Jun", "06"),
            ("Jul", "07"),
            ("Aug", "08"),
            ("Sep", "09"),
            ("Oct", "10"),
            ("Nov", "11"),
            ("Dec", "12"),
        ]
        for abbr, num in months:
            result = StockAnalysisAdapter._parse_date(f"{abbr} 1, 2026")
            assert result == f"2026-{num}-01T00:00:00Z"

    def test_invalid_month_raises_error(self) -> None:
        """Invalid month abbreviation raises AdapterError."""
        with pytest.raises(AdapterError, match="Invalid month"):
            StockAnalysisAdapter._parse_date("Foo 2, 2026")

    def test_invalid_format_raises_error(self) -> None:
        """Invalid date format raises AdapterError."""
        with pytest.raises(AdapterError, match="Invalid date format"):
            StockAnalysisAdapter._parse_date("2026-02-02")

    def test_invalid_day_raises_error(self) -> None:
        """Invalid day raises AdapterError."""
        with pytest.raises(AdapterError, match="invalid day"):
            StockAnalysisAdapter._parse_date("Feb XX, 2026")

    def test_invalid_year_raises_error(self) -> None:
        """Invalid year raises AdapterError."""
        with pytest.raises(AdapterError, match="invalid year"):
            StockAnalysisAdapter._parse_date("Feb 2, YYYY")


class TestParseVolume:
    """Test volume parsing (comma-separated -> plain number)."""

    def test_volume_with_commas(self) -> None:
        """Volume with commas is converted to plain number string."""
        result = StockAnalysisAdapter._parse_volume("73,368,699")
        assert result == "73368699"

    def test_volume_without_commas(self) -> None:
        """Volume without commas passes through."""
        result = StockAnalysisAdapter._parse_volume("12345")
        assert result == "12345"

    def test_empty_volume_raises_error(self) -> None:
        """Empty volume string raises AdapterError."""
        with pytest.raises(AdapterError, match="Empty volume"):
            StockAnalysisAdapter._parse_volume("")


class TestParseHtmlRow:
    """Test single HTML row parsing."""

    def test_parse_valid_row(self, stockanalysis_row_valid: list[str]) -> None:
        """Parse valid HTML row to OHLCV model."""
        adapter = StockAnalysisAdapter()

        ohlcv = adapter.parse_html_row(stockanalysis_row_valid, symbol="TSLA")

        assert ohlcv.symbol.root == "TSLA"
        assert ohlcv.open.root == 260.03
        assert ohlcv.high.root == 270.49
        assert ohlcv.low.root == 259.21
        assert ohlcv.close.root == 269.96
        assert ohlcv.volume.root == 73368699.0
        assert ohlcv.timestamp.root.isoformat() == "2026-02-02T00:00:00+00:00"

    def test_parse_row_with_insufficient_columns(
        self, stockanalysis_row_insufficient: list[str]
    ) -> None:
        """Parsing row with insufficient columns raises AdapterError."""
        adapter = StockAnalysisAdapter()

        with pytest.raises(AdapterError, match="Insufficient columns"):
            adapter.parse_html_row(stockanalysis_row_insufficient, symbol="TSLA")

    def test_parse_row_with_invalid_price(self) -> None:
        """Parsing row with invalid price raises AdapterError."""
        adapter = StockAnalysisAdapter()
        invalid_row = [
            "Feb 2, 2026",
            "not_a_number",
            "270.49",
            "259.21",
            "269.96",
            "269.96",
            "4.04%",
            "73,368,699",
        ]

        with pytest.raises(AdapterError):
            adapter.parse_html_row(invalid_row, symbol="TSLA")


class TestParseHtml:
    """Test full HTML table parsing."""

    def test_parse_html_with_multiple_rows(
        self, stockanalysis_html_content: str
    ) -> None:
        """Parse HTML table with multiple data rows."""
        adapter = StockAnalysisAdapter()

        ohlcvs = adapter.parse_html(stockanalysis_html_content, symbol="TSLA")

        assert len(ohlcvs) == 2
        assert all(isinstance(o, OHLCV) for o in ohlcvs)

        # Check first row
        assert ohlcvs[0].symbol.root == "TSLA"
        assert ohlcvs[0].open.root == 260.03
        assert ohlcvs[0].timestamp.root.isoformat() == "2026-02-02T00:00:00+00:00"

        # Check second row
        assert ohlcvs[1].open.root == 255.00
        assert ohlcvs[1].timestamp.root.isoformat() == "2026-02-01T00:00:00+00:00"

    def test_parse_empty_table_returns_empty_list(
        self, stockanalysis_html_empty: str
    ) -> None:
        """Parse HTML table with header only returns empty list."""
        adapter = StockAnalysisAdapter()

        ohlcvs = adapter.parse_html(stockanalysis_html_empty, symbol="TSLA")

        assert ohlcvs == []

    def test_parse_invalid_html_raises_error(
        self, stockanalysis_html_invalid: str
    ) -> None:
        """Parsing HTML without table raises AdapterError."""
        adapter = StockAnalysisAdapter()

        with pytest.raises(AdapterError, match="No table found"):
            adapter.parse_html(stockanalysis_html_invalid, symbol="TSLA")

    def test_parse_empty_html_raises_error(self) -> None:
        """Parsing empty HTML content raises AdapterError."""
        adapter = StockAnalysisAdapter()

        with pytest.raises(AdapterError, match="Empty HTML content"):
            adapter.parse_html("", symbol="TSLA")

    def test_parse_whitespace_only_html_raises_error(self) -> None:
        """Parsing whitespace-only HTML content raises AdapterError."""
        adapter = StockAnalysisAdapter()

        with pytest.raises(AdapterError, match="Empty HTML content"):
            adapter.parse_html("   \n\t  ", symbol="TSLA")

    def test_parse_html_without_tbody_raises_error(
        self, stockanalysis_html_no_tbody: str
    ) -> None:
        """Parsing HTML table without tbody raises AdapterError."""
        adapter = StockAnalysisAdapter()

        with pytest.raises(AdapterError, match="<tbody> element not found"):
            adapter.parse_html(stockanalysis_html_no_tbody, symbol="TSLA")


class TestAdapterRegistry:
    """Test adapter registration."""

    def test_adapter_is_registered(self) -> None:
        """StockAnalysisAdapter is registered in AdapterRegistry."""
        # Import triggers registration via @register decorator
        from examples.stockanalysis.adapter import StockAnalysisAdapter  # noqa: F401

        assert AdapterRegistry.is_registered("stockanalysis")

    def test_adapter_can_be_retrieved(self) -> None:
        """StockAnalysisAdapter can be retrieved from registry."""
        from examples.stockanalysis.adapter import StockAnalysisAdapter  # noqa: F401

        adapter = AdapterRegistry.get("stockanalysis")
        assert isinstance(adapter, StockAnalysisAdapter)
