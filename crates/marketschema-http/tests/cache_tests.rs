//! Tests for ResponseCache (US5: LRU キャッシュによるレスポンスキャッシュ).
//!
//! These tests verify the ResponseCache implementation according to FR-R030 ~ FR-R037.
//!
//! Test naming convention: test_<method>_<scenario>

use std::sync::Arc;
use std::time::Duration;

use marketschema_http::ResponseCache;

// =============================================================================
// T081: ResponseCache::new() tests
// =============================================================================

/// Test that ResponseCache can be created with custom max_size and default_ttl.
/// FR-R031, FR-R032: max_size と default_ttl で設定可能
#[tokio::test]
async fn test_new_with_custom_settings() {
    let cache = ResponseCache::new(500, Duration::from_secs(60));

    // Verify cache is created (we can only verify by using it)
    assert!(cache.get("nonexistent").await.is_none());
}

/// Test that ResponseCache can be created with default-like values.
/// FR-R031: デフォルト: 5分, FR-R032: デフォルト: 1000
#[tokio::test]
async fn test_new_with_defaults() {
    use marketschema_http::{DEFAULT_CACHE_SIZE, DEFAULT_CACHE_TTL_SECS};

    let cache = ResponseCache::new(
        DEFAULT_CACHE_SIZE,
        Duration::from_secs(DEFAULT_CACHE_TTL_SECS),
    );

    assert!(cache.get("nonexistent").await.is_none());
}

// =============================================================================
// T082: ResponseCache::get() and set() tests
// =============================================================================

/// Test basic set and get operations.
/// FR-R033, FR-R034: get はキャッシュされた値を返す、set はエントリを追加
#[tokio::test]
async fn test_set_and_get_basic() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "value1".to_string()).await;

    let result = cache.get("key1").await;
    assert_eq!(result, Some("value1".to_string()));
}

/// Test that get returns None for nonexistent keys.
/// FR-R033: キャッシュされた値または None を返す
#[tokio::test]
async fn test_get_nonexistent_returns_none() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    let result = cache.get("nonexistent").await;
    assert!(result.is_none());
}

/// Test that set overwrites existing values.
/// FR-R034: set はエントリを追加（既存キーは上書き）
#[tokio::test]
async fn test_set_overwrites_existing() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "original".to_string()).await;
    cache.set("key1", "updated".to_string()).await;

    let result = cache.get("key1").await;
    assert_eq!(result, Some("updated".to_string()));
}

/// Test multiple entries can be stored and retrieved.
#[tokio::test]
async fn test_multiple_entries() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "value1".to_string()).await;
    cache.set("key2", "value2".to_string()).await;
    cache.set("key3", "value3".to_string()).await;

    assert_eq!(cache.get("key1").await, Some("value1".to_string()));
    assert_eq!(cache.get("key2").await, Some("value2".to_string()));
    assert_eq!(cache.get("key3").await, Some("value3".to_string()));
}

// =============================================================================
// T083: Cache TTL expiration tests
// =============================================================================

/// Test that entries expire after TTL.
/// FR-R031: キャッシュの TTL が経過すると新しいリクエストが送信される
#[tokio::test]
async fn test_ttl_expiration() {
    // Use a very short TTL for testing
    let ttl = Duration::from_millis(100);
    let cache = ResponseCache::new(100, ttl);

    cache.set("key1", "value1".to_string()).await;

    // Value should be available immediately
    assert_eq!(cache.get("key1").await, Some("value1".to_string()));

    // Wait for TTL to expire (plus some buffer for moka's async eviction)
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Value should be expired
    assert!(cache.get("key1").await.is_none());
}

/// Test that entries remain available before TTL expires.
#[tokio::test]
async fn test_entry_available_before_ttl() {
    let ttl = Duration::from_secs(300); // 5 minutes
    let cache = ResponseCache::new(100, ttl);

    cache.set("key1", "value1".to_string()).await;

    // Small delay
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Value should still be available
    assert_eq!(cache.get("key1").await, Some("value1".to_string()));
}

// =============================================================================
// T084: Cache max_size LRU eviction tests
// =============================================================================

/// Test that cache evicts entries when max_size is reached.
/// FR-R032: max_size に達すると LRU アルゴリズムで最も古いエントリが削除される
///
/// Note: moka uses TinyLFU (not strict LRU) and eviction is asynchronous.
/// This test verifies that eviction eventually occurs, but doesn't guarantee
/// exact timing or which entries are evicted.
#[tokio::test]
async fn test_lru_eviction_on_max_size() {
    // Small cache size for testing
    let cache = ResponseCache::new(3, Duration::from_secs(300));

    // Fill the cache
    cache.set("key1", "value1".to_string()).await;
    cache.set("key2", "value2".to_string()).await;
    cache.set("key3", "value3".to_string()).await;

    // Add more entries to trigger eviction
    for i in 4..=10 {
        cache.set(&format!("key{}", i), format!("value{}", i)).await;
    }

    // Force moka to process eviction
    cache.sync().await;

    // Count how many entries remain
    let mut present_count = 0;
    for i in 1..=10 {
        if cache.get(&format!("key{}", i)).await.is_some() {
            present_count += 1;
        }
    }

    // Cache should have evicted some entries to stay near max_size
    // Due to TinyLFU policy, exact count may vary, but should be limited
    assert!(
        present_count <= 4,
        "Cache should evict entries to stay near max_size (3), found {} entries",
        present_count
    );
}

// =============================================================================
// T085: ResponseCache::delete() and clear() tests
// =============================================================================

/// Test that delete removes a specific entry.
/// FR-R035: delete はエントリを削除
#[tokio::test]
async fn test_delete_removes_entry() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "value1".to_string()).await;
    cache.set("key2", "value2".to_string()).await;

    // Delete key1
    cache.delete("key1").await;

    // key1 should be gone, key2 should remain
    assert!(cache.get("key1").await.is_none());
    assert_eq!(cache.get("key2").await, Some("value2".to_string()));
}

/// Test that delete on nonexistent key is a no-op.
#[tokio::test]
async fn test_delete_nonexistent_is_noop() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "value1".to_string()).await;

    // Delete nonexistent key should not affect existing entries
    cache.delete("nonexistent").await;

    assert_eq!(cache.get("key1").await, Some("value1".to_string()));
}

/// Test that clear removes all entries.
/// FR-R036: clear はすべてのエントリを削除
#[tokio::test]
async fn test_clear_removes_all_entries() {
    let cache = ResponseCache::new(100, Duration::from_secs(300));

    cache.set("key1", "value1".to_string()).await;
    cache.set("key2", "value2".to_string()).await;
    cache.set("key3", "value3".to_string()).await;

    // Clear all entries
    cache.clear();

    // Give moka time to process invalidation
    tokio::time::sleep(Duration::from_millis(50)).await;

    // All entries should be gone
    assert!(cache.get("key1").await.is_none());
    assert!(cache.get("key2").await.is_none());
    assert!(cache.get("key3").await.is_none());
}

// =============================================================================
// T086: Cache integration with AsyncHttpClient (basic tests)
// =============================================================================

/// Test that ResponseCache is Send + Sync (compile-time check).
/// FR-R037: ResponseCache は Send + Sync を実装
#[tokio::test]
async fn test_response_cache_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<ResponseCache>();
    assert_sync::<ResponseCache>();
}

/// Test that ResponseCache can be shared across tasks via Arc.
/// FR-R037: 複数タスク間で共有可能
#[tokio::test]
async fn test_cache_shared_across_tasks() {
    let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));

    let cache_clone = Arc::clone(&cache);
    let handle = tokio::spawn(async move {
        cache_clone.set("from_task", "task_value".to_string()).await;
    });

    handle.await.unwrap();

    // Value set in spawned task should be visible
    assert_eq!(cache.get("from_task").await, Some("task_value".to_string()));
}

/// Test concurrent access to cache.
#[tokio::test]
async fn test_concurrent_access() {
    let cache = Arc::new(ResponseCache::new(1000, Duration::from_secs(300)));

    let mut handles = vec![];

    // Spawn multiple tasks writing to the cache
    for i in 0..10 {
        let cache_clone = Arc::clone(&cache);
        let handle = tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            cache_clone.set(&key, value).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all entries are present
    for i in 0..10 {
        let key = format!("key{}", i);
        let expected = format!("value{}", i);
        assert_eq!(cache.get(&key).await, Some(expected));
    }
}

// =============================================================================
// AsyncHttpClient cache integration tests
// =============================================================================

mod client_integration {
    use super::*;
    use marketschema_http::AsyncHttpClientBuilder;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Test that AsyncHttpClientBuilder accepts cache configuration.
    #[tokio::test]
    async fn test_builder_accepts_cache() {
        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(60)));

        let client = AsyncHttpClientBuilder::new()
            .cache(cache)
            .build()
            .expect("Client should build with cache");

        // Just verify it builds successfully
        drop(client);
    }

    /// Test that cached responses are returned for repeated requests.
    /// US5 Acceptance Scenario 1: Same URL returns cached response without API call.
    #[tokio::test]
    async fn test_cached_response_returned_for_same_url() {
        let mock_server = MockServer::start().await;

        // Counter to track how many times the API is called
        Mock::given(method("GET"))
            .and(path("/ticker"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"price": 100}"#))
            .expect(1) // Should only be called once due to caching
            .mount(&mock_server)
            .await;

        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));
        let client = AsyncHttpClientBuilder::new()
            .cache(Arc::clone(&cache))
            .build()
            .unwrap();

        let url = format!("{}/ticker", mock_server.uri());

        // First request - should hit the API
        let response1 = client.get_text(&url).await.unwrap();
        assert_eq!(response1, r#"{"price": 100}"#);

        // Second request - should use cache
        let response2 = client.get_text(&url).await.unwrap();
        assert_eq!(response2, r#"{"price": 100}"#);
    }

    /// Test that JSON responses are cached correctly.
    #[tokio::test]
    async fn test_cached_json_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"value": 42}"#))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));
        let client = AsyncHttpClientBuilder::new()
            .cache(Arc::clone(&cache))
            .build()
            .unwrap();

        let url = format!("{}/data", mock_server.uri());

        // First request
        let json1 = client.get_json(&url).await.unwrap();
        assert_eq!(json1["value"], 42);

        // Second request - should use cache
        let json2 = client.get_json(&url).await.unwrap();
        assert_eq!(json2["value"], 42);
    }

    /// Test that different URLs are cached separately.
    #[tokio::test]
    async fn test_different_urls_cached_separately() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/ticker1"))
            .respond_with(ResponseTemplate::new(200).set_body_string("response1"))
            .expect(1)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/ticker2"))
            .respond_with(ResponseTemplate::new(200).set_body_string("response2"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));
        let client = AsyncHttpClientBuilder::new()
            .cache(Arc::clone(&cache))
            .build()
            .unwrap();

        let url1 = format!("{}/ticker1", mock_server.uri());
        let url2 = format!("{}/ticker2", mock_server.uri());

        // Both URLs should hit their respective endpoints
        assert_eq!(client.get_text(&url1).await.unwrap(), "response1");
        assert_eq!(client.get_text(&url2).await.unwrap(), "response2");

        // Second requests should use cache
        assert_eq!(client.get_text(&url1).await.unwrap(), "response1");
        assert_eq!(client.get_text(&url2).await.unwrap(), "response2");
    }

    /// Test that client works without cache (cache is optional).
    #[tokio::test]
    async fn test_client_works_without_cache() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string("response"))
            .expect(2) // Called twice since no caching
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();

        let url = format!("{}/data", mock_server.uri());

        // Both requests should hit the API
        assert_eq!(client.get_text(&url).await.unwrap(), "response");
        assert_eq!(client.get_text(&url).await.unwrap(), "response");
    }

    /// Test that error responses are NOT cached.
    /// Both requests should hit the server since errors are not cached.
    #[tokio::test]
    async fn test_error_responses_not_cached() {
        let mock_server = MockServer::start().await;

        // Expect server to be hit twice (error is not cached)
        Mock::given(method("GET"))
            .and(path("/flaky"))
            .respond_with(ResponseTemplate::new(500))
            .expect(2)
            .mount(&mock_server)
            .await;

        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));
        let client = AsyncHttpClientBuilder::new()
            .cache(Arc::clone(&cache))
            .build()
            .unwrap();

        let url = format!("{}/flaky", mock_server.uri());

        // First request fails
        let result1 = client.get_text(&url).await;
        assert!(result1.is_err());

        // Second request should also hit the server (not cached)
        let result2 = client.get_text(&url).await;
        assert!(result2.is_err());

        // wiremock will verify that exactly 2 requests were made
    }

    /// Test cache key generation with query parameters containing special characters.
    #[tokio::test]
    async fn test_cache_key_with_special_characters() {
        let mock_server = MockServer::start().await;

        use wiremock::matchers::query_param;

        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("q", "hello world"))
            .respond_with(ResponseTemplate::new(200).set_body_string("result1"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(300)));
        let client = AsyncHttpClientBuilder::new()
            .cache(Arc::clone(&cache))
            .build()
            .unwrap();

        let url = format!("{}/search", mock_server.uri());

        // First request with space in query param
        let response1 = client
            .get_text_with_params(&url, &[("q", "hello world")])
            .await
            .unwrap();
        assert_eq!(response1, "result1");

        // Second request should use cache (same URL+params)
        let response2 = client
            .get_text_with_params(&url, &[("q", "hello world")])
            .await
            .unwrap();
        assert_eq!(response2, "result1");
    }
}
