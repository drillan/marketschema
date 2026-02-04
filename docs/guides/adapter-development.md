# アダプター開発ガイド

外部データソースから marketschema モデルへのデータ変換アダプターを開発する方法を解説する。

## 概要

アダプターは外部データソース（取引所 API、データプロバイダなど）から取得したデータを marketschema の標準モデル（Quote, OHLCV, Trade など）に変換する。

このガイドの対象読者:

- 新しいデータソースのアダプターを開発する開発者
- 既存アダプターをメンテナンスする開発者
- データソース統合パターンを理解したい開発者

## 3層アーキテクチャ

アダプターは3層で構成される:

```
┌─────────────────────────────────────────────────┐
│                  Transport 層                    │
│  HTTP クライアント経由でデータを取得              │
│  AsyncHttpClient.get_json(), get_text()         │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│                   Extract 層                     │
│  レスポンスから必要なデータを抽出                 │
│  JSON パス解決、CSV パース、HTML パース          │
└─────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────┐
│                  Transform 層                    │
│  抽出したデータを marketschema モデルに変換      │
│  ModelMapping, Transforms                        │
└─────────────────────────────────────────────────┘
```

### 各層の責務

| 層 | 責務 | 主なクラス/メソッド |
|-----|------|-------------------|
| Transport | HTTP 通信、エラーハンドリング | `AsyncHttpClient`, `BaseAdapter.http_client` |
| Extract | レスポンス構造の解析とデータ抽出 | `parse_csv()`, `parse_html()`, JSON アクセス |
| Transform | フィールドマッピングと型変換 | `ModelMapping`, `Transforms`, `_apply_mapping()` |

## 命名規則

### 定数の命名

すべての定数は `{SOURCE}_` プレフィックスを付ける:

```python
# Good
BITBANK_BASE_URL = "https://public.bitbank.cc"
BITBANK_SUCCESS_CODE = 1
BITBANK_TIMEOUT_SECONDS = 30

STOOQ_BASE_URL = "https://stooq.com/q/d/l/"
STOOQ_INTERVAL_DAILY = "d"

STOCKANALYSIS_BASE_URL = "https://stockanalysis.com/stocks"
STOCKANALYSIS_USER_AGENT = "Mozilla/5.0 ..."
STOCKANALYSIS_MONTH_MAP = {"Jan": "01", ...}

# Bad - プレフィックスなし
API_BASE = "https://public.bitbank.cc"
SUCCESS_CODE = 1
USER_AGENT = "Mozilla/5.0 ..."
```

### URL 定数

ベース URL は必ず `{SOURCE}_BASE_URL` という名前にする:

```python
# Good
BITBANK_BASE_URL = "https://public.bitbank.cc"
STOOQ_BASE_URL = "https://stooq.com/q/d/l/"
STOCKANALYSIS_BASE_URL = "https://stockanalysis.com/stocks"

# Bad
BITBANK_API_BASE = "https://..."  # _API_BASE ではなく _BASE_URL
STOCKANALYSIS_URL = "https://..."  # _URL ではなく _BASE_URL
```

### インデックス定数

配列アクセス用のインデックスには意味のある名前を付ける:

```python
# CSV カラムインデックス（{SOURCE}_ プレフィックス例）
STOOQ_CSV_INDEX_DATE = 0
STOOQ_CSV_INDEX_OPEN = 1
STOOQ_CSV_INDEX_HIGH = 2
STOOQ_CSV_INDEX_LOW = 3
STOOQ_CSV_INDEX_CLOSE = 4
STOOQ_CSV_INDEX_VOLUME = 5

# OHLCV 配列インデックス（bitbank 形式）
BITBANK_OHLCV_INDEX_OPEN = 0
BITBANK_OHLCV_INDEX_HIGH = 1
BITBANK_OHLCV_INDEX_LOW = 2
BITBANK_OHLCV_INDEX_CLOSE = 3
BITBANK_OHLCV_INDEX_VOLUME = 4
BITBANK_OHLCV_INDEX_TIMESTAMP = 5
```

## BaseAdapter の継承

### 基本構造

```python
from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import Quote

# レジストリに登録
@register
class MySourceAdapter(BaseAdapter):
    """MySource API adapter."""

    # 必須: データソース識別子
    source_name = "my_source"

    def get_quote_mapping(self) -> list[ModelMapping]:
        """Quote モデルへのフィールドマッピングを定義."""
        return [
            ModelMapping("bid", "buy_price", transform=self.transforms.to_float),
            ModelMapping("ask", "sell_price", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "time", transform=self.transforms.unix_timestamp_ms
            ),
        ]

    async def fetch_ticker(self, pair: str) -> Quote:
        """Ticker データを取得して Quote モデルを返す."""
        url = f"{MYSOURCE_BASE_URL}/{pair}/ticker"
        data = await self.http_client.get_json(url)
        return self.parse_quote(data, symbol=pair)

    def parse_quote(self, raw_data: dict, *, symbol: str) -> Quote:
        """生データを Quote モデルに変換."""
        data_with_symbol = {**raw_data, "symbol": symbol}
        mappings = self.get_quote_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(data_with_symbol, mappings, Quote)
```

### 利用可能なマッピングメソッド

| メソッド | 対象モデル |
|---------|-----------|
| `get_quote_mapping()` | Quote |
| `get_ohlcv_mapping()` | OHLCV |
| `get_trade_mapping()` | Trade |
| `get_orderbook_mapping()` | OrderBook |
| `get_instrument_mapping()` | Instrument |

### 利用可能な変換関数

`self.transforms` で以下の変換関数を使用できる:

| 関数 | 説明 | 入力例 | 出力例 |
|-----|------|-------|-------|
| `to_float(value)` | 文字列を float に変換 | `"123.45"` | `123.45` |
| `to_int(value)` | 文字列を int に変換 | `"123"` | `123` |
| `unix_timestamp_ms(value)` | ミリ秒タイムスタンプを ISO 8601 に変換 | `1704067200000` | `"2024-01-01T00:00:00Z"` |
| `unix_timestamp_sec(value)` | 秒タイムスタンプを ISO 8601 に変換 | `1704067200` | `"2024-01-01T00:00:00Z"` |
| `iso_timestamp(value)` | ISO 8601 文字列を UTC に正規化 | `"2024-01-01T09:00:00+09:00"` | `"2024-01-01T00:00:00Z"` |
| `side_from_string(value)` | 文字列を `"buy"`/`"sell"` に正規化 | `"BUY"` | `"buy"` |

## データソース別パターン

### JSON API パターン

REST API から JSON を取得する一般的なパターン:

```python
from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import Quote

MYAPI_BASE_URL = "https://api.example.com"
MYAPI_SUCCESS_CODE = 0

@register
class MyApiAdapter(BaseAdapter):
    source_name = "myapi"

    def _validate_response(self, data: dict) -> None:
        """API レスポンスを検証."""
        if data.get("code") != MYAPI_SUCCESS_CODE:
            raise AdapterError(f"API error: {data}")

    async def fetch_ticker(self, symbol: str) -> Quote:
        url = f"{MYAPI_BASE_URL}/ticker"
        data = await self.http_client.get_json(url, params={"symbol": symbol})
        self._validate_response(data)
        return self.parse_quote(data["result"], symbol=symbol)

    def get_quote_mapping(self) -> list[ModelMapping]:
        return [
            ModelMapping("bid", "bid_price", transform=self.transforms.to_float),
            ModelMapping("ask", "ask_price", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.unix_timestamp_ms
            ),
        ]

    def parse_quote(self, raw_data: dict, *, symbol: str) -> Quote:
        data_with_symbol = {**raw_data, "symbol": symbol}
        mappings = self.get_quote_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(data_with_symbol, mappings, Quote)
```

### CSV パターン

CSV ファイルをパースするパターン:

```python
import csv
from io import StringIO
from typing import Any

from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV

MYCSV_BASE_URL = "https://data.example.com"
MYCSV_EXPECTED_COLUMNS = 6
MYCSV_INDEX_DATE = 0
MYCSV_INDEX_OPEN = 1
MYCSV_INDEX_HIGH = 2
MYCSV_INDEX_LOW = 3
MYCSV_INDEX_CLOSE = 4
MYCSV_INDEX_VOLUME = 5

@register
class MyCsvAdapter(BaseAdapter):
    source_name = "mycsv"

    async def fetch_csv(self, symbol: str) -> str:
        """CSV データを取得."""
        return await self.http_client.get_text(
            MYCSV_BASE_URL,
            params={"symbol": symbol},
        )

    def parse_csv_row(self, row: list[str], *, symbol: str) -> OHLCV:
        """CSV 行を OHLCV に変換."""
        if len(row) < MYCSV_EXPECTED_COLUMNS:
            raise AdapterError(
                f"Insufficient columns: expected {MYCSV_EXPECTED_COLUMNS}, "
                f"got {len(row)}"
            )

        ohlcv_dict: dict[str, Any] = {
            "symbol": symbol,
            "timestamp": f"{row[MYCSV_INDEX_DATE]}T00:00:00Z",
            "open": row[MYCSV_INDEX_OPEN],
            "high": row[MYCSV_INDEX_HIGH],
            "low": row[MYCSV_INDEX_LOW],
            "close": row[MYCSV_INDEX_CLOSE],
            "volume": row[MYCSV_INDEX_VOLUME],
        }

        mappings = self.get_ohlcv_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(ohlcv_dict, mappings, OHLCV)

    def parse_csv(self, csv_content: str, *, symbol: str) -> list[OHLCV]:
        """CSV 全体をパース."""
        reader = csv.reader(StringIO(csv_content))
        next(reader)  # ヘッダーをスキップ

        results: list[OHLCV] = []
        for row in reader:
            if row:
                results.append(self.parse_csv_row(row, symbol=symbol))
        return results

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        return [
            ModelMapping("open", "open", transform=self.transforms.to_float),
            ModelMapping("high", "high", transform=self.transforms.to_float),
            ModelMapping("low", "low", transform=self.transforms.to_float),
            ModelMapping("close", "close", transform=self.transforms.to_float),
            ModelMapping("volume", "volume", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.iso_timestamp
            ),
        ]
```

### HTML スクレイピングパターン

HTML テーブルをパースするパターン:

```python
from typing import Any

from bs4 import BeautifulSoup

from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.exceptions import AdapterError
from marketschema.models import OHLCV

MYHTML_BASE_URL = "https://www.example.com/stocks"
MYHTML_USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/120.0.0.0 Safari/537.36"
)
MYHTML_EXPECTED_COLUMNS = 6

@register
class MyHtmlAdapter(BaseAdapter):
    source_name = "myhtml"

    async def fetch_history(self, symbol: str) -> str:
        """HTML ページを取得."""
        url = f"{MYHTML_BASE_URL}/{symbol}/history/"
        headers = {"User-Agent": MYHTML_USER_AGENT}
        return await self.http_client.get_text(url, headers=headers)

    def parse_html(self, html_content: str, *, symbol: str) -> list[OHLCV]:
        """HTML テーブルをパース."""
        if not html_content.strip():
            raise AdapterError("Empty HTML content")

        soup = BeautifulSoup(html_content, "html.parser")
        table = soup.find("table")
        if table is None:
            raise AdapterError("No table found")

        tbody = table.find("tbody")
        if tbody is None:
            raise AdapterError("No tbody found")

        results: list[OHLCV] = []
        for row in tbody.find_all("tr"):
            cells = row.find_all("td")
            if cells:
                row_data = [cell.get_text(strip=True) for cell in cells]
                results.append(self.parse_html_row(row_data, symbol=symbol))

        return results

    def parse_html_row(self, row_data: list[str], *, symbol: str) -> OHLCV:
        """HTML テーブル行を OHLCV に変換."""
        if len(row_data) < MYHTML_EXPECTED_COLUMNS:
            raise AdapterError(f"Insufficient columns: {len(row_data)}")

        ohlcv_dict: dict[str, Any] = {
            "symbol": symbol,
            "timestamp": self._parse_date(row_data[0]),
            "open": row_data[1],
            "high": row_data[2],
            "low": row_data[3],
            "close": row_data[4],
            "volume": row_data[5].replace(",", ""),
        }

        mappings = self.get_ohlcv_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(ohlcv_dict, mappings, OHLCV)

    @staticmethod
    def _parse_date(date_str: str) -> str:
        """日付文字列を ISO 8601 に変換."""
        # データソース固有の日付パース実装
        return f"{date_str}T00:00:00Z"

    def get_ohlcv_mapping(self) -> list[ModelMapping]:
        return [
            ModelMapping("open", "open", transform=self.transforms.to_float),
            ModelMapping("high", "high", transform=self.transforms.to_float),
            ModelMapping("low", "low", transform=self.transforms.to_float),
            ModelMapping("close", "close", transform=self.transforms.to_float),
            ModelMapping("volume", "volume", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp", "timestamp", transform=self.transforms.iso_timestamp
            ),
        ]
```

## demo.py のパターン

各アダプターには動作確認用の `demo.py` を作成する。

### 標準構造

```python
#!/usr/bin/env python3
"""Demo script for {source} adapter.

Usage:
    uv run python -m examples.{source}.demo
    uv run python examples/{source}/demo.py [SYMBOL]
"""

from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# プロジェクトルートをパスに追加
if __name__ == "__main__":
    project_root = Path(__file__).resolve().parent.parent.parent
    if str(project_root) not in sys.path:
        sys.path.insert(0, str(project_root))

from examples.{source}.adapter import {Source}Adapter
from marketschema.exceptions import AdapterError
from marketschema.http.exceptions import (
    HttpConnectionError,
    HttpStatusError,
    HttpTimeoutError,
)

DEFAULT_SYMBOL = "btc_jpy"


async def demo_ticker(adapter: {Source}Adapter, symbol: str) -> None:
    """Demonstrate ticker parsing."""
    print(f"\n{'=' * 60}")
    print(f"Ticker ({symbol})")
    print("=" * 60)

    quote = await adapter.fetch_ticker(symbol)
    print(f"Bid: {quote.bid.root}")
    print(f"Ask: {quote.ask.root}")


async def main() -> None:
    """Run demo."""
    print("=" * 60)
    print("{Source} Adapter Demo")
    print("=" * 60)

    symbol = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SYMBOL

    try:
        async with {Source}Adapter() as adapter:
            await demo_ticker(adapter, symbol)
    except HttpStatusError as e:
        if e.status_code == 404:
            print(f"\nError: Symbol '{symbol}' not found")
        else:
            print(f"\nError: HTTP {e.status_code} - {e.message}")
        sys.exit(1)
    except HttpTimeoutError as e:
        print(f"\nError: Request timed out: {e}")
        sys.exit(1)
    except HttpConnectionError as e:
        print(f"\nError: Connection failed: {e}")
        sys.exit(1)
    except AdapterError as e:
        print(f"\nError: Failed to parse response: {e}")
        sys.exit(1)

    print(f"\n{'=' * 60}")
    print("Demo completed!")
    print("=" * 60)


if __name__ == "__main__":
    asyncio.run(main())
```

### エラーハンドリングのポイント

1. **try/except で主要な例外をキャッチ**: `HttpStatusError`, `HttpTimeoutError`, `HttpConnectionError`, `AdapterError`
2. **ユーザーフレンドリーなメッセージ**: 技術的詳細ではなく、ユーザーが理解できるメッセージを表示
3. **適切な終了コード**: エラー時は `sys.exit(1)` で非ゼロ終了
4. **例外オブジェクトのキャプチャ**: `as e` を使用してエラー詳細を取得

## ミドルウェアの使用例

アダプターでミドルウェアを使用する例:

```python
from marketschema.http import AsyncHttpClient
from marketschema.http.middleware import RateLimitMiddleware, RetryMiddleware

# リトライとレートリミットを設定
retry = RetryMiddleware(max_retries=3, backoff_factor=1.0)
rate_limit = RateLimitMiddleware(requests_per_second=5.0)

# カスタム HTTP クライアントでアダプターを初期化
http_client = AsyncHttpClient(
    timeout=60.0,
    retry=retry,
    rate_limit=rate_limit,
)

async with MyAdapter(http_client=http_client) as adapter:
    data = await adapter.fetch_ticker("btc_jpy")
```

## ファイル構成

アダプターのディレクトリ構成:

```
examples/{source}/
├── __init__.py      # パッケージ初期化
├── adapter.py       # アダプター実装
├── demo.py          # デモスクリプト
└── models.py        # 拡張モデル（必要な場合のみ）
```

### `__init__.py` の例

```python
"""Example adapter for {source}."""

from examples.{source}.adapter import {Source}Adapter

__all__ = ["{Source}Adapter"]
```

## チェックリスト

新しいアダプターを作成する際のチェックリスト:

### 命名規則

- [ ] 定数に `{SOURCE}_` プレフィックスを使用
- [ ] ベース URL は `{SOURCE}_BASE_URL` という名前
- [ ] インデックス定数に意味のある名前を使用

### 実装

- [ ] `BaseAdapter` を継承
- [ ] `source_name` を定義
- [ ] `@register` デコレータを使用
- [ ] 必要な `get_*_mapping()` メソッドを実装
- [ ] 型ヒントをすべてのメソッドに付与

### エラーハンドリング

- [ ] API レスポンスの検証を実装
- [ ] `AdapterError` で適切なエラーメッセージを提供
- [ ] demo.py に HTTP 例外のハンドリングを実装

### テスト

- [ ] ユニットテストを作成
- [ ] モックを使用して HTTP 通信をテスト
- [ ] エラーケースをテスト

## 参照

- [HTTP クライアント使用ガイド](http-client.md) - HTTP クライアントの詳細
- [モデル実装ガイド](models.md) - pydantic モデルの使い方

---

# Rust アダプター開発ガイド

外部データソースから marketschema モデルへのデータ変換アダプターを Rust で開発する方法を解説する。

## 概要

marketschema-adapters クレートは、外部データソース（取引所 API、データプロバイダなど）から取得したデータを marketschema の標準モデル（Quote, Ohlcv, Trade など）に変換するためのフレームワークを提供する。

このガイドの対象読者:

- 新しいデータソースの Rust アダプターを開発する開発者
- 既存アダプターをメンテナンスする開発者
- データソース統合パターンを理解したい開発者

## 主要コンポーネント

| コンポーネント | 説明 |
|--------------|------|
| `BaseAdapter` | すべてのアダプターが実装する基底 trait |
| `ModelMapping` | フィールドマッピング設定（ビルダーパターン） |
| `Transforms` | データ変換の共通関数群 |
| `AdapterRegistry` | スレッドセーフなグローバルアダプターレジストリ |

## BaseAdapter trait の実装

### 基本構造

```rust
use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
use async_trait::async_trait;

struct MyApiAdapter;

#[async_trait]
impl BaseAdapter for MyApiAdapter {
    fn source_name(&self) -> &'static str {
        "myapi"
    }

    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bid", "ticker.bid")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("ask", "ticker.ask")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "ticker.time")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("open", "open")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("high", "high")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("low", "low")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("close", "close")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("volume", "volume")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "time")
                .with_transform(Transforms::iso_timestamp_fn()),
        ]
    }
}
```

### 利用可能なマッピングメソッド

| メソッド | 対象モデル |
|---------|-----------|
| `get_quote_mapping()` | Quote |
| `get_ohlcv_mapping()` | Ohlcv |
| `get_trade_mapping()` | Trade |
| `get_orderbook_mapping()` | OrderBook |
| `get_instrument_mapping()` | Instrument |

各メソッドはデフォルトで空の `Vec<ModelMapping>` を返す。サポートするモデルのみオーバーライドする。

## ModelMapping

### ビルダーパターン

```rust
use marketschema_adapters::{ModelMapping, Transforms};
use serde_json::json;

// 基本的なマッピング（必須フィールド）
let mapping = ModelMapping::new("bid", "ticker.bid_price")
    .with_transform(Transforms::to_float_fn());

// デフォルト値とオプショナル設定
let mapping = ModelMapping::new("bid_size", "ticker.bid_qty")
    .with_transform(Transforms::to_float_fn())
    .with_default(json!(0.0))
    .optional();
```

### ドット記法によるネストアクセス

`source_field` にドット記法を使用すると、ネストした JSON から値を取得できる:

```rust
// {"response": {"data": {"price": 100.0}}} から価格を取得
let mapping = ModelMapping::new("price", "response.data.price")
    .with_transform(Transforms::to_float_fn());
```

### メソッド一覧

| メソッド | 説明 |
|---------|------|
| `new(target, source)` | 新しいマッピングを作成（`required=true`） |
| `with_transform(fn)` | 変換関数を設定 |
| `with_default(value)` | デフォルト値を設定 |
| `optional()` | `required=false` に設定 |
| `apply(data)` | ソースデータから値を取得・変換 |

## Transforms

### 利用可能な変換関数

| 関数 | 説明 | 入力例 | 出力例 |
|-----|------|-------|-------|
| `to_float_fn()` | 文字列/数値を f64 に変換 | `"123.45"` | `123.45` |
| `to_int_fn()` | 文字列/数値を i64 に変換 | `"123"` | `123` |
| `unix_timestamp_ms_fn()` | ミリ秒タイムスタンプを ISO 8601 に変換 | `1704067200000` | `"2024-01-01T00:00:00Z"` |
| `unix_timestamp_sec_fn()` | 秒タイムスタンプを ISO 8601 に変換 | `1704067200` | `"2024-01-01T00:00:00Z"` |
| `iso_timestamp_fn()` | ISO 8601 文字列を UTC に正規化 | `"2024-01-01T09:00:00+09:00"` | `"2024-01-01T00:00:00Z"` |
| `jst_to_utc_fn()` | JST タイムスタンプを UTC に変換 | `"2024-01-01T09:00:00"` | `"2024-01-01T00:00:00Z"` |
| `side_from_string_fn()` | 文字列を `"buy"`/`"sell"` に正規化 | `"BUY"` | `"buy"` |
| `uppercase_fn()` | 大文字に変換 | `"abc"` | `"ABC"` |
| `lowercase_fn()` | 小文字に変換 | `"ABC"` | `"abc"` |

### side_from_string の変換ルール

| 入力（大文字小文字不問） | 出力 |
|----------------------|-----|
| `buy`, `bid`, `b` | `"buy"` |
| `sell`, `ask`, `offer`, `s`, `a` | `"sell"` |

> **Note**: `long`/`short` は意図的にサポートされていない。これらはポジション方向を表し、約定の売買方向とは異なる意味を持つため。

## AdapterRegistry

### アダプターの登録と取得

```rust
use marketschema_adapters::{BaseAdapter, AdapterRegistry, ModelMapping};
use async_trait::async_trait;

struct MyAdapter;

#[async_trait]
impl BaseAdapter for MyAdapter {
    fn source_name(&self) -> &'static str {
        "myapi"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // アダプターを登録
    AdapterRegistry::register("myapi", || Box::new(MyAdapter))?;

    // 登録済みアダプター一覧を取得
    let adapters = AdapterRegistry::list_adapters()?;
    println!("Registered adapters: {:?}", adapters);

    // アダプターを取得
    if let Some(adapter) = AdapterRegistry::get("myapi")? {
        println!("Source: {}", adapter.source_name());
    }

    // 登録済みか確認
    if AdapterRegistry::is_registered("myapi")? {
        println!("myapi is registered");
    }

    Ok(())
}
```

### エラーハンドリング

```rust
use marketschema_adapters::{AdapterRegistry, AdapterError};

fn register_adapter() -> Result<(), AdapterError> {
    // 重複登録はエラー
    AdapterRegistry::register("myapi", || Box::new(MyAdapter))?;

    match AdapterRegistry::register("myapi", || Box::new(MyAdapter)) {
        Err(AdapterError::DuplicateRegistration(name)) => {
            println!("Adapter '{}' is already registered", name);
        }
        _ => {}
    }

    Ok(())
}
```

## 完全な実装例

### JSON API アダプター

```rust
use marketschema_adapters::{
    BaseAdapter, ModelMapping, Transforms, AdapterRegistry,
};
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, HttpError};
use async_trait::async_trait;
use std::sync::Arc;

const MYAPI_BASE_URL: &str = "https://api.example.com";

struct MyApiAdapter {
    http_client: Arc<AsyncHttpClient>,
}

impl MyApiAdapter {
    pub fn new() -> Result<Self, HttpError> {
        let http_client = Arc::new(AsyncHttpClientBuilder::new().build()?);
        Ok(Self { http_client })
    }

    pub async fn fetch_ticker(&self, symbol: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("{}/ticker/{}", MYAPI_BASE_URL, symbol);
        self.http_client.get_json(&url).await
    }
}

#[async_trait]
impl BaseAdapter for MyApiAdapter {
    fn source_name(&self) -> &'static str {
        "myapi"
    }

    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bid", "data.bid_price")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("ask", "data.ask_price")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("bid_size", "data.bid_qty")
                .with_transform(Transforms::to_float_fn())
                .optional(),
            ModelMapping::new("ask_size", "data.ask_qty")
                .with_transform(Transforms::to_float_fn())
                .optional(),
            ModelMapping::new("timestamp", "data.timestamp")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }
}
```

## エラー型

### AdapterError

```rust
pub enum AdapterError {
    /// 一般的なアダプターエラー
    General(String),
    /// 重複登録エラー
    DuplicateRegistration(String),
    /// マッピングエラー
    Mapping(MappingError),
    /// 変換エラー
    Transform(TransformError),
}
```

### MappingError

フィールドマッピングに失敗した場合のエラー。

```rust
// 必須フィールドが見つからない場合
if let Err(MappingError { message }) = mapping.apply(&data) {
    eprintln!("Mapping failed: {}", message);
}
```

### TransformError

値の変換に失敗した場合のエラー。

```rust
// 変換関数がエラーを返した場合
if let Err(TransformError { message }) = Transforms::to_float(&value) {
    eprintln!("Transform failed: {}", message);
}
```

## スレッドセーフ性

- `BaseAdapter` trait は `Send + Sync` を要求する
- `AdapterRegistry` は `RwLock` で保護されたグローバルシングルトン
- `TransformFn` は `Arc<dyn Fn(...) + Send + Sync>` として定義される
- 複数スレッドから安全にアダプターを登録・取得可能

## チェックリスト

新しい Rust アダプターを作成する際のチェックリスト:

### 実装

- [ ] `BaseAdapter` trait を実装
- [ ] `source_name()` で一意のデータソース識別子を返す
- [ ] 必要な `get_*_mapping()` メソッドをオーバーライド
- [ ] すべてのパブリック関数に型ヒントを付与

### エラーハンドリング

- [ ] 変換関数は失敗時に `TransformError` を返す
- [ ] サイレント障害を避け、エラーを伝播させる
- [ ] エラーメッセージに元の値を含める

### テスト

- [ ] マッピングのユニットテストを作成
- [ ] 変換関数の正常系・異常系をテスト
- [ ] モックを使用して HTTP 通信をテスト

## 参照

- [HTTP クライアント使用ガイド（Rust）](#rust-http-クライアント使用ガイド) - Rust HTTP クライアントの詳細
- [モデル実装ガイド（Rust）](#rust-モデル実装ガイド) - Rust モデルの使い方
- [004-adapter-rust spec](../specs/004-adapter-rust/spec.md) - 詳細仕様
