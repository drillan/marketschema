//! Error types for the bitbank adapter.

use thiserror::Error;

/// Errors that can occur when working with bitbank data.
#[derive(Debug, Error)]
pub enum BitbankError {
    /// API response indicates an error (success != 1).
    #[error("bitbank API error: success={success_code}, response={response}")]
    ApiError {
        /// The success code returned by the API.
        success_code: i64,
        /// The full API response for debugging.
        response: String,
    },

    /// Missing required field in API response.
    #[error("Missing required field '{field}' in {context}")]
    MissingField {
        /// The name of the missing field.
        field: String,
        /// Context where the field was expected (e.g., "ticker response").
        context: String,
    },

    /// Insufficient elements in array.
    #[error("Insufficient array length: expected {expected}, got {actual} in {context}")]
    InsufficientArrayLength {
        /// Expected minimum array length.
        expected: usize,
        /// Actual array length.
        actual: usize,
        /// Context where the array was found (e.g., "OHLCV data").
        context: String,
    },

    /// Data conversion error.
    #[error("Conversion error: {message}")]
    Conversion {
        /// Error message describing the conversion failure.
        message: String,
    },

    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] marketschema_http::HttpError),

    /// HTTP client not configured.
    #[error(
        "HTTP client not configured. Use BitbankAdapter::with_http_client() or BitbankAdapter::with_default_http_client()"
    )]
    HttpClientNotConfigured,

    /// Unexpected JSON value type.
    #[error("Unexpected type at index {index} in {context}: expected array, got {actual_type}")]
    UnexpectedType {
        /// Index of the element with unexpected type.
        index: usize,
        /// Context where the element was found (e.g., "OHLCV data").
        context: String,
        /// Description of the actual type.
        actual_type: String,
    },
}
