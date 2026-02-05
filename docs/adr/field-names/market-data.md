# ADR: マーケットデータフィールド名

## ステータス

Accepted

## 日付

2026-02-05

## コンテキスト

先物・オプション等のデリバティブ市場で重要なマーケットデータフィールドの標準名を決定する。特に以下のフィールドは、JPX先物API調査（`jpx-client/docs/research/futures-api.md`）で必要性が確認された：

- **清算値段（Settlement Price）**: 証拠金計算、日次損益計算の基準となる価格
- **建玉残（Open Interest）**: 未決済契約の総数、市場の流動性・関心度の指標

ソース番号は [情報ソース一覧](../../research/sources.md) を参照。

---

## 決定済みフィールド

### 1. 清算値段: `settlement_price`

**採用名:** `settlement_price`

#### 調査結果

| ソース | 名称 | 型/形式 | 参照 |
|--------|------|---------|------|
| FIX Protocol | `SettlPrice` (Tag 730) | Price | [^STD-2] |
| CME | "Settlement Price" / "Settle" | - | [^EX-4] |
| JPX | `ST` (Settlement Price) | カンマ区切り文字列 | [^JPX-FUTURES-API] |
| Binance Futures | `estimatedSettlePrice` | String | [^CRYPTO-1] |
| CCXT | `estimatedSettlePrice` (FundingRate内) | number | [^CRYPTO-6] |

**集計:**

| フィールド名パターン | 使用ソース |
|---------------------|-----------|
| `SettlPrice` / `settlement_price` | FIX Protocol, CME |
| `estimatedSettlePrice` | Binance, CCXT |
| `ST` | JPX（略称） |

#### 概念の整理

清算値段（Settlement Price）は以下の目的で使用される：

| 用途 | 説明 |
|------|------|
| 証拠金計算 | 当日の評価損益を計算し、必要証拠金を算出 |
| 日次損益計算 | 前日清算値との差額でP&Lを計算 |
| 最終決済 | 満期日の最終清算値（SQ値）として使用 |

**関連概念との違い:**

| 概念 | 説明 | フィールド名 |
|------|------|------------|
| Settlement Price | 取引所が決定する公式清算値 | `settlement_price` |
| Mark Price | リアルタイムの理論価格（清算判定に使用） | `mark_price` |
| Last Price | 最新約定価格 | `price` (Trade) |
| Close Price | 終値（OHLCVの終値） | `close` |

**決定理由:**
- FIX Protocol の `SettlPrice` (Tag 730) が業界標準
- CME、JPX 等の主要デリバティブ取引所で「Settlement Price」として使用
- snake_case に変換して `settlement_price` を採用
- `estimated_settle_price` は Binance の暫定値であり、確定値は `settlement_price` が適切

---

### 2. 建玉残: `open_interest`

**採用名:** `open_interest`

#### 調査結果

| ソース | 名称 | 型/形式 | 参照 |
|--------|------|---------|------|
| FIX Protocol | `OpenInterest` (Tag 746) | Amt | [^STD-2] |
| Interactive Brokers | `Open Interest` (Tick ID 22) | int | [^BRK-1] |
| Interactive Brokers | `Futures Open Interest` (Tick ID 86) | int | [^BRK-1] |
| Binance Futures | `openInterest` | String | [^CRYPTO-1] |
| OKX | `oi` (WebSocket), `openInterest` (REST) | String | [^CRYPTO-4] |
| Polygon.io | `open_interest` | number | [^DATA-1] |
| CCXT | `openInterestAmount`, `openInterestValue` | number | [^CRYPTO-6] |
| JPX | `DOI` (Day Open Interest) | カンマ区切り文字列 | [^JPX-FUTURES-API] |

**集計:**

| フィールド名パターン | 使用ソース数 | 主なソース |
|---------------------|-------------|-----------|
| `open_interest` / `openInterest` / `OpenInterest` | 6 | FIX, IB, Binance, OKX, Polygon, CCXT |
| `oi` | 1 | OKX (WebSocket略称) |
| `DOI` | 1 | JPX（略称） |

#### 概念の整理

建玉残（Open Interest）は以下の意味を持つ：

| 観点 | 説明 |
|------|------|
| 定義 | 未決済（オープン）のポジション総数 |
| 単位 | 契約数（枚数） |
| 更新タイミング | 通常は日次（取引終了後に確定） |
| 用途 | 市場の流動性・関心度の指標、テクニカル分析 |

**CCXT の設計:**

CCXT では Open Interest を2つのフィールドで表現：

| フィールド | 説明 |
|-----------|------|
| `openInterestAmount` | 契約数ベースの建玉 |
| `openInterestValue` | 想定元本ベースの建玉（金額） |

本スキーマでは、より一般的な契約数ベースの `open_interest` を採用。金額ベースが必要な場合は計算で導出可能：
```
open_interest_value = open_interest × contract_value × price
```

**決定理由:**
- FIX Protocol の `OpenInterest` (Tag 746) が業界標準
- Interactive Brokers、Binance、OKX、Polygon.io 等で統一的に使用
- snake_case で `open_interest` を採用
- 略称（`oi`、`DOI`）ではなく、明示的な名称を選択

---

## 推奨フィールド定義

| フィールド名 | 型 | 必須 | 説明 | 追加先スキーマ |
|-------------|-----|------|------|---------------|
| `settlement_price` | number | 任意 | 清算値段（証拠金計算・損益計算の基準） | DerivativeInfo または新規 |
| `open_interest` | number | 任意 | 建玉残（未決済契約数） | VolumeInfo または新規 |

---

## 結果

- これらのフィールド名は [用語集](../../glossary.md) に登録する
- 既存スキーマへの追加またはデリバティブマーケットデータ用の新規スキーマを検討
- JPX先物アダプター等での使用を想定

---

## 追加検討事項

### Mark Price（マーク価格）

暗号資産デリバティブでは `mark_price` も重要なフィールドとして使用される：

| ソース | 名称 | 参照 |
|--------|------|------|
| Interactive Brokers | `Mark Price` (Tick ID 37) | [^BRK-1] |
| Binance Futures | `markPrice` | [^CRYPTO-1] |
| OKX | `markPx` | [^CRYPTO-4] |
| CCXT | `markPrice` | [^CRYPTO-6] |

**用途:** 清算（Liquidation）判定、未実現損益の計算

**決定:** 暗号資産デリバティブで必要な場合は `mark_price` として追加を検討。伝統的市場では Settlement Price が主に使用される。

---

## 参考資料

[^STD-2]: [FIX Protocol Field Tags](https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html) - Tag 730 (SettlPrice), Tag 746 (OpenInterest)
[^BRK-1]: [Interactive Brokers TWS API Tick Types](https://interactivebrokers.github.io/tws-api/tick_types.html)
[^DATA-1]: [Polygon.io Options API](https://massive.com/docs/rest/options/snapshots/option-chain-snapshot)
[^EX-4]: [CME Market Data](https://www.cmegroup.com/market-data.html)
[^CRYPTO-1]: [Binance Futures API](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api)
[^CRYPTO-4]: [OKX API](https://www.okx.com/docs-v5/en/#public-data-rest-api-get-open-interest)
[^CRYPTO-6]: [CCXT Types](https://github.com/ccxt/ccxt/blob/master/python/ccxt/base/types.py)
[^JPX-FUTURES-API]: JPX先物API仕様（jpx-client/docs/research/futures-api.md）
