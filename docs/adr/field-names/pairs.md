# ADR: ペア商品フィールド名

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

FX・暗号資産などのペア商品で使用するフィールド名を標準化する必要がある。これらの商品は2つの通貨/資産のペアで取引され、基軸通貨と決済通貨を区別する必要がある。

ソース番号は [情報ソース一覧](../../research/sources.md) を参照。

## 決定

| フィールド | 採用名 | 説明 |
|-----------|--------|------|
| 基軸通貨 | `base_currency` | ペア商品の基軸通貨（FX・暗号資産） |
| 決済通貨 | `quote_currency` | ペア商品の決済通貨（FX・暗号資産） |
| 売買代金 | `quote_volume` | 決済通貨建ての出来高 |

## 根拠

### 1. 通貨ペア: `base_currency` / `quote_currency`

| ソース | 基軸/決済 | 参照 |
|--------|---------|------|
| FIX Protocol | `Currency` (Tag 15) / `SettlCurrency` (Tag 120) | [^STD-2] |
| Binance | `baseAsset` / `quoteAsset` | [^CRYPTO-1] |
| Kraken | `base` / `quote` | [^CRYPTO-3] |
| OKX | `baseCcy` / `quoteCcy` | [^CRYPTO-4] |
| CCXT | `base` / `quote` | [^CRYPTO-6] |

**集計**:

| パターン | 使用ソース数 | 主なソース |
|---------|-------------|-----------|
| `base` / `quote` | 2 | CCXT, Kraken |
| `baseAsset` / `quoteAsset` | 1 | Binance |
| `baseCcy` / `quoteCcy` | 1 | OKX |

**決定理由**:
- `base` / `quote` だけでは曖昧（何の base か不明）
- `baseAsset` は「資産」を指し、通貨コード（文字列）と混同しやすい
- `base_currency` / `quote_currency` は意味が明確
- FIX の `SettlCurrency` より直感的で、決済通貨の概念を正確に表現
- snake_case は既存ADRの命名規則と一致
- CCXT の `base` / `quote` を明確化した形式

### 2. 売買代金: `quote_volume`

| ソース | 名称 | 参照 |
|--------|------|------|
| 楽天証券 | 売買代金 | [^BRK-5] |
| Binance | `quoteVolume` | [^CRYPTO-1] |
| Bybit | `turnover` | [^CRYPTO-5] |
| OKX | `volCcy24h` | [^CRYPTO-4] |
| CCXT | `quoteVolume`（標準化名）| [^CRYPTO-6] |

**決定理由**:
- `quote_volume` は CCXT（暗号資産統一ライブラリ）の標準化名
- Binance（最大取引量の暗号資産取引所）で採用
- `quote_volume` は「決済通貨（quote currency）建て出来高」という意味が明確
- `turnover` は伝統的金融で使用されるが、「回転率」（turnover ratio）と混同される可能性がある
- `volume` が基軸通貨建て、`quote_volume` が決済通貨建てという区別が明確

## 商品タイプ別の使用指針

| 商品タイプ | 使用フィールド | 例 |
|-----------|--------------|-----|
| 株式・債券 | `currency` | `currency: "JPY"` |
| FX | `base_currency` / `quote_currency` | `base_currency: "EUR"`, `quote_currency: "USD"` |
| 暗号資産 | `base_currency` / `quote_currency` | `base_currency: "BTC"`, `quote_currency: "USDT"` |

## 結果

- これらのフィールド名は[用語集](../../glossary.md)に追加する
- 外部データソースからインポートする際は、これらの標準名にマッピングする

## 参考資料

詳細は [情報ソース一覧](../../research/sources.md) および [API調査レポート](../../research/api-field-names-sources.md) を参照。

[^STD-2]: [FIX Protocol Field Tags](https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html)
[^BRK-5]: [楽天証券 RSS](https://marketspeed.jp/ms2_rss/onlinehelp/)
[^CRYPTO-1]: [Binance](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints)
[^CRYPTO-3]: [Kraken](https://docs.kraken.com/api/)
[^CRYPTO-4]: [OKX](https://www.okx.com/docs-v5/)
[^CRYPTO-5]: [Bybit](https://bybit-exchange.github.io/docs/)
[^CRYPTO-6]: [CCXT](https://github.com/ccxt/ccxt)
