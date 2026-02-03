# Transform Functions Contract (Python)

**Feature**: 004-adapter-python
**Parent Spec**: [004-adapter-python](../spec.md)
**Date**: 2026-02-03

> Note: 変換関数は Python 静的メソッドとして提供され、ModelMapping の transform 属性に渡して使用する。

## Module: `marketschema.adapters.transforms`

### Transforms

```python
class Transforms:
    """Collection of common transform functions for adapter mappings.

    All methods are static and can be used directly as transform functions
    in ModelMapping definitions.

    Example:
        from marketschema.adapters.mapping import ModelMapping
        from marketschema.adapters.transforms import Transforms

        mappings = [
            ModelMapping("bid", "buy_price", transform=Transforms.to_float),
            ModelMapping("timestamp", "time", transform=Transforms.unix_timestamp_ms),
        ]
    """
    ...
```

## Function Specifications

### to_float

```python
@staticmethod
def to_float(value: Any) -> float:
    """Convert value to float.

    Args:
        value: Value to convert (string, int, or float).

    Returns:
        Float representation of the value.

    Raises:
        TransformError: If conversion fails (TypeError, ValueError).

    Examples:
        >>> Transforms.to_float("123.45")
        123.45
        >>> Transforms.to_float(100)
        100.0
        >>> Transforms.to_float("invalid")
        TransformError: Cannot convert 'invalid' to float
    """
    ...
```

### to_int

```python
@staticmethod
def to_int(value: Any) -> int:
    """Convert value to int.

    Args:
        value: Value to convert (string, float, or int).

    Returns:
        Integer representation of the value.

    Raises:
        TransformError: If conversion fails (TypeError, ValueError).

    Examples:
        >>> Transforms.to_int("123")
        123
        >>> Transforms.to_int(100.9)
        100
        >>> Transforms.to_int("invalid")
        TransformError: Cannot convert 'invalid' to int
    """
    ...
```

### iso_timestamp

```python
@staticmethod
def iso_timestamp(value: str) -> str:
    """Validate and return ISO 8601 timestamp string.

    Parses the input, validates it as a valid datetime, and re-formats
    to ensure consistent UTC ISO 8601 format with 'Z' suffix.

    Args:
        value: ISO 8601 formatted timestamp string.
            Accepts various formats: "2024-01-01T00:00:00Z",
            "2024-01-01T00:00:00+00:00", "2024-01-01T09:00:00+09:00".

    Returns:
        The validated timestamp string in UTC ISO 8601 format (with 'Z').

    Raises:
        TransformError: If the timestamp is not valid ISO 8601.

    Examples:
        >>> Transforms.iso_timestamp("2024-01-01T00:00:00Z")
        "2024-01-01T00:00:00Z"
        >>> Transforms.iso_timestamp("2024-01-01T09:00:00+09:00")
        "2024-01-01T00:00:00Z"
        >>> Transforms.iso_timestamp("invalid")
        TransformError: Invalid ISO timestamp: 'invalid'
    """
    ...
```

### unix_timestamp_ms

```python
@staticmethod
def unix_timestamp_ms(value: int | float) -> str:
    """Convert Unix timestamp in milliseconds to ISO 8601 string.

    Args:
        value: Unix timestamp in milliseconds since epoch.

    Returns:
        ISO 8601 formatted timestamp string (UTC with 'Z' suffix).

    Raises:
        TransformError: If conversion fails (TypeError, ValueError, OSError).
            OSError occurs for timestamps outside the valid range.

    Examples:
        >>> Transforms.unix_timestamp_ms(1704067200000)
        "2024-01-01T00:00:00Z"
        >>> Transforms.unix_timestamp_ms("invalid")
        TransformError: Cannot convert 'invalid' from unix ms
    """
    ...
```

### unix_timestamp_sec

```python
@staticmethod
def unix_timestamp_sec(value: int | float) -> str:
    """Convert Unix timestamp in seconds to ISO 8601 string.

    Args:
        value: Unix timestamp in seconds since epoch.

    Returns:
        ISO 8601 formatted timestamp string (UTC with 'Z' suffix).

    Raises:
        TransformError: If conversion fails (TypeError, ValueError, OSError).
            OSError occurs for timestamps outside the valid range.

    Examples:
        >>> Transforms.unix_timestamp_sec(1704067200)
        "2024-01-01T00:00:00Z"
        >>> Transforms.unix_timestamp_sec("invalid")
        TransformError: Cannot convert 'invalid' from unix seconds
    """
    ...
```

### jst_to_utc

```python
@staticmethod
def jst_to_utc(value: str) -> str:
    """Convert JST (Japan Standard Time) timestamp to UTC ISO 8601.

    If the input has no timezone info (naive datetime), it is assumed to be JST.
    If the input already has timezone info, it is converted to UTC.

    Args:
        value: ISO 8601 formatted timestamp in JST (or naive datetime assumed JST).

    Returns:
        ISO 8601 formatted timestamp string (UTC with 'Z' suffix).

    Raises:
        TransformError: If conversion fails (invalid format).

    Examples:
        >>> Transforms.jst_to_utc("2024-01-01T09:00:00")  # Naive, assumed JST
        "2024-01-01T00:00:00Z"
        >>> Transforms.jst_to_utc("2024-01-01T09:00:00+09:00")  # Explicit JST
        "2024-01-01T00:00:00Z"
        >>> Transforms.jst_to_utc("invalid")
        TransformError: Cannot convert JST timestamp: 'invalid'
    """
    ...
```

### side_from_string

```python
@staticmethod
def side_from_string(value: str) -> str:
    """Normalize side string to lowercase buy/sell.

    Handles common variations from different data sources.

    Args:
        value: Side string from source data.

    Returns:
        Normalized side string ("buy" or "sell").

    Raises:
        TransformError: If the value cannot be mapped to buy/sell.

    Mapping rules:
        - "buy", "BUY", "Buy", "bid", "BID", "b" -> "buy"
        - "sell", "SELL", "Sell", "ask", "ASK", "offer", "OFFER", "s", "a" -> "sell"

    Examples:
        >>> Transforms.side_from_string("BUY")
        "buy"
        >>> Transforms.side_from_string("ask")
        "sell"
        >>> Transforms.side_from_string("unknown")
        TransformError: Cannot normalize side value: 'unknown'
    """
    ...
```

### uppercase

```python
@staticmethod
def uppercase(value: str) -> str:
    """Convert string to uppercase.

    Args:
        value: String to convert.

    Returns:
        Uppercase string.

    Examples:
        >>> Transforms.uppercase("btc_jpy")
        "BTC_JPY"
    """
    ...
```

### lowercase

```python
@staticmethod
def lowercase(value: str) -> str:
    """Convert string to lowercase.

    Args:
        value: String to convert.

    Returns:
        Lowercase string.

    Examples:
        >>> Transforms.lowercase("BTC_JPY")
        "btc_jpy"
    """
    ...
```

## Constants

```python
# Time conversion constants
MS_PER_SECOND = 1000
SECONDS_PER_HOUR = 3600
JST_UTC_OFFSET_HOURS = 9
```

## Error Handling Contract

すべての変換関数は以下のエラーハンドリング規約に従う:

1. **サイレント障害禁止**: 変換失敗時は必ず `TransformError` を発生させる
2. **例外チェーン**: 元の例外を `from e` で保持する
3. **エラーメッセージ**: 入力値を含めて問題を特定可能にする

```python
# 実装パターン
try:
    return float(value)
except (TypeError, ValueError) as e:
    raise TransformError(f"Cannot convert {value!r} to float") from e
```

## Type Exports

```python
# marketschema.adapters.transforms.__init__.py
__all__ = ["Transforms"]
```
