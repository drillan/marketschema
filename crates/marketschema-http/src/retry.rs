//! Retry configuration for HTTP requests.
//!
//! This module provides [`RetryConfig`] for configuring automatic retry behavior
//! with exponential backoff and jitter.
//!
//! # Example
//!
//! ```rust
//! use marketschema_http::RetryConfig;
//! use std::collections::HashSet;
//!
//! let config = RetryConfig::new()
//!     .with_max_retries(5)
//!     .with_backoff_factor(1.0)
//!     .with_jitter(0.2);
//! ```

use std::collections::HashSet;
use std::time::Duration;

use rand::Rng;

use crate::{
    DEFAULT_BACKOFF_FACTOR, DEFAULT_JITTER, DEFAULT_MAX_RETRIES, HTTP_STATUS_BAD_GATEWAY,
    HTTP_STATUS_GATEWAY_TIMEOUT, HTTP_STATUS_INTERNAL_SERVER_ERROR,
    HTTP_STATUS_SERVICE_UNAVAILABLE, HTTP_STATUS_TOO_MANY_REQUESTS,
};

/// Retry configuration for failed HTTP requests.
///
/// Configures automatic retry behavior with exponential backoff and optional jitter
/// to handle transient server errors.
///
/// # Default Values
///
/// - `max_retries`: 3
/// - `backoff_factor`: 0.5 (delay = backoff_factor * 2^attempt seconds)
/// - `jitter`: 0.1 (±10% randomization)
/// - `retry_statuses`: [429, 500, 502, 503, 504]
///
/// # Example
///
/// ```rust
/// use marketschema_http::{AsyncHttpClientBuilder, RetryConfig};
///
/// let client = AsyncHttpClientBuilder::new()
///     .retry(RetryConfig::new().with_max_retries(5))
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    max_retries: u32,
    /// Multiplier for exponential backoff (delay = backoff_factor * 2^attempt).
    backoff_factor: f64,
    /// HTTP status codes that trigger retry.
    retry_statuses: HashSet<u16>,
    /// Random jitter factor (0.0 to 1.0) applied to delay.
    jitter: f64,
}

impl RetryConfig {
    /// Create a new retry configuration with default values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    ///
    /// let config = RetryConfig::new();
    /// assert_eq!(config.max_retries(), 3);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the maximum number of retry attempts.
    #[must_use]
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Get the backoff factor for exponential backoff.
    #[must_use]
    pub fn backoff_factor(&self) -> f64 {
        self.backoff_factor
    }

    /// Get the HTTP status codes that trigger retry.
    #[must_use]
    pub fn retry_statuses(&self) -> &HashSet<u16> {
        &self.retry_statuses
    }

    /// Get the jitter factor for randomizing retry delays.
    #[must_use]
    pub fn jitter(&self) -> f64 {
        self.jitter
    }

    /// Set the maximum number of retry attempts.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum number of retries (0 means no retries).
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    ///
    /// let config = RetryConfig::new().with_max_retries(5);
    /// ```
    #[must_use]
    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Set the backoff factor for exponential backoff.
    ///
    /// The delay is calculated as: `backoff_factor * 2^attempt` seconds.
    ///
    /// # Arguments
    ///
    /// * `factor` - The base multiplier for backoff delay. Must be non-negative;
    ///   negative values will be clamped to 0.0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    ///
    /// // With factor 1.0: delays will be 1s, 2s, 4s, 8s...
    /// let config = RetryConfig::new().with_backoff_factor(1.0);
    /// ```
    #[must_use]
    pub fn with_backoff_factor(mut self, factor: f64) -> Self {
        // Clamp negative values to 0.0 (CLAUDE.md: avoid silent failures, but document behavior)
        self.backoff_factor = factor.max(0.0);
        self
    }

    /// Set the HTTP status codes that should trigger retries.
    ///
    /// # Arguments
    ///
    /// * `statuses` - Set of HTTP status codes to retry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    /// use std::collections::HashSet;
    ///
    /// let statuses: HashSet<u16> = [500, 503].into_iter().collect();
    /// let config = RetryConfig::new().with_retry_statuses(statuses);
    /// ```
    #[must_use]
    pub fn with_retry_statuses(mut self, statuses: HashSet<u16>) -> Self {
        self.retry_statuses = statuses;
        self
    }

    /// Set the jitter factor for randomizing retry delays.
    ///
    /// Jitter helps prevent the "thundering herd" problem by randomizing
    /// retry timing across multiple clients.
    ///
    /// # Arguments
    ///
    /// * `jitter` - Factor between 0.0 and 1.0. Delay will be randomized
    ///   within ±jitter of the base delay. Values outside this range will be
    ///   clamped to [0.0, 1.0].
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    ///
    /// // 20% jitter: 1s delay becomes 0.8s-1.2s
    /// let config = RetryConfig::new().with_jitter(0.2);
    /// ```
    #[must_use]
    pub fn with_jitter(mut self, jitter: f64) -> Self {
        // Clamp to valid range [0.0, 1.0] (CLAUDE.md: avoid silent failures, but document behavior)
        self.jitter = jitter.clamp(0.0, 1.0);
        self
    }

    /// Check if the request should be retried.
    ///
    /// Returns `true` if:
    /// - The current attempt is less than `max_retries`
    /// - The status code is in the `retry_statuses` set
    ///
    /// # Arguments
    ///
    /// * `status_code` - The HTTP status code from the response.
    /// * `attempt` - The current retry attempt (0-indexed).
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    ///
    /// let config = RetryConfig::new(); // max_retries: 3
    /// assert!(config.should_retry(503, 0));  // First retry
    /// assert!(config.should_retry(503, 2));  // Third retry
    /// assert!(!config.should_retry(503, 3)); // Exceeded max_retries
    /// assert!(!config.should_retry(404, 0)); // Not in retry_statuses
    /// ```
    #[must_use]
    pub fn should_retry(&self, status_code: u16, attempt: u32) -> bool {
        attempt < self.max_retries && self.retry_statuses.contains(&status_code)
    }

    /// Calculate the delay before the next retry attempt.
    ///
    /// Uses exponential backoff with optional jitter:
    /// - Base delay: `backoff_factor * 2^attempt` seconds
    /// - With jitter: delay * (1.0 + random(-jitter, +jitter))
    ///
    /// # Arguments
    ///
    /// * `attempt` - The retry attempt number (0-indexed).
    ///
    /// # Returns
    ///
    /// The [`Duration`] to wait before the next retry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marketschema_http::RetryConfig;
    /// use std::time::Duration;
    ///
    /// let config = RetryConfig::new().with_jitter(0.0);
    /// // backoff_factor=0.5: 0.5 * 2^0 = 0.5s
    /// assert_eq!(config.get_delay(0), Duration::from_millis(500));
    /// // backoff_factor=0.5: 0.5 * 2^1 = 1.0s
    /// assert_eq!(config.get_delay(1), Duration::from_secs(1));
    /// ```
    #[must_use]
    pub fn get_delay(&self, attempt: u32) -> Duration {
        // Calculate base delay: backoff_factor * 2^attempt
        let base_delay_secs = self.backoff_factor * 2_f64.powi(attempt as i32);

        // Apply jitter if configured
        let final_delay_secs = if self.jitter > 0.0 {
            let mut rng = rand::thread_rng();
            // Generate random factor in range [1 - jitter, 1 + jitter]
            let jitter_factor = 1.0 + rng.gen_range(-self.jitter..=self.jitter);
            base_delay_secs * jitter_factor
        } else {
            base_delay_secs
        };

        // Convert to Duration (handle edge case of negative delay)
        Duration::from_secs_f64(final_delay_secs.max(0.0))
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            backoff_factor: DEFAULT_BACKOFF_FACTOR,
            retry_statuses: [
                HTTP_STATUS_TOO_MANY_REQUESTS,
                HTTP_STATUS_INTERNAL_SERVER_ERROR,
                HTTP_STATUS_BAD_GATEWAY,
                HTTP_STATUS_SERVICE_UNAVAILABLE,
                HTTP_STATUS_GATEWAY_TIMEOUT,
            ]
            .into_iter()
            .collect(),
            jitter: DEFAULT_JITTER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_debug_output() {
        let config = RetryConfig::new();
        let debug = format!("{:?}", config);
        assert!(debug.contains("RetryConfig"));
        assert!(debug.contains("max_retries"));
    }
}
