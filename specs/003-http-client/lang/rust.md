# Rust Implementation Guide

**Feature**: 003-http-client
**Date**: 2026-02-03
**Status**: Planned

## Overview

本ドキュメントは 003-http-client の Rust 実装ガイドを提供する。
現時点では設計方針のみを記載し、実装は将来対応とする。

## Design Direction

### Module Structure (Proposed)

```
crates/marketschema-http/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public exports
│   ├── client.rs        # AsyncHttpClient implementation
│   ├── error.rs         # Error types
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── retry.rs     # RetryMiddleware
│   │   └── rate_limit.rs # RateLimitMiddleware
│   └── cache.rs         # ResponseCache implementation
```

## Library Selection

### HTTP Client: reqwest (Recommended)

[reqwest](https://docs.rs/reqwest) を推奨する理由:

- **async/await ネイティブサポート**: tokio との統合が優れている
- **広く使われている**: Rust エコシステムで最も普及した HTTP クライアント
- **TLS サポート**: native-tls と rustls の両方をサポート
- **自動リダイレクト**: デフォルトで有効
- **コネクションプーリング**: デフォルトで有効

### Alternative: ureq

[ureq](https://docs.rs/ureq) は同期 API のみが必要な場合の代替:

- **シンプルな API**: 最小限の依存関係
- **ブロッキング I/O**: 同期処理に適している
- **軽量**: 非同期ランタイムが不要

### Async Runtime: tokio

[tokio](https://tokio.rs/) を選択した理由:

- **デファクトスタンダード**: Rust 非同期エコシステムの標準
- **reqwest との統合**: reqwest は tokio を前提としている
- **マルチスレッド対応**: `rt-multi-thread` feature で高スループット

### Testing: wiremock

[wiremock](https://docs.rs/wiremock) を選択した理由:

- **HTTP モックサーバー**: ローカルでモックサーバーを起動
- **リクエストマッチング**: 柔軟なリクエストマッチング
- **非同期対応**: tokio との統合が容易

## Dependencies (Proposed)

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
tracing = "0.1"

[dev-dependencies]
wiremock = "0.6"
tokio-test = "0.4"
```

## Error Types (Proposed)

```rust
use thiserror::Error;

/// Base error type for HTTP operations.
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("HTTP timeout: {message}")]
    Timeout {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("HTTP connection error: {message}")]
    Connection {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("HTTP status error: {status_code} - {message}")]
    Status {
        message: String,
        status_code: u16,
        url: Option<String>,
        response_body: Option<String>,
    },

    #[error("HTTP rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        url: Option<String>,
        response_body: Option<String>,
        retry_after: Option<f64>,
    },
}
```

### Error Mapping

| reqwest Error | marketschema Error |
|---------------|-------------------|
| `reqwest::Error::is_timeout()` | `HttpError::Timeout` |
| `reqwest::Error::is_connect()` | `HttpError::Connection` |
| `Response::status().is_client_error()` | `HttpError::Status` |
| `Response::status().is_server_error()` | `HttpError::Status` |
| `Response::status() == 429` | `HttpError::RateLimit` |

## Client Interface (Proposed)

```rust
use std::time::Duration;
use reqwest::header::HeaderMap;

/// Configuration for AsyncHttpClient.
#[derive(Clone, Debug)]
pub struct HttpClientConfig {
    pub timeout: Duration,
    pub max_connections: usize,
    pub default_headers: HeaderMap,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            default_headers: HeaderMap::new(),
        }
    }
}

/// Async HTTP client with retry, rate limiting, and caching support.
pub struct AsyncHttpClient {
    inner: reqwest::Client,
    config: HttpClientConfig,
    // middleware fields...
}

impl AsyncHttpClient {
    /// Create a new client with default configuration.
    pub fn new() -> Result<Self, HttpError> {
        Self::with_config(HttpClientConfig::default())
    }

    /// Create a new client with custom configuration.
    pub fn with_config(config: HttpClientConfig) -> Result<Self, HttpError> {
        let inner = reqwest::Client::builder()
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.max_connections)
            .default_headers(config.default_headers.clone())
            .build()
            .map_err(|e| HttpError::Connection {
                message: e.to_string(),
                url: None,
                source: Some(e),
            })?;

        Ok(Self { inner, config })
    }

    /// Fetch URL and parse response as JSON.
    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, HttpError> {
        let response = self.get(url).await?;
        let json = response.json::<T>().await.map_err(|e| HttpError::Connection {
            message: format!("Failed to parse JSON: {}", e),
            url: Some(url.to_string()),
            source: Some(e),
        })?;
        Ok(json)
    }

    /// Fetch URL and return response as text.
    pub async fn get_text(&self, url: &str) -> Result<String, HttpError> {
        let response = self.get(url).await?;
        let text = response.text().await.map_err(|e| HttpError::Connection {
            message: format!("Failed to read response body: {}", e),
            url: Some(url.to_string()),
            source: Some(e),
        })?;
        Ok(text)
    }

    /// Fetch URL and return raw response.
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, HttpError> {
        // Implementation with error mapping...
        todo!()
    }
}
```

## Resource Management

Rust では `Drop` トレイトと RAII パターンでリソース管理を行う:

```rust
impl Drop for AsyncHttpClient {
    fn drop(&mut self) {
        // reqwest::Client は内部で Drop を実装しているため、
        // 明示的なクリーンアップは不要
    }
}
```

## Exception Chaining

Rust では `#[source]` アトリビュートと `thiserror` で例外チェインを実装:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("HTTP timeout: {message}")]
    Timeout {
        message: String,
        #[source]  // 元の例外への参照
        source: Option<reqwest::Error>,
    },
}
```

## Usage Example (Proposed)

```rust
use marketschema_http::{AsyncHttpClient, HttpClientConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = HttpClientConfig {
        timeout: Duration::from_secs(10),
        ..Default::default()
    };

    let client = AsyncHttpClient::with_config(config)?;

    let data: serde_json::Value = client
        .get_json("https://api.example.com/data")
        .await?;

    println!("{:?}", data);

    Ok(())
}
```

## Implementation Priorities

1. **Phase 1**: Core client and error types
2. **Phase 2**: Retry middleware with exponential backoff
3. **Phase 3**: Rate limiting middleware (token bucket)
4. **Phase 4**: Response caching (LRU cache)
5. **Phase 5**: Integration with marketschema-adapters crate

## Notes

- Rust 実装は Python 実装の成熟後に着手予定
- reqwest の feature flags を最小限に抑える
- エラーハンドリングは `thiserror` を使用
- ロギングは `tracing` を使用

## Reference

- [Error Taxonomy](../contracts/error-taxonomy.md) - エラー分類
- [reqwest Documentation](https://docs.rs/reqwest) - reqwest 公式ドキュメント
- [wiremock Documentation](https://docs.rs/wiremock) - wiremock 公式ドキュメント
- [tokio Documentation](https://tokio.rs/) - tokio 公式ドキュメント
