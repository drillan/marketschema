# HTTP Client Layer アーキテクチャ設計

**ステータス**: 設計完了
**作成日**: 2026-02-03

## 評価したアプローチ

### A: 最小変更アプローチ

**特徴**:
- `HTTPAdapterMixin` による多重継承
- 既存 `BaseAdapter` を変更しない
- 全機能を一括実装

**構造**:
```
src/marketschema/
├── http/
│   ├── __init__.py
│   ├── client.py          # AsyncHTTPClient
│   ├── retry.py           # RetryPolicy
│   ├── rate_limit.py      # RateLimiter
│   └── cache.py           # ResponseCache
├── adapters/
│   ├── base.py            # 変更なし
│   └── http.py            # NEW: HTTPAdapterMixin
└── exceptions.py          # 拡張
```

**利点**:
- 既存コードへの影響がゼロ
- 後方互換性 100%

**欠点**:
- 多重継承はPythonで避けるべきパターンとも言われる
- 全機能一括実装は大きなPR

### B: クリーンアーキテクチャアプローチ

**特徴**:
- Policy パターンによる戦略の差し替え
- `BaseHttpAdapter` 基底クラス
- Protocol によるインターフェース定義

**構造**:
```
src/marketschema/
├── http/
│   ├── __init__.py
│   ├── client.py
│   ├── config.py          # 設定 dataclass
│   └── policies/
│       ├── __init__.py
│       ├── retry.py
│       ├── ratelimit.py
│       └── cache.py
├── adapters/
│   ├── base.py
│   └── http.py            # BaseHttpAdapter
└── exceptions.py
```

**利点**:
- 各ポリシーが独立してテスト可能
- 拡張性が高い

**欠点**:
- ファイル数が多い（17個）
- 学習コストが高い

### C: 実用的バランスアプローチ（採用）

**特徴**:
- 段階的実装（Phase 1-3）
- `BaseAdapter` にオプションプロパティ追加
- YAGNI に従い必要な時に拡張

**構造（Phase 1）**:
```
src/marketschema/
├── http/
│   ├── __init__.py
│   ├── client.py          # AsyncHttpClient
│   └── exceptions.py      # HTTP例外
├── adapters/
│   └── base.py            # http_client プロパティ追加
└── exceptions.py          # http.exceptions を参照
```

**利点**:
- 最小限のコードで開始
- 段階的に機能追加可能
- 学習コストが低い

**欠点**:
- 将来の拡張時に設計変更が必要な可能性
- Phase 1 だけでは機能が限定的

## 採用理由

**C: 実用的バランスアプローチ**を採用。

理由：
1. **Constitution 原則 III に適合**: 「80%のユースケースに最適化」「YAGNI」
2. **リスク低減**: 小さな変更から始めて問題があれば軌道修正可能
3. **チーム学習**: 新しい抽象概念を最小限に抑え、既存知識を活用

## データフロー

### 現在のフロー（urllib）

```
demo.py
    ↓
urllib.request.urlopen(url)
    ↓
JSON/CSV/HTML レスポンス
    ↓
adapter.parse_*(data)
    ↓
marketschema Model
```

### 新しいフロー（httpx）

```
adapter.fetch_*(symbol)
    ↓
self.http_client.get_json(url)
    ↓
[Phase 2: Middleware]
├─ RateLimitMiddleware.acquire()
├─ CacheMiddleware.get()
└─ RetryMiddleware.execute()
    ↓
httpx.AsyncClient.get()
    ↓
JSON Response
    ↓
adapter.parse_*(data)
    ↓
marketschema Model
```

## エラー処理設計

### 例外階層

```
MarketSchemaError
├── AdapterError
├── TransformError
├── MappingError
├── ValidationError
└── HttpError                    # NEW
    ├── HttpTimeoutError         # NEW
    ├── HttpConnectionError      # NEW
    └── HttpStatusError          # NEW
        └── HttpRateLimitError   # NEW
```

### エラー変換マッピング

| httpx 例外 | marketschema 例外 |
|-----------|------------------|
| `httpx.TimeoutException` | `HttpTimeoutError` |
| `httpx.ConnectError` | `HttpConnectionError` |
| `httpx.HTTPStatusError` (4xx) | `HttpStatusError` |
| `httpx.HTTPStatusError` (429) | `HttpRateLimitError` |
| `httpx.HTTPStatusError` (5xx) | `HttpStatusError` |

### エラー処理コード例

```python
# src/marketschema/http/client.py

async def get_json(self, url: str, **kwargs: Any) -> dict[str, Any]:
    try:
        response = await self._client.get(url, **kwargs)
        response.raise_for_status()
        return response.json()
    except httpx.TimeoutException as e:
        raise HttpTimeoutError(f"Request to {url} timed out") from e
    except httpx.ConnectError as e:
        raise HttpConnectionError(f"Failed to connect to {url}") from e
    except httpx.HTTPStatusError as e:
        status = e.response.status_code
        if status == 429:
            raise HttpRateLimitError(
                f"Rate limit exceeded for {url}",
                status_code=status,
            ) from e
        raise HttpStatusError(
            f"HTTP {status} from {url}",
            status_code=status,
        ) from e
```

## BaseAdapter 統合設計

### 変更前

```python
# src/marketschema/adapters/base.py

class BaseAdapter:
    source_name: str = ""
    transforms: type[Transforms] = Transforms

    def __init__(self) -> None:
        if not self.source_name:
            raise AdapterError(...)
```

### 変更後

```python
# src/marketschema/adapters/base.py

from marketschema.http import AsyncHttpClient

class BaseAdapter:
    source_name: str = ""
    transforms: type[Transforms] = Transforms
    _http_client: AsyncHttpClient | None = None

    def __init__(
        self,
        http_client: AsyncHttpClient | None = None,
    ) -> None:
        if not self.source_name:
            raise AdapterError(...)
        self._http_client = http_client

    @property
    def http_client(self) -> AsyncHttpClient:
        """Get or create HTTP client (lazy initialization)."""
        if self._http_client is None:
            self._http_client = AsyncHttpClient()
        return self._http_client

    async def close(self) -> None:
        """Close HTTP client if owned by adapter."""
        if self._http_client is not None:
            await self._http_client.close()

    async def __aenter__(self) -> Self:
        return self

    async def __aexit__(self, *args: Any) -> None:
        await self.close()
```

## 使用例

### Phase 1 実装後のアダプター例

```python
# examples/bitbank/adapter.py

from marketschema.adapters.base import BaseAdapter
from marketschema.models import Quote

class BitbankAdapter(BaseAdapter):
    source_name = "bitbank"

    async def fetch_quote(self, symbol: str) -> Quote:
        """Fetch and parse quote from bitbank API."""
        url = f"https://public.bitbank.cc/{symbol}/ticker"
        data = await self.http_client.get_json(url)

        if data.get("success") != 1:
            raise AdapterError(f"API error: {data}")

        return self.parse_quote(data["data"], symbol=symbol)
```

### デモスクリプト

```python
# examples/bitbank/async_demo.py

import asyncio
from examples.bitbank.adapter import BitbankAdapter

async def main():
    async with BitbankAdapter() as adapter:
        quote = await adapter.fetch_quote("btc_jpy")
        print(f"BTC/JPY: bid={quote.bid}, ask={quote.ask}")

if __name__ == "__main__":
    asyncio.run(main())
```

## 設定オプション

### AsyncHttpClient コンストラクタ

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|----------|------|
| `timeout` | `float` | `30.0` | リクエストタイムアウト（秒） |
| `max_connections` | `int` | `100` | 最大コネクション数 |
| `headers` | `dict[str, str] \| None` | `None` | デフォルトヘッダー |

### Phase 2 追加設定

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|----------|------|
| `max_retries` | `int` | `3` | 最大リトライ回数 |
| `backoff_factor` | `float` | `0.5` | バックオフ係数 |
| `rate_limit` | `float \| None` | `None` | レート制限（req/sec） |

### Phase 3 追加設定

| パラメータ | 型 | デフォルト | 説明 |
|-----------|-----|----------|------|
| `enable_cache` | `bool` | `False` | キャッシュ有効化 |
| `cache_ttl` | `int` | `300` | キャッシュ TTL（秒） |
| `cache_max_size` | `int` | `1000` | 最大キャッシュエントリ数 |

## テスト設計

### 単体テストの構造

```
tests/
├── unit/
│   ├── http/
│   │   ├── __init__.py
│   │   ├── test_client.py         # AsyncHttpClient テスト
│   │   ├── test_exceptions.py     # HTTP 例外テスト
│   │   ├── test_middleware.py     # Phase 2
│   │   └── test_cache.py          # Phase 3
│   └── adapters/
│       └── test_base.py           # http_client プロパティテスト
└── integration/
    └── test_http_adapter.py       # E2E テスト
```

### モッキング戦略

```python
import respx
import httpx

@pytest.fixture
def mock_api():
    with respx.mock(assert_all_called=False) as mock:
        mock.get("https://public.bitbank.cc/btc_jpy/ticker").respond(
            json={
                "success": 1,
                "data": {
                    "buy": "5000000",
                    "sell": "5000100",
                    "timestamp": 1700000000000,
                }
            }
        )
        yield mock
```

## パフォーマンス考慮事項

### コネクションプーリング

httpx の `AsyncClient` はデフォルトでコネクションプーリングを行う：
- 最大 100 コネクション
- キープアライブ対応

### 非同期処理

- すべての HTTP 操作は非同期
- 複数リクエストの並列実行が可能

```python
async def fetch_multiple(symbols: list[str]) -> list[Quote]:
    async with BitbankAdapter() as adapter:
        tasks = [adapter.fetch_quote(s) for s in symbols]
        return await asyncio.gather(*tasks)
```

### メモリ管理

- Phase 3 のキャッシュは LRU で最大サイズを制限
- レスポンスはストリーミング可能（必要に応じて）

## セキュリティ考慮事項

1. **TLS 検証**: httpx はデフォルトで TLS 証明書を検証
2. **タイムアウト強制**: 無限待ちを防止
3. **認証情報**: HTTP クライアントは認証を扱わない（アダプター責務）
4. **レスポンスサイズ**: httpx のデフォルト制限に依存

## 今後の拡張ポイント

1. **同期ラッパー**: `asyncio.run()` を使った同期 API
2. **プロキシ対応**: 企業環境向け
3. **カスタムトランスポート**: テスト用モック差し込み
4. **メトリクス収集**: リクエスト数、レイテンシの計測
