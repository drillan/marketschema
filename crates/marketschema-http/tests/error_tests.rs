//! Tests for HTTP error handling (US2)
//!
//! Test structure follows TDD approach for US2: Result 型による HTTP エラーの処理
//!
//! See: specs/003-http-client-rust/spec.md
//!
//! Requirements tested:
//! - FR-R011: HttpError::Timeout
//! - FR-R012: HttpError::Connection
//! - FR-R013: HttpError::Status
//! - FR-R014: HttpError::RateLimit with retry_after
//! - FR-R015: thiserror::Error derive
//! - FR-R017: #[source] attribute for exception chaining
//! - FR-R018: HttpError::Parse

use std::error::Error;
use std::time::Duration;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use marketschema_http::{AsyncHttpClientBuilder, HttpError};

// =============================================================================
// T032: HttpError::Timeout when request times out
// FR-R011: タイムアウト時は HttpError::Timeout を返さなければならない
// =============================================================================

mod timeout_tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_returns_timeout_error_variant() {
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

        // Verify it's the Timeout variant with correct fields
        match &err {
            HttpError::Timeout {
                message,
                url: err_url,
                source,
            } => {
                assert!(!message.is_empty(), "message should not be empty");
                assert!(err_url.is_some(), "url should be present");
                assert!(
                    err_url.as_ref().unwrap().contains("/api/slow"),
                    "url should contain the path"
                );
                assert!(source.is_some(), "source should be present");
            }
            _ => panic!("Expected HttpError::Timeout, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_timeout_error_is_retryable() {
        let err = HttpError::Timeout {
            message: "Request timed out".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert!(err.is_retryable(), "Timeout errors should be retryable");
    }

    #[tokio::test]
    async fn test_timeout_url_accessor() {
        let err = HttpError::Timeout {
            message: "Timeout".to_string(),
            url: Some("https://example.com/api".to_string()),
            source: None,
        };
        assert_eq!(err.url(), Some("https://example.com/api"));
    }
}

// =============================================================================
// T033: HttpError::Connection when connection fails
// FR-R012: 接続失敗時は HttpError::Connection を返さなければならない
// =============================================================================

mod connection_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_error_for_unreachable_host() {
        let client = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        // Use a non-routable IP address (TEST-NET-1, RFC 5737)
        let url = "http://192.0.2.1:12345/api/data";
        let result = client.get_json(url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        // Could be Timeout or Connection depending on network configuration
        // Both are valid outcomes for unreachable hosts
        match &err {
            HttpError::Connection {
                message,
                url: err_url,
                source,
            } => {
                assert!(!message.is_empty());
                assert!(err_url.is_some());
                assert!(source.is_some(), "source should contain original error");
            }
            HttpError::Timeout {
                message,
                url: err_url,
                source,
            } => {
                assert!(!message.is_empty());
                assert!(err_url.is_some());
                assert!(source.is_some(), "source should contain original error");
            }
            _ => panic!(
                "Expected HttpError::Connection or HttpError::Timeout, got {:?}",
                err
            ),
        }

        // Both Connection and Timeout errors should be retryable
        assert!(err.is_retryable());
    }

    #[tokio::test]
    async fn test_connection_error_is_retryable() {
        let err = HttpError::Connection {
            message: "Connection refused".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert!(err.is_retryable(), "Connection errors should be retryable");
    }

    #[tokio::test]
    async fn test_connection_url_accessor() {
        let err = HttpError::Connection {
            message: "Failed".to_string(),
            url: Some("https://example.com/api".to_string()),
            source: None,
        };
        assert_eq!(err.url(), Some("https://example.com/api"));
    }
}

// =============================================================================
// T034: HttpError::Status with 404 response
// FR-R013: HTTP 4xx/5xx ステータス時は HttpError::Status を返さなければならない
// =============================================================================

mod status_tests {
    use super::*;

    #[tokio::test]
    async fn test_404_returns_status_error() {
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

        match &err {
            HttpError::Status {
                message,
                url: err_url,
                status_code,
                response_body,
                source: _,
            } => {
                assert!(!message.is_empty());
                assert!(err_url.is_some());
                assert_eq!(*status_code, 404);
                assert!(response_body.is_some());
                assert_eq!(response_body.as_ref().unwrap(), "Not Found");
            }
            _ => panic!("Expected HttpError::Status, got {:?}", err),
        }

        assert_eq!(err.status_code(), Some(404));
    }

    #[tokio::test]
    async fn test_500_returns_status_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/error"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/error", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::Status {
                status_code,
                response_body,
                ..
            } => {
                assert_eq!(*status_code, 500);
                assert!(response_body.is_some());
            }
            _ => panic!("Expected HttpError::Status, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_status_error_404_is_not_retryable() {
        let err = HttpError::Status {
            message: "Not Found".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert!(!err.is_retryable(), "404 errors should not be retryable");
    }

    #[tokio::test]
    async fn test_status_error_500_is_retryable() {
        let err = HttpError::Status {
            message: "Internal Server Error".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 500,
            response_body: None,
            source: None,
        };
        assert!(err.is_retryable(), "500 errors should be retryable");
    }
}

// =============================================================================
// T035: HttpError::RateLimit with 429 response
// FR-R014: HTTP 429 ステータス時は HttpError::RateLimit を返さなければならない
// =============================================================================

mod rate_limit_tests {
    use super::*;

    #[tokio::test]
    async fn test_429_returns_rate_limit_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/limited"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "60"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/limited", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit {
                message,
                url: err_url,
                status_code,
                response_body,
                retry_after,
                source: _,
            } => {
                assert!(!message.is_empty());
                assert!(err_url.is_some());
                assert_eq!(*status_code, 429);
                assert!(response_body.is_some());
                // T047: Retry-After header should be parsed
                assert!(
                    retry_after.is_some(),
                    "retry_after should be parsed from Retry-After header"
                );
                assert_eq!(
                    *retry_after,
                    Some(Duration::from_secs(60)),
                    "retry_after should be 60 seconds"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }

        assert_eq!(err.status_code(), Some(429));
    }

    #[tokio::test]
    async fn test_429_without_retry_after_header() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/limited-no-header"))
            .respond_with(ResponseTemplate::new(429).set_body_string("Too Many Requests"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/limited-no-header", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert!(
                    retry_after.is_none(),
                    "retry_after should be None when header is missing"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_http_date_retry_after() {
        let mock_server = MockServer::start().await;

        // HTTP-date format (less common but valid)
        Mock::given(method("GET"))
            .and(path("/api/limited-date"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "Wed, 21 Oct 2026 07:28:00 GMT"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/limited-date", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        // HTTP-date format is complex to parse; we return None for retry_after
        // RFC 7231 allows HTTP-date, but we only support delta-seconds format
        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert!(
                    retry_after.is_none(),
                    "HTTP-date format should result in None retry_after"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_error_is_retryable() {
        let err = HttpError::RateLimit {
            message: "Rate limited".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 429,
            response_body: None,
            retry_after: Some(Duration::from_secs(60)),
            source: None,
        };
        assert!(err.is_retryable(), "Rate limit errors should be retryable");
    }

    // =========================================================================
    // T048-T053: Retry-After header edge cases
    // =========================================================================

    #[tokio::test]
    async fn test_429_with_empty_retry_after_header() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/empty-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", ""),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/empty-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert!(
                    retry_after.is_none(),
                    "Empty Retry-After header should result in None"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_zero_retry_after() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/zero-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "0"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/zero-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert_eq!(
                    *retry_after,
                    Some(Duration::from_secs(0)),
                    "Zero Retry-After should be valid"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_whitespace_padded_retry_after() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/whitespace-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "  60  "),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/whitespace-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert_eq!(
                    *retry_after,
                    Some(Duration::from_secs(60)),
                    "Whitespace-padded Retry-After should be trimmed and parsed"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_negative_retry_after() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/negative-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "-60"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/negative-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert!(
                    retry_after.is_none(),
                    "Negative Retry-After should result in None"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_overflow_retry_after() {
        let mock_server = MockServer::start().await;

        // u64::MAX + 1 = 18446744073709551616
        Mock::given(method("GET"))
            .and(path("/api/overflow-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "18446744073709551616"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/overflow-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                assert!(
                    retry_after.is_none(),
                    "Overflow Retry-After should result in None"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_429_with_decimal_retry_after() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/decimal-retry"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_string("Too Many Requests")
                    .insert_header("Retry-After", "60.5"),
            )
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/decimal-retry", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();

        match &err {
            HttpError::RateLimit { retry_after, .. } => {
                // RFC 7231 specifies delta-seconds as non-negative integer
                // Decimal values are invalid
                assert!(
                    retry_after.is_none(),
                    "Decimal Retry-After should result in None (RFC 7231 requires integer)"
                );
            }
            _ => panic!("Expected HttpError::RateLimit, got {:?}", err),
        }
    }
}

// =============================================================================
// T036: HttpError::Parse with invalid JSON
// FR-R018: JSON パースエラー時は HttpError::Parse を返さなければならない
// =============================================================================

mod parse_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_json_returns_parse_error() {
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

        match &err {
            HttpError::Parse {
                message,
                url: err_url,
                source,
            } => {
                assert!(!message.is_empty());
                assert!(err_url.is_some());
                assert!(source.is_some(), "source should contain serde_json error");
            }
            _ => panic!("Expected HttpError::Parse, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_truncated_json_returns_parse_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/truncated"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{\"key\":"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/truncated", mock_server.uri());

        let result = client.get_json(&url).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, HttpError::Parse { .. }));
    }

    #[tokio::test]
    async fn test_parse_error_is_not_retryable() {
        let err = HttpError::Parse {
            message: "Invalid JSON".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert!(!err.is_retryable(), "Parse errors should not be retryable");
    }
}

// =============================================================================
// T037: std::error::Error::source() returning original error
// FR-R017: エラーは #[source] 属性により元の reqwest エラーにアクセス可能でなければならない
// =============================================================================

mod source_chain_tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_error_source_chain() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/slow"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("slow")
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

        let err = result.unwrap_err();

        // Access source via std::error::Error trait
        let source = err.source();
        assert!(
            source.is_some(),
            "source() should return the original reqwest error"
        );

        // The source should be a reqwest::Error
        let source_err = source.unwrap();
        assert!(
            source_err.to_string().len() > 0,
            "source error should have a message"
        );
    }

    #[tokio::test]
    async fn test_parse_error_source_chain() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/invalid"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new().build().unwrap();
        let url = format!("{}/api/invalid", mock_server.uri());

        let result = client.get_json(&url).await;
        let err = result.unwrap_err();

        // Access source via std::error::Error trait
        let source = err.source();
        assert!(
            source.is_some(),
            "source() should return the original serde_json error"
        );
    }

    #[tokio::test]
    async fn test_connection_error_source_chain() {
        let client = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        // Use non-routable IP
        let url = "http://192.0.2.1:12345/api";
        let result = client.get_json(url).await;
        let err = result.unwrap_err();

        // Connection or Timeout errors should have source
        let source = err.source();
        assert!(
            source.is_some(),
            "source() should return the original error"
        );
    }

    #[test]
    fn test_build_error_source_chain() {
        // Build error without source
        let err = HttpError::Build {
            message: "Failed to build".to_string(),
            source: None,
        };

        // source() should return None when there's no underlying error
        assert!(
            err.source().is_none(),
            "Build error without source should return None"
        );
    }
}

// =============================================================================
// T038: HttpError::is_retryable() method
// Tests for the is_retryable() method across all error types
// =============================================================================

mod is_retryable_tests {
    use super::*;

    #[test]
    fn test_timeout_is_retryable() {
        let err = HttpError::Timeout {
            message: "Timeout".to_string(),
            url: None,
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_connection_is_retryable() {
        let err = HttpError::Connection {
            message: "Connection failed".to_string(),
            url: None,
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_rate_limit_is_retryable() {
        let err = HttpError::RateLimit {
            message: "Rate limited".to_string(),
            url: None,
            status_code: 429,
            response_body: None,
            retry_after: None,
            source: None,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_server_errors_are_retryable() {
        for status_code in [500, 502, 503, 504] {
            let err = HttpError::Status {
                message: format!("Server error {}", status_code),
                url: None,
                status_code,
                response_body: None,
                source: None,
            };
            assert!(
                err.is_retryable(),
                "Status {} should be retryable",
                status_code
            );
        }
    }

    #[test]
    fn test_client_errors_are_not_retryable() {
        for status_code in [400, 401, 403, 404, 405, 409, 410, 422] {
            let err = HttpError::Status {
                message: format!("Client error {}", status_code),
                url: None,
                status_code,
                response_body: None,
                source: None,
            };
            assert!(
                !err.is_retryable(),
                "Status {} should not be retryable",
                status_code
            );
        }
    }

    #[test]
    fn test_parse_error_is_not_retryable() {
        let err = HttpError::Parse {
            message: "Parse failed".to_string(),
            url: None,
            source: None,
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_build_error_is_not_retryable() {
        let err = HttpError::Build {
            message: "Build failed".to_string(),
            source: None,
        };
        assert!(!err.is_retryable());
    }
}

// =============================================================================
// Error Display Tests (FR-R015: thiserror::Error derive)
// =============================================================================

mod display_tests {
    use super::*;

    #[test]
    fn test_timeout_display() {
        let err = HttpError::Timeout {
            message: "Request timed out".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert_eq!(err.to_string(), "HTTP timeout: Request timed out");
    }

    #[test]
    fn test_connection_display() {
        let err = HttpError::Connection {
            message: "Connection refused".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert_eq!(err.to_string(), "HTTP connection error: Connection refused");
    }

    #[test]
    fn test_status_display() {
        let err = HttpError::Status {
            message: "Not Found".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 404,
            response_body: None,
            source: None,
        };
        assert_eq!(err.to_string(), "HTTP status error 404: Not Found");
    }

    #[test]
    fn test_rate_limit_display() {
        let err = HttpError::RateLimit {
            message: "Too many requests".to_string(),
            url: Some("https://example.com".to_string()),
            status_code: 429,
            response_body: None,
            retry_after: Some(Duration::from_secs(60)),
            source: None,
        };
        assert_eq!(
            err.to_string(),
            "HTTP rate limit exceeded: Too many requests"
        );
    }

    #[test]
    fn test_parse_display() {
        let err = HttpError::Parse {
            message: "Invalid JSON".to_string(),
            url: Some("https://example.com".to_string()),
            source: None,
        };
        assert_eq!(err.to_string(), "JSON parse error: Invalid JSON");
    }

    #[test]
    fn test_build_display() {
        let err = HttpError::Build {
            message: "Invalid configuration".to_string(),
            source: None,
        };
        assert_eq!(
            err.to_string(),
            "HTTP client build error: Invalid configuration"
        );
    }
}
