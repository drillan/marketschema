//! Async HTTP client for marketschema adapters.
//!
//! This crate provides a robust HTTP client layer for building market data adapters.
//! Features include connection pooling, configurable timeouts, and clean error handling.
//! Planned features include automatic retries, rate limiting, and response caching.
//!
//! # Example
//!
//! ```rust,no_run
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
/// Used as: delay = backoff_factor * 2^attempt seconds
pub const DEFAULT_BACKOFF_FACTOR: f64 = 0.5;

/// Default random jitter factor for retries (0.0 to 1.0).
/// Applied as: delay * (1.0 + random(-jitter, +jitter))
pub const DEFAULT_JITTER: f64 = 0.1;

/// Default cache TTL in seconds.
pub const DEFAULT_CACHE_TTL_SECS: u64 = 300;

/// Default maximum cache size (number of entries).
/// Note: Uses `u64` for moka Cache API compatibility (max_capacity parameter).
pub const DEFAULT_CACHE_SIZE: u64 = 1000;

// =============================================================================
// HTTP Status Codes
// Named constants to avoid magic numbers (CLAUDE.md: ハードコード禁止)
// =============================================================================

/// HTTP 429 Too Many Requests - Rate limit exceeded.
pub const HTTP_STATUS_TOO_MANY_REQUESTS: u16 = 429;

/// HTTP 500 Internal Server Error.
pub const HTTP_STATUS_INTERNAL_SERVER_ERROR: u16 = 500;

/// HTTP 502 Bad Gateway.
pub const HTTP_STATUS_BAD_GATEWAY: u16 = 502;

/// HTTP 503 Service Unavailable.
pub const HTTP_STATUS_SERVICE_UNAVAILABLE: u16 = 503;

/// HTTP 504 Gateway Timeout.
pub const HTTP_STATUS_GATEWAY_TIMEOUT: u16 = 504;

// =============================================================================
// Modules
// See: specs/003-http-client-rust/spec.md for details
// =============================================================================

mod client;
mod error;
mod retry;

// Phase 2 (US4)
// mod rate_limit;  // RateLimiter

// Phase 3 (US5)
// mod cache;       // ResponseCache

// =============================================================================
// Public Exports
// =============================================================================

pub use client::{AsyncHttpClient, AsyncHttpClientBuilder};
pub use error::HttpError;
pub use retry::RetryConfig;

// Phase 2 (US4)
// pub use rate_limit::RateLimiter;

// Phase 3
// pub use cache::ResponseCache;
