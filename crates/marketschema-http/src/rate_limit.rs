//! Rate limiter using token bucket algorithm.
//!
//! This module provides [`RateLimiter`] for controlling request rates to prevent
//! API rate limit violations (429 errors).
//!
//! # Example
//!
//! ```rust
//! use marketschema_http::RateLimiter;
//! use std::sync::Arc;
//!
//! // Create a rate limiter: 10 requests/second, burst of 20
//! let limiter = Arc::new(RateLimiter::new(10.0, 20));
//!
//! // Try to acquire a token without blocking
//! if limiter.try_acquire() {
//!     // Make request
//! }
//! ```

use std::sync::Mutex;
use std::time::Instant;

/// Internal state for the rate limiter.
struct RateLimiterState {
    /// Current number of available tokens.
    tokens: f64,
    /// Last time tokens were replenished.
    last_update: Instant,
}

/// Rate limiter using the token bucket algorithm.
///
/// The token bucket algorithm works as follows:
/// - Tokens are added to the bucket at a constant rate (`requests_per_second`)
/// - The bucket has a maximum capacity (`burst_size`)
/// - Each request consumes one token
/// - If no tokens are available, the request must wait
///
/// # Thread Safety
///
/// `RateLimiter` is `Send + Sync` and can be safely shared across tasks via `Arc`.
///
/// # Example
///
/// ```rust
/// use marketschema_http::RateLimiter;
/// use std::sync::Arc;
///
/// #[tokio::main]
/// async fn main() {
///     let limiter = Arc::new(RateLimiter::new(10.0, 10));
///
///     // Acquire a token, waiting if necessary
///     limiter.acquire().await;
///     // Make request...
/// }
/// ```
pub struct RateLimiter {
    /// Maximum requests per second (token refill rate).
    requests_per_second: f64,
    /// Maximum number of tokens in the bucket.
    burst_size: usize,
    /// Internal state protected by mutex.
    state: Mutex<RateLimiterState>,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// # Arguments
    ///
    /// * `requests_per_second` - Maximum requests per second. This determines
    ///   how fast tokens are replenished.
    /// * `burst_size` - Maximum number of tokens that can accumulate. This allows
    ///   short bursts of requests when the bucket is full.
    ///
    /// # Panics
    ///
    /// Panics if `requests_per_second` is not positive or `burst_size` is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RateLimiter;
    ///
    /// // 10 requests/second with burst capacity of 20
    /// let limiter = RateLimiter::new(10.0, 20);
    /// ```
    #[must_use]
    pub fn new(requests_per_second: f64, burst_size: usize) -> Self {
        assert!(
            requests_per_second > 0.0,
            "requests_per_second must be positive, got {}",
            requests_per_second
        );
        assert!(
            burst_size > 0,
            "burst_size must be positive, got {}",
            burst_size
        );

        Self {
            requests_per_second,
            burst_size,
            state: Mutex::new(RateLimiterState {
                tokens: burst_size as f64,
                last_update: Instant::now(),
            }),
        }
    }

    /// Get the configured requests per second.
    #[must_use]
    pub fn requests_per_second(&self) -> f64 {
        self.requests_per_second
    }

    /// Get the configured burst size.
    #[must_use]
    pub fn burst_size(&self) -> usize {
        self.burst_size
    }

    /// Acquire a token, waiting if necessary.
    ///
    /// This method will block (asynchronously) until a token becomes available.
    /// Use this for rate-limited API calls where you want to wait rather than fail.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use marketschema_http::RateLimiter;
    /// use std::sync::Arc;
    ///
    /// # async fn example() {
    /// let limiter = Arc::new(RateLimiter::new(10.0, 10));
    /// limiter.acquire().await;
    /// // Token acquired, safe to make request
    /// # }
    /// ```
    pub async fn acquire(&self) {
        loop {
            // Try to acquire immediately
            let wait_duration = {
                let mut state = self.state.lock().unwrap();
                self.replenish(&mut state);

                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    return; // Token acquired successfully
                }

                // Calculate how long to wait for 1 token
                // tokens_needed = 1.0 - state.tokens (since state.tokens < 1.0)
                let tokens_needed = 1.0 - state.tokens;
                let wait_secs = tokens_needed / self.requests_per_second;
                std::time::Duration::from_secs_f64(wait_secs)
            };

            // Wait outside the lock to avoid blocking other tasks
            tokio::time::sleep(wait_duration).await;
        }
    }

    /// Try to acquire a token without blocking.
    ///
    /// Returns `true` if a token was successfully acquired, `false` if no tokens
    /// are currently available.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(10.0, 10);
    /// if limiter.try_acquire() {
    ///     // Token acquired, safe to make request
    /// } else {
    ///     // Rate limited, try again later or use acquire().await
    /// }
    /// ```
    pub fn try_acquire(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        self.replenish(&mut state);

        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Replenish tokens based on elapsed time.
    ///
    /// This is called internally before token acquisition attempts.
    /// Returns the current token count after replenishment.
    fn replenish(&self, state: &mut RateLimiterState) -> f64 {
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_update);
        let tokens_to_add = elapsed.as_secs_f64() * self.requests_per_second;

        // Add tokens but don't exceed burst_size
        state.tokens = (state.tokens + tokens_to_add).min(self.burst_size as f64);
        state.last_update = now;

        state.tokens
    }
}

// Compile-time verification that RateLimiter is Send + Sync
// This is required for sharing via Arc across async tasks
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RateLimiter>();
};

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // T065: RateLimiter::new() tests
    // =============================================================================

    #[test]
    fn test_new_creates_limiter_with_correct_config() {
        let limiter = RateLimiter::new(10.0, 20);
        assert_eq!(limiter.requests_per_second(), 10.0);
        assert_eq!(limiter.burst_size(), 20);
    }

    #[test]
    fn test_new_initializes_with_full_bucket() {
        // When created, the bucket should be full (tokens == burst_size)
        // This allows initial burst of requests
        let limiter = RateLimiter::new(10.0, 5);

        // Should be able to acquire burst_size tokens immediately
        for _ in 0..5 {
            assert!(limiter.try_acquire(), "Should have tokens available");
        }
        // Next acquire should fail (bucket empty)
        assert!(!limiter.try_acquire(), "Bucket should be empty");
    }

    #[test]
    #[should_panic(expected = "requests_per_second must be positive")]
    fn test_new_panics_on_zero_rps() {
        RateLimiter::new(0.0, 10);
    }

    #[test]
    #[should_panic(expected = "requests_per_second must be positive")]
    fn test_new_panics_on_negative_rps() {
        RateLimiter::new(-5.0, 10);
    }

    #[test]
    #[should_panic(expected = "burst_size must be positive")]
    fn test_new_panics_on_zero_burst_size() {
        RateLimiter::new(10.0, 0);
    }

    // =============================================================================
    // T066: RateLimiter::try_acquire() tests
    // =============================================================================

    #[test]
    fn test_try_acquire_returns_true_when_token_available() {
        let limiter = RateLimiter::new(10.0, 5);
        assert!(limiter.try_acquire());
    }

    #[test]
    fn test_try_acquire_returns_false_when_bucket_empty() {
        let limiter = RateLimiter::new(10.0, 2);

        // Drain the bucket
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());

        // Should now return false
        assert!(!limiter.try_acquire());
    }

    #[test]
    fn test_try_acquire_decrements_tokens() {
        let limiter = RateLimiter::new(10.0, 3);

        // Each successful acquire should decrement
        assert!(limiter.try_acquire()); // 2 left
        assert!(limiter.try_acquire()); // 1 left
        assert!(limiter.try_acquire()); // 0 left
        assert!(!limiter.try_acquire()); // empty
    }

    // =============================================================================
    // T067: RateLimiter::acquire() async waiting tests
    // =============================================================================

    #[tokio::test]
    async fn test_acquire_returns_immediately_when_token_available() {
        let limiter = RateLimiter::new(10.0, 5);

        // Should complete immediately since bucket is full
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();

        // Should be very fast (< 10ms)
        assert!(
            elapsed.as_millis() < 10,
            "acquire() took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_acquire_waits_when_bucket_empty() {
        let limiter = RateLimiter::new(10.0, 1); // 10 rps = 100ms per token

        // Drain the bucket
        limiter.acquire().await;

        // Next acquire should wait for token replenishment
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();

        // Should wait approximately 100ms (1/10 second for 10 rps)
        // Allow 50-200ms tolerance for timing variance
        assert!(
            elapsed.as_millis() >= 50,
            "acquire() didn't wait long enough: {:?}",
            elapsed
        );
        assert!(
            elapsed.as_millis() < 200,
            "acquire() waited too long: {:?}",
            elapsed
        );
    }

    // =============================================================================
    // T068: Burst behavior tests
    // =============================================================================

    #[test]
    fn test_burst_allows_multiple_immediate_requests() {
        let limiter = RateLimiter::new(10.0, 5);

        // Should be able to acquire all burst_size tokens immediately
        let start = Instant::now();
        for i in 0..5 {
            assert!(limiter.try_acquire(), "Failed to acquire token {}", i);
        }
        let elapsed = start.elapsed();

        // All acquisitions should be nearly instant
        assert!(
            elapsed.as_millis() < 5,
            "Burst acquisition took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_burst_size_limits_concurrent_requests() {
        let limiter = RateLimiter::new(10.0, 3);

        // Consume all burst tokens
        limiter.acquire().await;
        limiter.acquire().await;
        limiter.acquire().await;

        // Fourth request should need to wait
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();

        // Should have waited for token replenishment
        assert!(
            elapsed.as_millis() >= 50,
            "Should have waited for replenishment: {:?}",
            elapsed
        );
    }

    // =============================================================================
    // T069: Token replenishment over time tests
    // =============================================================================

    #[tokio::test]
    async fn test_tokens_replenish_over_time() {
        let limiter = RateLimiter::new(10.0, 2); // 10 rps = 100ms per token

        // Drain the bucket
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(!limiter.try_acquire());

        // Wait for 1 token to replenish (100ms for 10 rps)
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;

        // Should have at least 1 token now
        assert!(
            limiter.try_acquire(),
            "Token should have replenished after waiting"
        );
    }

    #[tokio::test]
    async fn test_tokens_do_not_exceed_burst_size() {
        let limiter = RateLimiter::new(100.0, 3); // 100 rps, burst of 3

        // Wait long enough for many tokens to "accumulate"
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Should still only have burst_size tokens (3), not more
        assert!(limiter.try_acquire()); // 1
        assert!(limiter.try_acquire()); // 2
        assert!(limiter.try_acquire()); // 3
        assert!(!limiter.try_acquire(), "Should not exceed burst_size");
    }

    // =============================================================================
    // T070: Send + Sync bounds tests
    // =============================================================================

    #[test]
    fn test_rate_limiter_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RateLimiter>();
    }

    #[test]
    fn test_rate_limiter_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RateLimiter>();
    }

    #[tokio::test]
    async fn test_rate_limiter_works_across_tasks() {
        use std::sync::Arc;

        let limiter = Arc::new(RateLimiter::new(100.0, 10));
        let limiter_clone = Arc::clone(&limiter);

        // Spawn a task that uses the limiter
        let handle = tokio::spawn(async move {
            limiter_clone.acquire().await;
            true
        });

        // Use limiter in main task
        limiter.acquire().await;

        // Wait for spawned task
        let result = handle.await.unwrap();
        assert!(result);
    }
}
