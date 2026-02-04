# Data Model: HTTP Client Layer

**Feature**: 003-http-client
**Date**: 2026-02-03
**Status**: Complete

## Overview

HTTP クライアントレイヤーのエンティティ定義。スキーマ定義ではなく、Python クラス設計として定義する（インフラストラクチャレイヤーのため）。

## Entities

### AsyncHttpClient

非同期 HTTP クライアント。アダプター開発者が使用するメインクラス。

```python
class AsyncHttpClient:
    """Async HTTP client for adapter implementations."""

    # Constructor parameters
    timeout: float = 30.0
    max_connections: int = 100
    headers: dict[str, str] | None = None

    # Internal state
    _client: httpx.AsyncClient | None = None
```

**Behaviors**:
- `get(url, **kwargs) -> httpx.Response`: 生の HTTP レスポンスを取得
- `get_json(url, **kwargs) -> dict[str, Any]`: JSON レスポンスを dict として取得
- `get_text(url, **kwargs) -> str`: テキストレスポンスを str として取得
- `close() -> None`: コネクションをクローズ
- Context manager (`async with`) サポート

**Validation Rules**:
- `timeout > 0`: タイムアウトは正の値
- `max_connections > 0`: 最大接続数は正の値

### Exception Classes

#### HttpError

HTTP 関連エラーの基底クラス。

```python
class HttpError(MarketSchemaError):
    """Base for all HTTP errors."""

    message: str
    url: str | None = None
    __cause__: Exception | None = None
```

#### HttpTimeoutError

タイムアウトエラー。

```python
class HttpTimeoutError(HttpError):
    """Request timed out."""
```

#### HttpConnectionError

接続エラー。

```python
class HttpConnectionError(HttpError):
    """Connection failed."""
```

#### HttpStatusError

HTTP ステータスエラー。

```python
class HttpStatusError(HttpError):
    """HTTP status indicates error (4xx, 5xx)."""

    status_code: int
    response_body: str | None = None
```

#### HttpRateLimitError

レート制限エラー（429）。

```python
class HttpRateLimitError(HttpStatusError):
    """Rate limit exceeded (429)."""

    retry_after: float | None = None  # Retry-After header value
```

### RetryMiddleware

リトライロジック。

```python
class RetryMiddleware:
    """Retry failed requests with exponential backoff."""

    max_retries: int = 3
    backoff_factor: float = 0.5
    retry_statuses: set[int] = {429, 500, 502, 503, 504}
    jitter: float = 0.1  # ±10% randomization
```

**Behaviors**:
- `should_retry(status_code, attempt) -> bool`: リトライ判定
- `get_delay(attempt) -> float`: リトライ間隔の計算

**Validation Rules**:
- `max_retries >= 0`: リトライ回数は非負
- `backoff_factor > 0`: バックオフ係数は正の値
- `0 <= jitter <= 1`: ジッターは 0〜1 の範囲

### RateLimitMiddleware

レート制限ロジック（トークンバケット）。

```python
class RateLimitMiddleware:
    """Rate limiting using token bucket algorithm."""

    requests_per_second: float
    burst_size: int | None = None  # defaults to requests_per_second

    # Internal state
    _tokens: float
    _last_update: float
```

**Behaviors**:
- `acquire() -> None`: トークンを取得（必要に応じて待機）
- `try_acquire() -> bool`: 非ブロッキングでトークン取得を試行

**Validation Rules**:
- `requests_per_second > 0`: レートは正の値
- `burst_size > 0` (if specified): バーストサイズは正の値

### ResponseCache

レスポンスキャッシュ（LRU）。

```python
class ResponseCache:
    """LRU cache for HTTP responses."""

    max_size: int = 1000
    default_ttl: timedelta = timedelta(minutes=5)

    # Internal state
    _cache: OrderedDict[str, CacheEntry]
```

**CacheEntry**:
```python
@dataclass
class CacheEntry:
    """Single cache entry."""

    value: Any
    expires_at: float
```

**Behaviors**:
- `get(key) -> Any | None`: キャッシュから取得
- `set(key, value, ttl=None) -> None`: キャッシュに保存
- `delete(key) -> None`: キャッシュから削除
- `clear() -> None`: キャッシュをクリア

**Validation Rules**:
- `max_size > 0`: 最大サイズは正の値
- `default_ttl > 0`: TTL は正の値

## Entity Relationships

```
┌─────────────────────────────────────────────────────────────┐
│                       BaseAdapter                            │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                  AsyncHttpClient                     │    │
│  │  ┌─────────────────┐ ┌───────────────────────────┐  │    │
│  │  │ RetryMiddleware │ │ RateLimitMiddleware       │  │    │
│  │  └─────────────────┘ └───────────────────────────┘  │    │
│  │  ┌─────────────────┐ ┌───────────────────────────┐  │    │
│  │  │ ResponseCache   │ │ httpx.AsyncClient         │  │    │
│  │  └─────────────────┘ └───────────────────────────┘  │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘

Exception Hierarchy:
┌───────────────────────┐
│  MarketSchemaError    │
└──────────┬────────────┘
           │
┌──────────▼────────────┐
│      HttpError        │
└──────────┬────────────┘
           │
     ┌─────┼─────┬────────────────┐
     │     │     │                │
┌────▼───┐ │ ┌───▼────┐    ┌──────▼───────┐
│Timeout │ │ │Connect │    │ HttpStatus   │
│ Error  │ │ │ Error  │    │    Error     │
└────────┘ │ └────────┘    └──────┬───────┘
           │                      │
           │               ┌──────▼───────┐
           │               │ RateLimit    │
           │               │    Error     │
           │               └──────────────┘
```

## State Transitions

### AsyncHttpClient State

```
          ┌─────────┐
          │  Init   │
          └────┬────┘
               │ first request
               ▼
          ┌─────────┐
     ┌────│  Open   │────┐
     │    └────┬────┘    │
     │         │         │
     │ request │ close() │
     │         │         │
     │    ┌────▼────┐    │
     │    │ Request │    │
     │    │   ing   │    │
     │    └────┬────┘    │
     │         │         │
     │    response/error │
     │         │         │
     └─────────┘         │
                         ▼
                    ┌─────────┐
                    │ Closed  │
                    └─────────┘
```

### RateLimitMiddleware Token State

```
          ┌────────────────┐
          │ Full (tokens = │
          │  burst_size)   │
          └───────┬────────┘
                  │ acquire()
                  ▼
          ┌────────────────┐
          │ Available      │◄──────────────┐
          │ (tokens > 0)   │               │
          └───────┬────────┘               │
                  │ acquire() (tokens -= 1)│
                  ▼                        │
          ┌────────────────┐               │
          │ Depleted       │───────────────┘
          │ (tokens = 0)   │  time passes
          └───────┬────────┘  (refill)
                  │ acquire()
                  ▼
          ┌────────────────┐
          │ Waiting        │
          │ (blocking)     │
          └────────────────┘
```

## Constants

```python
# python/src/marketschema/http/client.py
DEFAULT_TIMEOUT_SECONDS: float = 30.0
DEFAULT_MAX_CONNECTIONS: int = 100

# python/src/marketschema/http/middleware.py
DEFAULT_MAX_RETRIES: int = 3
DEFAULT_BACKOFF_FACTOR: float = 0.5
DEFAULT_JITTER: float = 0.1
RETRYABLE_STATUS_CODES: frozenset[int] = frozenset({429, 500, 502, 503, 504})
NON_RETRYABLE_STATUS_CODES: frozenset[int] = frozenset({400, 401, 403, 404})
HTTP_STATUS_RATE_LIMIT: int = 429

# python/src/marketschema/http/cache.py
DEFAULT_CACHE_MAX_SIZE: int = 1000
DEFAULT_CACHE_TTL_SECONDS: int = 300  # 5 minutes
```

## Type Definitions

```python
# Type aliases for clarity
Headers = dict[str, str]
QueryParams = dict[str, str | int | float | bool]
JsonResponse = dict[str, Any]
```

## Public API Summary

```python
# python/src/marketschema/http/__init__.py
__all__ = [
    # Client
    "AsyncHttpClient",

    # Exceptions
    "HttpError",
    "HttpTimeoutError",
    "HttpConnectionError",
    "HttpStatusError",
    "HttpRateLimitError",

    # Middleware (Phase 2)
    "RetryMiddleware",
    "RateLimitMiddleware",

    # Cache (Phase 3)
    "ResponseCache",
]
```
