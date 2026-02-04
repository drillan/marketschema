//! Unit tests for the Transforms module.
//!
//! Tests cover all 9 transform functions with both success and error cases.

use marketschema_adapters::Transforms;
use serde_json::json;

// =============================================================================
// Constants Tests
// =============================================================================

mod constants {
    use marketschema_adapters::{JST_UTC_OFFSET_HOURS, MS_PER_SECOND, NS_PER_MS, SECONDS_PER_HOUR};

    #[test]
    fn ms_per_second_is_1000() {
        assert_eq!(MS_PER_SECOND, 1000);
    }

    #[test]
    fn ns_per_ms_is_1_000_000() {
        assert_eq!(NS_PER_MS, 1_000_000);
    }

    #[test]
    fn seconds_per_hour_is_3600() {
        assert_eq!(SECONDS_PER_HOUR, 3600);
    }

    #[test]
    fn jst_utc_offset_hours_is_9() {
        assert_eq!(JST_UTC_OFFSET_HOURS, 9);
    }
}

// =============================================================================
// to_float Tests
// =============================================================================

mod to_float {
    use super::*;

    #[test]
    fn converts_number_to_f64() {
        let result = Transforms::to_float(&json!(123.45));
        assert_eq!(result.unwrap(), 123.45);
    }

    #[test]
    fn converts_integer_to_f64() {
        let result = Transforms::to_float(&json!(42));
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn converts_string_number_to_f64() {
        let result = Transforms::to_float(&json!("123.45"));
        assert_eq!(result.unwrap(), 123.45);
    }

    #[test]
    fn converts_string_integer_to_f64() {
        let result = Transforms::to_float(&json!("42"));
        assert_eq!(result.unwrap(), 42.0);
    }

    #[test]
    fn errors_on_invalid_string() {
        let result = Transforms::to_float(&json!("invalid"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid"));
    }

    #[test]
    fn errors_on_null() {
        let result = Transforms::to_float(&json!(null));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_boolean() {
        let result = Transforms::to_float(&json!(true));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_object() {
        let result = Transforms::to_float(&json!({"key": "value"}));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_array() {
        let result = Transforms::to_float(&json!([1, 2, 3]));
        assert!(result.is_err());
    }

    #[test]
    fn to_float_fn_returns_json_value() {
        let transform = Transforms::to_float_fn();
        let result = transform(&json!("123.45")).unwrap();
        assert_eq!(result, json!(123.45));
    }
}

// =============================================================================
// to_int Tests
// =============================================================================

mod to_int {
    use super::*;

    #[test]
    fn converts_integer_to_i64() {
        let result = Transforms::to_int(&json!(42));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn converts_negative_integer_to_i64() {
        let result = Transforms::to_int(&json!(-100));
        assert_eq!(result.unwrap(), -100);
    }

    #[test]
    fn converts_string_integer_to_i64() {
        let result = Transforms::to_int(&json!("42"));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn converts_negative_string_integer_to_i64() {
        let result = Transforms::to_int(&json!("-100"));
        assert_eq!(result.unwrap(), -100);
    }

    #[test]
    fn errors_on_float_number() {
        // serde_json::Number cannot hold f64 as i64, so this should fail
        let result = Transforms::to_int(&json!(123.45));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_invalid_string() {
        let result = Transforms::to_int(&json!("invalid"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid"));
    }

    #[test]
    fn errors_on_null() {
        let result = Transforms::to_int(&json!(null));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_boolean() {
        let result = Transforms::to_int(&json!(false));
        assert!(result.is_err());
    }

    #[test]
    fn to_int_fn_returns_json_value() {
        let transform = Transforms::to_int_fn();
        let result = transform(&json!("42")).unwrap();
        assert_eq!(result, json!(42));
    }
}

// =============================================================================
// iso_timestamp Tests
// =============================================================================

mod iso_timestamp {
    use super::*;

    #[test]
    fn normalizes_utc_timestamp_with_z_suffix() {
        let result = Transforms::iso_timestamp(&json!("2024-01-01T00:00:00Z"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_positive_offset_to_utc() {
        let result = Transforms::iso_timestamp(&json!("2024-01-01T09:00:00+09:00"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_negative_offset_to_utc() {
        let result = Transforms::iso_timestamp(&json!("2023-12-31T19:00:00-05:00"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn preserves_milliseconds() {
        let result = Transforms::iso_timestamp(&json!("2024-01-01T00:00:00.123Z"));
        assert!(result.unwrap().contains("123"));
    }

    #[test]
    fn errors_on_invalid_format() {
        let result = Transforms::iso_timestamp(&json!("2024-01-01"));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_non_string() {
        let result = Transforms::iso_timestamp(&json!(12345));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_null() {
        let result = Transforms::iso_timestamp(&json!(null));
        assert!(result.is_err());
    }

    #[test]
    fn iso_timestamp_fn_returns_json_value() {
        let transform = Transforms::iso_timestamp_fn();
        let result = transform(&json!("2024-01-01T00:00:00Z")).unwrap();
        assert_eq!(result, json!("2024-01-01T00:00:00Z"));
    }
}

// =============================================================================
// unix_timestamp_ms Tests
// =============================================================================

mod unix_timestamp_ms {
    use super::*;

    #[test]
    fn converts_epoch_to_iso() {
        let result = Transforms::unix_timestamp_ms(&json!(0));
        assert_eq!(result.unwrap(), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn converts_known_timestamp_to_iso() {
        // 2024-01-01T00:00:00Z in milliseconds
        let result = Transforms::unix_timestamp_ms(&json!(1704067200000_i64));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_string_timestamp() {
        let result = Transforms::unix_timestamp_ms(&json!("1704067200000"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn errors_on_negative_timestamp() {
        let result = Transforms::unix_timestamp_ms(&json!(-1000));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().to_lowercase().contains("negative"));
    }

    #[test]
    fn errors_on_invalid_string() {
        let result = Transforms::unix_timestamp_ms(&json!("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn unix_timestamp_ms_fn_returns_json_value() {
        let transform = Transforms::unix_timestamp_ms_fn();
        let result = transform(&json!(1704067200000_i64)).unwrap();
        assert_eq!(result, json!("2024-01-01T00:00:00Z"));
    }
}

// =============================================================================
// unix_timestamp_sec Tests
// =============================================================================

mod unix_timestamp_sec {
    use super::*;

    #[test]
    fn converts_epoch_to_iso() {
        let result = Transforms::unix_timestamp_sec(&json!(0));
        assert_eq!(result.unwrap(), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn converts_known_timestamp_to_iso() {
        // 2024-01-01T00:00:00Z in seconds
        let result = Transforms::unix_timestamp_sec(&json!(1704067200));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_string_timestamp() {
        let result = Transforms::unix_timestamp_sec(&json!("1704067200"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn errors_on_negative_timestamp() {
        let result = Transforms::unix_timestamp_sec(&json!(-1));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().to_lowercase().contains("negative"));
    }

    #[test]
    fn errors_on_invalid_string() {
        let result = Transforms::unix_timestamp_sec(&json!("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn unix_timestamp_sec_fn_returns_json_value() {
        let transform = Transforms::unix_timestamp_sec_fn();
        let result = transform(&json!(1704067200)).unwrap();
        assert_eq!(result, json!("2024-01-01T00:00:00Z"));
    }
}

// =============================================================================
// jst_to_utc Tests
// =============================================================================

mod jst_to_utc {
    use super::*;

    #[test]
    fn converts_naive_jst_to_utc() {
        // JST 2024-01-01T09:00:00 -> UTC 2024-01-01T00:00:00Z
        let result = Transforms::jst_to_utc(&json!("2024-01-01T09:00:00"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_naive_jst_with_space_separator() {
        let result = Transforms::jst_to_utc(&json!("2024-01-01 09:00:00"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn passes_through_rfc3339_timestamp_with_timezone() {
        // Already has timezone, should convert to UTC
        let result = Transforms::jst_to_utc(&json!("2024-01-01T09:00:00+09:00"));
        assert_eq!(result.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn converts_midnight_jst_to_previous_day_utc() {
        // JST 2024-01-01T00:00:00 -> UTC 2023-12-31T15:00:00Z
        let result = Transforms::jst_to_utc(&json!("2024-01-01T00:00:00"));
        assert_eq!(result.unwrap(), "2023-12-31T15:00:00Z");
    }

    #[test]
    fn errors_on_invalid_format() {
        let result = Transforms::jst_to_utc(&json!("2024-01-01"));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_non_string() {
        let result = Transforms::jst_to_utc(&json!(12345));
        assert!(result.is_err());
    }

    #[test]
    fn jst_to_utc_fn_returns_json_value() {
        let transform = Transforms::jst_to_utc_fn();
        let result = transform(&json!("2024-01-01T09:00:00")).unwrap();
        assert_eq!(result, json!("2024-01-01T00:00:00Z"));
    }
}

// =============================================================================
// side_from_string Tests
// =============================================================================

mod side_from_string {
    use super::*;

    // Buy side variations
    #[test]
    fn normalizes_buy_to_buy() {
        assert_eq!(Transforms::side_from_string(&json!("buy")).unwrap(), "buy");
    }

    #[test]
    fn normalizes_bid_to_buy() {
        assert_eq!(Transforms::side_from_string(&json!("bid")).unwrap(), "buy");
    }

    #[test]
    fn normalizes_b_to_buy() {
        assert_eq!(Transforms::side_from_string(&json!("b")).unwrap(), "buy");
    }

    #[test]
    fn normalizes_uppercase_buy_to_buy() {
        assert_eq!(Transforms::side_from_string(&json!("BUY")).unwrap(), "buy");
    }

    #[test]
    fn normalizes_mixed_case_buy_to_buy() {
        assert_eq!(Transforms::side_from_string(&json!("Buy")).unwrap(), "buy");
    }

    // Sell side variations
    #[test]
    fn normalizes_sell_to_sell() {
        assert_eq!(
            Transforms::side_from_string(&json!("sell")).unwrap(),
            "sell"
        );
    }

    #[test]
    fn normalizes_ask_to_sell() {
        assert_eq!(Transforms::side_from_string(&json!("ask")).unwrap(), "sell");
    }

    #[test]
    fn normalizes_offer_to_sell() {
        assert_eq!(
            Transforms::side_from_string(&json!("offer")).unwrap(),
            "sell"
        );
    }

    #[test]
    fn normalizes_s_to_sell() {
        assert_eq!(Transforms::side_from_string(&json!("s")).unwrap(), "sell");
    }

    #[test]
    fn normalizes_a_to_sell() {
        assert_eq!(Transforms::side_from_string(&json!("a")).unwrap(), "sell");
    }

    #[test]
    fn normalizes_uppercase_sell_to_sell() {
        assert_eq!(
            Transforms::side_from_string(&json!("SELL")).unwrap(),
            "sell"
        );
    }

    // Error cases
    #[test]
    fn errors_on_unknown_side() {
        let result = Transforms::side_from_string(&json!("exchange"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("exchange"));
    }

    #[test]
    fn errors_on_empty_string() {
        let result = Transforms::side_from_string(&json!(""));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_non_string() {
        let result = Transforms::side_from_string(&json!(1));
        assert!(result.is_err());
    }

    #[test]
    fn side_from_string_fn_returns_json_value() {
        let transform = Transforms::side_from_string_fn();
        let result = transform(&json!("BUY")).unwrap();
        assert_eq!(result, json!("buy"));
    }
}

// =============================================================================
// uppercase Tests
// =============================================================================

mod uppercase {
    use super::*;

    #[test]
    fn converts_lowercase_to_uppercase() {
        let result = Transforms::uppercase(&json!("hello"));
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn leaves_uppercase_unchanged() {
        let result = Transforms::uppercase(&json!("HELLO"));
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn converts_mixed_case() {
        let result = Transforms::uppercase(&json!("HeLLo"));
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn handles_empty_string() {
        let result = Transforms::uppercase(&json!(""));
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn handles_unicode() {
        let result = Transforms::uppercase(&json!("café"));
        assert_eq!(result.unwrap(), "CAFÉ");
    }

    #[test]
    fn errors_on_non_string() {
        let result = Transforms::uppercase(&json!(123));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_null() {
        let result = Transforms::uppercase(&json!(null));
        assert!(result.is_err());
    }

    #[test]
    fn uppercase_fn_returns_json_value() {
        let transform = Transforms::uppercase_fn();
        let result = transform(&json!("hello")).unwrap();
        assert_eq!(result, json!("HELLO"));
    }
}

// =============================================================================
// lowercase Tests
// =============================================================================

mod lowercase {
    use super::*;

    #[test]
    fn converts_uppercase_to_lowercase() {
        let result = Transforms::lowercase(&json!("HELLO"));
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn leaves_lowercase_unchanged() {
        let result = Transforms::lowercase(&json!("hello"));
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn converts_mixed_case() {
        let result = Transforms::lowercase(&json!("HeLLo"));
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn handles_empty_string() {
        let result = Transforms::lowercase(&json!(""));
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn handles_unicode() {
        let result = Transforms::lowercase(&json!("CAFÉ"));
        assert_eq!(result.unwrap(), "café");
    }

    #[test]
    fn errors_on_non_string() {
        let result = Transforms::lowercase(&json!(123));
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_null() {
        let result = Transforms::lowercase(&json!(null));
        assert!(result.is_err());
    }

    #[test]
    fn lowercase_fn_returns_json_value() {
        let transform = Transforms::lowercase_fn();
        let result = transform(&json!("HELLO")).unwrap();
        assert_eq!(result, json!("hello"));
    }
}

// =============================================================================
// TransformFn Trait Tests
// =============================================================================

mod transform_fn_traits {
    use super::*;

    #[test]
    fn transform_fn_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<marketschema_adapters::TransformFn>();
    }

    #[test]
    fn transform_fn_is_clone() {
        let transform = Transforms::to_float_fn();
        let _cloned = transform.clone();
    }

    #[test]
    fn transform_fn_can_be_stored_in_vec() {
        let transforms: Vec<marketschema_adapters::TransformFn> = vec![
            Transforms::to_float_fn(),
            Transforms::to_int_fn(),
            Transforms::uppercase_fn(),
        ];
        assert_eq!(transforms.len(), 3);
    }
}
