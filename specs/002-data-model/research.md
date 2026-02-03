# Research: 統一マーケットデータスキーマ

**Date**: 2026-02-02
**Status**: Complete

## 1. JSON Schema Draft 2020-12 ベストプラクティス

### Draft-07 からの主な変更点

| 項目 | Draft-07 | Draft 2020-12 |
|------|----------|---------------|
| 定義格納場所 | `definitions` | `$defs` |
| 配列タプル定義 | `items` + `additionalItems` | `prefixItems` + `items` |
| 追加プロパティ制御 | `additionalProperties` | `unevaluatedProperties`（継承対応） |

**Decision**: Draft 2020-12 を採用。ISO 20022 も同バージョンを採用しており、OpenAPI 3.1.0 との整合性も確保できる。

### `$defs` による再利用可能な型定義

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://marketschema.example.com/schemas/definitions",
  "$defs": {
    "Symbol": {
      "type": "string",
      "minLength": 1,
      "description": "銘柄識別子"
    },
    "Timestamp": {
      "type": "string",
      "format": "date-time",
      "description": "ISO 8601形式のタイムスタンプ (UTC)"
    }
  }
}
```

**Decision**: 共通型は `definitions.json` に `$defs` で定義し、各スキーマから `$ref` で参照する。

### `unevaluatedProperties: false` と スキーマ継承

`additionalProperties` は `$ref` や `allOf` の内部を認識できないが、`unevaluatedProperties` は認識可能。

```json
{
  "allOf": [{ "$ref": "#/$defs/BaseMarketData" }],
  "type": "object",
  "properties": {
    "price": { "type": "number" }
  },
  "unevaluatedProperties": false
}
```

**Decision**: 各リーフスキーマに `unevaluatedProperties: false` を設定。継承が必要な基底スキーマには設定しない。

### Nullable フィールド

```json
{
  "side": {
    "type": ["string", "null"],
    "enum": ["buy", "sell", null]
  }
}
```

**Decision**: 型配列構文 `["type", "null"]` を優先使用。`$ref` を含む場合のみ `oneOf` を使用。

---

## 2. Python コード生成 (datamodel-code-generator)

### 推奨コマンド

```bash
datamodel-codegen \
  --input schemas/ \
  --input-file-type jsonschema \
  --output-model-type pydantic_v2.BaseModel \
  --target-python-version 3.13 \
  --use-annotated \
  --field-constraints \
  --strict-types str int float bool \
  --use-standard-collections \
  --use-union-operator \
  --snake-case-field \
  --use-schema-description \
  --use-field-description \
  --reuse-model \
  --reuse-scope tree \
  --disable-timestamp \
  --formatters ruff-format ruff-check \
  --output src/marketschema/models/
```

### 主要オプション

| オプション | 説明 |
|-----------|------|
| `--output-model-type pydantic_v2.BaseModel` | Pydantic v2 モデル出力 |
| `--use-annotated` | `typing.Annotated` を使用 |
| `--reuse-model` | 同一内容のモデルを再利用 |
| `--reuse-scope tree` | ファイル横断的な重複排除 |

### unevaluatedProperties 対応

- `unevaluatedProperties: false` → `model_config = ConfigDict(extra='forbid')`
- `unevaluatedProperties: true` → `model_config = ConfigDict(extra='allow')`

**Decision**: pyproject.toml に設定を記載し、Make/uv script で実行。

---

## 3. Rust コード生成 (typify)

### インストールと使用

```bash
cargo install cargo-typify
cargo typify schema.json --output types.rs
```

### 制限事項

1. **外部 `$ref` の解決に制限あり** - 事前にスキーマをバンドルする必要
2. **Draft 2020-12 の明示的サポートなし** - `$defs` と `definitions` 両方定義で互換性確保
3. **`anyOf`、`if/then/else` のサポートが限定的**

### 推奨ワークフロー

```bash
# 1. スキーマをバンドル
npx json-refs resolve schema.json > bundled-schema.json

# 2. Rust コード生成
cargo typify bundled-schema.json --output src/types.rs
```

### 自動生成される serde 属性

| 属性 | 条件 |
|-----|------|
| `#[derive(Serialize, Deserialize, Debug, Clone)]` | すべての型 |
| `#[serde(default)]` | `required` に含まれないプロパティ |
| `#[serde(deny_unknown_fields)]` | `additionalProperties: false` の場合 |

**Decision**: Python 優先でスキーマ設計し、Rust 生成時は事前バンドルで対応。

---

## 4. スキーマバリデーション (ajv-cli)

### インストールと使用

```bash
npm install -g ajv-cli
ajv validate -s schema.json -d data.json --spec=draft2020
```

### 推奨設定

```bash
ajv validate \
  --spec=draft2020 \
  --strict=true \
  --all-errors \
  -s schema.json \
  -d data.json
```

**Decision**: CI/CD で ajv-cli を使用してスキーマ準拠を検証。

---

## 5. プロジェクト固有の決定事項

### スキーマ ID 形式

```
https://marketschema.example.com/schemas/<name>
```

例:
- `https://marketschema.example.com/schemas/definitions`
- `https://marketschema.example.com/schemas/quote`
- `https://marketschema.example.com/schemas/ohlcv`

### ファイル構成

```
src/marketschema/schemas/
├── definitions.json    # 共通型定義
├── quote.json          # Quote スキーマ
├── ohlcv.json          # OHLCV スキーマ
├── trade.json          # Trade スキーマ
├── orderbook.json      # OrderBook スキーマ
├── volume_info.json    # VolumeInfo スキーマ
├── expiry_info.json    # ExpiryInfo スキーマ
├── option_info.json    # OptionInfo スキーマ
├── derivative_info.json # DerivativeInfo スキーマ
└── instrument.json     # Instrument スキーマ
```

### 共通型定義 (definitions.json)

| 型名 | 説明 |
|------|------|
| `Timestamp` | ISO 8601 date-time (UTC) |
| `Symbol` | 銘柄識別子 |
| `Price` | 価格（number） |
| `Size` | 数量（number） |
| `Side` | 売買方向 enum (buy, sell) |
| `AssetClass` | 資産クラス enum |
| `Currency` | ISO 4217 通貨コード |
| `Exchange` | ISO 10383 MIC |
| `PriceLevel` | 板情報の気配レベル |

---

## Sources

- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12)
- [Modelling Inheritance with JSON Schema](https://json-schema.org/blog/posts/modelling-inheritance)
- [datamodel-code-generator GitHub](https://github.com/koxudaxi/datamodel-code-generator)
- [typify GitHub](https://github.com/oxidecomputer/typify)
- [ISO 20022 JSON Schema Draft 2020-12 Generation](https://www.iso20022.org/)
