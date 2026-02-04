//! Field mapping configuration with builder pattern.

use crate::error::{MappingError, TransformError};
use serde_json::Value;
use std::sync::Arc;

/// Type alias for transform functions.
///
/// Transform functions take a reference to a JSON value and return a transformed
/// value or an error.
pub type TransformFn = Arc<dyn Fn(&Value) -> Result<Value, TransformError> + Send + Sync>;

/// Field mapping configuration that defines how to extract and transform
/// a value from source data to a target field.
#[derive(Clone)]
#[non_exhaustive]
pub struct ModelMapping {
    /// Name of the field in the target model.
    pub target_field: String,
    /// Path to the field in the source data (supports dot notation).
    pub source_field: String,
    /// Optional transform function to apply to the extracted value.
    pub transform: Option<TransformFn>,
    /// Optional default value when source field is missing or null.
    pub default: Option<Value>,
    /// Whether the field is required. When true and value is missing/null,
    /// returns error unless a default is set. When false, returns Value::Null
    /// for missing values (unless default is set).
    pub required: bool,
}

impl ModelMapping {
    /// Creates a new ModelMapping with the given target and source fields.
    ///
    /// By default, the mapping is required (required=true).
    pub fn new(target_field: impl Into<String>, source_field: impl Into<String>) -> Self {
        Self {
            target_field: target_field.into(),
            source_field: source_field.into(),
            transform: None,
            default: None,
            required: true,
        }
    }

    /// Sets the transform function for this mapping.
    pub fn with_transform(mut self, transform: TransformFn) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Sets the default value for this mapping.
    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    /// Marks this mapping as optional (required=false).
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Extracts a value from the source data using dot notation path.
    fn extract_value<'a>(&self, source: &'a Value) -> Option<&'a Value> {
        let mut current = source;
        for part in self.source_field.split('.') {
            current = current.get(part)?;
        }
        Some(current)
    }

    /// Applies the mapping to extract and transform a value from source data.
    ///
    /// Returns the transformed value, or an error if the value is required but missing
    /// or if the transform fails.
    pub fn apply(&self, source: &Value) -> Result<Value, MappingError> {
        // Try to extract the value from source
        let extracted = self.extract_value(source);

        match extracted {
            Some(value) if !value.is_null() => {
                // Apply transform if present
                if let Some(ref transform) = self.transform {
                    transform(value).map_err(|e| {
                        MappingError::new(format!(
                            "Transform failed for field '{}': {}",
                            self.target_field, e
                        ))
                    })
                } else {
                    Ok(value.clone())
                }
            }
            _ => {
                // Value not found or is null
                if let Some(ref default) = self.default {
                    Ok(default.clone())
                } else if self.required {
                    Err(MappingError::new(format!(
                        "Required field '{}' not found at path '{}'",
                        self.target_field, self.source_field
                    )))
                } else {
                    Ok(Value::Null)
                }
            }
        }
    }
}
