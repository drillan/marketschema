//! Async HTTP client for marketschema adapters.
//!
//! This crate provides a robust HTTP client layer for building market data adapters.
//! Features include connection pooling, configurable timeouts, automatic retries,
//! rate limiting, and response caching.
//!
//! # Example
//!
//! ```ignore
//! use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AsyncHttpClientBuilder::new()
//!         .timeout(Duration::from_secs(30))
//!         .build()?;
//!
//!     let data = client.get_json("https://api.example.com/ticker").await?;
//!     Ok(())
//! }
//! ```

// =============================================================================
// Default Constants
// =============================================================================

/// Default request timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default maximum number of concurrent connections.
pub const DEFAULT_MAX_CONNECTIONS: usize = 100;

/// Default maximum number of retry attempts.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default exponential backoff factor for retries.
pub const DEFAULT_BACKOFF_FACTOR: f64 = 0.5;

/// Default random jitter factor for retries (0.0 to 1.0).
pub const DEFAULT_JITTER: f64 = 0.1;

/// Default cache TTL in seconds.
pub const DEFAULT_CACHE_TTL_SECS: u64 = 300;

/// Default maximum cache size (number of entries).
pub const DEFAULT_CACHE_SIZE: u64 = 1000;

// =============================================================================
// Modules (to be implemented in future user stories)
// =============================================================================

// US1: Async HTTP Request Execution
// mod client;

// US2: Error Handling with Result Type
// mod error;

// US3: Automatic Retry with Exponential Backoff
// mod retry;

// US4: Rate Limiting with Token Bucket
// mod rate_limit;

// US5: Response Caching with LRU
// mod cache;

// =============================================================================
// Public Exports (to be added as modules are implemented)
// =============================================================================

// pub use client::{AsyncHttpClient, AsyncHttpClientBuilder};
// pub use error::HttpError;
// pub use retry::RetryConfig;
// pub use rate_limit::RateLimiter;
// pub use cache::ResponseCache;
