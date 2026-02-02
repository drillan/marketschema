# ADR: 銘柄情報フィールド名

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

銘柄情報（Instrument）スキーマで使用するフィールド名を標準化する必要がある。銘柄コード、資産クラス、通貨コード、取引所などの識別情報を定義する。

ソース番号は [情報ソース一覧](../../research/sources.md) を参照。

## 決定

| フィールド | 採用名 | 説明 |
|-----------|--------|------|
| 銘柄コード | `symbol` | 銘柄識別子 |
| 資産クラス | `asset_class` | 商品の資産分類 |
| 通貨コード | `currency` | 単一通貨（株式・債券等） |
| 取引所 | `exchange` | 上場取引所 |

## 根拠

### 1. 銘柄コード: `symbol`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `Symbol` (Tag 55) | [^STD-2] |
| Binance | `symbol` | [^CRYPTO-1] |
| Coinbase | `product_id` | [^CRYPTO-2] |
| Polygon.io | `ticker` | [^DATA-1] |
| Yahoo Finance | `symbol` | [^DATA-3] |
| OANDA | `instrument` | [^FX-1] |

**集計**: `symbol` 3ソース vs `ticker` 1ソース vs その他 2ソース

**決定理由**:
- FIX Protocol (Tag 55) で `Symbol` が国際標準
- `ticker` は米国株式市場のティッカーシンボルに由来するが、FX・暗号資産では `symbol` が一般的
- `product_id` や `instrument` は特定プラットフォーム固有の表現

### 2. 資産クラス: `asset_class`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `SecurityType` (Tag 167) | [^STD-2] |
| Interactive Brokers | `secType` | [^BRK-1] |
| Polygon.io | `asset_class` | [^DATA-1] |
| Alpaca | `asset_class` | [^DATA-2] |
| Saxo Bank | `AssetType` | [^FX-3] |
| OANDA | `type` | [^FX-1] |
| OKX | `instType` | [^CRYPTO-4] |
| CCXT | `type` | [^CRYPTO-6] |

**集計**:

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `type` | 5 | OANDA, Finnhub, Tradier, CCXT, OKX |
| `AssetType` / `assetType` | 3 | Saxo Bank, LSEG, Finnhub |
| `asset_class` | 2 | Polygon.io, Alpaca |
| `SecurityType` | 1 | FIX Protocol |
| `secType` | 1 | Interactive Brokers |

**決定理由**:
- `type` は5ソースで最多だが、汎用的すぎて他のフィールド（`order_type`, `account_type` 等）と名前が衝突しやすい
- `asset_class` は金融用語として「資産クラス」を明確に指し示し、他フィールドとの混同がない
- Polygon.io と Alpaca というモダンなRESTful API設計のデータプロバイダーが採用
- FIX Protocol の `SecurityType` は「証券タイプ」であり、`asset_class` は上位概念として適切
- snake_case は既存ADRの命名規則と一致

### 3. 通貨コード: `currency`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `Currency` (Tag 15) | [^STD-2] |
| Interactive Brokers | `currency` | [^BRK-1] |
| OKX | `ccy` | [^CRYPTO-4] |

**集計**:

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `currency` | 2 | FIX Protocol, Interactive Brokers |
| `ccy` | 1 | OKX |

**決定理由**:
- FIX Protocol (Tag 15) で国際標準として定義
- Interactive Brokers でも採用
- ISO 4217 の通貨コード（3文字）を格納するフィールドとして明確

**使用対象**: 株式・債券など単一通貨商品

### 4. 取引所: `exchange`

| ソース | 名称 | 値の形式 | 参照 |
|--------|------|---------|------|
| FIX Protocol | `SecurityExchange` (Tag 207) | ISO 10383 MIC | [^STD-2] |
| FIX Protocol | `ExDestination` (Tag 100) | ISO 10383 MIC | [^STD-2] |
| Interactive Brokers | `exchange` / `primaryExchange` | 取引所コード | [^BRK-1] |
| Polygon.io | `primary_exchange` | ISO 10383 MIC | [^DATA-1] |
| Alpaca | `exchange` | 取引所コード | [^DATA-2] |

**集計**:

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `exchange` | 3 | IB, Alpaca, FIX (ExDestination) |
| `primary_exchange` | 1 | Polygon.io |
| `SecurityExchange` | 1 | FIX Protocol |

**決定理由**:
- `exchange` が伝統的金融APIで最も一般的な名称
- Interactive Brokers、Alpaca で採用
- FIX Protocol の `SecurityExchange` (Tag 207) および `ExDestination` (Tag 100) に対応
- 暗号資産取引所は単一取引所APIのため `exchange` フィールドは不要だが、複数取引所からのデータ統合を想定

**値の形式**:
- 推奨: ISO 10383 MIC コード（例: `XNAS`, `XNYS`, `XTKS`）
- 代替: 取引所固有コード（例: `NASDAQ`, `NYSE`, `TSE`）

## 結果

- これらのフィールド名は[用語集](../../glossary.md)に追加する
- 外部データソースからインポートする際は、これらの標準名にマッピングする

## 参考資料

詳細は [情報ソース一覧](../../research/sources.md) および [API調査レポート](../../research/api-field-names-sources.md) を参照。

[^STD-2]: [FIX Protocol Field Tags](https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html)
[^BRK-1]: [Interactive Brokers TWS API](https://interactivebrokers.github.io/tws-api/tick_types.html)
[^DATA-1]: [Polygon.io](https://polygon.io/docs/rest/stocks/trades-quotes/quotes)
[^DATA-2]: [Alpaca](https://alpaca.markets/sdks/python/api_reference/data/models.html)
[^DATA-3]: [Yahoo Finance](https://finance.yahoo.com/)
[^FX-1]: [OANDA](https://developer.oanda.com/rest-live-v20/instrument-ep/)
[^FX-3]: [Saxo Bank](https://www.developer.saxo/openapi/referencedocs)
[^CRYPTO-1]: [Binance](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints)
[^CRYPTO-2]: [Coinbase](https://docs.cdp.coinbase.com/exchange/reference/)
[^CRYPTO-4]: [OKX](https://www.okx.com/docs-v5/)
[^CRYPTO-6]: [CCXT](https://github.com/ccxt/ccxt)
