//! Unit tests for the ModelMapping module.
//!
//! Test category identifiers (T027-T030) reference the adapter specification
//! test requirements defined in docs/specs/004-adapter-rust/spec.md.
//!
//! Tests cover:
//! - T027: Constructor tests
//! - T028: Builder pattern tests
//! - T029: Dot notation path access tests
//! - T030: apply() method tests

use marketschema_adapters::{ModelMapping, TransformFn, Transforms};
use serde_json::{Value, json};

// =============================================================================
// T027: Constructor Tests
// =============================================================================

mod constructor {
    use super::*;

    #[test]
    fn new_creates_mapping_with_target_and_source_fields() {
        let mapping = ModelMapping::new("bid", "bid_price");
        assert_eq!(mapping.target_field, "bid");
        assert_eq!(mapping.source_field, "bid_price");
    }

    #[test]
    fn new_sets_required_to_true_by_default() {
        let mapping = ModelMapping::new("bid", "bid_price");
        assert!(mapping.required);
    }

    #[test]
    fn new_sets_transform_to_none_by_default() {
        let mapping = ModelMapping::new("bid", "bid_price");
        assert!(mapping.transform.is_none());
    }

    #[test]
    fn new_sets_default_to_none_by_default() {
        let mapping = ModelMapping::new("bid", "bid_price");
        assert!(mapping.default.is_none());
    }

    #[test]
    fn new_accepts_string_types() {
        let mapping = ModelMapping::new(String::from("bid"), String::from("bid_price"));
        assert_eq!(mapping.target_field, "bid");
        assert_eq!(mapping.source_field, "bid_price");
    }

    #[test]
    fn new_accepts_str_references() {
        let target: &str = "bid";
        let source: &str = "bid_price";
        let mapping = ModelMapping::new(target, source);
        assert_eq!(mapping.target_field, "bid");
        assert_eq!(mapping.source_field, "bid_price");
    }
}

// =============================================================================
// T028: Builder Pattern Tests
// =============================================================================

mod builder_pattern {
    use super::*;

    #[test]
    fn with_transform_sets_transform_function() {
        let transform = Transforms::to_float_fn();
        let mapping = ModelMapping::new("price", "price_str").with_transform(transform);
        assert!(mapping.transform.is_some());
    }

    #[test]
    fn with_default_sets_default_value() {
        let mapping = ModelMapping::new("price", "price_str").with_default(json!(0.0));
        assert_eq!(mapping.default, Some(json!(0.0)));
    }

    #[test]
    fn optional_sets_required_to_false() {
        let mapping = ModelMapping::new("price", "price_str").optional();
        assert!(!mapping.required);
    }

    #[test]
    fn builder_methods_can_be_chained() {
        let transform = Transforms::to_float_fn();
        let mapping = ModelMapping::new("price", "price_str")
            .with_transform(transform)
            .with_default(json!(0.0))
            .optional();

        assert!(mapping.transform.is_some());
        assert_eq!(mapping.default, Some(json!(0.0)));
        assert!(!mapping.required);
    }

    #[test]
    fn builder_methods_consume_self_and_return_self() {
        // This test verifies the builder pattern uses consuming self
        let mapping1 = ModelMapping::new("a", "b");
        let mapping2 = mapping1.optional();
        // mapping1 is moved, mapping2 is the new owner
        assert!(!mapping2.required);
    }

    #[test]
    fn with_transform_multiple_times_replaces_previous() {
        let transform1 = Transforms::to_float_fn();
        let transform2 = Transforms::to_int_fn();
        let mapping = ModelMapping::new("value", "val")
            .with_transform(transform1)
            .with_transform(transform2);

        // The second transform should replace the first
        // Verify by applying to a value that to_int can handle
        let result = mapping.transform.as_ref().unwrap()(&json!("42"));
        assert_eq!(result.unwrap(), json!(42));
    }

    #[test]
    fn with_default_multiple_times_replaces_previous() {
        let mapping = ModelMapping::new("value", "val")
            .with_default(json!(0.0))
            .with_default(json!(100.0));

        assert_eq!(mapping.default, Some(json!(100.0)));
    }
}

// =============================================================================
// T029: Dot Notation Path Access Tests
// =============================================================================

mod dot_notation {
    use super::*;

    #[test]
    fn accesses_top_level_field() {
        let mapping = ModelMapping::new("price", "price");
        let source = json!({"price": 100.0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(100.0));
    }

    #[test]
    fn accesses_nested_field_with_dot_notation() {
        let mapping = ModelMapping::new("bid", "price.bid");
        let source = json!({"price": {"bid": 100.0}});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(100.0));
    }

    #[test]
    fn accesses_deeply_nested_field() {
        let mapping = ModelMapping::new("value", "a.b.c.d");
        let source = json!({"a": {"b": {"c": {"d": 42}}}});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(42));
    }

    #[test]
    fn returns_error_when_intermediate_key_missing() {
        let mapping = ModelMapping::new("value", "a.b.c");
        let source = json!({"a": {"x": 1}});
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn returns_error_when_top_level_key_missing() {
        let mapping = ModelMapping::new("value", "missing");
        let source = json!({"other": 1});
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn handles_empty_source_field_path() {
        // Empty path does NOT return the entire source object.
        // Instead, split('.') produces [""], which tries to get key "" (empty string),
        // causing a lookup failure.
        let mapping = ModelMapping::new("value", "");
        let source = json!({"a": 1});
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn handles_field_with_special_characters_in_key() {
        // Fields with dots in their names cannot be accessed with current implementation
        // This documents expected behavior
        let mapping = ModelMapping::new("value", "price");
        let source = json!({"price": {"with.dot": 100.0}});
        let result = mapping.apply(&source).unwrap();
        // Returns the nested object since "price" exists
        assert_eq!(result, json!({"with.dot": 100.0}));
    }

    #[test]
    fn accesses_array_elements_not_supported() {
        // Array index access is not currently supported
        let mapping = ModelMapping::new("value", "prices.0");
        let source = json!({"prices": [100.0, 200.0]});
        // This should fail because arrays don't have key "0"
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }
}

// =============================================================================
// T030: apply() Method Tests
// =============================================================================

mod apply_method {
    use super::*;

    // --- Basic apply tests ---

    #[test]
    fn apply_returns_value_when_found() {
        let mapping = ModelMapping::new("price", "price");
        let source = json!({"price": 100.0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(100.0));
    }

    #[test]
    fn apply_returns_string_value() {
        let mapping = ModelMapping::new("symbol", "symbol");
        let source = json!({"symbol": "BTC/USD"});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!("BTC/USD"));
    }

    #[test]
    fn apply_returns_object_value() {
        let mapping = ModelMapping::new("data", "data");
        let source = json!({"data": {"a": 1, "b": 2}});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn apply_returns_array_value() {
        let mapping = ModelMapping::new("prices", "prices");
        let source = json!({"prices": [100.0, 200.0]});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!([100.0, 200.0]));
    }

    // --- Required field tests ---

    #[test]
    fn apply_returns_error_when_required_field_missing() {
        let mapping = ModelMapping::new("price", "price");
        let source = json!({"other": 100.0});
        let result = mapping.apply(&source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("price"));
    }

    #[test]
    fn apply_returns_error_when_required_field_is_null() {
        let mapping = ModelMapping::new("price", "price");
        let source = json!({"price": null});
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    // --- Optional field tests ---

    #[test]
    fn apply_returns_null_when_optional_field_missing() {
        let mapping = ModelMapping::new("price", "price").optional();
        let source = json!({"other": 100.0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn apply_returns_null_when_optional_field_is_null() {
        let mapping = ModelMapping::new("price", "price").optional();
        let source = json!({"price": null});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, Value::Null);
    }

    // --- Default value tests ---

    #[test]
    fn apply_returns_default_when_field_missing() {
        let mapping = ModelMapping::new("price", "price").with_default(json!(0.0));
        let source = json!({"other": 100.0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(0.0));
    }

    #[test]
    fn apply_returns_default_when_field_is_null() {
        let mapping = ModelMapping::new("price", "price").with_default(json!(0.0));
        let source = json!({"price": null});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(0.0));
    }

    #[test]
    fn apply_returns_actual_value_over_default() {
        let mapping = ModelMapping::new("price", "price").with_default(json!(0.0));
        let source = json!({"price": 100.0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(100.0));
    }

    #[test]
    fn apply_with_default_and_optional_returns_default_when_missing() {
        let mapping = ModelMapping::new("price", "price")
            .with_default(json!(0.0))
            .optional();
        let source = json!({"other": 100.0});
        let result = mapping.apply(&source).unwrap();
        // Default takes precedence over returning null
        assert_eq!(result, json!(0.0));
    }

    // --- Transform function tests ---

    #[test]
    fn apply_with_transform_converts_value() {
        let mapping =
            ModelMapping::new("price", "price_str").with_transform(Transforms::to_float_fn());
        let source = json!({"price_str": "123.45"});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(123.45));
    }

    #[test]
    fn apply_with_transform_propagates_transform_error() {
        let mapping =
            ModelMapping::new("price", "price_str").with_transform(Transforms::to_float_fn());
        let source = json!({"price_str": "invalid"});
        let result = mapping.apply(&source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Error message should mention the target field
        assert!(err.to_string().contains("price"));
    }

    #[test]
    fn apply_transform_not_applied_when_using_default() {
        let mapping = ModelMapping::new("price", "price_str")
            .with_transform(Transforms::to_float_fn())
            .with_default(json!(0.0));
        let source = json!({"other": "value"});
        let result = mapping.apply(&source).unwrap();
        // Default is returned directly, transform is not applied
        assert_eq!(result, json!(0.0));
    }

    #[test]
    fn apply_with_timestamp_transform() {
        let mapping =
            ModelMapping::new("timestamp", "ts").with_transform(Transforms::unix_timestamp_ms_fn());
        let source = json!({"ts": 1704067200000_i64});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn apply_with_side_transform() {
        let mapping = ModelMapping::new("side", "direction")
            .with_transform(Transforms::side_from_string_fn());
        let source = json!({"direction": "BUY"});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!("buy"));
    }

    // --- Nested access with transform tests ---

    #[test]
    fn apply_nested_path_with_transform() {
        let mapping =
            ModelMapping::new("bid", "price.bid").with_transform(Transforms::to_float_fn());
        let source = json!({"price": {"bid": "100.50"}});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(100.50));
    }

    // --- Edge cases ---

    #[test]
    fn apply_handles_zero_value() {
        let mapping = ModelMapping::new("value", "value");
        let source = json!({"value": 0});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(0));
    }

    #[test]
    fn apply_handles_empty_string_value() {
        let mapping = ModelMapping::new("value", "value");
        let source = json!({"value": ""});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(""));
    }

    #[test]
    fn apply_handles_false_boolean_value() {
        let mapping = ModelMapping::new("flag", "flag");
        let source = json!({"flag": false});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!(false));
    }

    #[test]
    fn apply_handles_empty_object_value() {
        let mapping = ModelMapping::new("data", "data");
        let source = json!({"data": {}});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn apply_handles_empty_array_value() {
        let mapping = ModelMapping::new("items", "items");
        let source = json!({"items": []});
        let result = mapping.apply(&source).unwrap();
        assert_eq!(result, json!([]));
    }

    // --- Non-object source tests ---

    #[test]
    fn apply_returns_error_when_source_is_array() {
        let mapping = ModelMapping::new("value", "value");
        let source = json!([1, 2, 3]);
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn apply_returns_error_when_source_is_primitive_string() {
        let mapping = ModelMapping::new("value", "value");
        let source = json!("just a string");
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn apply_returns_error_when_source_is_primitive_number() {
        let mapping = ModelMapping::new("value", "value");
        let source = json!(42);
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }

    #[test]
    fn apply_returns_error_when_source_is_null() {
        let mapping = ModelMapping::new("value", "value");
        let source = Value::Null;
        let result = mapping.apply(&source);
        assert!(result.is_err());
    }
}

// =============================================================================
// Clone and Other Trait Tests
// =============================================================================

mod traits {
    use super::*;

    #[test]
    fn model_mapping_is_clone() {
        let transform = Transforms::to_float_fn();
        let mapping = ModelMapping::new("a", "b")
            .with_transform(transform)
            .with_default(json!(0.0))
            .optional();

        let cloned = mapping.clone();
        assert_eq!(cloned.target_field, mapping.target_field);
        assert_eq!(cloned.source_field, mapping.source_field);
        assert_eq!(cloned.default, mapping.default);
        assert_eq!(cloned.required, mapping.required);

        // Verify cloned transform actually works
        let source = json!({"b": "123.45"});
        let result = cloned.apply(&source).unwrap();
        assert_eq!(result, json!(123.45));
    }

    #[test]
    fn model_mapping_fields_are_public() {
        // This test verifies fields are public by directly accessing them
        let mapping = ModelMapping::new("target", "source");
        let _target: &str = &mapping.target_field;
        let _source: &str = &mapping.source_field;
        let _required: bool = mapping.required;
        let _default: &Option<Value> = &mapping.default;
        let _transform: &Option<TransformFn> = &mapping.transform;
    }

    #[test]
    fn model_mapping_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ModelMapping>();
    }

    #[test]
    fn model_mapping_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ModelMapping>();
    }
}
