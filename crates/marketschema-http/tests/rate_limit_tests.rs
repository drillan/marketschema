//! Integration tests for RateLimiter with AsyncHttpClient.
//!
//! These tests verify that the rate limiter correctly controls request rate
//! when integrated into the HTTP client.

use std::sync::Arc;
use std::time::{Duration, Instant};

use marketschema_http::{AsyncHttpClientBuilder, RateLimiter};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test that rate limiter can be configured via builder.
#[tokio::test]
async fn test_rate_limit_builder_method() {
    let limiter = Arc::new(RateLimiter::new(10.0, 5));
    let client = AsyncHttpClientBuilder::new()
        .rate_limit(limiter)
        .build()
        .unwrap();

    // Just verify the client builds successfully with rate limiter
    assert!(client.get_json("http://invalid.test").await.is_err());
}

/// Test that rate limiter throttles rapid requests.
///
/// This test verifies FR-R024: RateLimiter implements token bucket algorithm.
#[tokio::test]
async fn test_rate_limiter_throttles_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .expect(3)
        .mount(&mock_server)
        .await;

    // 2 requests per second, burst of 1 (effectively 1 req then wait 500ms)
    let limiter = Arc::new(RateLimiter::new(2.0, 1));
    let client = AsyncHttpClientBuilder::new()
        .rate_limit(limiter)
        .build()
        .unwrap();

    let url = format!("{}/test", mock_server.uri());

    let start = Instant::now();

    // First request: immediate (uses burst token)
    client.get_json(&url).await.unwrap();

    // Second request: should wait ~500ms (1/2 second for 2 rps)
    client.get_json(&url).await.unwrap();

    // Third request: should wait another ~500ms
    client.get_json(&url).await.unwrap();

    let elapsed = start.elapsed();

    // 3 requests at 2 rps with burst 1:
    // - Request 1: immediate (0ms)
    // - Request 2: wait 500ms
    // - Request 3: wait 500ms
    // Total: ~1000ms
    assert!(
        elapsed >= Duration::from_millis(800),
        "Rate limiting didn't slow down requests enough: {:?}",
        elapsed
    );
    assert!(
        elapsed < Duration::from_millis(1500),
        "Rate limiting was too slow: {:?}",
        elapsed
    );
}

/// Test that burst allows multiple immediate requests.
///
/// This test verifies FR-R026: burst_size allows immediate burst requests.
#[tokio::test]
async fn test_rate_limiter_allows_burst() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/burst"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .expect(5)
        .mount(&mock_server)
        .await;

    // 10 requests per second, burst of 5
    let limiter = Arc::new(RateLimiter::new(10.0, 5));
    let client = AsyncHttpClientBuilder::new()
        .rate_limit(limiter)
        .build()
        .unwrap();

    let url = format!("{}/burst", mock_server.uri());

    let start = Instant::now();

    // All 5 requests should complete quickly (using burst tokens)
    for _ in 0..5 {
        client.get_json(&url).await.unwrap();
    }

    let elapsed = start.elapsed();

    // 5 requests with burst of 5 should all be immediate
    // Allow some time for actual HTTP round-trips
    assert!(
        elapsed < Duration::from_millis(500),
        "Burst requests took too long: {:?}",
        elapsed
    );
}

/// Test that client works without rate limiter.
#[tokio::test]
async fn test_client_without_rate_limiter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/no-limit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .expect(3)
        .mount(&mock_server)
        .await;

    let client = AsyncHttpClientBuilder::new().build().unwrap();

    let url = format!("{}/no-limit", mock_server.uri());

    let start = Instant::now();

    // Requests should be nearly instantaneous without rate limiting
    for _ in 0..3 {
        client.get_json(&url).await.unwrap();
    }

    let elapsed = start.elapsed();

    // Without rate limiting, 3 requests should be very fast
    assert!(
        elapsed < Duration::from_millis(200),
        "Requests without rate limiter took too long: {:?}",
        elapsed
    );
}

/// Test that rate limiter shared across Arc works correctly.
///
/// This test verifies FR-R029: RateLimiter is Send + Sync for multi-task sharing.
#[tokio::test]
async fn test_rate_limiter_shared_across_tasks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/shared"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .expect(2)
        .mount(&mock_server)
        .await;

    // Shared rate limiter
    let limiter = Arc::new(RateLimiter::new(10.0, 10));

    // Client 1
    let client1 = AsyncHttpClientBuilder::new()
        .rate_limit(Arc::clone(&limiter))
        .build()
        .unwrap();

    // Client 2 shares the same limiter
    let client2 = AsyncHttpClientBuilder::new()
        .rate_limit(Arc::clone(&limiter))
        .build()
        .unwrap();

    let url = format!("{}/shared", mock_server.uri());
    let url_clone = url.clone();

    // Spawn tasks using both clients
    let handle1 = tokio::spawn(async move { client1.get_json(&url).await });

    let handle2 = tokio::spawn(async move { client2.get_json(&url_clone).await });

    // Both should succeed
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

/// Test that rate limiter works with retry.
///
/// Tests FR-R024 + FR-R019: Rate limiter applies to retry attempts.
#[tokio::test]
async fn test_rate_limiter_with_retry() {
    use marketschema_http::RetryConfig;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let mock_server = MockServer::start().await;

    // Use atomic counter to track request count
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = Arc::clone(&counter);

    // First request fails with 503, subsequent requests succeed
    Mock::given(method("GET"))
        .and(path("/retry-limited"))
        .respond_with(move |_: &wiremock::Request| {
            let count = counter_clone.fetch_add(1, Ordering::SeqCst);
            if count == 0 {
                ResponseTemplate::new(503)
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true}))
            }
        })
        .expect(2)
        .mount(&mock_server)
        .await;

    // Rate limiter: 10 rps, burst of 2
    let limiter = Arc::new(RateLimiter::new(10.0, 2));
    let retry_config = RetryConfig::new().with_max_retries(1).with_jitter(0.0);

    let client = AsyncHttpClientBuilder::new()
        .rate_limit(limiter)
        .retry(retry_config)
        .build()
        .unwrap();

    let url = format!("{}/retry-limited", mock_server.uri());

    // Should succeed after retry
    let result = client.get_json(&url).await;
    assert!(result.is_ok());
    assert_eq!(
        counter.load(Ordering::SeqCst),
        2,
        "Should have made 2 requests"
    );
}
