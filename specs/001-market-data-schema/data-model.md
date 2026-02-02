# Data Model: 統一マーケットデータスキーマ

**Date**: 2026-02-02
**Source**: [spec.md](spec.md), [field-requirements.csv](field-requirements.csv)

## Entity Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Market Data Entities                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐        │
│  │  Quote   │  │  OHLCV   │  │  Trade   │  │  OrderBook   │        │
│  │          │  │          │  │          │  │              │        │
│  │ • symbol │  │ • symbol │  │ • symbol │  │ • symbol     │        │
│  │ • bid    │  │ • open   │  │ • price  │  │ • bids[]     │        │
│  │ • ask    │  │ • high   │  │ • size   │  │ • asks[]     │        │
│  │ • bid_size│ │ • low    │  │ • side   │  │              │        │
│  │ • ask_size│ │ • close  │  │          │  │              │        │
│  └──────────┘  │ • volume │  └──────────┘  └──────────────┘        │
│                └──────────┘                                         │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                        Instrument                             │  │
│  │  • symbol      • asset_class   • currency    • exchange      │  │
│  │  • base_currency (pairs)       • quote_currency (pairs)      │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                         ▲                                           │
│                         │ extends                                   │
│  ┌──────────────────────┴───────────────────────────────────────┐  │
│  │                    DerivativeInfo                             │  │
│  │  • multiplier  • tick_size  • underlying_symbol              │  │
│  │  • is_perpetual • is_inverse • settlement_method             │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                         ▲                                           │
│         ┌───────────────┴───────────────┐                          │
│  ┌──────┴─────┐                  ┌──────┴─────┐                    │
│  │ ExpiryInfo │                  │ OptionInfo │                    │
│  │            │                  │            │                    │
│  │ • expiry   │                  │ • strike   │                    │
│  │ • exp_date │                  │ • opt_type │                    │
│  │ • last_day │                  │ • exercise │                    │
│  └────────────┘                  └────────────┘                    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Common Types (definitions.json)

| Type | JSON Schema | Description |
|------|-------------|-------------|
| `Timestamp` | `{ "type": "string", "format": "date-time" }` | ISO 8601 UTC |
| `Symbol` | `{ "type": "string", "minLength": 1 }` | 銘柄識別子 |
| `Price` | `{ "type": "number" }` | 価格 |
| `Size` | `{ "type": "number" }` | 数量 |
| `Side` | `{ "type": "string", "enum": ["buy", "sell"] }` | 売買方向 |
| `AssetClass` | `{ "type": "string", "enum": [...] }` | 資産クラス |
| `Currency` | `{ "type": "string", "pattern": "^[A-Z]{3}$" }` | ISO 4217 |
| `Exchange` | `{ "type": "string", "pattern": "^[A-Z]{4}$" }` | ISO 10383 MIC |

## Entity Definitions

### Quote (気配値)

最良気配値（BBO: Best Bid/Offer）を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `symbol` | string | true | false | 銘柄識別子 |
| `timestamp` | Timestamp | true | false | 気配値取得時刻 |
| `bid` | number | true | false | 買い気配値 |
| `ask` | number | true | false | 売り気配値 |
| `bid_size` | number | false | true | 買い気配の数量 |
| `ask_size` | number | false | true | 売り気配の数量 |

**Validation Rules**:
- `bid` と `ask` は正の数値
- `bid <= ask` が通常だが、スキーマレベルでは強制しない（異常検知はアプリケーション側）

---

### OHLCV (ローソク足)

一定期間の価格と出来高を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `symbol` | string | true | false | 銘柄識別子 |
| `timestamp` | Timestamp | true | false | 足の開始時刻 |
| `open` | number | true | false | 始値 |
| `high` | number | true | false | 高値 |
| `low` | number | true | false | 安値 |
| `close` | number | true | false | 終値 |
| `volume` | number | true | false | 出来高 |
| `quote_volume` | number | false | true | 売買代金（決済通貨建て） |

**Validation Rules**:
- `low <= open, close <= high`（スキーマレベルでは強制しない）
- `volume >= 0`

---

### Trade (約定)

個別の約定（歩み値 / Time & Sales）を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `symbol` | string | true | false | 銘柄識別子 |
| `timestamp` | Timestamp | true | false | 約定時刻 |
| `price` | number | true | false | 約定価格 |
| `size` | number | true | false | 約定数量 |
| `side` | Side | true | false | 売買方向 (buy/sell) |

---

### OrderBook (板情報)

複数レベルの板情報を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `symbol` | string | true | false | 銘柄識別子 |
| `timestamp` | Timestamp | true | false | 板情報取得時刻 |
| `bids` | PriceLevel[] | true | false | 買い板（価格降順） |
| `asks` | PriceLevel[] | true | false | 売り板（価格昇順） |

**PriceLevel**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `price` | number | true | 気配値 |
| `size` | number | true | 数量 |

---

### Instrument (銘柄情報)

銘柄の識別情報を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `symbol` | string | true | false | 銘柄識別子 |
| `asset_class` | AssetClass | true | false | 資産クラス |
| `currency` | Currency | false | true | 単一通貨（株式・債券等） |
| `exchange` | Exchange | false | true | 上場取引所 |
| `base_currency` | Currency | false | true | 基軸通貨（FX・暗号資産） |
| `quote_currency` | Currency | false | true | 決済通貨（FX・暗号資産） |

**Validation Rules**:
- 単一通貨商品: `currency` を使用
- ペア商品: `base_currency` + `quote_currency` を使用
- 相互排他ではないが、通常どちらかのパターンを使用

---

### DerivativeInfo (デリバティブ情報)

先物・オプション共通の情報を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `multiplier` | number | true | false | 契約乗数 |
| `tick_size` | number | true | false | 呼値単位 |
| `tick_value` | number | false | true | ティック価値 |
| `contract_value` | number | false | true | 契約基本価値 |
| `contract_value_currency` | Currency | false | true | 契約価値の通貨 |
| `lot_size` | number | false | true | 取引単位 |
| `min_order_size` | number | false | true | 最小注文数量 |
| `max_order_size` | number | false | true | 最大注文数量 |
| `underlying_symbol` | string | true | false | 原資産シンボル |
| `underlying_type` | UnderlyingType | true | false | 原資産タイプ |
| `is_perpetual` | boolean | false | true | 無期限契約か |
| `is_inverse` | boolean | false | true | インバース契約か |
| `settlement_method` | SettlementMethod | false | true | 決済方法 |
| `settlement_currency` | Currency | false | true | 決済通貨 |

---

### ExpiryInfo (満期情報)

先物・オプションの満期関連情報を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `expiry` | string | false | true | 満期系列識別子 |
| `last_trading_day` | date | false | true | 最終取引日 |
| `expiration_date` | date | true | false | 満期日/SQ日 |
| `settlement_date` | date | false | true | 決済日 |

**Format**:
- `expiry`: `YYYY-MM`, `YYYY-Www`, または `YYYY-MM-DD`
- 日付フィールド: `YYYY-MM-DD`

---

### OptionInfo (オプション情報)

オプション固有の情報を表現する。

| Field | Type | Required | Nullable | Description |
|-------|------|----------|----------|-------------|
| `strike_price` | number | true | false | 権利行使価格 |
| `option_type` | OptionType | true | false | コール/プット |
| `exercise_style` | ExerciseStyle | false | true | 行使スタイル |

---

## Enum Definitions

### Side

| Value | Description |
|-------|-------------|
| `buy` | 買い |
| `sell` | 売り |

### AssetClass

| Value | Description |
|-------|-------------|
| `equity` | 株式 |
| `fund` | 投資信託 |
| `bond` | 債券 |
| `future` | 先物 |
| `option` | オプション |
| `fx` | 外国為替 |
| `crypto` | 暗号資産 |
| `cfd` | CFD |

### OptionType

| Value | Description |
|-------|-------------|
| `call` | コールオプション |
| `put` | プットオプション |

### UnderlyingType

| Value | Description |
|-------|-------------|
| `stock` | 株式 |
| `index` | 指数 |
| `etf` | ETF |
| `commodity` | 商品 |
| `currency` | 通貨 |
| `crypto` | 暗号資産 |

### ExerciseStyle

| Value | Description |
|-------|-------------|
| `american` | アメリカン（満期前行使可能） |
| `european` | ヨーロピアン（満期日のみ行使可能） |
| `bermudan` | バミューダン（特定日のみ行使可能） |

### SettlementMethod

| Value | Description |
|-------|-------------|
| `cash` | 現金決済 |
| `physical` | 現物決済 |

---

## Schema Inheritance Pattern

```
                    ┌─────────────────┐
                    │ BaseMarketData  │  (内部定義、公開スキーマではない)
                    │                 │
                    │ • symbol        │
                    │ • timestamp     │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│     Quote     │   │    OHLCV      │   │    Trade      │
│               │   │               │   │               │
│ + bid, ask    │   │ + open, high  │   │ + price       │
│ + bid_size    │   │ + low, close  │   │ + size        │
│ + ask_size    │   │ + volume      │   │ + side        │
└───────────────┘   └───────────────┘   └───────────────┘
```

**Note**: 継承は `allOf` を使用して表現。各リーフスキーマには `unevaluatedProperties: false` を設定。

---

## State Transitions

本スキーマはマーケットデータのスナップショットを表現するため、状態遷移は定義しない。時系列データとしての順序はアプリケーション側で管理する。

---

## Cross-References

- [field-requirements.csv](field-requirements.csv) - フィールド定義の詳細
- [enum-values.csv](enum-values.csv) - Enum値の定義と代替案
- [format-conventions.csv](format-conventions.csv) - フォーマット規約
- [ADR: フィールド名](../../docs/adr/index.md) - 命名決定の根拠
