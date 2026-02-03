# Data Model: HTTP Client Rust Implementation

**Feature**: 003-http-client-rust
**Date**: 2026-02-03
**Spec**: [spec.md](./spec.md)

## Overview

本ドキュメントは Rust HTTP クライアント実装のデータモデルを定義する。
Rust の型シグネチャと構造体定義を言語固有の形式で記述。

## Entity Definitions

### Core Entities

#### AsyncHttpClient

非同期 HTTP クライアント。`reqwest::Client` をラップし、型安全な API を提供。

```rust
pub struct AsyncHttpClient {
    /// Wrapped reqwest client with connection pooling
    inner: reqwest::Client,
    /// Optional retry configuration (Phase 2)
    retry_config: Option<RetryConfig>,
    /// Optional rate limiter (Phase 2)
    rate_limiter: Option<Arc<RateLimiter>>,
    /// Optional response cache (Phase 3)
    cache: Option<Arc<ResponseCache>>,
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inner` | `reqwest::Client` | Yes | reqwest クライアント（コネクションプーリング付き） |
| `retry_config` | `Option<RetryConfig>` | No | リトライ設定 |
| `rate_limiter` | `Option<Arc<RateLimiter>>` | No | レート制限（共有可能） |
| `cache` | `Option<Arc<ResponseCache>>` | No | レスポンスキャッシュ（共有可能） |

**Traits**: `Send + Sync`（`Arc<AsyncHttpClient>` での共有を可能にする）

**Invariants**:
- `inner` は常に有効な `reqwest::Client` インスタンス
- タイムアウトが設定されている（デフォルト: 30秒）

---

#### AsyncHttpClientBuilder

ビルダーパターンでクライアントを構築。

```rust
pub struct AsyncHttpClientBuilder {
    timeout: Duration,
    max_connections: usize,
    default_headers: HeaderMap,
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
    cache: Option<Arc<ResponseCache>>,
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `timeout` | `Duration` | 30s | リクエストタイムアウト |
| `max_connections` | `usize` | 100 | 最大コネクション数 |
| `default_headers` | `HeaderMap` | empty | デフォルトヘッダー |
| `retry_config` | `Option<RetryConfig>` | None | リトライ設定 |
| `rate_limiter` | `Option<Arc<RateLimiter>>` | None | レート制限 |
| `cache` | `Option<Arc<ResponseCache>>` | None | レスポンスキャッシュ |

**Invariants**:
- `timeout > 0`
- `max_connections > 0`

---

### Error Types

#### HttpError

HTTP 関連エラーの列挙型。`thiserror::Error` を derive。

```rust
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

    #[error("HTTP status error {status_code}: {message}")]
    Status {
        message: String,
        url: Option<String>,
        status_code: u16,
        response_body: Option<String>,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("HTTP rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        url: Option<String>,
        status_code: u16,
        response_body: Option<String>,
        retry_after: Option<Duration>,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("JSON parse error: {message}")]
    Parse {
        message: String,
        url: Option<String>,
        #[source]
        source: Option<serde_json::Error>,
    },

    #[error("HTTP client build error: {message}")]
    Build {
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },
}
```

**Error Mapping** (from reqwest):

| reqwest Condition | HttpError Variant |
|------------------|-------------------|
| `error.is_timeout()` | `Timeout` |
| `error.is_connect()` | `Connection` |
| Status 429 | `RateLimit` |
| Status 4xx/5xx | `Status` |
| JSON parse failure | `Parse` |
| Client build failure | `Build` |

**Invariants**:
- `status_code` は有効な HTTP ステータスコード (100-599)
- `retry_after` は 429 レスポンスの `Retry-After` ヘッダーから取得

---

### Configuration Types

#### RetryConfig

リトライ設定。指数バックオフとジッタをサポート。

```rust
#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub backoff_factor: f64,
    pub retry_statuses: HashSet<u16>,
    pub jitter: f64,
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_retries` | `u32` | 3 | 最大リトライ回数 |
| `backoff_factor` | `f64` | 0.5 | 指数バックオフ係数 |
| `retry_statuses` | `HashSet<u16>` | {429, 500, 502, 503, 504} | リトライ対象ステータス |
| `jitter` | `f64` | 0.1 | ランダムジッタ係数。範囲: `0.0 <= jitter <= 1.0`。0.0 = ジッタなし、1.0 = 最大100%のランダム増加 |

**Delay Calculation**:
```
delay = backoff_factor * (2 ^ attempt) * (1 + random(0, jitter))
```

**Invariants**:
- `max_retries >= 0`
- `backoff_factor > 0`
- `0 <= jitter <= 1`

---

#### RateLimiter

トークンバケットアルゴリズムによるレート制限。

```rust
pub struct RateLimiter {
    requests_per_second: f64,
    burst_size: usize,
    tokens: Mutex<f64>,
    last_update: Mutex<Instant>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| `requests_per_second` | `f64` | 1秒あたりの許可リクエスト数 |
| `burst_size` | `usize` | バーストサイズ（最大トークン数） |
| `tokens` | `Mutex<f64>` | 現在のトークン数 |
| `last_update` | `Mutex<Instant>` | 最終更新時刻 |

**Token Replenishment**:
```
elapsed = now - last_update
tokens = min(burst_size, tokens + elapsed * requests_per_second)
```

**Traits**: `Send + Sync`

**Invariants**:
- `requests_per_second > 0`
- `burst_size > 0`
- `tokens <= burst_size`

---

#### ResponseCache

LRU キャッシュによるレスポンスキャッシュ。`moka` クレートを使用。

```rust
pub struct ResponseCache {
    inner: moka::future::Cache<String, CacheEntry>,
    default_ttl: Duration,
}

struct CacheEntry {
    value: String,
    // TTL is managed by moka
}
```

| Field | Type | Description |
|-------|------|-------------|
| `inner` | `moka::future::Cache<String, CacheEntry>` | moka キャッシュインスタンス |
| `default_ttl` | `Duration` | デフォルト TTL |

**Configuration** (via moka builder):
- `max_capacity`: 最大エントリ数（デフォルト: 1000）
- `time_to_live`: エントリの有効期限（デフォルト: 5分）

**Traits**: `Send + Sync`

**Invariants**:
- `default_ttl > 0`
- キャッシュキーは URL 全体（クエリパラメータ含む）

---

## Entity Relationships

```
┌─────────────────────┐
│ AsyncHttpClient     │
│─────────────────────│
│ inner: Client       │
│ retry_config?       │──────┐
│ rate_limiter?       │──────│───┐
│ cache?              │──────│───│───┐
└─────────────────────┘      │   │   │
                             │   │   │
                   ┌─────────┘   │   │
                   │             │   │
                   ▼             │   │
        ┌──────────────────┐    │   │
        │ RetryConfig      │    │   │
        │──────────────────│    │   │
        │ max_retries      │    │   │
        │ backoff_factor   │    │   │
        │ retry_statuses   │    │   │
        │ jitter           │    │   │
        └──────────────────┘    │   │
                                │   │
                   ┌────────────┘   │
                   │                │
                   ▼                │
        ┌──────────────────┐       │
        │ RateLimiter      │       │
        │ (Arc<T>)         │       │
        │──────────────────│       │
        │ requests_per_sec │       │
        │ burst_size       │       │
        │ tokens           │       │
        └──────────────────┘       │
                                   │
                   ┌───────────────┘
                   │
                   ▼
        ┌──────────────────┐
        │ ResponseCache    │
        │ (Arc<T>)         │
        │──────────────────│
        │ inner (moka)     │
        │ default_ttl      │
        └──────────────────┘

┌─────────────────────┐
│ HttpError           │
│ (enum)              │
├─────────────────────┤
│ ◆ Timeout           │
│ ◆ Connection        │
│ ◆ Status            │
│ ◆ RateLimit         │
│ ◆ Parse             │
│ ◆ Build             │
└─────────────────────┘
```

## Type Aliases & Constants

```rust
/// Default timeout for HTTP requests
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default maximum connections in pool
pub const DEFAULT_MAX_CONNECTIONS: usize = 100;

/// Default maximum retry attempts
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default backoff factor for retries
pub const DEFAULT_BACKOFF_FACTOR: f64 = 0.5;

/// Default jitter factor for retries
pub const DEFAULT_JITTER: f64 = 0.1;

/// Default cache TTL in seconds
pub const DEFAULT_CACHE_TTL_SECS: u64 = 300;

/// Default cache max size
pub const DEFAULT_CACHE_MAX_SIZE: u64 = 1000;

/// Retryable status codes
pub const RETRYABLE_STATUSES: [u16; 5] = [429, 500, 502, 503, 504];

/// Non-retryable status codes
pub const NON_RETRYABLE_STATUSES: [u16; 8] = [400, 401, 403, 404, 405, 409, 410, 422];
```

## Validation Rules

### AsyncHttpClientBuilder

| Rule | Validation | Error |
|------|------------|-------|
| Timeout must be positive | `timeout > Duration::ZERO` | `HttpError::Build` |
| Max connections must be positive | `max_connections > 0` | `HttpError::Build` |

### RetryConfig

| Rule | Validation | Error |
|------|------------|-------|
| Backoff factor must be positive | `backoff_factor > 0.0` | panic (builder pattern) |
| Jitter must be in range | `0.0 <= jitter <= 1.0` | panic (builder pattern) |

### RateLimiter

| Rule | Validation | Error |
|------|------------|-------|
| RPS must be positive | `requests_per_second > 0.0` | panic (constructor) |
| Burst size must be positive | `burst_size > 0` | panic (constructor) |

## State Transitions

### RateLimiter Token State

```
[tokens = burst_size]
        │
        ▼
    ┌───────┐
    │ Ready │ ◄─────────────────────┐
    └───┬───┘                       │
        │ acquire()                 │
        │ (tokens >= 1)             │
        ▼                           │
    ┌───────┐                       │
    │ Used  │                       │
    └───┬───┘                       │
        │ tokens -= 1               │
        │                           │
        ├─────────────────────────► │
        │ (tokens > 0)              │
        │                           │
        │ (tokens == 0)             │
        ▼                           │
    ┌─────────┐                     │
    │ Waiting │ ──── time passes ───┘
    └─────────┘     (tokens += elapsed * rps)
```

### ResponseCache Entry Lifecycle

```
[Cache Miss]
     │
     ▼
 ┌───────────────┐
 │ Fetch from    │
 │ HTTP          │
 └───────┬───────┘
         │ success
         ▼
 ┌───────────────┐
 │ Store in      │
 │ Cache         │
 └───────┬───────┘
         │
         ▼
 ┌───────────────┐        TTL expires
 │ Cached        │ ─────────────────────►  [Evicted]
 └───────┬───────┘
         │
         │ get() within TTL
         ▼
 ┌───────────────┐
 │ Cache Hit     │
 └───────────────┘
```
