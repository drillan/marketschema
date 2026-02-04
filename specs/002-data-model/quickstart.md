# Quickstart: 統一マーケットデータスキーマ

このドキュメントでは、marketschema の基本的な使用方法を説明します。

## 前提条件

- Python 3.13+
- uv (Python パッケージマネージャー)
- Node.js (ajv-cli 用、スキーマバリデーション)
- Rust (オプション、Rust コード生成用)

## インストール

```bash
# プロジェクトのクローン
git clone https://github.com/example/marketschema.git
cd marketschema

# Python 環境のセットアップ
uv sync

# JSON Schema バリデーションツールのインストール
npm install -g ajv-cli
```

## JSON Schema の使用

### スキーマファイルの場所

```
schemas/
├── definitions.json    # 共通型定義
├── quote.json          # Quote スキーマ
├── ohlcv.json          # OHLCV スキーマ
├── trade.json          # Trade スキーマ
├── orderbook.json      # OrderBook スキーマ
├── instrument.json     # Instrument スキーマ
├── derivative_info.json
├── expiry_info.json
└── option_info.json
```

### データのバリデーション

```bash
# Quote データのバリデーション
ajv validate \
  --spec=draft2020 \
  -s schemas/quote.json \
  -r "schemas/definitions.json" \
  -d sample_quote.json

# OHLCV データのバリデーション
ajv validate \
  --spec=draft2020 \
  -s schemas/ohlcv.json \
  -r "schemas/definitions.json" \
  -d sample_ohlcv.json
```

### サンプルデータ

**Quote**:
```json
{
  "symbol": "7203.T",
  "timestamp": "2026-02-02T09:00:00.000Z",
  "bid": 2850.0,
  "ask": 2851.0,
  "bid_size": 1000,
  "ask_size": 500
}
```

**OHLCV**:
```json
{
  "symbol": "BTCUSDT",
  "timestamp": "2026-02-02T00:00:00.000Z",
  "open": 50000.0,
  "high": 51500.0,
  "low": 49800.0,
  "close": 51200.0,
  "volume": 12345.67,
  "quote_volume": 628000000.0
}
```

**Trade**:
```json
{
  "symbol": "AAPL",
  "timestamp": "2026-02-02T14:30:00.123Z",
  "price": 175.50,
  "size": 100,
  "side": "buy"
}
```

**OrderBook**:
```json
{
  "symbol": "USDJPY",
  "timestamp": "2026-02-02T09:00:00.000Z",
  "bids": [
    { "price": 149.50, "size": 1000000 },
    { "price": 149.49, "size": 2000000 }
  ],
  "asks": [
    { "price": 149.51, "size": 1500000 },
    { "price": 149.52, "size": 3000000 }
  ]
}
```

## Python pydantic モデルの生成

```bash
# datamodel-code-generator のインストール
uv tool install datamodel-code-generator

# モデルの生成
datamodel-codegen \
  --input schemas/ \
  --input-file-type jsonschema \
  --output-model-type pydantic_v2.BaseModel \
  --target-python-version 3.13 \
  --use-annotated \
  --field-constraints \
  --use-standard-collections \
  --use-union-operator \
  --snake-case-field \
  --use-schema-description \
  --use-field-description \
  --reuse-model \
  --reuse-scope tree \
  --disable-timestamp \
  --output python/src/marketschema/models/
```

### 生成されたモデルの使用

```python
from marketschema.models import Quote, OHLCV, Trade, OrderBook

# Quote の作成
quote = Quote(
    symbol="7203.T",
    timestamp="2026-02-02T09:00:00.000Z",
    bid=2850.0,
    ask=2851.0,
    bid_size=1000,
    ask_size=500
)

# バリデーションエラーの例
try:
    invalid_quote = Quote(
        symbol="",  # 空文字列は不可
        timestamp="2026-02-02T09:00:00.000Z",
        bid=2850.0,
        ask=2851.0
    )
except ValidationError as e:
    print(e)
```

## Rust struct の生成

```bash
# cargo-typify のインストール
cargo install cargo-typify

# スキーマのバンドル（外部参照の解決）
npx json-refs resolve schemas/quote.json > bundled_quote.json

# Rust コードの生成
cargo typify bundled_quote.json --output src/types/quote.rs
```

### 生成された struct の使用

```rust
use serde::{Deserialize, Serialize};
use crate::types::quote::Quote;

let json_data = r#"{
    "symbol": "7203.T",
    "timestamp": "2026-02-02T09:00:00.000Z",
    "bid": 2850.0,
    "ask": 2851.0,
    "bid_size": 1000,
    "ask_size": 500
}"#;

let quote: Quote = serde_json::from_str(json_data)?;
println!("Symbol: {}", quote.symbol);
```

## アダプターの使用

### BaseAdapter の継承

```python
from marketschema.adapters import BaseAdapter, ModelMapping
from marketschema.models import Quote

class SampleAdapter(BaseAdapter):
    source_name = "sample_exchange"

    def get_quote_mapping(self) -> list[ModelMapping]:
        return [
            ModelMapping(
                target_field="symbol",
                source_field="ticker",
            ),
            ModelMapping(
                target_field="timestamp",
                source_field="time",
                transform=self.transforms.iso_timestamp,
            ),
            ModelMapping(
                target_field="bid",
                source_field="best_bid.price",
            ),
            ModelMapping(
                target_field="ask",
                source_field="best_ask.price",
            ),
        ]

    def parse_quote(self, raw_data: dict) -> Quote:
        return self._apply_mapping(raw_data, self.get_quote_mapping(), Quote)
```

### AdapterRegistry の使用

```python
from marketschema.adapters import AdapterRegistry, register

@register
class SampleAdapter(BaseAdapter):
    source_name = "sample_exchange"
    # ...

# レジストリからアダプターを取得
adapter = AdapterRegistry.get("sample_exchange")
quote = adapter.parse_quote(raw_data)
```

## 次のステップ

- [data-model.md](data-model.md) - データモデルの詳細仕様
- [research.md](research.md) - 技術調査結果
- [ADR](../../docs/adr/index.md) - アーキテクチャ決定記録
- [用語集](../../docs/glossary.md) - 用語の定義
