"""stooq.com stock data adapter for marketschema.

This adapter transforms CSV data from stooq.com into marketschema OHLCV models.

Data Source:
    https://stooq.com/q/d/l/?s={symbol}&i=d

CSV Format:
    Date,Open,High,Low,Close,Volume
    1999-04-06,898.471,919.49,879.213,919.49,890722
"""

import csv
from io import StringIO
from typing import Any

from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV

# Expected CSV column count
EXPECTED_COLUMN_COUNT = 6

# CSV column indices
CSV_INDEX_DATE = 0
CSV_INDEX_OPEN = 1
CSV_INDEX_HIGH = 2
CSV_INDEX_LOW = 3
CSV_INDEX_CLOSE = 4
CSV_INDEX_VOLUME = 5

# Expected CSV header
EXPECTED_HEADER = ["Date", "Open", "High", "Low", "Close", "Volume"]


@register
class StooqAdapter(BaseAdapter):
    """Adapter for stooq.com stock data.

    Transforms CSV data from stooq.com into standardized marketschema OHLCV models.

    Note:
        stooq.com CSV does not include symbol information.
        Symbol must be provided as a parameter to parse methods.
    """

    source_name = "stooq"

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        """Return field mappings for OHLCV model.

        stooq CSV format (after conversion to dict):
            - Date: Date string (YYYY-MM-DD)
            - Open: Open price (string)
            - High: High price (string)
            - Low: Low price (string)
            - Close: Close price (string)
            - Volume: Trading volume (string)
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
    def _date_to_iso_timestamp(date_str: str) -> str:
        """Convert date string to ISO 8601 timestamp.

        Args:
            date_str: Date in YYYY-MM-DD format

        Returns:
            ISO 8601 timestamp string (UTC midnight)

        Raises:
            AdapterError: If date format is invalid
        """
        # Validate format by checking structure
        parts = date_str.split("-")
        if len(parts) != 3:
            raise AdapterError(
                f"Invalid date format: {date_str!r}, expected YYYY-MM-DD"
            )

        year, month, day = parts
        if not (len(year) == 4 and len(month) == 2 and len(day) == 2):
            raise AdapterError(
                f"Invalid date format: {date_str!r}, expected YYYY-MM-DD"
            )

        try:
            # Validate numeric values
            int(year)
            int(month)
            int(day)
        except ValueError as e:
            raise AdapterError(
                f"Invalid date format: {date_str!r}, expected YYYY-MM-DD"
            ) from e

        return f"{date_str}T00:00:00Z"

    def parse_csv_row(self, row: list[str], *, symbol: str) -> OHLCV:
        """Parse a single CSV row into OHLCV model.

        Args:
            row: List of string values from CSV row
            symbol: Stock symbol (e.g., "spy.us")

        Returns:
            OHLCV model instance

        Raises:
            AdapterError: If row has insufficient columns or invalid data
        """
        if len(row) < EXPECTED_COLUMN_COUNT:
            raise AdapterError(
                f"Insufficient columns: expected {EXPECTED_COLUMN_COUNT}, got {len(row)}"
            )

        # Convert row to dict for mapping
        ohlcv_dict: dict[str, Any] = {
            "symbol": symbol,
            "timestamp": self._date_to_iso_timestamp(row[CSV_INDEX_DATE]),
            "open": row[CSV_INDEX_OPEN],
            "high": row[CSV_INDEX_HIGH],
            "low": row[CSV_INDEX_LOW],
            "close": row[CSV_INDEX_CLOSE],
            "volume": row[CSV_INDEX_VOLUME],
        }

        mappings = self.get_ohlcv_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(ohlcv_dict, mappings, OHLCV)

    def parse_csv(self, csv_content: str, *, symbol: str) -> list[OHLCV]:
        """Parse CSV content into list of OHLCV models.

        Args:
            csv_content: Full CSV content as string
            symbol: Stock symbol (e.g., "spy.us")

        Returns:
            List of OHLCV model instances

        Raises:
            AdapterError: If CSV format is invalid
        """
        reader = csv.reader(StringIO(csv_content))

        # Read and validate header
        try:
            header = next(reader)
        except StopIteration as e:
            raise AdapterError("Empty CSV: no header row") from e

        if header != EXPECTED_HEADER:
            raise AdapterError(
                f"Invalid CSV header: expected {EXPECTED_HEADER}, got {header}"
            )

        # Parse data rows
        results: list[OHLCV] = []
        for row in reader:
            if row:  # Skip empty rows
                results.append(self.parse_csv_row(row, symbol=symbol))

        return results


__all__ = ["StooqAdapter"]
