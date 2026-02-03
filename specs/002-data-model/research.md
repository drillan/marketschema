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

## 2. 言語別コード生成

各言語のコード生成ツールと設定の詳細は、言語別仕様を参照:

- **Python**: [lang/python.md](lang/python.md) - datamodel-code-generator + pydantic v2
- **Rust**: [lang/rust.md](lang/rust.md) - typify + serde

### 調査結果サマリ

| 言語 | ツール | 主な制約 |
|------|--------|----------|
| Python | datamodel-code-generator | $ref は相対パス推奨 |
| Rust | cargo-typify | 外部 $ref 解決に制限あり、事前バンドル必要 |

**Decision**: Python 優先でスキーマ設計し、Rust 生成時は事前バンドルで対応。

---

## 3. スキーマバリデーション (ajv-cli)

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

## 4. プロジェクト固有の決定事項

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
