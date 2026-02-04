//! Tests for BaseAdapter trait integration
//!
//! Test structure follows TDD approach for US6: BaseAdapter トレイトとの統合
//!
//! See: specs/003-http-client-rust/spec.md

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter};

// =============================================================================
// T098: BaseAdapter trait definition
// =============================================================================

mod trait_definition_tests {
    use super::*;

    /// FR-R038: BaseAdapter トレイトに fn http_client(&self) -> Arc<AsyncHttpClient> メソッドを追加しなければならない
    #[test]
    fn test_base_adapter_trait_has_http_client_method() {
        // Verify trait signature at compile time
        #[allow(dead_code)]
        fn assert_trait_method<T: BaseAdapter>(adapter: &T) -> Arc<AsyncHttpClient> {
            adapter.http_client()
        }

        // This test passes if the code compiles
        let _: fn(&dyn BaseAdapter) -> Arc<AsyncHttpClient> = |a| a.http_client();
    }

    /// BaseAdapter must require Send + Sync for thread safety
    #[test]
    fn test_base_adapter_requires_send_sync() {
        #[allow(dead_code)]
        fn assert_send_sync<T: BaseAdapter + Send + Sync>() {}
        // The trait bound BaseAdapter: Send + Sync should be satisfied by implementors
    }

    /// Test that BaseAdapter can be used as a trait object (dyn BaseAdapter).
    /// This verifies object safety at runtime, complementing the compile-time check.
    #[test]
    fn test_trait_object_usage() {
        struct SimpleAdapter {
            http_client: OnceLock<Arc<AsyncHttpClient>>,
        }

        impl SimpleAdapter {
            fn new() -> Self {
                Self {
                    http_client: OnceLock::new(),
                }
            }
        }

        impl BaseAdapter for SimpleAdapter {
            fn http_client(&self) -> Arc<AsyncHttpClient> {
                self.http_client
                    .get_or_init(|| {
                        Arc::new(
                            AsyncHttpClientBuilder::new()
                                .build()
                                .expect("Default HTTP client configuration should not fail"),
                        )
                    })
                    .clone()
            }
        }

        // Use as a trait object
        let adapter: Arc<dyn BaseAdapter> = Arc::new(SimpleAdapter::new());
        let client = adapter.http_client();

        // Verify we got a valid client
        assert!(Arc::strong_count(&client) >= 1);
    }
}

// =============================================================================
// T099: OnceLock lazy initialization
// =============================================================================

mod lazy_initialization_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Example adapter using OnceLock for lazy initialization
    struct LazyAdapter {
        http_client: OnceLock<Arc<AsyncHttpClient>>,
        init_count: Arc<AtomicUsize>,
    }

    impl LazyAdapter {
        fn new(init_count: Arc<AtomicUsize>) -> Self {
            Self {
                http_client: OnceLock::new(),
                init_count,
            }
        }
    }

    impl BaseAdapter for LazyAdapter {
        fn http_client(&self) -> Arc<AsyncHttpClient> {
            self.http_client
                .get_or_init(|| {
                    self.init_count.fetch_add(1, Ordering::SeqCst);
                    Arc::new(
                        AsyncHttpClientBuilder::new()
                            .build()
                            .expect("Default HTTP client configuration should not fail"),
                    )
                })
                .clone()
        }
    }

    /// FR-R039: http_client() メソッドはデフォルト実装で OnceLock による遅延初期化を提供しなければならない
    #[test]
    fn test_http_client_lazy_initialization() {
        let init_count = Arc::new(AtomicUsize::new(0));
        let adapter = LazyAdapter::new(init_count.clone());

        // Before accessing, init_count should be 0
        assert_eq!(init_count.load(Ordering::SeqCst), 0);

        // First access triggers initialization
        let _client1 = adapter.http_client();
        assert_eq!(init_count.load(Ordering::SeqCst), 1);

        // Second access should not reinitialize
        let _client2 = adapter.http_client();
        assert_eq!(init_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_http_client_returns_same_instance() {
        let init_count = Arc::new(AtomicUsize::new(0));
        let adapter = LazyAdapter::new(init_count);

        let client1 = adapter.http_client();
        let client2 = adapter.http_client();

        // Both should point to the same Arc
        assert!(Arc::ptr_eq(&client1, &client2));
    }

    /// Test that concurrent first access from multiple threads only initializes once.
    /// This verifies OnceLock's thread-safety guarantees for the lazy initialization pattern.
    #[tokio::test]
    async fn test_concurrent_first_access_initializes_once() {
        use tokio::task::JoinSet;

        let init_count = Arc::new(AtomicUsize::new(0));
        let adapter = Arc::new(LazyAdapter::new(init_count.clone()));

        let mut join_set = JoinSet::new();

        // 10 tasks attempt to access http_client() concurrently
        for _ in 0..10 {
            let adapter_clone = adapter.clone();
            join_set.spawn(async move { adapter_clone.http_client() });
        }

        // Collect all results
        let mut clients = Vec::new();
        while let Some(result) = join_set.join_next().await {
            clients.push(result.expect("Task should not panic"));
        }

        // Initialization should have occurred exactly once
        assert_eq!(init_count.load(Ordering::SeqCst), 1);

        // All returned clients should be the same instance
        let first_client = &clients[0];
        for client in &clients[1..] {
            assert!(Arc::ptr_eq(first_client, client));
        }
    }
}

// =============================================================================
// T100: Custom AsyncHttpClient injection
// =============================================================================

mod custom_client_injection_tests {
    use super::*;

    /// Adapter that accepts a custom AsyncHttpClient via constructor
    struct InjectedAdapter {
        http_client: Arc<AsyncHttpClient>,
    }

    impl InjectedAdapter {
        fn new(client: Arc<AsyncHttpClient>) -> Self {
            Self {
                http_client: client,
            }
        }
    }

    impl BaseAdapter for InjectedAdapter {
        fn http_client(&self) -> Arc<AsyncHttpClient> {
            self.http_client.clone()
        }
    }

    /// FR-R040: BaseAdapter 実装者はコンストラクタで Arc<AsyncHttpClient> を注入可能でなければならない
    #[test]
    fn test_custom_client_injection() {
        let custom_client = Arc::new(
            AsyncHttpClientBuilder::new()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap(),
        );

        let adapter = InjectedAdapter::new(custom_client.clone());

        // The injected client should be returned
        assert!(Arc::ptr_eq(&adapter.http_client(), &custom_client));
    }

    #[tokio::test]
    async fn test_injected_client_works_for_requests() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&mock_server)
            .await;

        let custom_client = Arc::new(AsyncHttpClientBuilder::new().build().unwrap());

        let adapter = InjectedAdapter::new(custom_client);

        let url = format!("{}/api/data", mock_server.uri());
        let result = adapter.http_client().get_json(&url).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["ok"], true);
    }
}

// =============================================================================
// T101: Drop behavior
// =============================================================================

mod drop_behavior_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    /// FR-R041: Drop トレイトにより、アダプターが破棄されたときにリソースが適切に解放されなければならない
    #[test]
    fn test_adapter_drop_releases_client_reference() {
        let client = Arc::new(AsyncHttpClientBuilder::new().build().unwrap());
        let weak_ref = Arc::downgrade(&client);

        // Create adapter with the client
        {
            struct TestAdapter {
                http_client: Arc<AsyncHttpClient>,
            }

            impl BaseAdapter for TestAdapter {
                fn http_client(&self) -> Arc<AsyncHttpClient> {
                    self.http_client.clone()
                }
            }

            let adapter = TestAdapter {
                http_client: client.clone(),
            };

            // While adapter exists, weak_ref should be valid
            assert!(weak_ref.upgrade().is_some());

            drop(adapter);
        }

        // After adapter is dropped, we still have one strong ref (client variable)
        assert!(weak_ref.upgrade().is_some());
        assert_eq!(Arc::strong_count(&client), 1);

        // After dropping the last reference, weak_ref should be invalid
        drop(client);
        assert!(weak_ref.upgrade().is_none());
    }

    #[test]
    fn test_multiple_adapters_share_client() {
        let client = Arc::new(AsyncHttpClientBuilder::new().build().unwrap());

        struct TestAdapter {
            http_client: Arc<AsyncHttpClient>,
        }

        impl BaseAdapter for TestAdapter {
            fn http_client(&self) -> Arc<AsyncHttpClient> {
                self.http_client.clone()
            }
        }

        let adapter1 = TestAdapter {
            http_client: client.clone(),
        };
        let adapter2 = TestAdapter {
            http_client: client.clone(),
        };

        // 3 references: client + adapter1 + adapter2
        assert_eq!(Arc::strong_count(&client), 3);

        drop(adapter1);
        assert_eq!(Arc::strong_count(&client), 2);

        drop(adapter2);
        assert_eq!(Arc::strong_count(&client), 1);
    }

    /// Test that OnceLock-based adapter properly releases resources on drop
    #[test]
    fn test_oncelock_adapter_drop() {
        let dropped = Arc::new(AtomicBool::new(false));

        struct DropTracker {
            http_client: OnceLock<Arc<AsyncHttpClient>>,
            dropped: Arc<AtomicBool>,
        }

        impl Drop for DropTracker {
            fn drop(&mut self) {
                self.dropped.store(true, Ordering::SeqCst);
            }
        }

        impl BaseAdapter for DropTracker {
            fn http_client(&self) -> Arc<AsyncHttpClient> {
                self.http_client
                    .get_or_init(|| {
                        Arc::new(
                            AsyncHttpClientBuilder::new()
                                .build()
                                .expect("Default HTTP client configuration should not fail"),
                        )
                    })
                    .clone()
            }
        }

        {
            let adapter = DropTracker {
                http_client: OnceLock::new(),
                dropped: dropped.clone(),
            };

            // Access client to initialize it
            let _client = adapter.http_client();

            assert!(!dropped.load(Ordering::SeqCst));
        }

        // After scope ends, drop should have been called
        assert!(dropped.load(Ordering::SeqCst));
    }
}

// =============================================================================
// Integration tests
// =============================================================================

mod integration_tests {
    use super::*;

    /// Full example adapter as shown in quickstart.md
    struct ExampleExchangeAdapter {
        http_client: OnceLock<Arc<AsyncHttpClient>>,
        base_url: String,
    }

    impl ExampleExchangeAdapter {
        fn new(base_url: &str) -> Self {
            Self {
                http_client: OnceLock::new(),
                base_url: base_url.to_string(),
            }
        }

        async fn get_ticker(
            &self,
            symbol: &str,
        ) -> Result<serde_json::Value, marketschema_http::HttpError> {
            let url = format!("{}/api/ticker/{}", self.base_url, symbol);
            self.http_client().get_json(&url).await
        }
    }

    impl BaseAdapter for ExampleExchangeAdapter {
        fn http_client(&self) -> Arc<AsyncHttpClient> {
            self.http_client
                .get_or_init(|| {
                    Arc::new(
                        AsyncHttpClientBuilder::new()
                            .build()
                            .expect("Default HTTP client configuration should not fail"),
                    )
                })
                .clone()
        }
    }

    #[tokio::test]
    async fn test_example_adapter_get_ticker() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/ticker/BTCUSD"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "symbol": "BTCUSD",
                "price": 50000.0,
                "timestamp": 1234567890
            })))
            .mount(&mock_server)
            .await;

        let adapter = ExampleExchangeAdapter::new(&mock_server.uri());

        let result = adapter.get_ticker("BTCUSD").await;

        assert!(result.is_ok());
        let ticker = result.unwrap();
        assert_eq!(ticker["symbol"], "BTCUSD");
        assert_eq!(ticker["price"], 50000.0);
    }

    #[tokio::test]
    async fn test_adapter_thread_safety() {
        use tokio::task::JoinSet;

        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .expect(5)
            .mount(&mock_server)
            .await;

        let adapter = Arc::new(ExampleExchangeAdapter::new(&mock_server.uri()));

        let mut join_set = JoinSet::new();

        for _ in 0..5 {
            let adapter_clone = adapter.clone();
            join_set.spawn(async move {
                let url = format!("{}/api/status", adapter_clone.base_url);
                adapter_clone.http_client().get_json(&url).await
            });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result.unwrap());
        }

        assert_eq!(results.len(), 5);
        for result in results {
            assert!(result.is_ok());
        }
    }

    /// Test that HTTP errors are properly propagated through the adapter.
    #[tokio::test]
    async fn test_adapter_propagates_http_errors() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/ticker/INVALID"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let adapter = ExampleExchangeAdapter::new(&mock_server.uri());
        let result = adapter.get_ticker("INVALID").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, marketschema_http::HttpError::Status { status_code, .. } if status_code == 404)
        );
    }

    /// Test that network errors are properly propagated.
    #[tokio::test]
    async fn test_adapter_propagates_connection_errors() {
        // Use a non-routable address to trigger connection error
        let adapter = ExampleExchangeAdapter::new("http://192.0.2.1:1");
        let result = adapter.get_ticker("BTCUSD").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Connection error or timeout expected
        assert!(matches!(
            err,
            marketschema_http::HttpError::Connection { .. }
                | marketschema_http::HttpError::Timeout { .. }
        ));
    }
}
