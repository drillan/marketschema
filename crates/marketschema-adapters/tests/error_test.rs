//! Tests for error types.
//!
//! These tests verify the acceptance criteria for Issue #139:
//! - All error types derive `thiserror::Error`
//! - `#[from]` conversions work correctly

use marketschema_adapters::{AdapterError, MappingError, TransformError};

// ============================================================================
// TransformError Tests
// ============================================================================

#[test]
fn transform_error_new_creates_error_with_message() {
    let error = TransformError::new("test error message");
    assert_eq!(error.to_string(), "test error message");
}

#[test]
fn transform_error_new_accepts_string() {
    let error = TransformError::new(String::from("string message"));
    assert_eq!(error.to_string(), "string message");
}

#[test]
fn transform_error_implements_std_error() {
    let error = TransformError::new("test");
    let _: &dyn std::error::Error = &error;
}

#[test]
fn transform_error_is_clone() {
    let error = TransformError::new("test");
    let cloned = error.clone();
    assert_eq!(error.to_string(), cloned.to_string());
}

#[test]
fn transform_error_is_debug() {
    let error = TransformError::new("test");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("TransformError"));
}

// ============================================================================
// MappingError Tests
// ============================================================================

#[test]
fn mapping_error_new_creates_error_with_message() {
    let error = MappingError::new("test error message");
    assert_eq!(error.to_string(), "test error message");
}

#[test]
fn mapping_error_new_accepts_string() {
    let error = MappingError::new(String::from("string message"));
    assert_eq!(error.to_string(), "string message");
}

#[test]
fn mapping_error_implements_std_error() {
    let error = MappingError::new("test");
    let _: &dyn std::error::Error = &error;
}

#[test]
fn mapping_error_is_clone() {
    let error = MappingError::new("test");
    let cloned = error.clone();
    assert_eq!(error.to_string(), cloned.to_string());
}

#[test]
fn mapping_error_is_debug() {
    let error = MappingError::new("test");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("MappingError"));
}

// ============================================================================
// AdapterError Tests
// ============================================================================

#[test]
fn adapter_error_general_displays_message() {
    let error = AdapterError::General("general error".to_string());
    assert_eq!(error.to_string(), "general error");
}

#[test]
fn adapter_error_duplicate_registration_displays_message() {
    let error = AdapterError::DuplicateRegistration("test_adapter".to_string());
    assert!(error.to_string().contains("test_adapter"));
    assert!(error.to_string().contains("already registered"));
}

#[test]
fn adapter_error_implements_std_error() {
    let error = AdapterError::General("test".to_string());
    let _: &dyn std::error::Error = &error;
}

#[test]
fn adapter_error_is_debug() {
    let error = AdapterError::General("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("General"));
}

// ============================================================================
// #[from] Conversion Tests
// ============================================================================

#[test]
fn adapter_error_from_mapping_error() {
    let mapping_error = MappingError::new("mapping failed");
    let adapter_error: AdapterError = mapping_error.into();

    match adapter_error {
        AdapterError::Mapping(e) => {
            assert_eq!(e.to_string(), "mapping failed");
        }
        _ => panic!("Expected AdapterError::Mapping variant"),
    }
}

#[test]
fn adapter_error_from_transform_error() {
    let transform_error = TransformError::new("transform failed");
    let adapter_error: AdapterError = transform_error.into();

    match adapter_error {
        AdapterError::Transform(e) => {
            assert_eq!(e.to_string(), "transform failed");
        }
        _ => panic!("Expected AdapterError::Transform variant"),
    }
}

#[test]
fn adapter_error_mapping_is_transparent() {
    let mapping_error = MappingError::new("inner message");
    let adapter_error: AdapterError = mapping_error.into();

    // With #[error(transparent)], the display should show the inner error's message
    assert_eq!(adapter_error.to_string(), "inner message");
}

#[test]
fn adapter_error_transform_is_transparent() {
    let transform_error = TransformError::new("inner message");
    let adapter_error: AdapterError = transform_error.into();

    // With #[error(transparent)], the display should show the inner error's message
    assert_eq!(adapter_error.to_string(), "inner message");
}

/// Test that `?` operator works with #[from] conversion
fn function_returning_mapping_error() -> Result<(), MappingError> {
    Err(MappingError::new("test"))
}

fn function_returning_adapter_error() -> Result<(), AdapterError> {
    function_returning_mapping_error()?;
    Ok(())
}

#[test]
fn from_conversion_works_with_question_mark_operator() {
    let result = function_returning_adapter_error();
    assert!(result.is_err());
    match result.unwrap_err() {
        AdapterError::Mapping(_) => {}
        _ => panic!("Expected Mapping variant"),
    }
}
