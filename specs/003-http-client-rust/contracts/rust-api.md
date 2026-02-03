# Rust API Contract: HTTP Client Layer

**Feature**: 003-http-client-rust
**Parent Spec**: [003-http-client-rust](../spec.md)
**Date**: 2026-02-03

> Note: HTTP クライアントレイヤーは Rust ライブラリ内部 API のため、REST/GraphQL スキーマではなく、Rust の型シグネチャとしてコントラクトを定義する。

## Crate: `marketschema-http`

### AsyncHttpClient

```rust
use reqwest::{header::HeaderMap, Response};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

/// Async HTTP client for adapter implementations.
///
/// Features:
/// - Connection pooling
/// - Configurable timeouts
/// - Clean error handling
///
/// # Example
///
/// ```rust
/// use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = AsyncHttpClientBuilder::new()
///         .timeout(Duration::from_secs(30))
///         .build()?;
///
///     let data = client.get_json("https://api.example.com/ticker").await?;
///     Ok(())
/// }
/// ```
pub struct AsyncHttpClient {
    inner: reqwest::Client,
    // Phase 2
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
    // Phase 3
    cache: Option<Arc<ResponseCache>>,
}

impl AsyncHttpClient {
    /// Create a new client builder.
    pub fn builder() -> AsyncHttpClientBuilder {
        AsyncHttpClientBuilder::new()
    }

    /// Send a GET request and return the raw response.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The reqwest::Response object.
    ///
    /// # Errors
    ///
    /// * `HttpError::Timeout` - If the request times out.
    /// * `HttpError::Connection` - If connection fails.
    /// * `HttpError::Status` - If the response has an error status code.
    /// * `HttpError::RateLimit` - If rate limited (429).
    pub async fn get(&self, url: &str) -> Result<Response, HttpError> {
        todo!()
    }

    /// Send a GET request with query parameters.
    pub async fn get_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<Response, HttpError> {
        todo!()
    }

    /// Send a GET request and return the JSON response.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The parsed JSON response as a serde_json::Value.
    ///
    /// # Errors
    ///
    /// * `HttpError::Timeout` - If the request times out.
    /// * `HttpError::Connection` - If connection fails.
    /// * `HttpError::Status` - If the response has an error status code.
    /// * `HttpError::RateLimit` - If rate limited (429).
    /// * `HttpError::Parse` - If the response is not valid JSON.
    pub async fn get_json(&self, url: &str) -> Result<Value, HttpError> {
        todo!()
    }

    /// Send a GET request with query parameters and return the JSON response.
    pub async fn get_json_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<Value, HttpError> {
        todo!()
    }

    /// Send a GET request and return the text response.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to request.
    ///
    /// # Returns
    ///
    /// The response body as a String.
    ///
    /// # Errors
    ///
    /// * `HttpError::Timeout` - If the request times out.
    /// * `HttpError::Connection` - If connection fails.
    /// * `HttpError::Status` - If the response has an error status code.
    /// * `HttpError::RateLimit` - If rate limited (429).
    pub async fn get_text(&self, url: &str) -> Result<String, HttpError> {
        todo!()
    }

    /// Send a GET request with query parameters and return the text response.
    pub async fn get_text_with_params(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> Result<String, HttpError> {
        todo!()
    }
}

// AsyncHttpClient is Send + Sync for sharing across tasks
unsafe impl Send for AsyncHttpClient {}
unsafe impl Sync for AsyncHttpClient {}
```

### AsyncHttpClientBuilder

```rust
/// Builder for AsyncHttpClient.
///
/// # Example
///
/// ```rust
/// let client = AsyncHttpClientBuilder::new()
///     .timeout(Duration::from_secs(60))
///     .max_connections(200)
///     .default_headers(headers)
///     .retry(RetryConfig::default())
///     .rate_limit(RateLimiter::new(10.0, 10))
///     .cache(ResponseCache::new(1000, Duration::from_secs(300)))
///     .build()?;
/// ```
pub struct AsyncHttpClientBuilder {
    timeout: Duration,
    max_connections: usize,
    default_headers: HeaderMap,
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
    cache: Option<Arc<ResponseCache>>,
}

impl AsyncHttpClientBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            default_headers: HeaderMap::new(),
            retry_config: None,
            rate_limiter: None,
            cache: None,
        }
    }

    /// Set the request timeout.
    ///
    /// Default: 30 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of concurrent connections.
    ///
    /// Default: 100.
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Set default headers for all requests.
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    /// Set retry configuration (Phase 2).
    pub fn retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    /// Set rate limiter (Phase 2).
    pub fn rate_limit(mut self, limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// Set response cache (Phase 3).
    pub fn cache(mut self, cache: Arc<ResponseCache>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Build the AsyncHttpClient.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be built.
    pub fn build(self) -> Result<AsyncHttpClient, HttpError> {
        todo!()
    }
}

impl Default for AsyncHttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

### HttpError

```rust
use std::time::Duration;
use thiserror::Error;

/// HTTP error types.
///
/// All errors include a message and optionally the URL that caused the error.
/// The `source` field provides access to the underlying reqwest error.
#[derive(Error, Debug)]
pub enum HttpError {
    /// Request timed out.
    #[error("HTTP timeout: {message}")]
    Timeout {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Connection failed.
    #[error("HTTP connection error: {message}")]
    Connection {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// HTTP status indicates error (4xx, 5xx).
    #[error("HTTP status error {status_code}: {message}")]
    Status {
        message: String,
        url: Option<String>,
        status_code: u16,
        response_body: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Rate limit exceeded (429).
    #[error("HTTP rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        url: Option<String>,
        status_code: u16,
        response_body: Option<String>,
        retry_after: Option<Duration>,
        #[source]
        source: Option<reqwest::Error>,
    },

    /// JSON parse error.
    #[error("JSON parse error: {message}")]
    Parse {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<serde_json::Error>,
    },

    /// Client build error.
    #[error("HTTP client build error: {message}")]
    Build {
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },
}

impl HttpError {
    /// Get the URL that caused the error.
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
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Status { status_code, .. } => Some(*status_code),
            Self::RateLimit { status_code, .. } => Some(*status_code),
            _ => None,
        }
    }

    /// Check if this error is retryable.
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
```

### RetryConfig (Phase 2)

```rust
use std::collections::HashSet;

/// Retry configuration for failed requests.
///
/// # Example
///
/// ```rust
/// let retry = RetryConfig::new()
///     .max_retries(5)
///     .backoff_factor(1.0)
///     .jitter(0.2);
///
/// let client = AsyncHttpClientBuilder::new()
///     .retry(retry)
///     .build()?;
/// ```
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Multiplier for exponential backoff.
    pub backoff_factor: f64,
    /// Status codes to retry.
    pub retry_statuses: HashSet<u16>,
    /// Random jitter factor (0.0 to 1.0).
    pub jitter: f64,
}

impl RetryConfig {
    /// Create a new retry configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum number of retries.
    pub fn max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Set backoff factor.
    pub fn backoff_factor(mut self, factor: f64) -> Self {
        self.backoff_factor = factor;
        self
    }

    /// Set retry statuses.
    pub fn retry_statuses(mut self, statuses: HashSet<u16>) -> Self {
        self.retry_statuses = statuses;
        self
    }

    /// Set jitter factor.
    pub fn jitter(mut self, jitter: f64) -> Self {
        self.jitter = jitter;
        self
    }

    /// Check if the request should be retried.
    pub fn should_retry(&self, status_code: u16, attempt: u32) -> bool {
        attempt < self.max_retries && self.retry_statuses.contains(&status_code)
    }

    /// Calculate the delay before the next retry.
    pub fn get_delay(&self, attempt: u32) -> Duration {
        todo!()
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_factor: 0.5,
            retry_statuses: [429, 500, 502, 503, 504].into_iter().collect(),
            jitter: 0.1,
        }
    }
}
```

### RateLimiter (Phase 2)

```rust
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Rate limiter using token bucket algorithm.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
///
/// let limiter = Arc::new(RateLimiter::new(10.0, 20));
/// let client = AsyncHttpClientBuilder::new()
///     .rate_limit(limiter.clone())
///     .build()?;
/// ```
pub struct RateLimiter {
    requests_per_second: f64,
    burst_size: usize,
    tokens: Mutex<f64>,
    last_update: Mutex<Instant>,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// # Arguments
    ///
    /// * `requests_per_second` - Maximum requests per second.
    /// * `burst_size` - Maximum burst size.
    pub fn new(requests_per_second: f64, burst_size: usize) -> Self {
        Self {
            requests_per_second,
            burst_size,
            tokens: Mutex::new(burst_size as f64),
            last_update: Mutex::new(Instant::now()),
        }
    }

    /// Acquire a token, waiting if necessary.
    ///
    /// Blocks until a token is available.
    pub async fn acquire(&self) {
        todo!()
    }

    /// Try to acquire a token without blocking.
    ///
    /// Returns true if a token was acquired, false otherwise.
    pub fn try_acquire(&self) -> bool {
        todo!()
    }
}

// RateLimiter is Send + Sync for sharing across tasks
unsafe impl Send for RateLimiter {}
unsafe impl Sync for RateLimiter {}
```

### ResponseCache (Phase 3)

```rust
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// LRU cache for HTTP responses.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
///
/// let cache = Arc::new(ResponseCache::new(500, Duration::from_secs(60)));
/// let client = AsyncHttpClientBuilder::new()
///     .cache(cache.clone())
///     .build()?;
/// ```
pub struct ResponseCache {
    max_size: usize,
    default_ttl: Duration,
    entries: RwLock<HashMap<String, CacheEntry>>,
}

struct CacheEntry {
    value: String,
    expires_at: Instant,
}

impl ResponseCache {
    /// Create a new response cache.
    ///
    /// # Arguments
    ///
    /// * `max_size` - Maximum number of cached entries.
    /// * `default_ttl` - Default time-to-live for cache entries.
    pub fn new(max_size: usize, default_ttl: Duration) -> Self {
        Self {
            max_size,
            default_ttl,
            entries: RwLock::new(HashMap::new()),
        }
    }

    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key (typically the URL).
    ///
    /// # Returns
    ///
    /// The cached value, or None if not found or expired.
    pub fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    /// Set a value in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key.
    /// * `value` - The value to cache.
    /// * `ttl` - Time-to-live. Defaults to default_ttl.
    pub fn set(&self, key: &str, value: String, ttl: Option<Duration>) {
        todo!()
    }

    /// Delete a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key.
    pub fn delete(&self, key: &str) {
        todo!()
    }

    /// Clear all cached entries.
    pub fn clear(&self) {
        todo!()
    }
}

// ResponseCache is Send + Sync for sharing across tasks
unsafe impl Send for ResponseCache {}
unsafe impl Sync for ResponseCache {}
```

## Crate: `marketschema` (BaseAdapter integration)

### BaseAdapter Trait

```rust
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Base adapter trait with HTTP client support.
///
/// # Example
///
/// ```rust
/// use marketschema::{BaseAdapter, AsyncHttpClient};
/// use std::sync::Arc;
///
/// struct MyAdapter {
///     http_client: Arc<OnceCell<Arc<AsyncHttpClient>>>,
/// }
///
/// impl BaseAdapter for MyAdapter {
///     fn http_client(&self) -> Arc<AsyncHttpClient> {
///         self.http_client
///             .get_or_init(|| async {
///                 Arc::new(AsyncHttpClient::builder().build().unwrap())
///             })
///             .await
///             .clone()
///     }
/// }
/// ```
pub trait BaseAdapter: Send + Sync {
    /// Get the HTTP client.
    ///
    /// Default implementation provides lazy initialization via OnceCell.
    fn http_client(&self) -> Arc<AsyncHttpClient>;
}
```

## Public Exports

```rust
// marketschema_http/src/lib.rs

pub use client::{AsyncHttpClient, AsyncHttpClientBuilder};
pub use error::HttpError;

// Phase 2
pub use retry::RetryConfig;
pub use rate_limit::RateLimiter;

// Phase 3
pub use cache::ResponseCache;

mod client;
mod error;
mod retry;
mod rate_limit;
mod cache;
```
