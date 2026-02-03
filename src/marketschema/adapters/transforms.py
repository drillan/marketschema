"""Common transform functions for adapter mappings."""

from datetime import UTC, datetime, timedelta, timezone
from typing import Any

from marketschema.exceptions import TransformError

# Constants for time conversions
MS_PER_SECOND = 1000
SECONDS_PER_HOUR = 3600
JST_UTC_OFFSET_HOURS = 9


class Transforms:
    """Collection of common transform functions for adapter mappings.

    All methods are static and can be used directly as transform functions
    in ModelMapping definitions.
    """

    @staticmethod
    def to_float(value: Any) -> float:
        """Convert value to float.

        Args:
            value: Value to convert (string, int, or float)

        Returns:
            Float representation of the value

        Raises:
            TransformError: If conversion fails
        """
        try:
            return float(value)
        except (TypeError, ValueError) as e:
            raise TransformError(f"Cannot convert {value!r} to float") from e

    @staticmethod
    def to_int(value: Any) -> int:
        """Convert value to int.

        Args:
            value: Value to convert (string, float, or int)

        Returns:
            Integer representation of the value

        Raises:
            TransformError: If conversion fails
        """
        try:
            return int(value)
        except (TypeError, ValueError) as e:
            raise TransformError(f"Cannot convert {value!r} to int") from e

    @staticmethod
    def iso_timestamp(value: str) -> str:
        """Validate and normalize ISO 8601 timestamp to UTC.

        Args:
            value: ISO 8601 formatted timestamp string

        Returns:
            The validated timestamp string in ISO 8601 format (UTC)

        Raises:
            TransformError: If the timestamp is not valid ISO 8601
        """
        try:
            # Parse ISO timestamp and normalize to UTC
            dt = datetime.fromisoformat(value.replace("Z", "+00:00"))
            utc_dt = dt.astimezone(UTC)
            return utc_dt.isoformat().replace("+00:00", "Z")
        except (TypeError, ValueError) as e:
            raise TransformError(f"Invalid ISO timestamp: {value!r}") from e

    @staticmethod
    def unix_timestamp_ms(value: int | float) -> str:
        """Convert Unix timestamp in milliseconds to ISO 8601 string.

        Args:
            value: Unix timestamp in milliseconds

        Returns:
            ISO 8601 formatted timestamp string (UTC)

        Raises:
            TransformError: If conversion fails
        """
        try:
            timestamp_seconds = float(value) / MS_PER_SECOND
            dt = datetime.fromtimestamp(timestamp_seconds, tz=UTC)
            return dt.isoformat().replace("+00:00", "Z")
        except (TypeError, ValueError, OSError) as e:
            raise TransformError(f"Cannot convert {value!r} from unix ms") from e

    @staticmethod
    def unix_timestamp_sec(value: int | float) -> str:
        """Convert Unix timestamp in seconds to ISO 8601 string.

        Args:
            value: Unix timestamp in seconds

        Returns:
            ISO 8601 formatted timestamp string (UTC)

        Raises:
            TransformError: If conversion fails
        """
        try:
            dt = datetime.fromtimestamp(float(value), tz=UTC)
            return dt.isoformat().replace("+00:00", "Z")
        except (TypeError, ValueError, OSError) as e:
            raise TransformError(f"Cannot convert {value!r} from unix seconds") from e

    @staticmethod
    def side_from_string(value: str) -> str:
        """Normalize side string to lowercase buy/sell.

        Handles common variations:
        - "buy", "BUY", "Buy" -> "buy"
        - "sell", "SELL", "Sell" -> "sell"
        - "bid", "BID" -> "buy"
        - "ask", "ASK", "offer", "OFFER" -> "sell"

        Args:
            value: Side string from source data

        Returns:
            Normalized side string ("buy" or "sell")

        Raises:
            TransformError: If the value cannot be mapped to buy/sell
        """
        normalized = value.lower().strip()

        buy_values = {"buy", "bid", "b"}
        sell_values = {"sell", "ask", "offer", "s", "a"}

        if normalized in buy_values:
            return "buy"
        if normalized in sell_values:
            return "sell"

        raise TransformError(f"Cannot normalize side value: {value!r}")

    @staticmethod
    def jst_to_utc(value: str) -> str:
        """Convert JST (Japan Standard Time) timestamp to UTC ISO 8601.

        Args:
            value: ISO 8601 formatted timestamp in JST (or naive datetime assumed JST)

        Returns:
            ISO 8601 formatted timestamp string (UTC)

        Raises:
            TransformError: If conversion fails
        """
        try:
            # Parse the timestamp
            dt = datetime.fromisoformat(value.replace("Z", "+00:00"))

            # If naive (no timezone), assume JST
            if dt.tzinfo is None:
                jst_offset = timedelta(hours=JST_UTC_OFFSET_HOURS)
                dt = dt.replace(tzinfo=timezone(jst_offset))

            # Convert to UTC
            utc_dt = dt.astimezone(UTC)
            return utc_dt.isoformat().replace("+00:00", "Z")
        except (TypeError, ValueError) as e:
            raise TransformError(f"Cannot convert JST timestamp: {value!r}") from e

    @staticmethod
    def uppercase(value: str) -> str:
        """Convert string to uppercase.

        Args:
            value: String to convert

        Returns:
            Uppercase string
        """
        return str(value).upper()

    @staticmethod
    def lowercase(value: str) -> str:
        """Convert string to lowercase.

        Args:
            value: String to convert

        Returns:
            Lowercase string
        """
        return str(value).lower()


__all__ = ["Transforms"]
