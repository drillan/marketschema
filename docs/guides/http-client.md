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
