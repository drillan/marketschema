"""stockanalysis.com HTML stock data adapter for marketschema.

This adapter transforms HTML table data from stockanalysis.com into
marketschema OHLCV models.

Data Source:
    https://stockanalysis.com/stocks/{symbol}/history/

HTML Table Format:
    Date       | Open   | High   | Low    | Close  | Adj Close | Change | Volume
    Feb 2, 2026| 260.03 | 270.49 | 259.21 | 269.96 | 269.96    | 4.04%  | 73,368,699
"""

import logging
from typing import Any

from bs4 import BeautifulSoup

from examples.stockanalysis.models import ExtendedOHLCV
from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV

logger = logging.getLogger(__name__)

# Expected minimum HTML column count (Date, Open, High, Low, Close, Adj Close, Change, Volume)
EXPECTED_COLUMN_COUNT = 8

# HTML column indices
HTML_INDEX_DATE = 0
HTML_INDEX_OPEN = 1
HTML_INDEX_HIGH = 2
HTML_INDEX_LOW = 3
HTML_INDEX_CLOSE = 4
HTML_INDEX_ADJ_CLOSE = 5
HTML_INDEX_VOLUME = 7

# Month abbreviation mapping
MONTH_MAP = {
    "Jan": "01",
    "Feb": "02",
    "Mar": "03",
    "Apr": "04",
    "May": "05",
    "Jun": "06",
    "Jul": "07",
    "Aug": "08",
    "Sep": "09",
    "Oct": "10",
    "Nov": "11",
    "Dec": "12",
}


@register
class StockAnalysisAdapter(BaseAdapter):
    """Adapter for stockanalysis.com HTML stock data.

    Transforms HTML table data from stockanalysis.com into standardized
    marketschema OHLCV models.

    Note:
        stockanalysis.com HTML does not include symbol information.
        Symbol must be provided as a parameter to parse methods.
    """

    source_name = "stockanalysis"

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OHLCV model.

        Internal dict keys (after HTML row conversion):
            - timestamp: ISO 8601 timestamp string (converted from Date)
            - open: Open price (string, converted to float)
            - high: High price (string, converted to float)
            - low: Low price (string, converted to float)
            - close: Close price (string, converted to float)
            - volume: Trading volume (string, converted to float)
        """
        return [
            ModelMapping("open", "open", transform=self.transforms.to_float),
            ModelMapping("high", "high", transform=self.transforms.to_float),
            ModelMapping("low", "low", transform=self.transforms.to_float),
            ModelMapping("close", "close", transform=self.transforms.to_float),
            ModelMapping("volume", "volume", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.iso_timestamp
            ),
        ]

    @staticmethod
    def _parse_date(date_str: str) -> str:
        """Convert date string to ISO 8601 timestamp.

        Args:
            date_str: Date in "MMM D, YYYY" format (e.g., "Feb 2, 2026")

        Returns:
            ISO 8601 timestamp string (UTC midnight)

        Raises:
            AdapterError: If date format is invalid
        """
        # Expected format: "MMM D, YYYY" (e.g., "Feb 2, 2026")
        parts = date_str.split()
        if len(parts) != 3:
            raise AdapterError(
                f"Invalid date format: {date_str!r}, expected 'MMM D, YYYY'"
            )

        month_abbr, day_str, year_str = parts

        # Validate and convert month
        if month_abbr not in MONTH_MAP:
            raise AdapterError(f"Invalid month: {month_abbr!r}")
        month = MONTH_MAP[month_abbr]

        # Remove comma from day and validate
        day_str = day_str.rstrip(",")
        try:
            day = int(day_str)
        except ValueError as e:
            raise AdapterError(f"Invalid date format: {date_str!r}, invalid day") from e

        # Validate year
        try:
            year = int(year_str)
        except ValueError as e:
            raise AdapterError(
                f"Invalid date format: {date_str!r}, invalid year"
            ) from e

        return f"{year:04d}-{month}-{day:02d}T00:00:00Z"

    @staticmethod
    def _parse_volume(volume_str: str) -> str:
        """Remove commas from volume string.

        Args:
            volume_str: Volume with commas (e.g., "73,368,699")

        Returns:
            Volume without commas (e.g., "73368699")

        Raises:
            AdapterError: If volume string is empty
        """
        if not volume_str:
            raise AdapterError("Empty volume string")
        return volume_str.replace(",", "")

    def parse_html_row(self, row_data: list[str], *, symbol: str) -> OHLCV:
        """Parse a single HTML table row into OHLCV model.

        Args:
            row_data: List of string values from HTML table row
            symbol: Stock symbol (e.g., "TSLA")

        Returns:
            OHLCV model instance

        Raises:
            AdapterError: If row has insufficient columns or invalid data
        """
        if len(row_data) < EXPECTED_COLUMN_COUNT:
            raise AdapterError(
                f"Insufficient columns: expected {EXPECTED_COLUMN_COUNT}, "
                f"got {len(row_data)}"
            )

        # Convert row to dict for mapping
        ohlcv_dict: dict[str, Any] = {
            "symbol": symbol,
            "timestamp": self._parse_date(row_data[HTML_INDEX_DATE]),
            "open": row_data[HTML_INDEX_OPEN],
            "high": row_data[HTML_INDEX_HIGH],
            "low": row_data[HTML_INDEX_LOW],
            "close": row_data[HTML_INDEX_CLOSE],
            "volume": self._parse_volume(row_data[HTML_INDEX_VOLUME]),
        }

        mappings = self.get_ohlcv_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(ohlcv_dict, mappings, OHLCV)

    def parse_html(self, html_content: str, *, symbol: str) -> list[OHLCV]:
        """Parse HTML content into list of OHLCV models.

        Args:
            html_content: Full HTML content as string
            symbol: Stock symbol (e.g., "TSLA")

        Returns:
            List of OHLCV model instances

        Raises:
            AdapterError: If HTML format is invalid or no table found
        """
        if not html_content or not html_content.strip():
            raise AdapterError("Empty HTML content provided")

        soup = BeautifulSoup(html_content, "html.parser")

        # Find the table element
        table = soup.find("table")
        if table is None:
            raise AdapterError("No table found in HTML content")

        # Find tbody and extract data rows
        tbody = table.find("tbody")
        if tbody is None:
            raise AdapterError(
                "Table structure error: <tbody> element not found. "
                "The page structure may have changed."
            )

        rows = tbody.find_all("tr")
        if not rows:
            logger.warning(
                "No <tr> elements found in <tbody>. "
                "If this is unexpected, the page structure may have changed."
            )
            return []

        # Parse data rows
        results: list[OHLCV] = []
        for row in rows:
            cells = row.find_all("td")
            if cells:
                row_data = [cell.get_text(strip=True) for cell in cells]
                results.append(self.parse_html_row(row_data, symbol=symbol))

        return results

    def get_extended_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for ExtendedOHLCV model.

        Extends the base OHLCV mapping with adj_close field.

        Internal dict keys (after HTML row conversion):
            - All keys from get_ohlcv_mapping()
            - adj_close: Adjusted close price (string, converted to float)
        """
        return self.get_ohlcv_mapping() + [
            ModelMapping("adj_close", "adj_close", transform=self.transforms.to_float),
        ]

    def parse_html_row_extended(
        self, row_data: list[str], *, symbol: str
    ) -> ExtendedOHLCV:
        """Parse a single HTML table row into ExtendedOHLCV model.

        Args:
            row_data: List of string values from HTML table row
            symbol: Stock symbol (e.g., "TSLA")

        Returns:
            ExtendedOHLCV model instance with adj_close field

        Raises:
            AdapterError: If row has insufficient columns or invalid data
        """
        if len(row_data) < EXPECTED_COLUMN_COUNT:
            raise AdapterError(
                f"Insufficient columns: expected {EXPECTED_COLUMN_COUNT}, "
                f"got {len(row_data)}"
            )

        # Convert row to dict for mapping (includes adj_close)
        ohlcv_dict: dict[str, Any] = {
            "symbol": symbol,
            "timestamp": self._parse_date(row_data[HTML_INDEX_DATE]),
            "open": row_data[HTML_INDEX_OPEN],
            "high": row_data[HTML_INDEX_HIGH],
            "low": row_data[HTML_INDEX_LOW],
            "close": row_data[HTML_INDEX_CLOSE],
            "adj_close": row_data[HTML_INDEX_ADJ_CLOSE],
            "volume": self._parse_volume(row_data[HTML_INDEX_VOLUME]),
        }

        mappings = self.get_extended_ohlcv_mapping() + [
            ModelMapping("symbol", "symbol")
        ]
        return self._apply_mapping(ohlcv_dict, mappings, ExtendedOHLCV)

    def parse_html_extended(
        self, html_content: str, *, symbol: str
    ) -> list[ExtendedOHLCV]:
        """Parse HTML content into list of ExtendedOHLCV models.

        Args:
            html_content: Full HTML content as string
            symbol: Stock symbol (e.g., "TSLA")

        Returns:
            List of ExtendedOHLCV model instances with adj_close field

        Raises:
            AdapterError: If HTML format is invalid or no table found
        """
        if not html_content or not html_content.strip():
            raise AdapterError("Empty HTML content provided")

        soup = BeautifulSoup(html_content, "html.parser")

        # Find the table element
        table = soup.find("table")
        if table is None:
            raise AdapterError("No table found in HTML content")

        # Find tbody and extract data rows
        tbody = table.find("tbody")
        if tbody is None:
            raise AdapterError(
                "Table structure error: <tbody> element not found. "
                "The page structure may have changed."
            )

        rows = tbody.find_all("tr")
        if not rows:
            logger.warning(
                "No <tr> elements found in <tbody>. "
                "If this is unexpected, the page structure may have changed."
            )
            return []

        # Parse data rows
        results: list[ExtendedOHLCV] = []
        for row in rows:
            cells = row.find_all("td")
            if cells:
                row_data = [cell.get_text(strip=True) for cell in cells]
                results.append(self.parse_html_row_extended(row_data, symbol=symbol))

        return results


__all__ = ["StockAnalysisAdapter"]
