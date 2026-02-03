# Transform Functions Contract (Rust)

**Feature**: 004-adapter-rust
**Parent Spec**: [004-adapter-rust](../spec.md)
**Date**: 2026-02-03
**Status**: Planned

## Transforms Module

```rust
use chrono::{DateTime, Utc, TimeZone, FixedOffset};
use crate::error::TransformError;

/// Collection of common transform functions.
pub struct Transforms;

impl Transforms {
    /// Convert value to f64.
    pub fn to_float(value: &serde_json::Value) -> Result<f64, TransformError> {
        match value {
            serde_json::Value::Number(n) => n.as_f64()
                .ok_or_else(|| TransformError::new(format!("Cannot convert {:?} to float", value))),
            serde_json::Value::String(s) => s.parse::<f64>()
                .map_err(|_| TransformError::new(format!("Cannot convert {:?} to float", value))),
            _ => Err(TransformError::new(format!("Cannot convert {:?} to float", value))),
        }
    }

    /// Convert value to i64.
    pub fn to_int(value: &serde_json::Value) -> Result<i64, TransformError> {
        match value {
            serde_json::Value::Number(n) => n.as_i64()
                .ok_or_else(|| TransformError::new(format!("Cannot convert {:?} to int", value))),
            serde_json::Value::String(s) => s.parse::<i64>()
                .map_err(|_| TransformError::new(format!("Cannot convert {:?} to int", value))),
            _ => Err(TransformError::new(format!("Cannot convert {:?} to int", value))),
        }
    }

    /// Validate and return ISO 8601 timestamp in UTC.
    pub fn iso_timestamp(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;

        let dt = DateTime::parse_from_rfc3339(s)
            .map_err(|_| TransformError::new(format!("Invalid ISO timestamp: {:?}", s)))?;

        Ok(dt.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }

    /// Convert Unix milliseconds to ISO 8601.
    pub fn unix_timestamp_ms(value: &serde_json::Value) -> Result<String, TransformError> {
        let ms = Self::to_float(value)? as i64;
        let secs = ms / 1000;
        let nsecs = ((ms % 1000) * 1_000_000) as u32;

        Utc.timestamp_opt(secs, nsecs)
            .single()
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .ok_or_else(|| TransformError::new(format!("Invalid timestamp: {}", ms)))
    }

    /// Convert Unix seconds to ISO 8601.
    pub fn unix_timestamp_sec(value: &serde_json::Value) -> Result<String, TransformError> {
        let secs = Self::to_float(value)? as i64;

        Utc.timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .ok_or_else(|| TransformError::new(format!("Invalid timestamp: {}", secs)))
    }

    /// Convert JST timestamp to UTC ISO 8601.
    pub fn jst_to_utc(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;

        // Try parsing with timezone
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(dt.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%SZ").to_string());
        }

        // Parse as naive datetime and assume JST (+09:00)
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            .map_err(|_| TransformError::new(format!("Cannot parse JST timestamp: {:?}", s)))?;

        let dt = jst.from_local_datetime(&naive)
            .single()
            .ok_or_else(|| TransformError::new(format!("Invalid JST timestamp: {:?}", s)))?;

        Ok(dt.with_timezone(&Utc).format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }

    /// Normalize side string to "buy" or "sell".
    pub fn side_from_string(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;

        let normalized = s.to_lowercase();
        match normalized.as_str() {
            "buy" | "bid" | "b" => Ok("buy".to_string()),
            "sell" | "ask" | "offer" | "s" | "a" => Ok("sell".to_string()),
            _ => Err(TransformError::new(format!("Cannot normalize side value: {:?}", s))),
        }
    }

    /// Convert string to uppercase.
    pub fn uppercase(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;
        Ok(s.to_uppercase())
    }

    /// Convert string to lowercase.
    pub fn lowercase(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;
        Ok(s.to_lowercase())
    }
}
```

## Constants

```rust
/// Milliseconds per second.
pub const MS_PER_SECOND: i64 = 1000;

/// Seconds per hour.
pub const SECONDS_PER_HOUR: i64 = 3600;

/// JST offset from UTC in hours.
pub const JST_UTC_OFFSET_HOURS: i32 = 9;
```

## Transform Function Factory Pattern

For use with ModelMapping, transform functions can be wrapped as closures:

```rust
impl Transforms {
    /// Returns a transform function that converts to float.
    pub fn to_float_fn() -> TransformFn {
        Arc::new(|value| {
            Self::to_float(value).map(|f| serde_json::json!(f))
        })
    }

    /// Returns a transform function that converts Unix ms to ISO 8601.
    pub fn unix_timestamp_ms_fn() -> TransformFn {
        Arc::new(|value| {
            Self::unix_timestamp_ms(value).map(|s| serde_json::json!(s))
        })
    }

    // ... similar for other transforms
}
```

## Error Handling

All transform functions follow the error handling contract:

1. **No Silent Failures**: Return `Err(TransformError)` on conversion failure
2. **Source Preservation**: TransformError includes context for debugging
3. **Error Messages**: Include input value to help identify the problem

```rust
// Implementation pattern
match value {
    serde_json::Value::String(s) => s.parse::<f64>()
        .map_err(|e| TransformError::new(format!("Cannot convert {:?} to float: {}", value, e))),
    _ => Err(TransformError::new(format!("Cannot convert {:?} to float", value))),
}
```

## Usage Example

```rust
use marketschema_adapters::{ModelMapping, Transforms};

let mapping = ModelMapping::new("bid", "price.bid")
    .with_transform(Transforms::to_float_fn());

let source = serde_json::json!({
    "price": {
        "bid": "123.45"
    }
});

let result = mapping.apply(&source).unwrap();
assert_eq!(result, serde_json::json!(123.45));
```
