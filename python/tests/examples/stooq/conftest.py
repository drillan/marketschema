"""Test fixtures for stooq adapter tests."""

import pytest


@pytest.fixture
def stooq_csv_content() -> str:
    """Fixture for stooq CSV content.

    Example CSV from:
    https://stooq.com/q/d/l/?s=spy.us&i=d
    """
    return """Date,Open,High,Low,Close,Volume
1999-04-06,898.471,919.49,879.213,919.49,890722
1999-04-07,919.49,930.0,910.0,925.0,750000"""


@pytest.fixture
def stooq_csv_single_row() -> str:
    """Fixture for stooq CSV with single data row."""
    return """Date,Open,High,Low,Close,Volume
2025-01-15,100.50,105.25,99.75,104.00,1234567"""


@pytest.fixture
def stooq_csv_empty() -> str:
    """Fixture for empty stooq CSV (header only)."""
    return """Date,Open,High,Low,Close,Volume"""


@pytest.fixture
def stooq_csv_invalid_header() -> str:
    """Fixture for stooq CSV with invalid header."""
    return """Wrong,Header,Format
2025-01-15,100.50,105.25"""


@pytest.fixture
def stooq_row_valid() -> list[str]:
    """Fixture for valid CSV row as list."""
    return ["2025-01-15", "100.50", "105.25", "99.75", "104.00", "1234567"]


@pytest.fixture
def stooq_row_insufficient() -> list[str]:
    """Fixture for CSV row with insufficient columns."""
    return ["2025-01-15", "100.50", "105.25"]
