//! Tests for AsyncHttpClient and AsyncHttpClientBuilder
//!
//! Test structure follows TDD approach for US1: 非同期 HTTP リクエストの実行
//!
//! See: specs/003-http-client-rust/spec.md

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, HttpError};

// =============================================================================
// T010: AsyncHttpClientBuilder::new() and build()
// =============================================================================

mod builder_tests {
    use super::*;

    #[test]
    fn test_builder_new_creates_default_builder() {
        let builder = AsyncHttpClientBuilder::new();
        // Builder should be created without panic
        assert!(std::mem::size_of_val(&builder) > 0);
    }

    #[test]
    fn test_builder_default_trait() {
        let builder = AsyncHttpClientBuilder::default();
        // Default trait should work the same as new()
        assert!(std::mem::size_of_val(&builder) > 0);
    }

    #[test]
    fn test_builder_build_succeeds() {
        let result = AsyncHttpClientBuilder::new().build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_timeout_sets_custom_timeout() {
        let result = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_max_connections_sets_pool_size() {
        let result = AsyncHttpClientBuilder::new().max_connections(200).build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_default_headers_sets_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("test-agent"));

        let result = AsyncHttpClientBuilder::new()
            .default_headers(headers)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_chained_configuration() {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("test-agent"));

        let result = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_secs(45))
            .max_connections(150)
            .default_headers(headers)
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_client_builder_static_method() {
        // AsyncHttpClient::builder() should return AsyncHttpClientBuilder
        let builder = AsyncHttpClient::builder();
        let result = builder.build();
        assert!(result.is_ok());
    }
}

// =============================================================================
// T011: AsyncHttpClient::get_json()
// =============================================================================

mod get_json_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_json_returns_valid_json() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/ticker"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "symbol": "BTC/USD",
                "price": 50000.0
            })))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/ticker", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["symbol"], "BTC/USD");
        assert_eq!(value["price"], 50000.0);
    }

    #[tokio::test]
    async fn test_get_json_with_empty_object() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/empty"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/empty", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_object());
        assert!(value.as_object().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_json_with_array() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {"id": 1},
                {"id": 2}
            ])))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/list", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_array());
        assert_eq!(value.as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_json_parse_error_returns_parse_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/invalid"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/invalid", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HttpError::Parse { .. }));
    }

    #[tokio::test]
    async fn test_get_json_status_error_returns_status_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/notfound"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/notfound", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            HttpError::Status {
                status_code: 404,
                ..
            }
        ));
        assert_eq!(err.status_code(), Some(404));
    }
}

// =============================================================================
// T012: AsyncHttpClient::get_text()
// =============================================================================

mod get_text_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_text_returns_string() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/text"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Hello, World!"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/text", mock_server.uri());

        let result = client.get_text(&url).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_text_with_empty_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/empty"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/empty", mock_server.uri());

        let result = client.get_text(&url).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[tokio::test]
    async fn test_get_text_status_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/error"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/error", mock_server.uri());

        let result = client.get_text(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            HttpError::Status {
                status_code: 500,
                ..
            }
        ));
    }
}

// =============================================================================
// T013: AsyncHttpClient::get()
// =============================================================================

mod get_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_returns_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string("raw data"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/data", mock_server.uri());

        let result = client.get(&url).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn test_get_can_read_body_from_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .respond_with(ResponseTemplate::new(200).set_body_string("raw data"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/data", mock_server.uri());

        let response = client.get(&url).await.unwrap();
        let body = response.text().await.unwrap();
        assert_eq!(body, "raw data");
    }

    #[tokio::test]
    async fn test_get_error_status_returns_status_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/forbidden"))
            .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/forbidden", mock_server.uri());

        let result = client.get(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            HttpError::Status {
                status_code: 403,
                ..
            }
        ));
    }
}

// =============================================================================
// T014: Query parameters
// =============================================================================

mod query_params_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_json_with_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/search"))
            .and(query_param("symbol", "BTC"))
            .and(query_param("limit", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": []
            })))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/search", mock_server.uri());

        let result = client
            .get_json_with_params(&url, &[("symbol", "BTC"), ("limit", "10")])
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_text_with_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/text"))
            .and(query_param("format", "plain"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Plain text response"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/text", mock_server.uri());

        let result = client
            .get_text_with_params(&url, &[("format", "plain")])
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Plain text response");
    }

    #[tokio::test]
    async fn test_get_with_params() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/raw"))
            .and(query_param("verbose", "true"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/raw", mock_server.uri());

        let result = client.get_with_params(&url, &[("verbose", "true")]).await;

        assert!(result.is_ok());
    }
}

// =============================================================================
// T015: Custom headers
// =============================================================================

mod custom_headers_tests {
    use super::*;
    use wiremock::matchers::header;

    #[tokio::test]
    async fn test_default_headers_are_sent() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .and(header("User-Agent", "custom-agent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&mock_server)
            .await;

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("custom-agent"));

        let client = AsyncHttpClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = format!("{}/api/data", mock_server.uri());
        let result = client.get_json(&url).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_default_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .and(header("X-Custom-Header", "custom-value"))
            .and(header("Accept", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .mount(&mock_server)
            .await;

        let mut headers = HeaderMap::new();
        headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        let client = AsyncHttpClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = format!("{}/api/data", mock_server.uri());
        let result = client.get_json(&url).await;

        assert!(result.is_ok());
    }
}

// =============================================================================
// T016: Arc<AsyncHttpClient> thread-safety
// =============================================================================

mod thread_safety_tests {
    use super::*;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_client_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AsyncHttpClient>();
    }

    #[tokio::test]
    async fn test_arc_client_shared_across_tasks() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
            .expect(3) // Expect exactly 3 requests
            .mount(&mock_server)
            .await;

        let client = Arc::new(AsyncHttpClientBuilder::new().build().unwrap());
        let url = format!("{}/api/data", mock_server.uri());

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            let client_clone = Arc::clone(&client);
            let url_clone = url.clone();
            join_set.spawn(async move { client_clone.get_json(&url_clone).await });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result.unwrap());
        }

        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_concurrent_requests_with_arc_client() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/concurrent"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"status": "ok"}))
                    .set_delay(Duration::from_millis(10)),
            )
            .expect(5)
            .mount(&mock_server)
            .await;

        let client = Arc::new(AsyncHttpClientBuilder::new().build().unwrap());
        let url = format!("{}/api/concurrent", mock_server.uri());

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let client_clone = Arc::clone(&client);
                let url_clone = url.clone();
                tokio::spawn(async move { client_clone.get_json(&url_clone).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }
}

// =============================================================================
// Error handling tests
// =============================================================================

mod error_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_error_429() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/limited"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Rate limit exceeded")
                    .insert_header("Retry-After", "60"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/limited", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            HttpError::RateLimit {
                status_code: 429,
                ..
            }
        ));
        assert_eq!(err.status_code(), Some(429));
    }

    #[tokio::test]
    async fn test_error_url_accessor() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/error"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/error", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.url().is_some());
        assert!(err.url().unwrap().contains("/api/error"));
    }

    #[tokio::test]
    async fn test_is_retryable_for_server_errors() {
        let mock_server = MockServer::start().await;

        for status in [500, 502, 503, 504] {
            Mock::given(method("GET"))
                .and(path(format!("/api/{}", status)))
                .respond_with(ResponseTemplate::new(status))
                .mount(&mock_server)
                .await;

            let client = AsyncHttpClientBuilder::new().build().unwrap();
            let url = format!("{}/api/{}", mock_server.uri(), status);

            let result = client.get_json(&url).await;
            let err = result.unwrap_err();
            assert!(err.is_retryable(), "Status {} should be retryable", status);
        }
    }

    #[tokio::test]
    async fn test_is_not_retryable_for_client_errors() {
        let mock_server = MockServer::start().await;

        for status in [400, 401, 403, 404] {
            Mock::given(method("GET"))
                .and(path(format!("/api/{}", status)))
                .respond_with(ResponseTemplate::new(status))
                .mount(&mock_server)
                .await;

            let client = AsyncHttpClientBuilder::new().build().unwrap();
            let url = format!("{}/api/{}", mock_server.uri(), status);

            let result = client.get_json(&url).await;
            let err = result.unwrap_err();
            assert!(
                !err.is_retryable(),
                "Status {} should not be retryable",
                status
            );
        }
    }

    #[tokio::test]
    async fn test_timeout_returns_timeout_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/slow"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("slow response")
                    .set_delay(Duration::from_secs(5)),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        let url = format!("{}/api/slow", mock_server.uri());
        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HttpError::Timeout { .. }));
        assert!(err.is_retryable());
        assert!(err.url().is_some());
    }

    #[tokio::test]
    async fn test_connection_error_returns_connection_error() {
        let client = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        // Use a non-routable IP address to trigger connection error
        let url = "http://192.0.2.1:12345/api/data";
        let result = client.get_json(url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Could be Timeout or Connection depending on network config
        assert!(
            matches!(
                err,
                HttpError::Connection { .. } | HttpError::Timeout { .. }
            ),
            "Expected Connection or Timeout error, got {:?}",
            err
        );
        assert!(err.is_retryable());
    }

    #[tokio::test]
    async fn test_error_response_body_is_captured() {
        let mock_server = MockServer::start().await;

        let error_body = r#"{"error": "Not Found", "code": 404}"#;
        Mock::given(method("GET"))
            .and(path("/api/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_string(error_body))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/missing", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        if let HttpError::Status { response_body, .. } = err {
            assert!(response_body.is_some());
            assert_eq!(response_body.unwrap(), error_body);
        } else {
            panic!("Expected Status error");
        }
    }

    #[tokio::test]
    async fn test_rate_limit_response_body_is_captured() {
        let mock_server = MockServer::start().await;

        let error_body = r#"{"error": "Too Many Requests", "retry_after": 60}"#;
        Mock::given(method("GET"))
            .and(path("/api/rate-limited"))
            .respond_with(ResponseTemplate::new(429).set_body_string(error_body))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/rate-limited", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        if let HttpError::RateLimit { response_body, .. } = err {
            assert!(response_body.is_some());
            assert_eq!(response_body.unwrap(), error_body);
        } else {
            panic!("Expected RateLimit error");
        }
    }
}
