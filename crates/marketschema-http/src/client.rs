//! Async HTTP client implementation for marketschema adapters.
//!
//! This module provides [`AsyncHttpClient`] and [`AsyncHttpClientBuilder`] for
//! making HTTP requests with configurable timeouts, connection pooling, and headers.
//!
//! # Example
//!
//! ```rust
//! use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = AsyncHttpClientBuilder::new()
//!     .timeout(Duration::from_secs(30))
//!     .build()?;
//!
//! let data = client.get_json("https://api.example.com/ticker").await?;
//! # Ok(())
//! # }
//! ```

use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::Response;
use serde_json::Value;

use crate::error::HttpError;
use crate::{DEFAULT_MAX_CONNECTIONS, DEFAULT_TIMEOUT_SECS, HTTP_STATUS_TOO_MANY_REQUESTS};

/// Async HTTP client for adapter implementations.
///
/// Features:
/// - Connection pooling via reqwest
/// - Configurable timeouts
/// - Clean error handling with [`HttpError`]
///
/// The client is `Send + Sync` and can be safely shared across tasks via `Arc`.
///
/// # Example
///
/// ```rust
/// use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
/// use std::sync::Arc;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Arc::new(AsyncHttpClientBuilder::new().build()?);
///
/// // Share across multiple tasks
/// let client_clone = Arc::clone(&client);
/// tokio::spawn(async move {
///     let data = client_clone.get_json("https://api.example.com/data").await;
/// });
/// # Ok(())
/// # }
/// ```
pub struct AsyncHttpClient {
    inner: reqwest::Client,
}

impl AsyncHttpClient {
    /// Create a new client builder.
    ///
    /// This is a convenience method equivalent to [`AsyncHttpClientBuilder::new()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::AsyncHttpClient;
    ///
    /// let client = AsyncHttpClient::builder()
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn builder() -> AsyncHttpClientBuilder {
        AsyncHttpClientBuilder::new()
    }

    /// Send a GET request and return the raw response.
    ///
    /// This method checks for error status codes and converts them to appropriate
    /// [`HttpError`] variants. For success responses (2xx), the raw [`Response`]
    /// is returned.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The [`reqwest::Response`] object for success status codes.
    ///
    /// # Errors
    ///
    /// * [`HttpError::Timeout`] - If the request times out.
    /// * [`HttpError::Connection`] - If connection fails.
    /// * [`HttpError::Status`] - If the response has an error status code (4xx/5xx except 429).
    /// * [`HttpError::RateLimit`] - If rate limited (429).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let response = client.get("https://api.example.com/data").await?;
    /// println!("Status: {}", response.status());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, url: &str) -> Result<Response, HttpError> {
        self.get_with_params(url, &[]).await
    }

    /// Send a GET request with query parameters.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL to request.
    /// * `params` - Query parameters as key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let response = client
    ///     .get_with_params("https://api.example.com/search", &[("q", "bitcoin")])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<Response, HttpError> {
        let response = self
            .inner
            .get(url)
            .query(params)
            .send()
            .await
            .map_err(|e| Self::convert_reqwest_error(e, url))?;

        Self::check_status(response, url).await
    }

    /// Send a GET request and return the JSON response.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The parsed JSON response as a [`serde_json::Value`].
    ///
    /// # Errors
    ///
    /// * [`HttpError::Timeout`] - If the request times out.
    /// * [`HttpError::Connection`] - If connection fails.
    /// * [`HttpError::Status`] - If the response has an error status code.
    /// * [`HttpError::RateLimit`] - If rate limited (429).
    /// * [`HttpError::Parse`] - If the response is not valid JSON.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let data = client.get_json("https://api.example.com/ticker").await?;
    /// println!("Price: {}", data["price"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_json(&self, url: &str) -> Result<Value, HttpError> {
        self.get_json_with_params(url, &[]).await
    }

    /// Send a GET request with query parameters and return the JSON response.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL to request.
    /// * `params` - Query parameters as key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let data = client
    ///     .get_json_with_params("https://api.example.com/search", &[("symbol", "BTC")])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_json_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<Value, HttpError> {
        let response = self.get_with_params(url, params).await?;
        let text = response.text().await.map_err(|e| HttpError::Connection {
            message: e.to_string(),
            url: Some(url.to_string()),
            source: Some(e),
        })?;

        serde_json::from_str(&text).map_err(|e| HttpError::Parse {
            message: e.to_string(),
            url: Some(url.to_string()),
            source: Some(e),
        })
    }

    /// Send a GET request and return the text response.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The response body as a [`String`].
    ///
    /// # Errors
    ///
    /// * [`HttpError::Timeout`] - If the request times out.
    /// * [`HttpError::Connection`] - If connection fails.
    /// * [`HttpError::Status`] - If the response has an error status code.
    /// * [`HttpError::RateLimit`] - If rate limited (429).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let text = client.get_text("https://api.example.com/status").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_text(&self, url: &str) -> Result<String, HttpError> {
        self.get_text_with_params(url, &[]).await
    }

    /// Send a GET request with query parameters and return the text response.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL to request.
    /// * `params` - Query parameters as key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::AsyncHttpClientBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AsyncHttpClientBuilder::new().build()?;
    /// let text = client
    ///     .get_text_with_params("https://api.example.com/report", &[("format", "plain")])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_text_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<String, HttpError> {
        let response = self.get_with_params(url, params).await?;
        response.text().await.map_err(|e| HttpError::Connection {
            message: e.to_string(),
            url: Some(url.to_string()),
            source: Some(e),
        })
    }

    /// Convert a reqwest error to an HttpError.
    fn convert_reqwest_error(error: reqwest::Error, url: &str) -> HttpError {
        if error.is_timeout() {
            HttpError::Timeout {
                message: error.to_string(),
                url: Some(url.to_string()),
                source: Some(error),
            }
        } else {
            // Handle both connection errors and other network errors as Connection
            HttpError::Connection {
                message: error.to_string(),
                url: Some(url.to_string()),
                source: Some(error),
            }
        }
    }

    /// Check response status and convert to HttpError if needed.
    async fn check_status(response: Response, url: &str) -> Result<Response, HttpError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        let status_code = status.as_u16();

        // Parse Retry-After header before consuming response body
        // FR-R014: HTTP 429 ステータス時は HttpError::RateLimit を返す（retry_after はヘッダーから取得）
        let retry_after = if status_code == HTTP_STATUS_TOO_MANY_REQUESTS {
            Self::parse_retry_after_header(&response)
        } else {
            None
        };

        // Explicitly handle body read errors instead of using .ok()
        // (CLAUDE.md: 暗黙的フォールバック禁止)
        let response_body = match response.text().await {
            Ok(body) if body.is_empty() => None,
            Ok(body) => Some(body),
            Err(e) => {
                // Log the error but don't fail - the primary error is the HTTP status
                // This is explicit handling, not silent suppression
                tracing::warn!(
                    url = %url,
                    status_code = %status_code,
                    error = %e,
                    "Failed to read error response body"
                );
                None
            }
        };

        if status_code == HTTP_STATUS_TOO_MANY_REQUESTS {
            return Err(HttpError::RateLimit {
                message: format!("Rate limit exceeded: {}", status),
                url: Some(url.to_string()),
                status_code,
                response_body,
                retry_after,
                source: None,
            });
        }

        Err(HttpError::Status {
            message: format!("HTTP error: {}", status),
            url: Some(url.to_string()),
            status_code,
            response_body,
            source: None,
        })
    }

    /// Parse the Retry-After header from a response.
    ///
    /// The Retry-After header can be either:
    /// - An integer representing delay in seconds (e.g., "60")
    /// - An HTTP-date (e.g., "Wed, 21 Oct 2026 07:28:00 GMT")
    ///
    /// This implementation only parses the integer format.
    /// HTTP-date format is complex and rarely used; we return None for it.
    fn parse_retry_after_header(response: &Response) -> Option<Duration> {
        let header_value = response.headers().get("retry-after")?;

        // Explicitly handle non-ASCII header values instead of using .ok()?
        // (CLAUDE.md: 暗黙的フォールバック禁止)
        let header_str = match header_value.to_str() {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    header_value = ?header_value,
                    "Retry-After header contains non-ASCII characters, ignoring"
                );
                return None;
            }
        };

        // Try to parse as seconds (integer format)
        // RFC 7231: Retry-After can be either delta-seconds or HTTP-date
        // We only support delta-seconds; HTTP-date format is complex and rarely used
        match header_str.trim().parse::<u64>() {
            Ok(seconds) => Some(Duration::from_secs(seconds)),
            Err(parse_err) => {
                // Could be HTTP-date format, negative value, overflow, or other invalid input
                tracing::warn!(
                    retry_after = %header_str,
                    error = %parse_err,
                    "Retry-After header is not a valid positive integer, ignoring (may be HTTP-date format)"
                );
                None
            }
        }
    }
}

// Compile-time verification that AsyncHttpClient is Send + Sync
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AsyncHttpClient>();
};

/// Builder for [`AsyncHttpClient`].
///
/// Provides a fluent API for configuring the HTTP client with custom timeouts,
/// connection pool settings, and default headers.
///
/// # Example
///
/// ```rust
/// use marketschema_http::AsyncHttpClientBuilder;
/// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
/// use std::time::Duration;
///
/// let mut headers = HeaderMap::new();
/// headers.insert(USER_AGENT, HeaderValue::from_static("my-app/1.0"));
///
/// let client = AsyncHttpClientBuilder::new()
///     .timeout(Duration::from_secs(60))
///     .max_connections(200)
///     .default_headers(headers)
///     .build()
///     .unwrap();
/// ```
pub struct AsyncHttpClientBuilder {
    timeout: Duration,
    max_connections: usize,
    default_headers: HeaderMap,
}

impl AsyncHttpClientBuilder {
    /// Create a new builder with default settings.
    ///
    /// Default values:
    /// - Timeout: 30 seconds
    /// - Max connections: 100
    /// - No default headers
    #[must_use]
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            default_headers: HeaderMap::new(),
        }
    }

    /// Set the request timeout.
    ///
    /// Default: 30 seconds.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The maximum time to wait for a response.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of idle connections per host in the connection pool.
    ///
    /// Note: This controls `reqwest::ClientBuilder::pool_max_idle_per_host()`.
    ///
    /// Default: 100.
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum number of idle connections per host.
    #[must_use]
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Set default headers for all requests.
    ///
    /// These headers will be included in every request made by the client.
    ///
    /// # Arguments
    ///
    /// * `headers` - The headers to include in all requests.
    #[must_use]
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    /// Build the [`AsyncHttpClient`].
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Build`] if the client cannot be built.
    pub fn build(self) -> Result<AsyncHttpClient, HttpError> {
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .pool_max_idle_per_host(self.max_connections)
            .default_headers(self.default_headers)
            .build()
            .map_err(|e| HttpError::Build {
                message: e.to_string(),
                source: Some(e),
            })?;

        Ok(AsyncHttpClient { inner: client })
    }
}

impl Default for AsyncHttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
