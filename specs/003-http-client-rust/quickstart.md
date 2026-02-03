# Quickstart: marketschema-http (Rust)

**Feature**: 003-http-client-rust
**Date**: 2026-02-03

## Installation

`Cargo.toml` に依存関係を追加:

```toml
[dependencies]
marketschema-http = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Basic Usage

### Simple GET Request

```rust
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default settings
    let client = AsyncHttpClientBuilder::new().build()?;

    // GET JSON
    let data = client.get_json("https://api.example.com/ticker").await?;
    println!("Response: {}", data);

    // GET text
    let text = client.get_text("https://api.example.com/status").await?;
    println!("Status: {}", text);

    Ok(())
}
```

### Custom Configuration

```rust
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Custom headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("my-app/1.0"));

    // Build client with custom settings
    let client = AsyncHttpClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .max_connections(50)
        .default_headers(headers)
        .build()?;

    let data = client.get_json("https://api.example.com/data").await?;
    Ok(())
}
```

### Query Parameters

```rust
use marketschema_http::AsyncHttpClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncHttpClientBuilder::new().build()?;

    // With query parameters
    let params = [("symbol", "BTCUSD"), ("interval", "1h")];
    let data = client
        .get_json_with_params("https://api.example.com/candles", &params)
        .await?;

    Ok(())
}
```

## Error Handling

```rust
use marketschema_http::{AsyncHttpClientBuilder, HttpError};

#[tokio::main]
async fn main() {
    let client = AsyncHttpClientBuilder::new().build().unwrap();

    match client.get_json("https://api.example.com/data").await {
        Ok(data) => println!("Success: {}", data),
        Err(HttpError::Timeout { message, url, .. }) => {
            eprintln!("Timeout: {} (URL: {:?})", message, url);
        }
        Err(HttpError::Connection { message, url, .. }) => {
            eprintln!("Connection failed: {} (URL: {:?})", message, url);
        }
        Err(HttpError::Status { status_code, message, response_body, .. }) => {
            eprintln!("HTTP {}: {} (Body: {:?})", status_code, message, response_body);
        }
        Err(HttpError::RateLimit { retry_after, .. }) => {
            eprintln!("Rate limited. Retry after: {:?}", retry_after);
        }
        Err(HttpError::Parse { message, .. }) => {
            eprintln!("JSON parse error: {}", message);
        }
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

### Using `?` Operator

```rust
use marketschema_http::{AsyncHttpClientBuilder, HttpError};

async fn fetch_ticker(symbol: &str) -> Result<serde_json::Value, HttpError> {
    let client = AsyncHttpClientBuilder::new().build()?;
    let url = format!("https://api.example.com/ticker/{}", symbol);
    client.get_json(&url).await
}
```

### Checking Retryable Errors

```rust
use marketschema_http::HttpError;

fn handle_error(error: &HttpError) {
    if error.is_retryable() {
        println!("This error is retryable, will retry automatically");
    } else {
        println!("This error is not retryable: {}", error);
    }
}
```

## Automatic Retry (Phase 2)

```rust
use marketschema_http::{AsyncHttpClientBuilder, RetryConfig};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let retry_config = RetryConfig::new()
        .max_retries(5)
        .backoff_factor(1.0)
        .jitter(0.2);

    let client = AsyncHttpClientBuilder::new()
        .retry(retry_config)
        .build()?;

    // Automatically retries on 429, 500, 502, 503, 504
    let data = client.get_json("https://api.example.com/data").await?;
    Ok(())
}
```

## Rate Limiting (Phase 2)

```rust
use marketschema_http::{AsyncHttpClientBuilder, RateLimiter};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 10 requests per second, burst of 20
    let limiter = Arc::new(RateLimiter::new(10.0, 20));

    let client = AsyncHttpClientBuilder::new()
        .rate_limit(limiter.clone())
        .build()?;

    // Requests are automatically throttled
    for i in 0..100 {
        let data = client.get_json("https://api.example.com/data").await?;
        println!("Request {} completed", i);
    }

    Ok(())
}
```

### Manual Token Acquisition

```rust
use marketschema_http::RateLimiter;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let limiter = Arc::new(RateLimiter::new(5.0, 10));

    // Async acquisition (waits for token)
    limiter.acquire().await;
    println!("Token acquired");

    // Try without waiting
    if limiter.try_acquire() {
        println!("Token acquired immediately");
    } else {
        println!("No tokens available");
    }
}
```

## Response Caching (Phase 3)

```rust
use marketschema_http::{AsyncHttpClientBuilder, ResponseCache};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 500 entries max, 60 second TTL
    let cache = Arc::new(ResponseCache::new(500, Duration::from_secs(60)));

    let client = AsyncHttpClientBuilder::new()
        .cache(cache.clone())
        .build()?;

    // First request: fetches from API
    let data1 = client.get_json("https://api.example.com/ticker").await?;

    // Second request: returns cached response
    let data2 = client.get_json("https://api.example.com/ticker").await?;

    // Manually clear cache if needed
    cache.clear();

    Ok(())
}
```

## Sharing Client Across Tasks

```rust
use marketschema_http::AsyncHttpClientBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(AsyncHttpClientBuilder::new().build()?);

    let mut handles = vec![];

    for i in 0..10 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let url = format!("https://api.example.com/data/{}", i);
            client.get_json(&url).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await??;
        println!("Result: {}", result);
    }

    Ok(())
}
```

## Full Example: Adapter Integration

```rust
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, HttpError, RetryConfig, RateLimiter};
use std::sync::{Arc, OnceLock};

pub struct MyExchangeAdapter {
    http_client: OnceLock<Arc<AsyncHttpClient>>,
    base_url: String,
}

impl MyExchangeAdapter {
    pub fn new(base_url: &str) -> Self {
        Self {
            http_client: OnceLock::new(),
            base_url: base_url.to_string(),
        }
    }

    fn http_client(&self) -> &Arc<AsyncHttpClient> {
        self.http_client.get_or_init(|| {
            let retry = RetryConfig::new().max_retries(3);
            let limiter = Arc::new(RateLimiter::new(10.0, 20));

            Arc::new(
                AsyncHttpClientBuilder::new()
                    .retry(retry)
                    .rate_limit(limiter)
                    .build()
                    .expect("Failed to build HTTP client"),
            )
        })
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("{}/ticker/{}", self.base_url, symbol);
        self.http_client().get_json(&url).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = MyExchangeAdapter::new("https://api.example.com");

    let ticker = adapter.get_ticker("BTCUSD").await?;
    println!("Ticker: {}", ticker);

    Ok(())
}
```

## Testing with wiremock

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_get_json() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock
        Mock::given(method("GET"))
            .and(path("/api/ticker"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"price": 50000})))
            .mount(&mock_server)
            .await;

        // Create client
        let client = AsyncHttpClientBuilder::new().build().unwrap();

        // Make request
        let url = format!("{}/api/ticker", mock_server.uri());
        let result = client.get_json(&url).await.unwrap();

        assert_eq!(result["price"], 50000);
    }

    #[tokio::test]
    async fn test_timeout_error() {
        let mock_server = MockServer::start().await;

        // Simulate slow response
        Mock::given(method("GET"))
            .and(path("/slow"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(5)))
            .mount(&mock_server)
            .await;

        let client = AsyncHttpClientBuilder::new()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();

        let url = format!("{}/slow", mock_server.uri());
        let result = client.get_json(&url).await;

        assert!(matches!(result, Err(HttpError::Timeout { .. })));
    }
}
```

## Next Steps

- [API Contract](./contracts/rust-api.md) - 詳細な API 仕様
- [Data Model](./data-model.md) - データモデル定義
- [Error Taxonomy](../003-http-client/contracts/error-taxonomy.md) - エラー分類
