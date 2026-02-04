# marketschema

金融マーケットデータのための統一スキーマ。

## 概要

marketschema は金融マーケットデータを統一的に扱うための標準データモデルを提供する:

- **Quote** - 最良気配値（BBO）
- **OHLCV** - ローソク足データ
- **Trade** - 約定データ（Time & Sales）
- **OrderBook** - 板情報
- **Instrument** - 銘柄情報

## 特徴

- **Schema First** - JSON Schema を単一の真実の源として Python/Rust コードを自動生成
- **アダプターフレームワーク** - 外部データソースから標準モデルへの変換
- **非同期 HTTP クライアント** - リトライ、レートリミット、キャッシュ対応
- **マルチ言語対応** - Python (pydantic v2) / Rust (serde)

## インストール

### ライブラリとして使う

```bash
# pip
pip install "git+https://github.com/drillan/marketschema.git#subdirectory=python"

# uv
uv pip install "marketschema @ git+https://github.com/drillan/marketschema.git#subdirectory=python"
```

### 開発環境のセットアップ

```bash
git clone https://github.com/drillan/marketschema.git
cd marketschema/python
uv sync --group dev
```

## クイックスタート

### Python

```python
from marketschema import Quote, Trade, OHLCV, Side

# Quote の作成
quote = Quote(
    symbol="AAPL",
    timestamp="2026-02-02T14:30:00Z",
    bid=175.00,
    ask=175.50,
)

# Trade の作成
trade = Trade(
    symbol="AAPL",
    timestamp="2026-02-02T14:30:00.123Z",
    price=175.25,
    size=100,
    side=Side.buy,
)
```

### HTTP クライアント

```python
from marketschema import AsyncHttpClient

async with AsyncHttpClient() as client:
    data = await client.get_json("https://api.example.com/ticker")
```

### アダプター

```python
from marketschema import BaseAdapter, ModelMapping, register

@register
class MyExchangeAdapter(BaseAdapter):
    source_name = "my_exchange"

    def get_quote_mapping(self):
        return [
            ModelMapping("symbol", "ticker"),
            ModelMapping("timestamp", "time", transform=self.transforms.unix_timestamp_ms),
            ModelMapping("bid", "best_bid", transform=self.transforms.to_float),
            ModelMapping("ask", "best_ask", transform=self.transforms.to_float),
        ]
```

### Rust

```rust
use marketschema::Quote;

let json = r#"{"symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z", "bid": 175.00, "ask": 175.50}"#;
let quote: Quote = serde_json::from_str(json)?;
```

## ドキュメント

詳細なドキュメントは [docs/](docs/) を参照:

- [アーキテクチャ](docs/architecture.md) - Schema First 設計と3層アーキテクチャ
- [コード生成](docs/code-generation.md) - Python/Rust コードの生成方法
- [Python モデル実装ガイド](docs/guides/models.md)
- [Python HTTP クライアント使用ガイド](docs/guides/http-client.md)
- [Python アダプター開発ガイド](docs/guides/adapter-development.md)

## コード生成

```bash
make generate-models  # Python pydantic モデル生成
make generate-rust    # Rust serde 構造体生成
```

詳細は [コード生成ガイド](docs/code-generation.md) を参照。

## 開発

```bash
make lint       # リンター
make typecheck  # 型チェック
make test       # テスト
make all        # 全チェック実行
```

## ライセンス

MIT
