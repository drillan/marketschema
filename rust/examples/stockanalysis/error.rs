//! Error types for the StockAnalysis adapter.

use thiserror::Error;

/// Errors that can occur when working with StockAnalysis data.
#[derive(Debug, Error)]
pub enum StockAnalysisError {
    /// Invalid date format encountered.
    #[error("Invalid date format: {value:?}, expected 'MMM D, YYYY' (e.g., 'Feb 2, 2026'): {reason}")]
    InvalidDateFormat {
        /// The invalid date value.
        value: String,
        /// The reason for the failure.
        reason: String,
    },

    /// Invalid month abbreviation in date.
    #[error("Invalid month abbreviation: {value:?}")]
    InvalidMonth {
        /// The invalid month value.
        value: String,
    },

    /// Insufficient columns in HTML table row.
    #[error("Insufficient columns: expected {expected}, got {actual}")]
    InsufficientColumns {
        /// Expected number of columns.
        expected: usize,
        /// Actual number of columns.
        actual: usize,
    },

    /// Empty HTML content provided.
    #[error("Empty HTML content provided")]
    EmptyHtml,

    /// No table found in HTML content.
    #[error("No table found in HTML content")]
    NoTableFound,

    /// Table structure error (e.g., missing tbody).
    #[error("Table structure error: {message}")]
    TableStructureError {
        /// Error message describing the structure issue.
        message: String,
    },

    /// Empty volume string.
    #[error("Empty volume string")]
    EmptyVolume,

    /// HTTP request error.
    #[error("HTTP error: {0}")]
    Http(#[from] marketschema_http::HttpError),

    /// Data conversion error (e.g., string to float).
    #[error("Conversion error at row {row_index}: {message}")]
    Conversion {
        /// Error message.
        message: String,
        /// Row index (1-based) where the error occurred.
        row_index: usize,
    },

    /// HTTP client not configured.
    #[error(
        "HTTP client not configured. Use StockAnalysisAdapter::with_http_client() or StockAnalysisAdapter::with_default_http_client()"
    )]
    HttpClientNotConfigured,
}
