"""Unit tests for ExtendedOHLCV model and adapter methods."""

import pytest

from examples.stockanalysis.adapter import StockAnalysisAdapter
from examples.stockanalysis.models import ExtendedOHLCV
from marketschema.models import OHLCV


class TestExtendedOHLCVModel:
    """Test ExtendedOHLCV model structure and validation."""

    def test_inherits_from_ohlcv(self) -> None:
        """ExtendedOHLCV inherits from OHLCV."""
        assert issubclass(ExtendedOHLCV, OHLCV)

    def test_has_adj_close_field(self) -> None:
        """ExtendedOHLCV has adj_close field in model_fields."""
        assert "adj_close" in ExtendedOHLCV.model_fields

    def test_forbids_extra_fields(self) -> None:
        """ExtendedOHLCV forbids extra fields (FR-010)."""
        with pytest.raises(ValueError, match="Extra inputs are not permitted"):
            ExtendedOHLCV(
                symbol="TSLA",  # type: ignore[arg-type]
                timestamp="2026-02-02T00:00:00Z",  # type: ignore[arg-type]
                open=260.03,  # type: ignore[arg-type]
                high=270.49,  # type: ignore[arg-type]
                low=259.21,  # type: ignore[arg-type]
                close=269.96,  # type: ignore[arg-type]
                volume=73368699,  # type: ignore[arg-type]
                adj_close=269.96,  # type: ignore[arg-type]
                unknown_field=123.45,  # type: ignore[call-arg]
            )


class TestParseHtmlRowExtended:
    """Test parse_html_row_extended method."""

    def test_returns_extended_ohlcv(self, stockanalysis_row_valid: list[str]) -> None:
        """parse_html_row_extended returns ExtendedOHLCV type."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_row_extended(stockanalysis_row_valid, symbol="TSLA")

        assert isinstance(result, ExtendedOHLCV)

    def test_includes_adj_close(self, stockanalysis_row_valid: list[str]) -> None:
        """parse_html_row_extended includes adj_close value."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_row_extended(stockanalysis_row_valid, symbol="TSLA")

        # Row format: Date, Open, High, Low, Close, Adj Close, Change, Volume
        # stockanalysis_row_valid[5] = "269.96" (adj_close)
        assert result.adj_close.root == 269.96

    def test_includes_base_fields(self, stockanalysis_row_valid: list[str]) -> None:
        """parse_html_row_extended includes all base OHLCV fields."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_row_extended(stockanalysis_row_valid, symbol="TSLA")

        assert result.symbol.root == "TSLA"
        assert result.open.root == 260.03
        assert result.high.root == 270.49
        assert result.low.root == 259.21
        assert result.close.root == 269.96
        assert result.volume.root == 73368699.0
        assert result.timestamp.root.isoformat() == "2026-02-02T00:00:00+00:00"


class TestParseHtmlExtended:
    """Test parse_html_extended method."""

    def test_returns_extended_ohlcv_list(self, stockanalysis_html_content: str) -> None:
        """parse_html_extended returns list of ExtendedOHLCV."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_extended(stockanalysis_html_content, symbol="TSLA")

        assert len(result) == 2
        assert all(isinstance(item, ExtendedOHLCV) for item in result)

    def test_includes_adj_close_values(self, stockanalysis_html_content: str) -> None:
        """parse_html_extended includes adj_close in all records."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_extended(stockanalysis_html_content, symbol="TSLA")

        # First row: adj_close = 269.96
        assert result[0].adj_close.root == 269.96
        # Second row: adj_close = 259.50
        assert result[1].adj_close.root == 259.50


class TestBackwardCompatibility:
    """Test backward compatibility of existing methods."""

    def test_parse_html_row_still_returns_ohlcv(
        self, stockanalysis_row_valid: list[str]
    ) -> None:
        """parse_html_row still returns OHLCV (not ExtendedOHLCV)."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html_row(stockanalysis_row_valid, symbol="TSLA")

        assert isinstance(result, OHLCV)
        # Should NOT be ExtendedOHLCV
        assert type(result) is OHLCV

    def test_parse_html_still_returns_ohlcv_list(
        self, stockanalysis_html_content: str
    ) -> None:
        """parse_html still returns list of OHLCV (not ExtendedOHLCV)."""
        adapter = StockAnalysisAdapter()

        result = adapter.parse_html(stockanalysis_html_content, symbol="TSLA")

        assert len(result) == 2
        assert all(isinstance(item, OHLCV) for item in result)
        # Should NOT be ExtendedOHLCV
        assert all(type(item) is OHLCV for item in result)
