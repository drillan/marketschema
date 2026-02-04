//! Tests for RetryConfig functionality (US3).
//!
//! TDD: These tests are written BEFORE the implementation.
//! All tests should FAIL initially until RetryConfig is implemented.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use marketschema_http::{AsyncHttpClientBuilder, HttpError, RetryConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Respond, ResponseTemplate};

// =============================================================================
// T049: Test RetryConfig::new() with default values
// =============================================================================

#[test]
fn test_retry_config_new_has_default_max_retries() {
    // FR-R020: max_retries defaults to 3
    let config = RetryConfig::new();
    assert_eq!(config.max_retries, 3);
}

#[test]
fn test_retry_config_new_has_default_backoff_factor() {
    // FR-R021: backoff_factor defaults to 0.5
    let config = RetryConfig::new();
    assert!((config.backoff_factor - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_retry_config_new_has_default_jitter() {
    // FR-R022: jitter defaults to 0.1
    let config = RetryConfig::new();
    assert!((config.jitter - 0.1).abs() < f64::EPSILON);
}

#[test]
fn test_retry_config_new_has_default_retry_statuses() {
    // FR-R019: retry_statuses defaults to [429, 500, 502, 503, 504]
    let config = RetryConfig::new();
    let expected: HashSet<u16> = [429, 500, 502, 503, 504].into_iter().collect();
    assert_eq!(config.retry_statuses, expected);
}

#[test]
fn test_retry_config_default_trait() {
    // RetryConfig::default() should equal RetryConfig::new()
    let from_new = RetryConfig::new();
    let from_default = RetryConfig::default();

    assert_eq!(from_new.max_retries, from_default.max_retries);
    assert!((from_new.backoff_factor - from_default.backoff_factor).abs() < f64::EPSILON);
    assert!((from_new.jitter - from_default.jitter).abs() < f64::EPSILON);
    assert_eq!(from_new.retry_statuses, from_default.retry_statuses);
}

// =============================================================================
// T050: Test RetryConfig builder methods
// =============================================================================

#[test]
fn test_retry_config_max_retries_builder() {
    let config = RetryConfig::new().max_retries(5);
    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_retry_config_backoff_factor_builder() {
    let config = RetryConfig::new().backoff_factor(1.0);
    assert!((config.backoff_factor - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_retry_config_jitter_builder() {
    let config = RetryConfig::new().jitter(0.2);
    assert!((config.jitter - 0.2).abs() < f64::EPSILON);
}

#[test]
fn test_retry_config_retry_statuses_builder() {
    let custom_statuses: HashSet<u16> = [500, 503].into_iter().collect();
    let config = RetryConfig::new().retry_statuses(custom_statuses.clone());
    assert_eq!(config.retry_statuses, custom_statuses);
}

#[test]
fn test_retry_config_builder_chaining() {
    let config = RetryConfig::new()
        .max_retries(10)
        .backoff_factor(2.0)
        .jitter(0.3);

    assert_eq!(config.max_retries, 10);
    assert!((config.backoff_factor - 2.0).abs() < f64::EPSILON);
    assert!((config.jitter - 0.3).abs() < f64::EPSILON);
}

// =============================================================================
// T051: Test RetryConfig::should_retry() logic
// =============================================================================

#[test]
fn test_should_retry_returns_true_for_retryable_status_within_limit() {
    let config = RetryConfig::new(); // max_retries: 3

    // First attempt (attempt=0), status 503 -> should retry
    assert!(config.should_retry(503, 0));
    // Second attempt (attempt=1), status 503 -> should retry
    assert!(config.should_retry(503, 1));
    // Third attempt (attempt=2), status 503 -> should retry
    assert!(config.should_retry(503, 2));
}

#[test]
fn test_should_retry_returns_false_when_max_retries_exceeded() {
    let config = RetryConfig::new(); // max_retries: 3

    // Fourth attempt (attempt=3) exceeds max_retries -> should NOT retry
    assert!(!config.should_retry(503, 3));
    // Fifth attempt -> definitely should NOT retry
    assert!(!config.should_retry(503, 4));
}

#[test]
fn test_should_retry_returns_false_for_non_retryable_status() {
    let config = RetryConfig::new();

    // FR-R023: 400, 401, 403, 404 are NOT retryable
    assert!(!config.should_retry(400, 0));
    assert!(!config.should_retry(401, 0));
    assert!(!config.should_retry(403, 0));
    assert!(!config.should_retry(404, 0));
}

#[test]
fn test_should_retry_for_all_default_retryable_statuses() {
    let config = RetryConfig::new();

    // All default retry statuses should be retryable
    for status in [429, 500, 502, 503, 504] {
        assert!(
            config.should_retry(status, 0),
            "Status {} should be retryable",
            status
        );
    }
}

// =============================================================================
// T052: Test RetryConfig::get_delay() exponential backoff
// =============================================================================

#[test]
fn test_get_delay_first_attempt() {
    // With backoff_factor=0.5 and jitter=0:
    // delay = 0.5 * 2^0 = 0.5 seconds
    let config = RetryConfig::new().jitter(0.0);
    let delay = config.get_delay(0);
    assert_eq!(delay, Duration::from_millis(500));
}

#[test]
fn test_get_delay_second_attempt() {
    // With backoff_factor=0.5 and jitter=0:
    // delay = 0.5 * 2^1 = 1.0 seconds
    let config = RetryConfig::new().jitter(0.0);
    let delay = config.get_delay(1);
    assert_eq!(delay, Duration::from_secs(1));
}

#[test]
fn test_get_delay_third_attempt() {
    // With backoff_factor=0.5 and jitter=0:
    // delay = 0.5 * 2^2 = 2.0 seconds
    let config = RetryConfig::new().jitter(0.0);
    let delay = config.get_delay(2);
    assert_eq!(delay, Duration::from_secs(2));
}

#[test]
fn test_get_delay_with_custom_backoff_factor() {
    // With backoff_factor=1.0 and jitter=0:
    // delay = 1.0 * 2^0 = 1.0 seconds
    // delay = 1.0 * 2^1 = 2.0 seconds
    // delay = 1.0 * 2^2 = 4.0 seconds
    let config = RetryConfig::new().backoff_factor(1.0).jitter(0.0);

    assert_eq!(config.get_delay(0), Duration::from_secs(1));
    assert_eq!(config.get_delay(1), Duration::from_secs(2));
    assert_eq!(config.get_delay(2), Duration::from_secs(4));
}

#[test]
fn test_get_delay_with_jitter_is_within_range() {
    // With jitter=0.1, the delay should be within ±10% of base delay
    // Base delay for attempt 0 with backoff_factor=0.5 is 500ms
    // So delay should be between 450ms and 550ms
    let config = RetryConfig::new(); // backoff_factor=0.5, jitter=0.1

    // Run multiple times to account for randomness
    for _ in 0..100 {
        let delay = config.get_delay(0);
        let base_delay_ms = 500;
        let min_delay = Duration::from_millis((base_delay_ms as f64 * 0.9) as u64);
        let max_delay = Duration::from_millis((base_delay_ms as f64 * 1.1) as u64);

        assert!(
            delay >= min_delay && delay <= max_delay,
            "Delay {:?} should be between {:?} and {:?}",
            delay,
            min_delay,
            max_delay
        );
    }
}

// =============================================================================
// T053: Test automatic retry on 503 then success
// =============================================================================

/// A responder that returns different responses on each call.
struct SequentialResponder {
    call_count: AtomicU32,
    responses: Vec<ResponseTemplate>,
}

impl SequentialResponder {
    fn new(responses: Vec<ResponseTemplate>) -> Self {
        Self {
            call_count: AtomicU32::new(0),
            responses,
        }
    }
}

impl Respond for SequentialResponder {
    fn respond(&self, _request: &wiremock::Request) -> ResponseTemplate {
        let count = self.call_count.fetch_add(1, Ordering::SeqCst) as usize;
        if count < self.responses.len() {
            self.responses[count].clone()
        } else {
            self.responses.last().unwrap().clone()
        }
    }
}

#[tokio::test]
async fn test_retry_succeeds_after_transient_503_errors() {
    // Given: API returns 503, 503, then 200
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/test"))
        .respond_with(SequentialResponder::new(vec![
            ResponseTemplate::new(503).set_body_string("Service Unavailable"),
            ResponseTemplate::new(503).set_body_string("Service Unavailable"),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
        ]))
        .expect(3) // Expect exactly 3 requests
        .mount(&mock_server)
        .await;

    // When: Client with retry config makes request
    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new().max_retries(3).jitter(0.0))
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/test", mock_server.uri()))
        .await;

    // Then: Should succeed with final response
    assert!(result.is_ok());
    let json = result.unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_retry_respects_backoff_timing() {
    // This test verifies that retries happen with increasing delays
    // We can't easily test exact timing, but we can verify the sequence
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/timing"))
        .respond_with(SequentialResponder::new(vec![
            ResponseTemplate::new(503),
            ResponseTemplate::new(503),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({})),
        ]))
        .expect(3)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(
            RetryConfig::new()
                .max_retries(3)
                .backoff_factor(0.1) // Short delays for testing
                .jitter(0.0),
        )
        .build()
        .unwrap();

    let start = std::time::Instant::now();
    let result = client
        .get_json(&format!("{}/api/timing", mock_server.uri()))
        .await;
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    // With backoff_factor=0.1:
    // - First retry after 0.1s (attempt 0: 0.1 * 2^0)
    // - Second retry after 0.2s (attempt 1: 0.1 * 2^1)
    // Total minimum wait: 0.3s
    assert!(
        elapsed >= Duration::from_millis(250),
        "Expected at least 250ms delay, got {:?}",
        elapsed
    );
}

// =============================================================================
// T054: Test no retry on 400/401/403/404
// =============================================================================

#[tokio::test]
async fn test_no_retry_on_400_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/bad"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request"))
        .expect(1) // Should only be called once - no retry
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new())
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/bad", mock_server.uri()))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::Status { status_code, .. } => assert_eq!(status_code, 400),
        other => panic!("Expected Status error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_no_retry_on_401_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/auth"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new())
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/auth", mock_server.uri()))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::Status { status_code, .. } => assert_eq!(status_code, 401),
        other => panic!("Expected Status error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_no_retry_on_403_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/forbidden"))
        .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new())
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/forbidden", mock_server.uri()))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::Status { status_code, .. } => assert_eq!(status_code, 403),
        other => panic!("Expected Status error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_no_retry_on_404_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/notfound"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new())
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/notfound", mock_server.uri()))
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::Status { status_code, .. } => assert_eq!(status_code, 404),
        other => panic!("Expected Status error, got {:?}", other),
    }
}

// =============================================================================
// T055: Test max_retries exceeded
// =============================================================================

#[tokio::test]
async fn test_returns_error_when_max_retries_exceeded() {
    let mock_server = MockServer::start().await;

    // Server always returns 503
    Mock::given(method("GET"))
        .and(path("/api/always503"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .expect(4) // Initial request + 3 retries = 4 total
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(
            RetryConfig::new()
                .max_retries(3)
                .backoff_factor(0.01) // Very short delays for testing
                .jitter(0.0),
        )
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/always503", mock_server.uri()))
        .await;

    // Should fail after exhausting retries
    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::Status { status_code, .. } => assert_eq!(status_code, 503),
        other => panic!("Expected Status error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_retry_on_429_with_retry_after_header() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/ratelimit"))
        .respond_with(SequentialResponder::new(vec![
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "1")
                .set_body_string("Too Many Requests"),
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "success"})),
        ]))
        .expect(2)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new()
        .retry(RetryConfig::new().jitter(0.0))
        .build()
        .unwrap();

    let result = client
        .get_json(&format!("{}/api/ratelimit", mock_server.uri()))
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()["status"], "success");
}

// =============================================================================
// Additional edge case tests
// =============================================================================

#[tokio::test]
async fn test_retry_config_clone() {
    let config = RetryConfig::new().max_retries(5).backoff_factor(1.0);
    let cloned = config.clone();

    assert_eq!(cloned.max_retries, 5);
    assert!((cloned.backoff_factor - 1.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_client_without_retry_config_does_not_retry() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/noretry"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .expect(1) // Only one request - no retry without config
        .mount(&mock_server)
        .await;

    // Client without retry config
    let client = AsyncHttpClientBuilder::new().build().unwrap();

    let result = client
        .get_json(&format!("{}/api/noretry", mock_server.uri()))
        .await;

    assert!(result.is_err());
}
