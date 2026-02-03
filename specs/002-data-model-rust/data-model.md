# Data Model: Rust Data Model Implementation

**Feature Branch**: `002-data-model-rust`
**Date**: 2026-02-03

## Overview

JSON Schema から typify で自動生成される Rust struct の概要。すべての型は親仕様（002-data-model）の JSON Schema から生成される。

## Entity Definitions

### Quote

最良気配値（BBO: Best Bid/Offer）を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| symbol | QuoteSymbol (newtype) | ✅ | 銘柄識別子 |
| timestamp | DateTime<Utc> | ✅ | 気配値取得時刻 |
| bid | f64 | ✅ | 買い気配値 |
| ask | f64 | ✅ | 売り気配値 |
| bid_size | Option<f64> | ❌ | 買い気配の数量 |
| ask_size | Option<f64> | ❌ | 売り気配の数量 |

**Source Schema**: `src/marketschema/schemas/quote.json`

---

### Ohlcv

ローソク足（OHLCV）データを表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| symbol | OhlcvSymbol (newtype) | ✅ | 銘柄識別子 |
| timestamp | DateTime<Utc> | ✅ | 足の開始時刻 |
| open | f64 | ✅ | 始値 |
| high | f64 | ✅ | 高値 |
| low | f64 | ✅ | 安値 |
| close | f64 | ✅ | 終値 |
| volume | f64 | ✅ | 出来高 |

**Source Schema**: `src/marketschema/schemas/ohlcv.json`

---

### Trade

個別約定（歩み値）を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| symbol | TradeSymbol (newtype) | ✅ | 銘柄識別子 |
| timestamp | DateTime<Utc> | ✅ | 約定時刻 |
| price | f64 | ✅ | 約定価格 |
| size | f64 | ✅ | 約定数量 |
| side | Option<Side> | ❌ | 売買方向 (buy/sell) |

**Source Schema**: `src/marketschema/schemas/trade.json`

---

### OrderBook

板情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| symbol | OrderBookSymbol (newtype) | ✅ | 銘柄識別子 |
| timestamp | DateTime<Utc> | ✅ | 板情報取得時刻 |
| bids | Vec<PriceLevel> | ✅ | 買い板（価格降順） |
| asks | Vec<PriceLevel> | ✅ | 売り板（価格昇順） |

**Source Schema**: `src/marketschema/schemas/orderbook.json`

---

### PriceLevel

板情報の気配レベルを表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| price | f64 | ✅ | 気配値 |
| size | f64 | ✅ | 数量 |

**Source Schema**: `src/marketschema/schemas/definitions.json#/$defs/PriceLevel`

---

### Instrument

銘柄情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| symbol | InstrumentSymbol (newtype) | ✅ | 銘柄識別子 |
| asset_class | AssetClass | ✅ | 資産クラス |
| currency | Currency (newtype) | ✅ | 通貨コード (ISO 4217) |
| exchange | Option<Exchange> (newtype) | ❌ | 取引所 (ISO 10383) |

**Source Schema**: `src/marketschema/schemas/instrument.json`

---

### VolumeInfo

出来高・売買代金情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| volume | Option<f64> | ❌ | 出来高 |
| quote_volume | Option<f64> | ❌ | 売買代金 |

**Source Schema**: `src/marketschema/schemas/volume_info.json`

---

### ExpiryInfo

先物・オプションの満期情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| expiry_series | ExpirySeries (newtype) | ✅ | 限月識別子 |
| expiry_date | Option<Date> (newtype) | ❌ | 満期日 |
| last_trading_date | Option<Date> (newtype) | ❌ | 最終取引日 |
| settlement_date | Option<Date> (newtype) | ❌ | 決済日 |

**Source Schema**: `src/marketschema/schemas/expiry_info.json`

---

### OptionInfo

オプション固有情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| strike | f64 | ✅ | 権利行使価格 |
| option_type | OptionType | ✅ | call / put |
| exercise_style | Option<ExerciseStyle> | ❌ | 行使スタイル |

**Source Schema**: `src/marketschema/schemas/option_info.json`

---

### DerivativeInfo

デリバティブ（先物・オプション）情報を表現する struct。

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| underlying_symbol | UnderlyingSymbol (newtype) | ✅ | 原資産銘柄 |
| underlying_type | UnderlyingType | ✅ | 原資産タイプ |
| multiplier | Option<f64> | ❌ | 乗数 |
| settlement_method | Option<SettlementMethod> | ❌ | 決済方法 |
| expiry | Option<ExpiryInfo> | ❌ | 満期情報 |
| option | Option<OptionInfo> | ❌ | オプション情報 |

**Source Schema**: `src/marketschema/schemas/derivative_info.json`

## Enum Definitions

### Side

売買方向を表現する enum。

```rust
pub enum Side {
    Buy,
    Sell,
}
```

### AssetClass

資産クラスを表現する enum。

```rust
pub enum AssetClass {
    Equity,
    Fund,
    Bond,
    Future,
    Option,
    Fx,
    Crypto,
    Cfd,
}
```

### OptionType

オプションタイプを表現する enum。

```rust
pub enum OptionType {
    Call,
    Put,
}
```

### ExerciseStyle

オプション行使スタイルを表現する enum。

```rust
pub enum ExerciseStyle {
    American,
    European,
    Bermudan,
}
```

### UnderlyingType

原資産タイプを表現する enum。

```rust
pub enum UnderlyingType {
    Stock,
    Index,
    Etf,
    Commodity,
    Currency,
    Crypto,
}
```

### SettlementMethod

決済方法を表現する enum。

```rust
pub enum SettlementMethod {
    Cash,
    Physical,
}
```

## Newtype Definitions

typify は minLength, pattern などの制約を持つ string を newtype として生成する。

| Newtype | Validation | Example |
|---------|------------|---------|
| QuoteSymbol | minLength: 1 | "7203.T" |
| Currency | pattern: ^[A-Z]{3}$ | "JPY" |
| Exchange | pattern: ^[A-Z]{4}$ | "XJPX" |
| Date | pattern: ^\d{4}-\d{2}-\d{2}$ | "2026-02-03" |
| ExpirySeries | pattern | "2026-03", "2026-W10" |

## Type Relationships

```
Instrument ──────────────────────────────────────┐
    │                                            │
    ├── symbol: InstrumentSymbol                 │
    ├── asset_class: AssetClass                  │
    ├── currency: Currency                       │
    └── exchange: Option<Exchange>               │
                                                 │
DerivativeInfo ──────────────────────────────────┤
    │                                            │
    ├── underlying_symbol: UnderlyingSymbol      │
    ├── underlying_type: UnderlyingType          │
    ├── expiry: Option<ExpiryInfo> ─────────────►ExpiryInfo
    │                                            │
    └── option: Option<OptionInfo> ─────────────►OptionInfo

OrderBook ───────────────────────────────────────┤
    │                                            │
    ├── bids: Vec<PriceLevel> ──────────────────►PriceLevel
    └── asks: Vec<PriceLevel> ──────────────────►PriceLevel
```

## Generation Notes

1. **$ref 解決**: すべての `$ref` はバンドル時に解決され、インライン化される
2. **unevaluatedProperties**: `additionalProperties: false` に変換され、`#[serde(deny_unknown_fields)]` が生成される
3. **Doc comments**: JSON Schema の `description` が Rust doc comments として生成される
4. **Builder pattern**: 各 struct に `builder()` メソッドが生成される

## Related Documents

- [Parent Spec: 002-data-model](../002-data-model/spec.md)
- [ADR: フィールド名](../../docs/adr/index.md)
- [用語集](../../docs/glossary.md)
