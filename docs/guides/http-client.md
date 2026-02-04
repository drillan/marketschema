# HTTP クライアント使用ガイド

marketschema の HTTP クライアント層を使用してデータソースからデータを取得する方法を解説する。

## 概要

marketschema は非同期 HTTP クライアント (`AsyncHttpClient`) を提供し、以下の機能をサポートする:

- コネクションプーリング
- 設定可能なタイムアウト
- 統一されたエラーハンドリング
- リトライ（指数バックオフ）
- レートリミット（トークンバケット）
- レスポンスキャッシュ（LRU）

このガイドの対象読者:

- アダプター開発者
- HTTP 通信のエラーハンドリングを実装する開発者
- 高頻度リクエストを行うアプリケーションの開発者

## クイックスタート

### 基本的な使い方

```python
import asyncio
from marketschema.http import AsyncHttpClient

async def main():
    async with AsyncHttpClient() as client:
        # JSON レスポンスを取得
        data = await client.get_json("https://api.example.com/ticker")
        print(data)

        # テキストレスポンスを取得（CSV など）
        text = await client.get_text("https://example.com/data.csv")
        print(text)

asyncio.run(main())
```

### パラメータ付きリクエスト

```python
async with AsyncHttpClient() as client:
    # クエリパラメータを指定
    data = await client.get_json(
        "https://api.example.com/ohlcv",
        params={"symbol": "btc_jpy", "interval": "1h"},
    )

    # カスタムヘッダーを指定
    data = await client.get_json(
        "https://api.example.com/data",
        headers={"Authorization": "Bearer token123"},
    )
```

## AsyncHttpClient

### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `timeout` | float | 30.0 | リクエストタイムアウト（秒） |
| `max_connections` | int | 100 | 最大同時接続数 |
| `headers` | dict[str, str] \| None | None | デフォルトヘッダー |
| `retry` | RetryMiddleware \| None | None | リトライ設定 |
| `rate_limit` | RateLimitMiddleware \| None | None | レートリミット設定 |
| `cache` | ResponseCache \| None | None | レスポンスキャッシュ |

### 主要メソッド

#### `get_json(url, *, headers=None, params=None, timeout=None)`

GET リクエストを送信し、JSON をパースして返す。

```python
data = await client.get_json("https://api.example.com/ticker")
```

#### `get_text(url, *, headers=None, params=None, timeout=None)`

GET リクエストを送信し、テキストとして返す。CSV や HTML の取得に使用。

```python
csv_content = await client.get_text("https://example.com/data.csv")
```

#### `get(url, *, headers=None, params=None, timeout=None)`

GET リクエストを送信し、生の `httpx.Response` を返す。

```python
response = await client.get("https://api.example.com/ticker")
print(response.status_code)
print(response.headers)
```

## ミドルウェア

### RetryMiddleware

失敗したリクエストを指数バックオフでリトライする。

```python
from marketschema.http import AsyncHttpClient
from marketschema.http.middleware import RetryMiddleware

# リトライ設定
retry = RetryMiddleware(
    max_retries=3,      # 最大リトライ回数
    backoff_factor=0.5, # バックオフ係数
    jitter=0.1,         # ジッター係数（0.0-1.0）
)

async with AsyncHttpClient(retry=retry) as client:
    data = await client.get_json("https://api.example.com/ticker")
```

#### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `max_retries` | int | 3 | 最大リトライ回数 |
| `backoff_factor` | float | 0.5 | バックオフ係数 |
| `retry_statuses` | set[int] \| None | {429, 500, 502, 503, 504} | リトライ対象ステータスコード |
| `jitter` | float | 0.1 | ジッター係数（0.0-1.0） |

#### バックオフ計算

遅延時間は以下の式で計算される:

```
delay = backoff_factor * (2 ** attempt) * (1 ± jitter)
```

| 試行 | backoff_factor=0.5 の場合 |
|------|-------------------------|
| 1回目 | 0.5秒 (±jitter) |
| 2回目 | 1.0秒 (±jitter) |
| 3回目 | 2.0秒 (±jitter) |

### RateLimitMiddleware

トークンバケットアルゴリズムによるレートリミット。

```python
from marketschema.http import AsyncHttpClient
from marketschema.http.middleware import RateLimitMiddleware

# 1秒あたり10リクエスト、バースト20まで許可
rate_limit = RateLimitMiddleware(
    requests_per_second=10.0,
    burst_size=20,
)

async with AsyncHttpClient(rate_limit=rate_limit) as client:
    # リクエストは自動的にレートリミットされる
    for i in range(100):
        data = await client.get_json(f"https://api.example.com/item/{i}")
```

#### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `requests_per_second` | float | - | 1秒あたりの最大リクエスト数（必須） |
| `burst_size` | int \| None | int(requests_per_second) | バーストサイズ |

## レスポンスキャッシュ

LRU（Least Recently Used）キャッシュによるレスポンスのキャッシュ。

```python
from datetime import timedelta
from marketschema.http import AsyncHttpClient
from marketschema.http.cache import ResponseCache

# 最大500エントリ、TTL 1分
cache = ResponseCache(
    max_size=500,
    default_ttl=timedelta(minutes=1),
)

async with AsyncHttpClient(cache=cache) as client:
    # 最初のリクエスト: API を呼び出し、キャッシュに保存
    data1 = await client.get_json("https://api.example.com/ticker")

    # 2回目のリクエスト: キャッシュから取得（API 呼び出しなし）
    data2 = await client.get_json("https://api.example.com/ticker")
```

#### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `max_size` | int | 1000 | 最大キャッシュエントリ数 |
| `default_ttl` | timedelta | 5分 | デフォルトの有効期限 |

#### 注意事項

- キャッシュキーは URL とクエリパラメータから自動生成される
- ヘッダーの違いは区別されない
- キャッシュは成功レスポンス（2xx）のみを保存

## 例外階層

HTTP 通信で発生する例外の階層:

```
MarketSchemaError
└── HttpError               # HTTP エラーの基底クラス
    ├── HttpTimeoutError    # タイムアウト
    ├── HttpConnectionError # 接続エラー
    └── HttpStatusError     # HTTP ステータスエラー (4xx, 5xx)
        └── HttpRateLimitError  # レートリミット (429)
```

### 例外の属性

#### HttpError

| 属性 | 型 | 説明 |
|------|-----|------|
| `message` | str | エラーメッセージ |
| `url` | str \| None | リクエスト URL |

#### HttpStatusError

`HttpError` の属性に加えて:

| 属性 | 型 | 説明 |
|------|-----|------|
| `status_code` | int | HTTP ステータスコード |
| `response_body` | str \| None | レスポンスボディ |

#### HttpRateLimitError

`HttpStatusError` の属性に加えて:

| 属性 | 型 | 説明 |
|------|-----|------|
| `retry_after` | float \| None | Retry-After ヘッダーの値（秒） |

## エラーハンドリングパターン

### 基本パターン

```python
import sys
from marketschema.http import AsyncHttpClient
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpStatusError,
    HttpTimeoutError,
)

async def fetch_data():
    try:
        async with AsyncHttpClient() as client:
            data = await client.get_json("https://api.example.com/ticker")
            return data
    except HttpStatusError as e:
        print(f"Error: HTTP {e.status_code} - {e.message}")
        sys.exit(1)
    except HttpTimeoutError:
        print("Error: Request timed out")
        sys.exit(1)
    except HttpConnectionError:
        print("Error: Connection failed")
        sys.exit(1)
```

### 詳細なエラーハンドリング

```python
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpRateLimitError,
    HttpStatusError,
    HttpTimeoutError,
)

async def fetch_with_details():
    try:
        async with AsyncHttpClient() as client:
            return await client.get_json("https://api.example.com/data")
    except HttpRateLimitError as e:
        # レートリミット: Retry-After を使用
        if e.retry_after:
            print(f"Rate limited. Retry after {e.retry_after} seconds")
        else:
            print("Rate limited. Please try again later")
        raise
    except HttpStatusError as e:
        # 特定のステータスコードを処理
        if e.status_code == 404:
            print("Resource not found")
        elif e.status_code >= 500:
            print("Server error. Please try again later")
        else:
            print(f"HTTP error: {e.status_code}")
        raise
    except HttpTimeoutError as e:
        print(f"Timeout: {e.url}")
        raise
    except HttpConnectionError as e:
        print(f"Connection failed: {e.url}")
        raise
```

## 組み合わせ使用例

リトライ、レートリミット、キャッシュをすべて組み合わせた例:

```python
from datetime import timedelta
from marketschema.http import AsyncHttpClient
from marketschema.http.cache import ResponseCache
from marketschema.http.middleware import RateLimitMiddleware, RetryMiddleware

# すべての機能を有効化
retry = RetryMiddleware(max_retries=3, backoff_factor=1.0)
rate_limit = RateLimitMiddleware(requests_per_second=5.0, burst_size=10)
cache = ResponseCache(max_size=100, default_ttl=timedelta(seconds=30))

async with AsyncHttpClient(
    timeout=60.0,
    retry=retry,
    rate_limit=rate_limit,
    cache=cache,
) as client:
    # 自動的にリトライ、レートリミット、キャッシュが適用される
    data = await client.get_json("https://api.example.com/ticker")
```

## BaseAdapter での使用

アダプター実装では、`BaseAdapter` が HTTP クライアントを自動管理する:

```python
from marketschema.adapters.base import BaseAdapter

class MyAdapter(BaseAdapter):
    source_name = "my_source"

    async def fetch_data(self, symbol: str) -> dict:
        # self.http_client は自動的に初期化される
        url = f"https://api.example.com/{symbol}"
        return await self.http_client.get_json(url)

# コンテキストマネージャーで自動クローズ
async with MyAdapter() as adapter:
    data = await adapter.fetch_data("btc_jpy")
```

カスタム HTTP クライアントを使用する場合:

```python
from marketschema.http import AsyncHttpClient
from marketschema.http.middleware import RetryMiddleware

# カスタムクライアントを作成
http_client = AsyncHttpClient(
    timeout=60.0,
    retry=RetryMiddleware(max_retries=5),
)

# アダプターに渡す
async with MyAdapter(http_client=http_client) as adapter:
    data = await adapter.fetch_data("btc_jpy")
```

## 参照

- [アダプター開発ガイド](adapter-development.md) - アダプター実装の詳細
- [モデル実装ガイド](models.md) - pydantic モデルの使い方

---

# Rust HTTP クライアント使用ガイド

marketschema の Rust HTTP クライアント層を使用してデータソースからデータを取得する方法を解説する。

## 概要

marketschema-http クレートは非同期 HTTP クライアント (`AsyncHttpClient`) を提供し、以下の機能をサポートする:

- reqwest ベースのコネクションプーリング
- 設定可能なタイムアウト
- Result 型による統一されたエラーハンドリング
- リトライ（指数バックオフ）
- レートリミット（トークンバケット）
- レスポンスキャッシュ（LRU）

このガイドの対象読者:

- Rust アダプター開発者
- HTTP 通信のエラーハンドリングを実装する開発者
- 高頻度リクエストを行うアプリケーションの開発者

## クイックスタート

### 基本的な使い方

```rust
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncHttpClientBuilder::new().build()?;

    // JSON レスポンスを取得
    let data = client.get_json("https://api.example.com/ticker").await?;
    println!("{:?}", data);

    // テキストレスポンスを取得（CSV など）
    let text = client.get_text("https://example.com/data.csv").await?;
    println!("{}", text);

    Ok(())
}
```

### パラメータ付きリクエスト

```rust
use marketschema_http::AsyncHttpClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncHttpClientBuilder::new().build()?;

    // クエリパラメータを指定
    let data = client
        .get_json_with_params(
            "https://api.example.com/ohlcv",
            &[("symbol", "btc_jpy"), ("interval", "1h")],
        )
        .await?;

    Ok(())
}
```

## AsyncHttpClient

### ビルダーパターン

```rust
use marketschema_http::AsyncHttpClientBuilder;
use std::time::Duration;

let client = AsyncHttpClientBuilder::new()
    .timeout(Duration::from_secs(60))
    .max_connections(200)
    .build()?;
```

### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `timeout` | Duration | 30秒 | リクエストタイムアウト |
| `max_connections` | usize | 100 | 最大同時接続数 |
| `headers` | HeaderMap | None | デフォルトヘッダー |
| `retry` | RetryConfig | None | リトライ設定 |
| `rate_limiter` | Arc<RateLimiter> | None | レートリミット設定 |
| `cache` | Arc<ResponseCache> | None | レスポンスキャッシュ |

### 主要メソッド

#### `get_json(url) -> Result<Value, HttpError>`

GET リクエストを送信し、JSON をパースして返す。

```rust
let data = client.get_json("https://api.example.com/ticker").await?;
```

#### `get_text(url) -> Result<String, HttpError>`

GET リクエストを送信し、テキストとして返す。CSV や HTML の取得に使用。

```rust
let csv_content = client.get_text("https://example.com/data.csv").await?;
```

#### `get(url) -> Result<Response, HttpError>`

GET リクエストを送信し、生の `reqwest::Response` を返す。

```rust
let response = client.get("https://api.example.com/ticker").await?;
println!("Status: {}", response.status());
```

## RetryConfig

失敗したリクエストを指数バックオフでリトライする。

```rust
use marketschema_http::{AsyncHttpClientBuilder, RetryConfig};

let retry = RetryConfig::new()
    .with_max_retries(3)
    .with_backoff_factor(0.5)
    .with_jitter(0.1);

let client = AsyncHttpClientBuilder::new()
    .retry(retry)
    .build()?;
```

### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `max_retries` | u32 | 3 | 最大リトライ回数 |
| `backoff_factor` | f64 | 0.5 | バックオフ係数 |
| `retry_statuses` | HashSet<u16> | {429, 500, 502, 503, 504} | リトライ対象ステータスコード |
| `jitter` | f64 | 0.1 | ジッター係数（0.0-1.0） |

### バックオフ計算

遅延時間は以下の式で計算される:

```
delay = backoff_factor * (2 ^ attempt) * (1 ± jitter)
```

## RateLimiter

トークンバケットアルゴリズムによるレートリミット。

```rust
use marketschema_http::{AsyncHttpClientBuilder, RateLimiter};
use std::sync::Arc;

// 1秒あたり10リクエスト、バースト20まで許可
let rate_limiter = Arc::new(RateLimiter::new(10.0, 20));

let client = AsyncHttpClientBuilder::new()
    .rate_limiter(rate_limiter)
    .build()?;
```

### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `requests_per_second` | f64 | - | 1秒あたりの最大リクエスト数（必須） |
| `burst_size` | usize | requests_per_second と同値 | バーストサイズ |

## ResponseCache

LRU（Least Recently Used）キャッシュによるレスポンスのキャッシュ。

```rust
use marketschema_http::{AsyncHttpClientBuilder, ResponseCache};
use std::sync::Arc;
use std::time::Duration;

// 最大500エントリ、TTL 1分
let cache = Arc::new(ResponseCache::new(500, Duration::from_secs(60)));

let client = AsyncHttpClientBuilder::new()
    .cache(cache)
    .build()?;
```

### 初期化パラメータ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|---------|------|
| `max_size` | u64 | 1000 | 最大キャッシュエントリ数 |
| `default_ttl` | Duration | 5分 | デフォルトの有効期限 |

## HttpError 列挙型

HTTP 通信で発生するエラーの列挙型:

```rust
pub enum HttpError {
    Timeout { message, url, source },
    Connection { message, url, source },
    Status { message, url, status_code, response_body, source },
    RateLimit { message, url, status_code, response_body, retry_after, source },
    Parse { message, url, source },
    Build { message, source },
}
```

### エラーの属性

#### 共通メソッド

| メソッド | 戻り値 | 説明 |
|---------|-------|------|
| `url()` | Option<&str> | リクエスト URL |
| `status_code()` | Option<u16> | HTTP ステータスコード（Status/RateLimit のみ） |
| `is_retryable()` | bool | リトライ可能かどうか |

## エラーハンドリングパターン

### 基本パターン

```rust
use marketschema_http::{AsyncHttpClientBuilder, HttpError};

async fn fetch_data() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncHttpClientBuilder::new().build()?;

    match client.get_json("https://api.example.com/ticker").await {
        Ok(data) => {
            println!("Data: {:?}", data);
            Ok(())
        }
        Err(HttpError::Status { status_code, message, .. }) => {
            eprintln!("HTTP {}: {}", status_code, message);
            std::process::exit(1);
        }
        Err(HttpError::Timeout { .. }) => {
            eprintln!("Request timed out");
            std::process::exit(1);
        }
        Err(HttpError::Connection { .. }) => {
            eprintln!("Connection failed");
            std::process::exit(1);
        }
        Err(e) => Err(e.into()),
    }
}
```

### 詳細なエラーハンドリング

```rust
use marketschema_http::{AsyncHttpClientBuilder, HttpError};

async fn fetch_with_details() -> Result<serde_json::Value, HttpError> {
    let client = AsyncHttpClientBuilder::new().build()?;

    match client.get_json("https://api.example.com/data").await {
        Ok(data) => Ok(data),
        Err(HttpError::RateLimit { retry_after, .. }) => {
            if let Some(delay) = retry_after {
                eprintln!("Rate limited. Retry after {:?}", delay);
            } else {
                eprintln!("Rate limited. Please try again later");
            }
            Err(HttpError::RateLimit {
                message: "Rate limited".to_string(),
                url: None,
                status_code: 429,
                response_body: None,
                retry_after,
                source: None,
            })
        }
        Err(e) if e.is_retryable() => {
            eprintln!("Retryable error: {}", e);
            Err(e)
        }
        Err(e) => {
            eprintln!("Non-retryable error: {}", e);
            Err(e)
        }
    }
}
```

## 組み合わせ使用例

リトライ、レートリミット、キャッシュをすべて組み合わせた例:

```rust
use marketschema_http::{
    AsyncHttpClientBuilder, ResponseCache, RateLimiter, RetryConfig,
};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // すべての機能を有効化
    let retry = RetryConfig::new()
        .max_retries(3)
        .backoff_factor(1.0);
    let rate_limiter = Arc::new(RateLimiter::new(5.0, 10));
    let cache = Arc::new(ResponseCache::new(100, Duration::from_secs(30)));

    let client = AsyncHttpClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .retry(retry)
        .rate_limiter(rate_limiter)
        .cache(cache)
        .build()?;

    // 自動的にリトライ、レートリミット、キャッシュが適用される
    let data = client.get_json("https://api.example.com/ticker").await?;
    println!("{:?}", data);

    Ok(())
}
```

## アダプターでの使用例

アダプター実装では、HTTP クライアントを内部で保持して使用する:

```rust
use marketschema_adapters::BaseAdapter;
use marketschema_http::AsyncHttpClient;
use std::sync::Arc;

struct MyAdapter {
    http_client: Arc<AsyncHttpClient>,
}

impl BaseAdapter for MyAdapter {
    fn source_name(&self) -> &'static str {
        "my_source"
    }

    // get_quote_mapping(), get_ohlcv_mapping() などを必要に応じて実装
}

impl MyAdapter {
    pub fn new(http_client: Arc<AsyncHttpClient>) -> Self {
        Self { http_client }
    }

    pub async fn fetch_data(&self, symbol: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://api.example.com/{}", symbol);
        let data = self.http_client.get_json(&url).await?;
        Ok(data)
    }
}
```

## 参照

- [アダプター開発ガイド（Rust）](adapter-development.md#rust-アダプター開発ガイド) - Rust アダプター実装の詳細
- [モデル実装ガイド（Rust）](models.md#rust-モデル実装ガイド) - Rust モデルの使い方
- [003-http-client-rust spec](../specs/003-http-client-rust/spec.md) - 詳細仕様
