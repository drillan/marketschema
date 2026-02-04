//! Integration tests for marketschema-http crate
//!
//! These tests verify real-world-like scenarios where multiple features
//! (retry, rate limiting, caching, and error handling) work together.
//!
//! # Test Coverage
//!
//! - Market data API simulation with realistic response patterns
//! - Combined usage of retry, rate limiting, and caching
//! - Error recovery and graceful degradation scenarios
//! - Thread safety under concurrent workloads
//!
//! See: specs/003-http-client-rust/spec.md (Phase 9: T110)

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use marketschema_http::{
    AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter, HttpError, RateLimiter, ResponseCache,
    RetryConfig,
};

// =============================================================================
// Real-world scenario: Market Data Adapter
// =============================================================================

/// Simulated market data response (matches typical exchange API format)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TickerResponse {
    symbol: String,
    bid: f64,
    ask: f64,
    #[serde(rename = "timestamp")]
    timestamp: u64,
}

/// Simulated OHLCV candle data
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CandleResponse {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: u64,
}

/// A realistic market data adapter implementation
/// Demonstrates: OnceLock lazy initialization, retry, rate limiting, and caching
struct MarketDataAdapter {
    http_client: OnceLock<Arc<AsyncHttpClient>>,
    base_url: String,
    retry_config: RetryConfig,
    rate_limiter: Arc<RateLimiter>,
    cache: Arc<ResponseCache>,
}

impl MarketDataAdapter {
    fn new(base_url: &str) -> Self {
        // Configure for production-like settings
        let retry_config = RetryConfig::new()
            .with_max_retries(3)
            .with_backoff_factor(0.1) // Faster for tests
            .with_jitter(0.1);

        // 50 requests per second with burst of 10
        let rate_limiter = Arc::new(RateLimiter::new(50.0, 10));

        // Cache with 100 entries and 30 second TTL
        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(30)));

        Self {
            http_client: OnceLock::new(),
            base_url: base_url.to_string(),
            retry_config,
            rate_limiter,
            cache,
        }
    }

    /// Create adapter with custom client configuration (for testing)
    fn with_client(base_url: &str, client: Arc<AsyncHttpClient>) -> Self {
        let http_client = OnceLock::new();
        let _ = http_client.set(client);

        Self {
            http_client,
            base_url: base_url.to_string(),
            retry_config: RetryConfig::new(),
            rate_limiter: Arc::new(RateLimiter::new(100.0, 20)),
            cache: Arc::new(ResponseCache::new(100, Duration::from_secs(30))),
        }
    }

    /// Get ticker data for a symbol
    async fn get_ticker(&self, symbol: &str) -> Result<TickerResponse, HttpError> {
        let url = format!("{}/api/v1/ticker/{}", self.base_url, symbol);
        let json = self.http_client().get_json(&url).await?;
        serde_json::from_value(json).map_err(|e| HttpError::Parse {
            message: e.to_string(),
            url: Some(url),
            source: Some(e),
        })
    }

    /// Get candle data with parameters
    async fn get_candles(
        &self,
        symbol: &str,
        interval: &str,
        limit: u32,
    ) -> Result<Vec<CandleResponse>, HttpError> {
        let url = format!("{}/api/v1/candles", self.base_url);
        let params = [
            ("symbol", symbol),
            ("interval", interval),
            ("limit", &limit.to_string()),
        ];
        let json = self
            .http_client()
            .get_json_with_params(&url, &params)
            .await?;
        serde_json::from_value(json).map_err(|e| HttpError::Parse {
            message: e.to_string(),
            url: Some(url),
            source: Some(e),
        })
    }
}

impl BaseAdapter for MarketDataAdapter {
    fn http_client(&self) -> Arc<AsyncHttpClient> {
        self.http_client
            .get_or_init(|| {
                Arc::new(
                    AsyncHttpClientBuilder::new()
                        .timeout(Duration::from_secs(10))
                        .retry(self.retry_config.clone())
                        .rate_limit(self.rate_limiter.clone())
                        .cache(self.cache.clone())
                        .build()
                        .expect("Failed to build HTTP client"),
                )
            })
            .clone()
    }
}

// =============================================================================
// Scenario 1: Normal Operation
// =============================================================================

mod normal_operation_tests {
    use super::*;

    /// Test basic ticker retrieval simulating a real exchange API
    #[tokio::test]
    async fn test_get_ticker_success() {
        let mock_server = MockServer::start().await;

        let ticker_data = json!({
            "symbol": "BTCUSD",
            "bid": 49999.50,
            "ask": 50000.50,
            "timestamp": 1706000000000_u64
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/BTCUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&ticker_data))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());
        let ticker = adapter.get_ticker("BTCUSD").await.unwrap();

        assert_eq!(ticker.symbol, "BTCUSD");
        assert_eq!(ticker.bid, 49999.50);
        assert_eq!(ticker.ask, 50000.50);
    }

    /// Test candle data retrieval with query parameters
    #[tokio::test]
    async fn test_get_candles_with_params() {
        let mock_server = MockServer::start().await;

        let candles_data = json!([
            {"open": 50000.0, "high": 50100.0, "low": 49900.0, "close": 50050.0, "volume": 100.5, "timestamp": 1706000000000_u64},
            {"open": 50050.0, "high": 50200.0, "low": 50000.0, "close": 50150.0, "volume": 150.2, "timestamp": 1706003600000_u64}
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/candles"))
            .and(query_param("symbol", "BTCUSD"))
            .and(query_param("interval", "1h"))
            .and(query_param("limit", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&candles_data))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());
        let candles = adapter.get_candles("BTCUSD", "1h", 2).await.unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].open, 50000.0);
        assert_eq!(candles[1].close, 50150.0);
    }
}

// =============================================================================
// Scenario 2: Retry with Transient Failures
// =============================================================================

mod retry_scenario_tests {
    use super::*;

    /// Simulate an API that fails twice (503) then succeeds
    #[tokio::test]
    async fn test_retry_recovers_from_transient_503() {
        let mock_server = MockServer::start().await;
        let request_count = Arc::new(AtomicU32::new(0));

        // This mock will be called multiple times
        // First 2 calls return 503, third call succeeds
        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/ETHUSD"))
            .respond_with({
                let count = request_count.clone();
                move |_: &wiremock::Request| {
                    let attempt = count.fetch_add(1, Ordering::SeqCst);
                    if attempt < 2 {
                        ResponseTemplate::new(503).set_body_string("Service Unavailable")
                    } else {
                        ResponseTemplate::new(200).set_body_json(json!({
                            "symbol": "ETHUSD",
                            "bid": 2999.0,
                            "ask": 3001.0,
                            "timestamp": 1706000000000_u64
                        }))
                    }
                }
            })
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        // Should eventually succeed after retries
        let start = Instant::now();
        let ticker = adapter.get_ticker("ETHUSD").await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(ticker.symbol, "ETHUSD");
        // Verify retries occurred (should take some time due to backoff)
        assert!(elapsed.as_millis() > 50, "Expected retry delays");
        // Should have made 3 requests total
        assert_eq!(request_count.load(Ordering::SeqCst), 3);
    }

    /// Test that 429 rate limit triggers retry with Retry-After
    #[tokio::test]
    async fn test_retry_respects_rate_limit_429() {
        let mock_server = MockServer::start().await;
        let request_count = Arc::new(AtomicU32::new(0));

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/XRPUSD"))
            .respond_with({
                let count = request_count.clone();
                move |_: &wiremock::Request| {
                    let attempt = count.fetch_add(1, Ordering::SeqCst);
                    if attempt == 0 {
                        ResponseTemplate::new(429)
                            .insert_header("Retry-After", "1")
                            .set_body_string("Too Many Requests")
                    } else {
                        ResponseTemplate::new(200).set_body_json(json!({
                            "symbol": "XRPUSD",
                            "bid": 0.50,
                            "ask": 0.51,
                            "timestamp": 1706000000000_u64
                        }))
                    }
                }
            })
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        let start = Instant::now();
        let ticker = adapter.get_ticker("XRPUSD").await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(ticker.symbol, "XRPUSD");
        // Should have waited at least ~1 second for Retry-After
        assert!(
            elapsed.as_millis() >= 800,
            "Expected Retry-After delay, got {:?}",
            elapsed
        );
    }

    /// Test that non-retryable errors (404) are returned immediately
    #[tokio::test]
    async fn test_no_retry_on_404() {
        let mock_server = MockServer::start().await;
        let request_count = Arc::new(AtomicU32::new(0));

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/INVALID"))
            .respond_with({
                let count = request_count.clone();
                move |_: &wiremock::Request| {
                    count.fetch_add(1, Ordering::SeqCst);
                    ResponseTemplate::new(404).set_body_string("Symbol not found")
                }
            })
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());
        let result = adapter.get_ticker("INVALID").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::Status { status_code, .. } => assert_eq!(status_code, 404),
            e => panic!("Expected Status error, got {:?}", e),
        }
        // Should only make 1 request (no retries for 404)
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }
}

// =============================================================================
// Scenario 3: Caching Behavior
// =============================================================================

mod caching_scenario_tests {
    use super::*;

    /// Test that cached responses are returned without hitting the API
    #[tokio::test]
    async fn test_cache_hit_avoids_api_call() {
        let mock_server = MockServer::start().await;
        let request_count = Arc::new(AtomicU32::new(0));

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/SOLUSD"))
            .respond_with({
                let count = request_count.clone();
                move |_: &wiremock::Request| {
                    count.fetch_add(1, Ordering::SeqCst);
                    ResponseTemplate::new(200).set_body_json(json!({
                        "symbol": "SOLUSD",
                        "bid": 100.0,
                        "ask": 100.5,
                        "timestamp": 1706000000000_u64
                    }))
                }
            })
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        // First request - should hit API
        let ticker1 = adapter.get_ticker("SOLUSD").await.unwrap();
        assert_eq!(ticker1.symbol, "SOLUSD");
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        // Second request - should use cache
        let ticker2 = adapter.get_ticker("SOLUSD").await.unwrap();
        assert_eq!(ticker2.symbol, "SOLUSD");
        // Request count should still be 1 (cached)
        assert_eq!(request_count.load(Ordering::SeqCst), 1);

        // Third request - still cached
        let ticker3 = adapter.get_ticker("SOLUSD").await.unwrap();
        assert_eq!(ticker3.symbol, "SOLUSD");
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }

    /// Test that different symbols use different cache keys
    #[tokio::test]
    async fn test_different_symbols_cached_separately() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/BTCUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "symbol": "BTCUSD", "bid": 50000.0, "ask": 50001.0, "timestamp": 1706000000000_u64
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/ETHUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "symbol": "ETHUSD", "bid": 3000.0, "ask": 3001.0, "timestamp": 1706000000000_u64
            })))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        let btc = adapter.get_ticker("BTCUSD").await.unwrap();
        let eth = adapter.get_ticker("ETHUSD").await.unwrap();

        assert_eq!(btc.symbol, "BTCUSD");
        assert_eq!(btc.bid, 50000.0);
        assert_eq!(eth.symbol, "ETHUSD");
        assert_eq!(eth.bid, 3000.0);
    }
}

// =============================================================================
// Scenario 4: Rate Limiting
// =============================================================================

mod rate_limiting_scenario_tests {
    use super::*;

    /// Test that rate limiter prevents exceeding request rate
    #[tokio::test]
    async fn test_rate_limiter_throttles_burst() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/DOTUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "symbol": "DOTUSD", "bid": 5.0, "ask": 5.1, "timestamp": 1706000000000_u64
            })))
            .mount(&mock_server)
            .await;

        // Create adapter with strict rate limit: 5 req/sec, burst of 3
        let rate_limiter = Arc::new(RateLimiter::new(5.0, 3));
        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(1)));

        let client = Arc::new(
            AsyncHttpClientBuilder::new()
                .rate_limit(rate_limiter)
                .cache(cache)
                .build()
                .unwrap(),
        );

        let adapter = MarketDataAdapter::with_client(&mock_server.uri(), client);

        // Clear any cached responses for this test
        adapter.cache.clear();

        let start = Instant::now();

        // Make 6 requests (3 burst + 3 throttled)
        // Need unique URLs to avoid cache hits
        for i in 0..6 {
            let url = format!("{}/api/v1/ticker/DOTUSD?req={}", mock_server.uri(), i);
            let _ = adapter.http_client().get_json(&url).await;
        }

        let elapsed = start.elapsed();

        // With 3 burst and 5 req/sec, requests 4-6 should each wait ~200ms
        // Total expected: ~600ms minimum for the throttled requests
        assert!(
            elapsed.as_millis() >= 400,
            "Rate limiting should have throttled requests, elapsed: {:?}",
            elapsed
        );
    }
}

// =============================================================================
// Scenario 5: Concurrent Access
// =============================================================================

mod concurrent_access_tests {
    use super::*;
    use tokio::task::JoinSet;

    /// Test thread-safe concurrent access to the adapter
    #[tokio::test]
    async fn test_concurrent_ticker_requests() {
        let mock_server = MockServer::start().await;

        let symbols = ["BTCUSD", "ETHUSD", "XRPUSD", "SOLUSD", "ADAUSD"];

        for symbol in symbols {
            Mock::given(method("GET"))
                .and(path(format!("/api/v1/ticker/{}", symbol)))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "symbol": symbol, "bid": 100.0, "ask": 100.5, "timestamp": 1706000000000_u64
                })))
                .mount(&mock_server)
                .await;
        }

        let adapter = Arc::new(MarketDataAdapter::new(&mock_server.uri()));
        let mut join_set = JoinSet::new();

        // Launch concurrent requests for each symbol
        for symbol in symbols {
            let adapter = adapter.clone();
            let symbol = symbol.to_string();
            join_set.spawn(async move { adapter.get_ticker(&symbol).await });
        }

        // Collect all results
        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result.unwrap().unwrap());
        }

        assert_eq!(results.len(), 5);
        // All requests should succeed
        for ticker in &results {
            assert!(symbols.contains(&ticker.symbol.as_str()));
        }
    }

    /// Test that multiple calls to http_client() return the same Arc instance
    #[tokio::test]
    async fn test_shared_client_same_instance() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        // Get client references multiple times
        let client1 = adapter.http_client();
        let client2 = adapter.http_client();
        let client3 = adapter.http_client();

        // All should point to the same Arc (verified via Arc::ptr_eq)
        assert!(
            Arc::ptr_eq(&client1, &client2),
            "http_client() should return the same instance"
        );
        assert!(
            Arc::ptr_eq(&client2, &client3),
            "http_client() should return the same instance"
        );
    }
}

// =============================================================================
// Scenario 6: Error Handling
// =============================================================================

mod error_handling_tests {
    use super::*;

    /// Test proper error propagation for invalid JSON
    #[tokio::test]
    async fn test_parse_error_on_invalid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/LINKUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());
        let result = adapter.get_ticker("LINKUSD").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HttpError::Parse { .. }));
    }

    /// Test proper error types for different HTTP statuses
    #[tokio::test]
    async fn test_error_types_for_http_statuses() {
        let mock_server = MockServer::start().await;

        // 401 Unauthorized
        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/PRIVATE"))
            .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());
        let result = adapter.get_ticker("PRIVATE").await;

        match result.unwrap_err() {
            HttpError::Status { status_code, .. } => assert_eq!(status_code, 401),
            e => panic!("Expected Status error, got {:?}", e),
        }
    }

    /// Test is_retryable() method on different errors
    #[tokio::test]
    async fn test_is_retryable_classification() {
        // Timeout error should be retryable
        let timeout_err = HttpError::Timeout {
            message: "timeout".to_string(),
            url: None,
            source: None,
        };
        assert!(timeout_err.is_retryable());

        // 404 should not be retryable
        let not_found_err = HttpError::Status {
            message: "not found".to_string(),
            url: None,
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert!(!not_found_err.is_retryable());

        // 503 should be retryable
        let service_err = HttpError::Status {
            message: "service unavailable".to_string(),
            url: None,
            status_code: 503,
            response_body: None,
            source: None,
        };
        assert!(service_err.is_retryable());

        // Rate limit should be retryable
        let rate_limit_err = HttpError::RateLimit {
            message: "rate limited".to_string(),
            url: None,
            status_code: 429,
            response_body: None,
            retry_after: Some(Duration::from_secs(60)),
            source: None,
        };
        assert!(rate_limit_err.is_retryable());
    }
}

// =============================================================================
// Scenario 7: Complete Workflow
// =============================================================================

mod complete_workflow_tests {
    use super::*;

    /// Simulates a realistic market data polling workflow
    #[tokio::test]
    async fn test_market_data_polling_workflow() {
        let mock_server = MockServer::start().await;
        let request_count = Arc::new(AtomicU32::new(0));

        // API that occasionally returns 503 (simulating server instability)
        Mock::given(method("GET"))
            .and(path("/api/v1/ticker/AVAXUSD"))
            .respond_with({
                let count = request_count.clone();
                move |_: &wiremock::Request| {
                    let attempt = count.fetch_add(1, Ordering::SeqCst);
                    // First request: 503 (will be retried)
                    // Second request: success
                    // Third+ requests: success (may be cached)
                    if attempt == 0 {
                        ResponseTemplate::new(503)
                    } else {
                        ResponseTemplate::new(200).set_body_json(json!({
                            "symbol": "AVAXUSD",
                            "bid": 20.0 + (attempt as f64 * 0.1),
                            "ask": 20.1 + (attempt as f64 * 0.1),
                            "timestamp": 1706000000000_u64 + (attempt as u64 * 1000)
                        }))
                    }
                }
            })
            .mount(&mock_server)
            .await;

        let adapter = MarketDataAdapter::new(&mock_server.uri());

        // First poll - should retry once (503 -> success)
        let ticker1 = adapter.get_ticker("AVAXUSD").await.unwrap();
        assert_eq!(ticker1.symbol, "AVAXUSD");

        // Second poll - should be cached
        let ticker2 = adapter.get_ticker("AVAXUSD").await.unwrap();
        assert_eq!(ticker2.symbol, "AVAXUSD");

        // Verify retry happened (2 requests for first poll) + cached second poll
        // Total requests: 2 (503 + success)
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }
}
