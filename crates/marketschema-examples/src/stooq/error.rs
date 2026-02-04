//! Error types for the Stooq adapter.

use thiserror::Error;

/// Errors that can occur when working with Stooq data.
#[derive(Debug, Error)]
pub enum StooqError {
    /// Invalid date format encountered.
    #[error("Invalid date format: {value:?}, expected YYYY-MM-DD")]
    InvalidDateFormat {
        /// The invalid date value.
        value: String,
    },

    /// Insufficient columns in CSV row.
    #[error("Insufficient columns: expected {expected}, got {actual}")]
    InsufficientColumns {
        /// Expected number of columns.
        expected: usize,
        /// Actual number of columns.
        actual: usize,
    },

    /// Empty CSV content (no header row).
    #[error("Empty CSV: no header row")]
    EmptyCsv,

    /// Invalid CSV header.
    #[error("Invalid CSV header: expected {expected:?}, got {actual:?}")]
    InvalidHeader {
        /// Expected header columns.
        expected: Vec<String>,
        /// Actual header columns.
        actual: Vec<String>,
    },

    /// CSV parsing error.
    #[error("CSV parsing error: {message}")]
    CsvParse {
        /// Error message.
        message: String,
    },

    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] marketschema_http::HttpError),

    /// Data conversion error (e.g., string to float).
    #[error("Conversion error: {message}")]
    Conversion {
        /// Error message.
        message: String,
    },

    /// HTTP client not configured.
    #[error(
        "HTTP client not configured. Use StooqAdapter::with_http_client() or StooqAdapter::with_default_http_client()"
    )]
    HttpClientNotConfigured,
}
