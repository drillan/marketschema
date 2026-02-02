# ADR: 全商品共通フィールド名

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

金融データスキーマで使用するフィールド名を標準化する必要がある。一貫性のあるフィールド名を採用することで、開発者の認知負荷を減らし、外部システムとの統合を容易にする。

## 決定

複数の一次ソースで一致が確認された以下の標準名を採用する：

| フィールド名 | 説明 |
|-------------|------|
| `bid` | 買い気配値 |
| `ask` | 売り気配値 |
| `open` | 始値 |
| `high` | 高値 |
| `low` | 安値 |
| `close` | 終値 |
| `volume` | 出来高 |
| `price` | 価格（約定価格、現在値など） |
| `side` | 売買方向 |
| `bids` | 買い板情報の配列 |
| `asks` | 売り板情報の配列 |
| `timestamp` | タイムスタンプ |
| `size` | 約定数量 |
| `bid_size` | 買い気配の数量 |
| `ask_size` | 売り気配の数量 |

## 根拠

以下の業界標準ソースで統一されていることを確認した。
ソース番号は [情報ソース一覧](../../research/sources.md) を参照。

### 各フィールドの調査結果

#### bid / ask

- FIX Protocol: `BidPx` (Tag 132) / `OfferPx` (Tag 133) [^STD-1]
- Binance, Coinbase, IB, Yahoo Finance: `bid` / `ask` [^CRYPTO-1][^CRYPTO-2][^BRK-1][^DATA-3]
- OANDA, IG, Saxo: `bid` / `ask` [^FX-1][^FX-2][^FX-3]
- FIXは `Offer` を使用するが、現代APIでは `ask` が事実上の標準

#### open / high / low / close / volume

- 全ソースで統一 [^STD-1][^EX-1][^BRK-1][^DATA-1][^DATA-2][^FX-1][^CRYPTO-1][^CRYPTO-2][^CRYPTO-3]

#### price

- 広く統一 [^STD-1][^BRK-1][^CRYPTO-1][^CRYPTO-2]

#### side

- FIX Protocol: `Side` (Tag 54) - 数値コード（1=Buy, 2=Sell）[^STD-2]
- Binance, Coinbase, Kraken, OKX: `side` - 文字列 [^CRYPTO-1][^CRYPTO-2][^CRYPTO-3][^CRYPTO-4]
- フィールド名としては全ソースで `side` が統一されている

#### bids / asks

- Binance, Coinbase, Kraken, OKX: `bids` / `asks` [^CRYPTO-1][^CRYPTO-2][^CRYPTO-3][^CRYPTO-4]
- Bybitのみ短縮形 `b` / `a` を使用 [^CRYPTO-5]
- 板情報配列として業界標準

#### timestamp

- FIX Protocol: `TransactTime` (Tag 60) [^STD-2]
- Binance: `time` / `timestamp` [^CRYPTO-1]
- Coinbase, Kraken, Alpaca: `timestamp` [^CRYPTO-2][^CRYPTO-3][^DATA-2]
- OANDA: ISO 8601形式の `time` [^FX-1]
- フィールド名としては `timestamp` が最も一般的

#### size / bid_size / ask_size

| ソース | 約定数量 | 気配数量 | 参照 |
|--------|---------|---------|------|
| FIX Protocol | `LastQty` / `OrderQty` | `BidSize` / `OfferSize` | [^STD-2] |
| Interactive Brokers | `size` (Tick 5/6) | Tick 5 (Bid Size) / Tick 6 (Ask Size) | [^BRK-1] |
| E\*TRADE | - | `askSize` / `bidSize` | [^BRK-3] |
| Polygon.io | `size` | `bid_size` / `ask_size` | [^DATA-1] |
| Alpaca | `size` | `bid_size` / `ask_size` | [^DATA-2] |
| Coinbase | `size` | - | [^CRYPTO-2] |
| Binance | `qty` / `quantity` | `bidQty` / `askQty` | [^CRYPTO-1] |

**集計**: `size` 7ソース vs `qty` 3ソース

**決定理由**: 伝統的金融（IB, E*TRADE, Polygon, Alpaca）および米国系取引所（Coinbase）で `size` が標準。FIX Protocol でも `BidSize`/`OfferSize` が使用されている。

## 結果

- これらのフィールド名は[用語集](../../glossary.md)に登録し、プロジェクト全体で使用する
- 派生フィールド（例：`bid_price`, `open_price`）を作成する場合も、これらの標準名をベースとする
- 外部データソースからインポートする際は、これらの標準名にマッピングする

## 参考資料

詳細は [情報ソース一覧](../../research/sources.md) および [API調査レポート](../../research/api-field-names-sources.md) を参照。

[^STD-1]: [FIX Protocol](https://www.fixtrading.org/online-specification/)
[^STD-2]: [FIX Protocol Field Tags](https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html)
[^EX-1]: [JPX J-Quants API](https://jpx-jquants.com/en/spec)
[^BRK-1]: [Interactive Brokers TWS API](https://interactivebrokers.github.io/tws-api/tick_types.html)
[^BRK-3]: [E\*TRADE](https://apisb.etrade.com/docs/api/market/api-quote-v1.html)
[^DATA-1]: [Polygon.io](https://polygon.io/docs/rest/stocks/trades-quotes/quotes)
[^DATA-2]: [Alpaca](https://alpaca.markets/sdks/python/api_reference/data/models.html)
[^DATA-3]: [Yahoo Finance](https://finance.yahoo.com/)
[^FX-1]: [OANDA](https://developer.oanda.com/rest-live-v20/instrument-ep/)
[^FX-2]: [IG Group](https://labs.ig.com/rest-trading-api-reference.html)
[^FX-3]: [Saxo Bank](https://www.developer.saxo/openapi/referencedocs)
[^CRYPTO-1]: [Binance](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints)
[^CRYPTO-2]: [Coinbase](https://docs.cdp.coinbase.com/exchange/reference/)
[^CRYPTO-3]: [Kraken](https://docs.kraken.com/api/)
[^CRYPTO-4]: [OKX](https://www.okx.com/docs-v5/)
[^CRYPTO-5]: [Bybit](https://bybit-exchange.github.io/docs/)
