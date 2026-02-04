//! HTTP error types for marketschema-http.
//!
//! This module defines the error types returned by HTTP client operations.
//! All errors include contextual information and support error chaining via
//! the `source()` method.
//!
//! # Error Types
//!
//! - [`HttpError::Build`] - Client construction failed
//! - [`HttpError::Timeout`] - Request timed out
//! - [`HttpError::Connection`] - Connection failed
//! - [`HttpError::Status`] - HTTP error status (4xx/5xx)
//! - [`HttpError::RateLimit`] - Rate limit exceeded (429)
//! - [`HttpError::Parse`] - JSON parse error

use std::time::Duration;
use thiserror::Error;

/// HTTP error types.
///
/// All errors include a message and optionally the URL that caused the error.
/// The `source` field provides access to the underlying reqwest error.
///
/// # Example
///
/// ```rust
/// use marketschema_http::HttpError;
///
/// fn handle_error(err: HttpError) {
///     match err {
///         HttpError::Status { status_code, url, .. } => {
///             eprintln!("HTTP {} for {:?}", status_code, url);
///         }
///         HttpError::RateLimit { retry_after, .. } => {
///             if let Some(delay) = retry_after {
///                 eprintln!("Rate limited, retry after {:?}", delay);
///             }
///         }
///         _ => eprintln!("Error: {}", err),
///     }
/// }
/// ```
#[derive(Error, Debug)]
pub enum HttpError {
    /// Request timed out.
    #[error("HTTP timeout: {message}")]
    Timeout {
        /// Error message describing the timeout.
        message: String,
        /// URL that caused the timeout.
        url: Option<String>,
        /// Underlying reqwest error.
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Connection failed.
    #[error("HTTP connection error: {message}")]
    Connection {
        /// Error message describing the connection failure.
        message: String,
        /// URL that caused the connection failure.
        url: Option<String>,
        /// Underlying reqwest error.
        #[source]
        source: Option<reqwest::Error>,
    },

    /// HTTP status indicates error (4xx, 5xx).
    #[error("HTTP status error {status_code}: {message}")]
    Status {
        /// Error message describing the status error.
        message: String,
        /// URL that returned the error status.
        url: Option<String>,
        /// HTTP status code (e.g., 404, 500).
        status_code: u16,
        /// Response body content, if available.
        response_body: Option<String>,
        /// Underlying reqwest error.
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Rate limit exceeded (429).
    #[error("HTTP rate limit exceeded: {message}")]
    RateLimit {
        /// Error message describing the rate limit.
        message: String,
        /// URL that triggered the rate limit.
        url: Option<String>,
        /// HTTP status code (always 429).
        status_code: u16,
        /// Response body content, if available.
        response_body: Option<String>,
        /// Suggested retry delay from Retry-After header.
        retry_after: Option<Duration>,
        /// Underlying reqwest error.
        #[source]
        source: Option<reqwest::Error>,
    },

    /// JSON parse error.
    #[error("JSON parse error: {message}")]
    Parse {
        /// Error message describing the parse failure.
        message: String,
        /// URL that returned unparseable content.
        url: Option<String>,
        /// Underlying serde_json error.
        #[source]
        source: Option<serde_json::Error>,
    },

    /// Client build error.
    #[error("HTTP client build error: {message}")]
    Build {
        /// Error message describing the build failure.
        message: String,
        /// Underlying reqwest error.
        #[source]
        source: Option<reqwest::Error>,
    },
}

impl HttpError {
    /// Get the URL that caused the error.
    ///
    /// Returns `None` for build errors which are not associated with a URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::HttpError;
    ///
    /// fn log_error(err: &HttpError) {
    ///     if let Some(url) = err.url() {
    ///         eprintln!("Error for URL: {}", url);
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Timeout { url, .. } => url.as_deref(),
            Self::Connection { url, .. } => url.as_deref(),
            Self::Status { url, .. } => url.as_deref(),
            Self::RateLimit { url, .. } => url.as_deref(),
            Self::Parse { url, .. } => url.as_deref(),
            Self::Build { .. } => None,
        }
    }

    /// Get the status code if this is a status error.
    ///
    /// Returns `Some(status_code)` for `Status` and `RateLimit` errors,
    /// `None` for all other error types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::HttpError;
    ///
    /// fn is_not_found(err: &HttpError) -> bool {
    ///     err.status_code() == Some(404)
    /// }
    /// ```
    #[must_use]
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Status { status_code, .. } => Some(*status_code),
            Self::RateLimit { status_code, .. } => Some(*status_code),
            _ => None,
        }
    }

    /// Check if this error is retryable.
    ///
    /// The following errors are considered retryable:
    /// - Timeout errors
    /// - Connection errors (temporary network issues)
    /// - Server errors (500, 502, 503, 504)
    /// - Rate limit errors (429)
    ///
    /// Note: Connection errors are always considered retryable in this implementation,
    /// as temporary network issues are common.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::HttpError;
    ///
    /// async fn retry_if_possible(err: &HttpError) {
    ///     if err.is_retryable() {
    ///         // Schedule retry with exponential backoff
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Timeout { .. } => true,
            Self::Connection { .. } => true,
            Self::Status { status_code, .. } => {
                matches!(status_code, 500 | 502 | 503 | 504)
            }
            Self::RateLimit { .. } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_accessor() {
        let err = HttpError::Status {
            message: "Not Found".to_string(),
            url: Some("https://example.com/api".to_string()),
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert_eq!(err.url(), Some("https://example.com/api"));
    }

    #[test]
    fn test_url_accessor_returns_none_for_build() {
        let err = HttpError::Build {
            message: "Failed".to_string(),
            source: None,
        };
        assert_eq!(err.url(), None);
    }

    #[test]
    fn test_status_code_accessor() {
        let err = HttpError::Status {
            message: "Not Found".to_string(),
            url: None,
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert_eq!(err.status_code(), Some(404));
    }

    #[test]
    fn test_status_code_returns_none_for_timeout() {
        let err = HttpError::Timeout {
            message: "Timeout".to_string(),
            url: None,
            source: None,
        };
        assert_eq!(err.status_code(), None);
    }

    #[test]
    fn test_is_retryable_for_server_errors() {
        for status in [500, 502, 503, 504] {
            let err = HttpError::Status {
                message: format!("Error {}", status),
                url: None,
                status_code: status,
                response_body: None,
                source: None,
            };
            assert!(err.is_retryable(), "Status {} should be retryable", status);
        }
    }

    #[test]
    fn test_is_not_retryable_for_client_errors() {
        for status in [400, 401, 403, 404] {
            let err = HttpError::Status {
                message: format!("Error {}", status),
                url: None,
                status_code: status,
                response_body: None,
                source: None,
            };
            assert!(
                !err.is_retryable(),
                "Status {} should not be retryable",
                status
            );
        }
    }

    #[test]
    fn test_is_retryable_for_rate_limit() {
        let err = HttpError::RateLimit {
            message: "Rate limited".to_string(),
            url: None,
            status_code: 429,
            response_body: None,
            retry_after: Some(Duration::from_secs(60)),
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_for_timeout() {
        let err = HttpError::Timeout {
            message: "Timeout".to_string(),
            url: None,
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_for_connection() {
        let err = HttpError::Connection {
            message: "Connection failed".to_string(),
            url: None,
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = HttpError::Status {
            message: "Not Found".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert_eq!(err.to_string(), "HTTP status error 404: Not Found");
    }

    #[test]
    fn test_rate_limit_error_display() {
        let err = HttpError::RateLimit {
            message: "Too many requests".to_string(),
            url: None,
            status_code: 429,
            response_body: None,
            retry_after: None,
            source: None,
        };
        assert_eq!(
            err.to_string(),
            "HTTP rate limit exceeded: Too many requests"
        );
    }
}
