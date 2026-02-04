//! Async HTTP client for marketschema adapters.
//!
//! This crate will provide a robust HTTP client layer for building market data adapters.
//! Planned features include connection pooling, configurable timeouts, automatic retries,
//! rate limiting, and response caching.
//!
//! # Example (available after US1 implementation)
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
// Modules
// See: specs/003-http-client-rust/spec.md for details
// =============================================================================

mod client;
mod error;

// Phase 2 (US3, US4)
// mod retry;       // RetryConfig
// mod rate_limit;  // RateLimiter

// Phase 3 (US5)
// mod cache;       // ResponseCache

// =============================================================================
// Public Exports
// =============================================================================

pub use client::{AsyncHttpClient, AsyncHttpClientBuilder};
pub use error::HttpError;

// Phase 2
// pub use retry::RetryConfig;
// pub use rate_limit::RateLimiter;

// Phase 3
// pub use cache::ResponseCache;
