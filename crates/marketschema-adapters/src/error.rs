//! Error types for the adapter framework.

use thiserror::Error;

/// Error type for mapping operations.
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct MappingError {
    message: String,
}

impl MappingError {
    /// Creates a new MappingError with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Error type for transform operations.
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct TransformError {
    message: String,
}

impl TransformError {
    /// Creates a new TransformError with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Main error type for adapter operations.
#[derive(Debug, Error)]
pub enum AdapterError {
    /// General adapter error.
    #[error("{0}")]
    General(String),

    /// Duplicate registration error.
    #[error("Adapter '{0}' is already registered")]
    DuplicateRegistration(String),

    /// Mapping error.
    #[error(transparent)]
    Mapping(#[from] MappingError),

    /// Transform error.
    #[error(transparent)]
    Transform(#[from] TransformError),
}
