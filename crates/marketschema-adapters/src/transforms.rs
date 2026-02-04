//! Common transformation functions for data conversion.

use crate::error::TransformError;
use crate::mapping::TransformFn;
use serde_json::Value;
use std::sync::Arc;

/// Struct providing common transformation functions as associated functions.
pub struct Transforms;

impl Transforms {
    /// Converts a value to a floating-point number.
    pub fn to_float(value: &Value) -> Result<f64, TransformError> {
        match value {
            Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| TransformError::new(format!("Cannot convert number to f64: {}", n))),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| TransformError::new(format!("Cannot parse string as f64: '{}'", s))),
            _ => Err(TransformError::new(format!(
                "Cannot convert to f64: {:?}",
                value
            ))),
        }
    }

    /// Returns a TransformFn that converts values to f64.
    pub fn to_float_fn() -> TransformFn {
        Arc::new(|v| Self::to_float(v).map(Value::from))
    }

    /// Converts a value to an integer.
    pub fn to_int(value: &Value) -> Result<i64, TransformError> {
        match value {
            Value::Number(n) => n
                .as_i64()
                .ok_or_else(|| TransformError::new(format!("Cannot convert number to i64: {}", n))),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| TransformError::new(format!("Cannot parse string as i64: '{}'", s))),
            _ => Err(TransformError::new(format!(
                "Cannot convert to i64: {:?}",
                value
            ))),
        }
    }

    /// Returns a TransformFn that converts values to i64.
    pub fn to_int_fn() -> TransformFn {
        Arc::new(|v| Self::to_int(v).map(Value::from))
    }

    /// Converts an ISO 8601 timestamp string to UTC normalized format.
    ///
    /// Uses RFC 3339 format with automatic sub-second precision to preserve
    /// millisecond/microsecond information from the input.
    pub fn iso_timestamp(value: &Value) -> Result<String, TransformError> {
        let s = value.as_str().ok_or_else(|| {
            TransformError::new(format!("Expected string for timestamp: {:?}", value))
        })?;

        // Parse and re-format to ensure UTC with Z suffix
        let dt = chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|e| TransformError::new(format!("Invalid ISO timestamp '{}': {}", s, e)))?;

        // Use RFC 3339 format with automatic sub-second precision
        Ok(dt
            .with_timezone(&chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true))
    }

    /// Returns a TransformFn for ISO timestamp normalization.
    pub fn iso_timestamp_fn() -> TransformFn {
        Arc::new(|v| Self::iso_timestamp(v).map(Value::from))
    }

    /// Converts Unix milliseconds timestamp to ISO 8601 UTC string.
    pub fn unix_timestamp_ms(value: &Value) -> Result<String, TransformError> {
        let ms = Self::to_int(value)?;
        if ms < 0 {
            return Err(TransformError::new(format!(
                "Negative timestamp not allowed: {}",
                ms
            )));
        }

        const MS_PER_SECOND: i64 = 1000;
        let secs = ms / MS_PER_SECOND;
        let nsecs = ((ms % MS_PER_SECOND) * 1_000_000) as u32;

        let dt = chrono::DateTime::from_timestamp(secs, nsecs)
            .ok_or_else(|| TransformError::new(format!("Invalid Unix timestamp ms: {}", ms)))?;

        Ok(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }

    /// Returns a TransformFn for Unix millisecond timestamp conversion.
    pub fn unix_timestamp_ms_fn() -> TransformFn {
        Arc::new(|v| Self::unix_timestamp_ms(v).map(Value::from))
    }

    /// Converts Unix seconds timestamp to ISO 8601 UTC string.
    pub fn unix_timestamp_sec(value: &Value) -> Result<String, TransformError> {
        let secs = Self::to_int(value)?;
        if secs < 0 {
            return Err(TransformError::new(format!(
                "Negative timestamp not allowed: {}",
                secs
            )));
        }

        let dt = chrono::DateTime::from_timestamp(secs, 0)
            .ok_or_else(|| TransformError::new(format!("Invalid Unix timestamp sec: {}", secs)))?;

        Ok(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
    }

    /// Returns a TransformFn for Unix seconds timestamp conversion.
    pub fn unix_timestamp_sec_fn() -> TransformFn {
        Arc::new(|v| Self::unix_timestamp_sec(v).map(Value::from))
    }

    /// Converts JST (Japan Standard Time) timestamp to UTC.
    pub fn jst_to_utc(value: &Value) -> Result<String, TransformError> {
        let s = value.as_str().ok_or_else(|| {
            TransformError::new(format!("Expected string for timestamp: {:?}", value))
        })?;

        const JST_UTC_OFFSET_HOURS: i32 = 9;

        // Try parsing with timezone first
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
            return Ok(dt
                .with_timezone(&chrono::Utc)
                .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true));
        }

        // Parse as naive datetime and assume JST
        let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
            .map_err(|e| TransformError::new(format!("Invalid datetime '{}': {}", s, e)))?;

        let jst = chrono::FixedOffset::east_opt(JST_UTC_OFFSET_HOURS * 3600).ok_or_else(|| {
            TransformError::new(format!("Invalid JST offset: {} hours", JST_UTC_OFFSET_HOURS))
        })?;
        let dt = naive
            .and_local_timezone(jst)
            .single()
            .ok_or_else(|| TransformError::new(format!("Ambiguous datetime: {}", s)))?;

        Ok(dt
            .with_timezone(&chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true))
    }

    /// Returns a TransformFn for JST to UTC conversion.
    pub fn jst_to_utc_fn() -> TransformFn {
        Arc::new(|v| Self::jst_to_utc(v).map(Value::from))
    }

    /// Normalizes trade side strings to "buy" or "sell".
    pub fn side_from_string(value: &Value) -> Result<String, TransformError> {
        let s = value
            .as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string for side: {:?}", value)))?;

        match s.to_lowercase().as_str() {
            "buy" | "bid" | "long" => Ok("buy".to_string()),
            "sell" | "ask" | "short" => Ok("sell".to_string()),
            _ => Err(TransformError::new(format!(
                "Cannot normalize side value: '{}'",
                s
            ))),
        }
    }

    /// Returns a TransformFn for side normalization.
    pub fn side_from_string_fn() -> TransformFn {
        Arc::new(|v| Self::side_from_string(v).map(Value::from))
    }

    /// Converts a string to uppercase.
    pub fn uppercase(value: &Value) -> Result<String, TransformError> {
        let s = value
            .as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string: {:?}", value)))?;
        Ok(s.to_uppercase())
    }

    /// Returns a TransformFn for uppercase conversion.
    pub fn uppercase_fn() -> TransformFn {
        Arc::new(|v| Self::uppercase(v).map(Value::from))
    }

    /// Converts a string to lowercase.
    pub fn lowercase(value: &Value) -> Result<String, TransformError> {
        let s = value
            .as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string: {:?}", value)))?;
        Ok(s.to_lowercase())
    }

    /// Returns a TransformFn for lowercase conversion.
    pub fn lowercase_fn() -> TransformFn {
        Arc::new(|v| Self::lowercase(v).map(Value::from))
    }
}
