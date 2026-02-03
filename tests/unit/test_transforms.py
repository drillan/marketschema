"""Test transform functions."""

import pytest

from marketschema.adapters.transforms import Transforms
from marketschema.exceptions import TransformError


class TestToFloat:
    """Test Transforms.to_float."""

    def test_convert_string_to_float(self) -> None:
        """String numeric value converts to float."""
        assert Transforms.to_float("123.45") == 123.45

    def test_convert_int_to_float(self) -> None:
        """Integer converts to float."""
        assert Transforms.to_float(123) == 123.0

    def test_convert_float_to_float(self) -> None:
        """Float passes through unchanged."""
        assert Transforms.to_float(123.45) == 123.45

    def test_invalid_string_raises_error(self) -> None:
        """Invalid string raises TransformError."""
        with pytest.raises(TransformError, match="Cannot convert"):
            Transforms.to_float("not a number")


class TestToInt:
    """Test Transforms.to_int."""

    def test_convert_string_to_int(self) -> None:
        """String numeric value converts to int."""
        assert Transforms.to_int("123") == 123

    def test_convert_float_to_int(self) -> None:
        """Float converts to int (truncates)."""
        assert Transforms.to_int(123.99) == 123

    def test_invalid_string_raises_error(self) -> None:
        """Invalid string raises TransformError."""
        with pytest.raises(TransformError, match="Cannot convert"):
            Transforms.to_int("not a number")


class TestIsoTimestamp:
    """Test Transforms.iso_timestamp."""

    def test_valid_iso_timestamp_with_z(self) -> None:
        """Valid ISO timestamp with Z suffix passes."""
        result = Transforms.iso_timestamp("2026-02-02T09:00:00Z")
        assert result == "2026-02-02T09:00:00Z"

    def test_valid_iso_timestamp_with_offset(self) -> None:
        """Valid ISO timestamp with offset converts to Z suffix."""
        result = Transforms.iso_timestamp("2026-02-02T09:00:00+00:00")
        assert result == "2026-02-02T09:00:00Z"

    def test_jst_offset_converts_to_utc(self) -> None:
        """JST timestamp (+09:00) is converted to UTC."""
        result = Transforms.iso_timestamp("2026-02-02T09:00:00+09:00")
        assert result == "2026-02-02T00:00:00Z"

    def test_negative_offset_converts_to_utc(self) -> None:
        """Negative offset timestamp is converted to UTC."""
        result = Transforms.iso_timestamp("2026-02-01T19:00:00-05:00")
        assert result == "2026-02-02T00:00:00Z"

    def test_invalid_timestamp_raises_error(self) -> None:
        """Invalid timestamp string raises TransformError."""
        with pytest.raises(TransformError, match="Invalid ISO timestamp"):
            Transforms.iso_timestamp("not a timestamp")


class TestUnixTimestampMs:
    """Test Transforms.unix_timestamp_ms."""

    def test_convert_milliseconds_to_iso(self) -> None:
        """Unix milliseconds converts to ISO timestamp."""
        # 2026-02-02T00:00:00Z in milliseconds
        ms = 1769990400000
        result = Transforms.unix_timestamp_ms(ms)
        assert result == "2026-02-02T00:00:00Z"

    def test_convert_float_milliseconds(self) -> None:
        """Float milliseconds also converts."""
        ms = 1769990400000.0
        result = Transforms.unix_timestamp_ms(ms)
        assert "2026-02-02" in result

    def test_invalid_value_raises_error(self) -> None:
        """Non-numeric value raises TransformError."""
        with pytest.raises(TransformError, match="Cannot convert"):
            Transforms.unix_timestamp_ms("not a number")  # type: ignore[arg-type]


class TestUnixTimestampSec:
    """Test Transforms.unix_timestamp_sec."""

    def test_convert_seconds_to_iso(self) -> None:
        """Unix seconds converts to ISO timestamp."""
        # 2026-02-02T00:00:00Z in seconds
        sec = 1769990400
        result = Transforms.unix_timestamp_sec(sec)
        assert result == "2026-02-02T00:00:00Z"


class TestSideFromString:
    """Test Transforms.side_from_string."""

    @pytest.mark.parametrize(
        ("input_value", "expected"),
        [
            ("buy", "buy"),
            ("BUY", "buy"),
            ("Buy", "buy"),
            ("bid", "buy"),
            ("BID", "buy"),
            ("b", "buy"),
            ("sell", "sell"),
            ("SELL", "sell"),
            ("Sell", "sell"),
            ("ask", "sell"),
            ("ASK", "sell"),
            ("offer", "sell"),
            ("OFFER", "sell"),
            ("s", "sell"),
            ("a", "sell"),
        ],
    )
    def test_normalize_side_values(self, input_value: str, expected: str) -> None:
        """Various side strings normalize correctly."""
        assert Transforms.side_from_string(input_value) == expected

    def test_invalid_side_raises_error(self) -> None:
        """Invalid side string raises TransformError."""
        with pytest.raises(TransformError, match="Cannot normalize side"):
            Transforms.side_from_string("invalid")


class TestJstToUtc:
    """Test Transforms.jst_to_utc."""

    def test_naive_timestamp_assumed_jst(self) -> None:
        """Naive timestamp is assumed to be JST and converted to UTC."""
        # 09:00 JST should become 00:00 UTC
        result = Transforms.jst_to_utc("2026-02-02T09:00:00")
        assert result == "2026-02-02T00:00:00Z"

    def test_utc_timestamp_stays_utc(self) -> None:
        """UTC timestamp stays as UTC."""
        result = Transforms.jst_to_utc("2026-02-02T00:00:00Z")
        assert result == "2026-02-02T00:00:00Z"

    def test_invalid_timestamp_raises_error(self) -> None:
        """Invalid timestamp raises TransformError."""
        with pytest.raises(TransformError, match="Cannot convert JST"):
            Transforms.jst_to_utc("not a timestamp")


class TestStringTransforms:
    """Test string transforms."""

    def test_uppercase(self) -> None:
        """Uppercase transform works."""
        assert Transforms.uppercase("hello") == "HELLO"

    def test_lowercase(self) -> None:
        """Lowercase transform works."""
        assert Transforms.lowercase("HELLO") == "hello"
