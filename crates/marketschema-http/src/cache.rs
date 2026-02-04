//! LRU cache for HTTP responses (US5).
//!
//! This module provides [`ResponseCache`] for caching HTTP responses.
//! It uses the moka library which implements a TinyLFU-based eviction policy
//! for optimal cache hit rates.
//!
//! # Example
//!
//! ```rust
//! use marketschema_http::ResponseCache;
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! # async fn example() {
//! let cache = Arc::new(ResponseCache::new(500, Duration::from_secs(60)));
//!
//! // Store a value
//! cache.set("https://api.example.com/ticker", "{\"price\": 100}".to_string(), None).await;
//!
//! // Retrieve it later
//! if let Some(cached) = cache.get("https://api.example.com/ticker").await {
//!     println!("Cached: {}", cached);
//! }
//! # }
//! ```

use std::time::Duration;

use moka::future::Cache;

/// LRU cache for HTTP responses using moka.
///
/// Uses moka's TinyLFU eviction policy for optimal hit rates.
/// TTL and max capacity are managed by moka internally.
///
/// # Thread Safety
///
/// `ResponseCache` is `Send + Sync` and can be safely shared across tasks
/// via `Arc`. The underlying moka cache handles all synchronization internally.
///
/// # Example
///
/// ```rust
/// use marketschema_http::ResponseCache;
/// use std::sync::Arc;
/// use std::time::Duration;
///
/// # async fn example() {
/// let cache = Arc::new(ResponseCache::new(500, Duration::from_secs(60)));
///
/// // Share across multiple tasks
/// let cache_clone = Arc::clone(&cache);
/// tokio::spawn(async move {
///     cache_clone.set("key", "value".to_string(), None).await;
/// });
/// # }
/// ```
pub struct ResponseCache {
    /// The underlying moka cache.
    /// Key: URL (String), Value: Response body (String)
    inner: Cache<String, String>,

    /// Default TTL for cache entries.
    /// This is stored for reference but moka manages TTL internally.
    #[allow(dead_code)]
    default_ttl: Duration,
}

impl ResponseCache {
    /// Create a new response cache.
    ///
    /// # Arguments
    ///
    /// * `max_size` - Maximum number of cached entries.
    /// * `default_ttl` - Default time-to-live for cache entries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// // Create a cache with 1000 entries and 5 minute TTL
    /// let cache = ResponseCache::new(1000, Duration::from_secs(300));
    /// ```
    #[must_use]
    pub fn new(max_size: u64, default_ttl: Duration) -> Self {
        let inner = Cache::builder()
            .max_capacity(max_size)
            .time_to_live(default_ttl)
            .build();

        Self { inner, default_ttl }
    }

    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key (typically the URL).
    ///
    /// # Returns
    ///
    /// The cached value, or `None` if not found or expired.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let cache = ResponseCache::new(100, Duration::from_secs(60));
    ///
    /// if let Some(cached) = cache.get("https://api.example.com/data").await {
    ///     println!("Found in cache: {}", cached);
    /// }
    /// # }
    /// ```
    pub async fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).await
    }

    /// Set a value in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key.
    /// * `value` - The value to cache.
    /// * `_ttl` - Per-entry TTL (currently ignored, uses default_ttl from construction).
    ///
    /// # Note
    ///
    /// The `ttl` parameter is currently ignored because moka's `Cache::insert`
    /// does not support per-entry TTL. All entries use the `default_ttl`
    /// configured during cache construction. This parameter is kept for
    /// API compatibility and potential future enhancement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let cache = ResponseCache::new(100, Duration::from_secs(60));
    ///
    /// cache.set("https://api.example.com/data", "{\"result\": \"ok\"}".to_string(), None).await;
    /// # }
    /// ```
    pub async fn set(&self, key: &str, value: String, _ttl: Option<Duration>) {
        self.inner.insert(key.to_string(), value).await;
    }

    /// Delete a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let cache = ResponseCache::new(100, Duration::from_secs(60));
    ///
    /// cache.set("key", "value".to_string(), None).await;
    /// cache.delete("key").await;
    /// assert!(cache.get("key").await.is_none());
    /// # }
    /// ```
    pub async fn delete(&self, key: &str) {
        self.inner.invalidate(key).await;
    }

    /// Clear all cached entries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let cache = ResponseCache::new(100, Duration::from_secs(60));
    ///
    /// cache.set("key1", "value1".to_string(), None).await;
    /// cache.set("key2", "value2".to_string(), None).await;
    /// cache.clear();
    /// # }
    /// ```
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    /// Synchronize pending operations (eviction, expiration).
    ///
    /// This method is primarily useful for testing to ensure all pending
    /// eviction and expiration tasks are processed before assertions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::ResponseCache;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let cache = ResponseCache::new(3, Duration::from_secs(60));
    ///
    /// // Add entries exceeding max_size
    /// for i in 0..10 {
    ///     cache.set(&format!("key{}", i), format!("value{}", i), None).await;
    /// }
    ///
    /// // Force eviction to run
    /// cache.sync().await;
    /// # }
    /// ```
    pub async fn sync(&self) {
        self.inner.run_pending_tasks().await;
    }
}

// Compile-time verification that ResponseCache is Send + Sync
// FR-R037: ResponseCache は Send + Sync を実装し、複数タスク間で共有可能
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ResponseCache>();
};
