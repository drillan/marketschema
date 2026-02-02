# 情報ソース一覧

ADRで参照するソースの一元管理。各ADRでは同じ番号で footnote を定義する。

## カテゴリコード

| コード | カテゴリ | 説明 |
|--------|---------|------|
| STD | 標準 | 国際標準規格、業界標準 |
| EX | 取引所 | 取引所公式API |
| BRK | ブローカー | 証券会社・ブローカーAPI |
| DATA | データ | データプロバイダー |
| FX | FX | 外国為替専門 |
| CRYPTO | 暗号資産 | 暗号資産取引所・ライブラリ |

## ソース番号表

### 標準 (STD)

| ID | ソース | URL |
|----|--------|-----|
| STD-1 | FIX Protocol | https://www.fixtrading.org/online-specification/ |
| STD-2 | FIX Protocol Field Tags | https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html |
| STD-3 | OpenFIGI | https://www.openfigi.com/api/documentation |
| STD-4 | ISO 10383 MIC | https://www.iso20022.org/market-identifier-codes |
| STD-5 | ISO 6166 ISIN | https://www.anna-web.org/standards/isin-iso-6166/ |
| STD-6 | ISO 4217 通貨コード | https://www.six-group.com/en/products-services/financial-information/data-standards.html |
| STD-7 | ISO 20022 | https://www.iso20022.org/ |

### 取引所 (EX)

| ID | ソース | URL |
|----|--------|-----|
| EX-1 | JPX J-Quants API | https://jpx-jquants.com/en/spec |
| EX-2 | NYSE TAQ | https://www.nyse.com/publicdocs/nyse/data/Daily_TAQ_Client_Spec_v4.0.pdf |
| EX-3 | NASDAQ ITCH | https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf |
| EX-4 | CME Market Data API | https://www.cmegroup.com/market-data/market-data-api.html |
| EX-5 | SGX | https://www.sgx.com/data-connectivity |
| EX-6 | LSEG Developer Portal | https://developers.lseg.com/en/api-catalog |
| EX-7 | Euronext | https://www.euronext.com/en/data/how-access-market-data/web-services |

### ブローカー (BRK)

| ID | ソース | URL |
|----|--------|-----|
| BRK-1 | Interactive Brokers TWS API | https://interactivebrokers.github.io/tws-api/tick_types.html |
| BRK-2 | Charles Schwab | https://developer.schwab.com/ |
| BRK-3 | E\*TRADE | https://apisb.etrade.com/docs/api/market/api-quote-v1.html |
| BRK-4 | Webull | https://developer.webull.com/api-doc/ |
| BRK-5 | 楽天証券 RSS | https://marketspeed.jp/ms2_rss/onlinehelp/ |
| BRK-6 | Tradier | https://documentation.tradier.com/ |

### データプロバイダー (DATA)

| ID | ソース | URL |
|----|--------|-----|
| DATA-1 | Polygon.io | https://polygon.io/docs/rest/stocks/trades-quotes/quotes |
| DATA-2 | Alpaca | https://alpaca.markets/sdks/python/api_reference/data/models.html |
| DATA-3 | Yahoo Finance | https://finance.yahoo.com/ |
| DATA-4 | Alpha Vantage | https://www.alphavantage.co/ |
| DATA-5 | Finnhub | https://finnhub.io/ |
| DATA-6 | Twelve Data | https://twelvedata.com/ |
| DATA-7 | Tiingo | https://www.tiingo.com/documentation/ |
| DATA-8 | Intrinio | https://intrinio.com/ |
| DATA-9 | dxFeed | https://dxfeed.com/ |
| DATA-10 | Nasdaq Data Link | https://docs.data.nasdaq.com/ |
| DATA-11 | Bloomberg | https://www.bloomberg.com/professional/products/data/ |

### FX (FX)

| ID | ソース | URL |
|----|--------|-----|
| FX-1 | OANDA | https://developer.oanda.com/rest-live-v20/instrument-ep/ |
| FX-2 | IG Group | https://labs.ig.com/rest-trading-api-reference.html |
| FX-3 | Saxo Bank | https://www.developer.saxo/openapi/referencedocs |
| FX-4 | GMO Coin FX | https://api.coin.z.com/fxdocs/en/ |

### 暗号資産 (CRYPTO)

| ID | ソース | URL |
|----|--------|-----|
| CRYPTO-1 | Binance | https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints |
| CRYPTO-2 | Coinbase | https://docs.cdp.coinbase.com/exchange/reference/ |
| CRYPTO-3 | Kraken | https://docs.kraken.com/api/ |
| CRYPTO-4 | OKX | https://www.okx.com/docs-v5/ |
| CRYPTO-5 | Bybit | https://bybit-exchange.github.io/docs/ |
| CRYPTO-6 | CCXT | https://github.com/ccxt/ccxt |
| CRYPTO-7 | Huobi/HTX | https://www.htx.com/en-us/opend/newApiPages/ |

## 使用方法

各ADRでは、以下の形式で footnote を定義する：

```markdown
#### bid / ask
- 全ソースで統一 [^STD-1][^BRK-1][^CRYPTO-1][^CRYPTO-2]

<!-- ファイル末尾 -->
[^STD-1]: [FIX Protocol](https://www.fixtrading.org/online-specification/)
[^BRK-1]: [Interactive Brokers](https://interactivebrokers.github.io/tws-api/tick_types.html)
[^CRYPTO-1]: [Binance](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints)
[^CRYPTO-2]: [Coinbase](https://docs.cdp.coinbase.com/exchange/reference/)
```

IDは本ファイルのカテゴリ別IDと一致させること。
