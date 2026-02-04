# HTTP Client Layer 実装計画

**ステータス**: 計画済み（未実装）
**作成日**: 2026-02-03
**優先度**: 中

## 概要

examples で共通利用可能な HTTP クライアントレイヤーを `python/src/marketschema/http/` に追加し、
プロジェクトの正式機能として提供する。

## 背景

### 現状の課題

現在、各 example で HTTP 取得コードが個別に実装されている：

| Example | HTTP 取得方式 | コード位置 |
|---------|--------------|-----------|
| bitbank | `urllib.request.urlopen()` + JSON パース | `examples/bitbank/demo.py:31-48` |
| stooq | `urllib.request.urlopen()` + UTF-8 デコード | `examples/stooq/demo.py:31-54` |
| stockanalysis | `urllib.request.Request()` + User-Agent ヘッダー | `examples/stockanalysis/demo.py:31-64` |

### 決定事項

1. **httpx を使用** - モダンな非同期 HTTP クライアント
2. **非同期必須** - async/await ベース
3. **フル機能** - リトライ、レート制限、タイムアウト、キャッシュ対応
4. **Constitution 改訂** - 「通信処理」をコアに含めることを許可

## アーキテクチャ

### 選択したアプローチ: 実用的バランス（段階的実装）

YAGNI 原則に従い、Phase 1 でコア機能を実装し、Phase 2 以降で拡張機能を追加する。

### ディレクトリ構造

```
python/src/marketschema/
├── http/                          # NEW MODULE
│   ├── __init__.py               # Public API exports
│   ├── client.py                 # AsyncHttpClient (Phase 1)
│   ├── exceptions.py             # HTTP-specific exceptions (Phase 1)
│   ├── middleware.py             # Retry/RateLimit middleware (Phase 2)
│   └── cache.py                  # Response caching (Phase 3)
├── adapters/
│   ├── base.py                   # MODIFY: Add http_client property
│   └── ...
└── exceptions.py                 # Reference http.exceptions
```

### コンポーネント設計

#### Phase 1: AsyncHttpClient（コア機能）

**ファイル**: `python/src/marketschema/http/client.py`

```python
from typing import Any
import httpx

# Constants
DEFAULT_TIMEOUT_SECONDS = 30
DEFAULT_MAX_CONNECTIONS = 100

class AsyncHttpClient:
    """Async HTTP client for adapter implementations.

    Features:
    - Connection pooling
    - Configurable timeouts
    - Clean error handling

    Example:
        async with AsyncHttpClient() as client:
            data = await client.get_json("https://api.example.com/ticker")
    """

    def __init__(
        self,
        timeout: float = DEFAULT_TIMEOUT_SECONDS,
        max_connections: int = DEFAULT_MAX_CONNECTIONS,
        headers: dict[str, str] | None = None,
    ) -> None: ...

    async def get(self, url: str, **kwargs: Any) -> httpx.Response: ...
    async def get_json(self, url: str, **kwargs: Any) -> dict[str, Any]: ...
    async def get_text(self, url: str, **kwargs: Any) -> str: ...

    async def __aenter__(self) -> "AsyncHttpClient": ...
    async def __aexit__(self, ...) -> None: ...
    async def close(self) -> None: ...
```

#### Phase 1: HTTP Exceptions

**ファイル**: `python/src/marketschema/http/exceptions.py`

```python
from marketschema.exceptions import MarketSchemaError

class HttpError(MarketSchemaError):
    """Base for all HTTP errors."""

class HttpTimeoutError(HttpError):
    """Request timed out."""

class HttpConnectionError(HttpError):
    """Connection failed."""

class HttpStatusError(HttpError):
    """HTTP status indicates error (4xx, 5xx)."""
    status_code: int

class HttpRateLimitError(HttpStatusError):
    """Rate limit exceeded (429)."""
```

#### Phase 2: Middleware（リトライ・レート制限）

**ファイル**: `python/src/marketschema/http/middleware.py`

```python
class RetryMiddleware:
    """Retry failed requests with exponential backoff."""

    def __init__(
        self,
        max_retries: int = 3,
        backoff_factor: float = 0.5,
        retry_statuses: set[int] = {429, 500, 502, 503, 504},
    ) -> None: ...

class RateLimitMiddleware:
    """Rate limiting using token bucket algorithm."""

    def __init__(
        self,
        requests_per_second: float,
        burst_size: int | None = None,
    ) -> None: ...
```

#### Phase 3: Cache

**ファイル**: `python/src/marketschema/http/cache.py`

```python
class ResponseCache:
    """LRU cache for HTTP responses."""

    def __init__(
        self,
        max_size: int = 1000,
        default_ttl: timedelta = timedelta(minutes=5),
    ) -> None: ...
```

## 実装タスク

### Phase 1: Core HTTP Client

| # | タスク | 依存 | 優先度 |
|---|--------|------|--------|
| 1.1 | ~~Constitution 改訂~~ ✅ 完了 (v0.5.0) | - | P0 |
| 1.2 | pyproject.toml に httpx 依存追加 | - | P0 |
| 1.3 | `http/exceptions.py` 作成 | 1.2 | P0 |
| 1.4 | `http/exceptions.py` テスト作成 | 1.3 | P0 |
| 1.5 | `http/client.py` 作成 | 1.3 | P0 |
| 1.6 | `http/client.py` テスト作成 | 1.5 | P0 |
| 1.7 | `http/__init__.py` エクスポート設定 | 1.5 | P1 |
| 1.8 | `adapters/base.py` に http_client 追加 | 1.5 | P1 |
| 1.9 | 統合テスト作成 | 1.8 | P1 |
| 1.10 | 品質チェック（lint, mypy, pytest） | 1.9 | P1 |

### Phase 2: Middleware

| # | タスク | 依存 | 優先度 |
|---|--------|------|--------|
| 2.1 | `http/middleware.py` - RetryMiddleware | Phase 1 | P2 |
| 2.2 | RetryMiddleware テスト | 2.1 | P2 |
| 2.3 | `http/middleware.py` - RateLimitMiddleware | Phase 1 | P2 |
| 2.4 | RateLimitMiddleware テスト | 2.3 | P2 |
| 2.5 | AsyncHttpClient にミドルウェア統合 | 2.1, 2.3 | P2 |

### Phase 3: Cache

| # | タスク | 依存 | 優先度 |
|---|--------|------|--------|
| 3.1 | `http/cache.py` 作成 | Phase 1 | P3 |
| 3.2 | Cache テスト | 3.1 | P3 |
| 3.3 | AsyncHttpClient にキャッシュ統合 | 3.1 | P3 |

### Phase 4: Examples 更新

| # | タスク | 依存 | 優先度 |
|---|--------|------|--------|
| 4.1 | `examples/bitbank/` 非同期対応 | Phase 1 | P2 |
| 4.2 | `examples/stooq/` 非同期対応 | Phase 1 | P2 |
| 4.3 | `examples/stockanalysis/` 非同期対応 | Phase 1 | P2 |

## Constitution 改訂

**ステータス**: 改訂完了（2026-02-03）

Constitution v0.5.0 で以下の改訂が完了しました：

- **原則 II「軽量コア」**: 共通 HTTP クライアントをコアに含めることを明記
- **In Scope**: 共通 HTTP クライアント（AsyncHttpClient）を追加
- **Out of Scope**: 「データソース接続」を「個別データソースの API 仕様」に変更

詳細は `.specify/memory/constitution.md` を参照してください。

## 依存関係の追加

### pyproject.toml 変更

```toml
[project]
dependencies = [
    "pydantic>=2.0.0",
    "httpx>=0.27.0",  # NEW
]

[dependency-groups]
dev = [
    "pytest>=8.0.0",
    "pytest-asyncio>=0.24.0",  # NEW
    "respx>=0.21.0",           # NEW - httpx mocking
    "ruff>=0.6.0",
    "mypy>=1.11.0",
    # ...existing...
]
```

## テスト戦略

### 単体テスト

```python
# python/tests/unit/http/test_client.py
import pytest
import respx
import httpx

@pytest.mark.asyncio
@respx.mock
async def test_get_json_success():
    client = AsyncHttpClient()

    respx.get("https://api.example.com/data").mock(
        return_value=httpx.Response(200, json={"key": "value"})
    )

    async with client:
        result = await client.get_json("https://api.example.com/data")

    assert result == {"key": "value"}

@pytest.mark.asyncio
@respx.mock
async def test_get_json_timeout():
    client = AsyncHttpClient(timeout=0.1)

    respx.get("https://api.example.com/data").mock(side_effect=httpx.TimeoutException)

    async with client:
        with pytest.raises(HttpTimeoutError):
            await client.get_json("https://api.example.com/data")
```

### 統合テスト

```python
# python/tests/integration/test_http_adapter.py
@pytest.mark.asyncio
async def test_bitbank_adapter_with_http_client():
    adapter = BitbankAdapter()

    async with adapter:
        quote = await adapter.fetch_quote("btc_jpy")

    assert quote.symbol == "btc_jpy"
    assert quote.bid > 0
    assert quote.ask > 0
```

## 成功基準

| 基準 | 測定方法 |
|------|----------|
| 全テスト合格 | `uv run pytest` |
| 型チェック合格 | `uv run mypy src` |
| Lint 合格 | `uv run ruff check .` |
| examples 動作 | `uv run python -m examples.bitbank.async_demo` |
| テストカバレッジ | http モジュールで 90% 以上 |

## リスクと緩和策

| リスク | 影響 | 緩和策 |
|--------|------|--------|
| httpx の破壊的変更 | 中 | バージョン固定（`>=0.27.0,<1.0.0`） |
| 既存アダプターへの影響 | 低 | http_client はオプション、後方互換を維持 |
| 非同期導入の複雑さ | 中 | 同期ラッパーを提供（Phase 2） |
| Constitution 改訂の合意 | 低 | 改訂理由を明確に文書化 |

## 参考資料

- [httpx 公式ドキュメント](https://www.python-httpx.org/)
- [respx - httpx mocking](https://lundberg.github.io/respx/)
- [pytest-asyncio](https://pytest-asyncio.readthedocs.io/)
- marketschema Constitution: `.specify/memory/constitution.md`
