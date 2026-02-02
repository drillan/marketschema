# 金融データAPIフィールド名 調査レポート

## 調査日

2026-02-02

## 調査目的

金融データスキーマで使用するフィールド名（bid, ask, open, high, low, close, volume, price, side, timestamp等）の業界標準を確認するため、主要な取引所、ブローカー、FX業者、暗号資産業者のAPIドキュメントを調査した。

---

## 1. 取引所

### 1.1 JPX J-Quants API（日本取引所グループ）

- **URL**: https://jpx-jquants.com/en/spec
- **OHLC**: `Open`, `High`, `Low`, `Close`（大文字始まり）
- **Volume**: `Volume`
- **特記**: 日本市場に特化、bid/askはリアルタイムデータで提供

### 1.2 NYSE TAQ（Trade and Quote）

- **URL**: https://www.nyse.com/market-data/historical/daily-taq
- **仕様書**: [Daily TAQ Client Spec v4.0](https://www.nyse.com/publicdocs/nyse/data/Daily_TAQ_Client_Spec_v4.0.pdf)
- **フィールド**: bid price, ask price, bid volume, ask volume, time, exchange, NBBO indicator
- **Timestamp**: ナノ秒精度（HH:MM:SS.mmmuuunnn）、ESTタイムゾーン

### 1.3 NASDAQ ITCH Protocol

- **URL**: https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf
- **フィールド**: `Price`, `shares`, buy/sell indicator
- **特記**: バイナリプロトコル、Order Book再構築用

### 1.4 CME Group

- **URL**: https://www.cmegroup.com/market-data/market-data-api.html
- **Top-of-Book**: top bid, bid size, top ask, ask size, last trade, trade volume, timestamp
- **形式**: JSON（WebSocket API）
- **配信**: 500ms conflation

### 1.5 SGX（シンガポール取引所）

- **URL**: https://www.sgx.com/data-connectivity
- **データ**: Level 1 (Best Bid/Ask), Level 2 (MBO/MBP)
- **特記**: 詳細仕様は登録後アクセス

---

## 2. 証券ブローカー

### 2.1 Interactive Brokers TWS API

- **URL**: https://interactivebrokers.github.io/tws-api/tick_types.html
- **Tick Types**:
  - Tick 1: Bid Price
  - Tick 2: Ask Price
  - Tick 4: Last Price
  - Tick 5: Bid Size
  - Tick 6: Ask Size
  - Tick 8: Volume
- **コールバック**: `tickPrice`, `tickSize`
- **Client Portal API**: Field IDs（31=last, 55=symbol, 84=bid, 86=ask）

### 2.2 Charles Schwab Trader API

- **URL**: https://developer.schwab.com/
- **Level 1**: bid, ask, last price, volume
- **Level 2**: order book (bids/asks配列)
- **形式**: WebSocket streaming
- **ライブラリ**: [schwab-py](https://schwab-py.readthedocs.io/)

### 2.3 E*TRADE API

- **URL**: https://apisb.etrade.com/docs/api/market/api-quote-v1.html
- **フィールド**: `ask`, `askSize`, `askTime`, `bid`, `bidSize`, `bidTime`
- **形式**: JSON/XML

### 2.4 Webull OpenAPI

- **URL**: https://developer.webull.com/api-doc/
- **Quote**: `asks` (配列), `bids` (配列)
- **AskBid**: `price`, `size`, `order`, `broker`
- **Snapshot**: `open`, `high`, `low`, `pre_close`, `volume`, `price`

### 2.5 Fidelity

- **特記**: 公開APIなし。Integration Xchange（機関向け）のみ
- **FIX Protocol**: 機関向けに対応

---

## 3. 日本国内証券

### 3.1 SBI証券

- **特記**: 株式の公式APIなし（先物オプションのみ2018年公開）
- **サードパーティ（BRiSK）での表記**:
  - `openPrice`, `highPrice`, `lowPrice`, `lastPrice`
  - `askPrice`, `askQuantity`, `bidPrice`, `bidQuantity`
  - `volume`, `turnover`

### 3.2 楽天証券 マーケットスピード II RSS

- **URL**: https://marketspeed.jp/ms2_rss/onlinehelp/
- **RssChart関数**: 銘柄名称, 市場名称, 足種, 日付, 時刻, **始値, 高値, 安値, 終値, 出来高**
- **形式**: Excel RSS（日本語フィールド名）

---

## 4. FX業者

### 4.1 OANDA REST v20 API

- **URL**: https://developer.oanda.com/rest-live-v20/
- **Candles**:
  - `bid`: { `o`, `h`, `l`, `c` }
  - `ask`: { `o`, `h`, `l`, `c` }
  - `mid`: { `o`, `h`, `l`, `c` }
  - `time`: ISO 8601 (例: "2016-10-17T15:00:00.000000000Z")
  - `volume`: integer
- **Pricing**: `asks` (配列), `bids` (配列), `time`, `instrument`

### 4.2 IG Group Trading API

- **URL**: https://labs.ig.com/rest-trading-api-reference.html
- **Tick Data**: `Epic`, `Time`, `Bid`, `Ask`
- **Historical**: ask/bid OHLC (Open, High, Low, Close)
- **Streaming**: Lightstreamer based

### 4.3 Saxo Bank OpenAPI

- **URL**: https://www.developer.saxo/openapi/referencedocs
- **Quote**: `Ask`, `Bid`, `Mid`, `Amount`, `LastUpdated`
- **FX OHLC**: `CloseAsk`, `CloseBid`, `OpenAsk`, `OpenBid` 等
- **PriceType**: `Tradable`, `Indicative`, `OldIndicative`

### 4.4 GMO Coin FX API

- **URL**: https://api.coin.z.com/fxdocs/en/
- **Ticker**: `ask`, `bid`, `symbol`, `timestamp`, `status`
- **形式**: JSON (REST/WebSocket)

---

## 5. 暗号資産取引所

（既存ADRで調査済み - 参照: [common.md](../adr/field-names/common.md)）

- **Binance**: `bid`, `ask`, `bids`, `asks`, `open`, `high`, `low`, `close`, `volume`, `side`
- **Coinbase**: `bid`, `ask`, `bids`, `asks`, `open`, `high`, `low`, `close`, `volume`, `side`, `size`
- **Kraken**: `bid`, `ask`, `bids`, `asks`, `open`, `high`, `low`, `close`, `volume`, `side`, `qty`

---

## 6. 調査結果サマリー

### 6.1 統一されているフィールド名

| フィールド | 統一度 | 備考 |
|-----------|--------|------|
| `bid` / `ask` | ★★★★★ | 全ソースで統一 |
| `open` / `high` / `low` / `close` | ★★★★★ | 全ソースで統一 |
| `volume` | ★★★★★ | 全ソースで統一 |
| `price` | ★★★★★ | 約定価格として統一 |
| `bids` / `asks` | ★★★★☆ | 板情報配列として広く採用（一部 `b`/`a` 短縮形） |
| `side` | ★★★★☆ | 売買方向として広く採用 |
| `timestamp` / `time` | ★★★★☆ | `timestamp` が優勢、一部 `time` |

### 6.2 分かれているフィールド名

| 概念 | 選択肢 | 採用 |
|------|--------|------|
| 約定数量 | `size` vs `qty` | `size`（[common.md](../adr/field-names/common.md) 参照） |
| 気配数量 | `bid_size` vs `bid_qty` | `bid_size` |
| 銘柄コード | `symbol` vs `ticker` | `symbol`（[instrument.md](../adr/field-names/instrument.md) 参照） |

### 6.3 フォーマット

| 項目 | 標準 | 備考 |
|------|------|------|
| Timestamp | ISO 8601 | OANDA, Saxo, IG等で採用。Binance等はUnixミリ秒 |
| Currency | ISO 4217 | 法定通貨の国際標準 |

---

## 7. 参考資料

### 取引所
- [J-Quants API Spec](https://jpx-jquants.com/en/spec)
- [NYSE Daily TAQ](https://www.nyse.com/market-data/historical/daily-taq)
- [NYSE TAQ Client Spec v4.0](https://www.nyse.com/publicdocs/nyse/data/Daily_TAQ_Client_Spec_v4.0.pdf)
- [NASDAQ TotalView-ITCH 5.0](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf)
- [CME Market Data API](https://www.cmegroup.com/market-data/market-data-api.html)
- [SGX Data Connectivity](https://www.sgx.com/data-connectivity)

### ブローカー
- [Interactive Brokers TWS API - Tick Types](https://interactivebrokers.github.io/tws-api/tick_types.html)
- [Charles Schwab Developer Portal](https://developer.schwab.com/)
- [E*TRADE Quote API](https://apisb.etrade.com/docs/api/market/api-quote-v1.html)
- [Webull OpenAPI](https://developer.webull.com/api-doc/)

### 日本国内証券
- [楽天証券 マーケットスピード II RSS](https://marketspeed.jp/ms2_rss/onlinehelp/)

### FX業者
- [OANDA REST v20 API - Instrument](https://developer.oanda.com/rest-live-v20/instrument-ep/)
- [OANDA REST v20 API - Pricing](https://developer.oanda.com/rest-live-v20/pricing-ep/)
- [IG Labs REST Trading API](https://labs.ig.com/rest-trading-api-reference.html)
- [Saxo Bank OpenAPI](https://www.developer.saxo/openapi/referencedocs)
- [GMO Coin FX API](https://api.coin.z.com/fxdocs/en/)

### 暗号資産取引所
- [Binance API](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints)
- [Coinbase API](https://docs.cdp.coinbase.com/exchange/reference/)
- [Kraken API](https://docs.kraken.com/api/)
