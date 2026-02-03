# Transform Functions Contract

**Feature**: 004-adapter
**Date**: 2026-02-03

## Overview

本ドキュメントは変換関数の言語非依存な仕様を定義する。
各言語での型シグネチャは言語別 spec を参照:
- [004-adapter-python](../../004-adapter-python/contracts/transforms.md)
- [004-adapter-rust](../../004-adapter-rust/contracts/transforms.md)

## Transforms

### Concept

ModelMapping の `transform` 属性に渡して使用する共通変換関数群。
すべての関数は入力値を受け取り、変換後の値を返す。変換失敗時は TransformError を発生させる。

---

## Function Specifications

### Numeric Conversions

| Function | Input | Output | Error Condition | Description |
|----------|-------|--------|-----------------|-------------|
| `to_float` | any | float | Conversion fails | Convert string/number to float |
| `to_int` | any | integer | Conversion fails | Convert string/number to integer |

### Timestamp Conversions

All timestamp conversion functions return **UTC ISO 8601 format** (with trailing `Z`).

| Function | Input | Output | Error Condition | Description |
|----------|-------|--------|-----------------|-------------|
| `iso_timestamp` | string | ISO 8601 (UTC) | Invalid format | Validate and normalize ISO 8601 string |
| `unix_timestamp_ms` | integer/float | ISO 8601 (UTC) | Conversion fails | Convert Unix milliseconds to ISO 8601 |
| `unix_timestamp_sec` | integer/float | ISO 8601 (UTC) | Conversion fails | Convert Unix seconds to ISO 8601 |
| `jst_to_utc` | string | ISO 8601 (UTC) | Invalid format | Convert JST timestamp to UTC |

### String Conversions

| Function | Input | Output | Error Condition | Description |
|----------|-------|--------|-----------------|-------------|
| `side_from_string` | string | "buy" or "sell" | Invalid value | Normalize side/direction string |
| `uppercase` | string | string | - | Convert to uppercase |
| `lowercase` | string | string | - | Convert to lowercase |

---

## Detailed Specifications

### to_float

Convert value to floating-point number.

**Input Examples**:
- `"123.45"` → `123.45`
- `100` → `100.0`
- `"invalid"` → TransformError

### to_int

Convert value to integer.

**Input Examples**:
- `"123"` → `123`
- `100.9` → `100` (truncated)
- `"invalid"` → TransformError

### iso_timestamp

Validate and return ISO 8601 timestamp string in UTC.

Parses the input, validates it as a valid datetime, and re-formats to ensure consistent UTC ISO 8601 format with 'Z' suffix.

**Accepted Input Formats**:
- `"2024-01-01T00:00:00Z"` → `"2024-01-01T00:00:00Z"`
- `"2024-01-01T00:00:00+00:00"` → `"2024-01-01T00:00:00Z"`
- `"2024-01-01T09:00:00+09:00"` → `"2024-01-01T00:00:00Z"` (converted to UTC)

### unix_timestamp_ms

Convert Unix timestamp in milliseconds to ISO 8601 string.

**Input Examples**:
- `1704067200000` → `"2024-01-01T00:00:00Z"`
- Negative values → TransformError (OSError in some implementations)

### unix_timestamp_sec

Convert Unix timestamp in seconds to ISO 8601 string.

**Input Examples**:
- `1704067200` → `"2024-01-01T00:00:00Z"`
- Negative values → TransformError

### jst_to_utc

Convert JST (Japan Standard Time) timestamp to UTC ISO 8601.

If the input has no timezone info (naive datetime), it is assumed to be JST (+09:00).
If the input already has timezone info, it is converted to UTC.

**Input Examples**:
- `"2024-01-01T09:00:00"` (naive, assumed JST) → `"2024-01-01T00:00:00Z"`
- `"2024-01-01T09:00:00+09:00"` (explicit JST) → `"2024-01-01T00:00:00Z"`

### side_from_string

Normalize side string to lowercase "buy" or "sell".

**Mapping Rules**:

| Input | Output |
|-------|--------|
| `"buy"`, `"BUY"`, `"Buy"`, `"bid"`, `"BID"`, `"b"` | `"buy"` |
| `"sell"`, `"SELL"`, `"Sell"`, `"ask"`, `"ASK"`, `"offer"`, `"OFFER"`, `"s"`, `"a"` | `"sell"` |
| Other values | TransformError |

### uppercase / lowercase

Convert string to uppercase or lowercase respectively.

**Input Examples**:
- `uppercase("btc_jpy")` → `"BTC_JPY"`
- `lowercase("BTC_JPY")` → `"btc_jpy"`

---

## Constants

The following constants are recommended for implementations:

| Constant | Value | Description |
|----------|-------|-------------|
| `MS_PER_SECOND` | 1000 | Milliseconds per second |
| `SECONDS_PER_HOUR` | 3600 | Seconds per hour |
| `JST_UTC_OFFSET_HOURS` | 9 | JST offset from UTC in hours |

---

## Error Handling Contract

All transform functions must follow these error handling conventions:

1. **No Silent Failures**: Always raise TransformError on conversion failure
2. **Exception Chaining**: Preserve original exception with language-appropriate mechanism
   - Python: `raise TransformError(...) from e`
   - Rust: `#[source]` attribute
3. **Error Messages**: Include input value to help identify the problem

---

## Testing Requirements

Each language implementation must include tests for:

1. **Normal Cases**: Valid inputs for each function
2. **Error Cases**: Invalid inputs that should raise TransformError
3. **Edge Cases**: Boundary values, empty strings, null/None values
4. **Timestamp Edge Cases**: Negative timestamps, very large timestamps
