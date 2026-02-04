# bitbank Adapter

bitbank Public API から暗号資産の市場データ（JSON 形式）を取得し、marketschema モデルに変換するアダプター。

## Usage

### ビルド

```bash
cargo build -p marketschema --example bitbank_demo
```

ビルド後のバイナリ:

```
target/debug/examples/bitbank_demo
```

### 実行

cargo run を使用:

```bash
cargo run -p marketschema --example bitbank_demo -- btc_jpy
```

または直接実行:

```bash
./target/debug/examples/bitbank_demo btc_jpy
```

通貨ペアを省略すると `btc_jpy` がデフォルトで使用される。

## API

### BitbankAdapter

主要なメソッド:

| メソッド | 説明 |
|----------|------|
| `new()` | HTTP クライアントなしでアダプターを作成 |
| `with_default_http_client()` | デフォルト HTTP クライアントで作成 |
| `with_http_client(client)` | カスタム HTTP クライアントで作成（テスト用） |
| `fetch_ticker(pair)` | Ticker データを取得し Quote を返す |
| `fetch_transactions(pair)` | 約定履歴を取得し Trade[] を返す |
| `fetch_candlestick(pair, type, date)` | ローソク足を取得し OHLCV[] を返す |
| `fetch_depth(pair)` | 板情報を取得し OrderBook を返す |

### 使用例

```rust
mod bitbank;

use bitbank::{BitbankAdapter, BitbankError};

#[tokio::main]
async fn main() -> Result<(), BitbankError> {
    let adapter = BitbankAdapter::with_default_http_client()?;

    // Quote (Ticker)
    let quote = adapter.fetch_ticker("btc_jpy").await?;
    println!("BTC/JPY Bid: {:.2}, Ask: {:.2}", quote.bid, quote.ask);

    // Trade (Transactions)
    let trades = adapter.fetch_transactions("btc_jpy").await?;
    println!("Latest trade: {} @ {}", trades[0].side, trades[0].price);

    // OHLCV (Candlestick)
    let ohlcvs = adapter.fetch_candlestick("btc_jpy", "1hour", "20240101").await?;
    for ohlcv in ohlcvs.iter().take(3) {
        println!("{}: O={} C={}", ohlcv.timestamp, ohlcv.open, ohlcv.close);
    }

    // OrderBook (Depth)
    let orderbook = adapter.fetch_depth("btc_jpy").await?;
    println!("Best ask: {}, Best bid: {}",
             orderbook.asks[0].price, orderbook.bids[0].price);

    Ok(())
}
```

## Output Models

### Quote

`fetch_ticker` メソッドは `Quote` を返す:

- `symbol`: 通貨ペア（例: "btc_jpy"）
- `timestamp`: ISO 8601 形式（例: "2024-01-01T00:00:00Z"）
- `bid`: 買い気配値（f64）
- `ask`: 売り気配値（f64）

### Trade

`fetch_transactions` メソッドは `Vec<Trade>` を返す:

- `symbol`: 通貨ペア
- `timestamp`: ISO 8601 形式
- `price`: 約定価格（f64）
- `size`: 約定数量（f64）
- `side`: 売買方向（"buy" or "sell"）

### Ohlcv

`fetch_candlestick` メソッドは `Vec<Ohlcv>` を返す:

- `symbol`: 通貨ペア
- `timestamp`: ISO 8601 形式
- `open`, `high`, `low`, `close`: 価格（f64）
- `volume`: 出来高（f64）

### OrderBook

`fetch_depth` メソッドは `OrderBook` を返す:

- `symbol`: 通貨ペア
- `timestamp`: ISO 8601 形式
- `asks`: 売り注文リスト（`Vec<PriceLevel>`）
- `bids`: 買い注文リスト（`Vec<PriceLevel>`）

各 `PriceLevel` は `price` と `size` を持つ。

## Supported Pairs

bitbank で取引可能な通貨ペア:

| ペア | 説明 |
|------|------|
| `btc_jpy` | ビットコイン/日本円 |
| `eth_jpy` | イーサリアム/日本円 |
| `xrp_jpy` | リップル/日本円 |
| `ltc_jpy` | ライトコイン/日本円 |
| `bcc_jpy` | ビットコインキャッシュ/日本円 |
| `mona_jpy` | モナコイン/日本円 |

その他のペアは [bitbank API ドキュメント](https://github.com/bitbankinc/bitbank-api-docs) を参照。

## Candlestick Types

`fetch_candlestick` で使用可能な期間:

| タイプ | 説明 |
|--------|------|
| `1min` | 1分足 |
| `5min` | 5分足 |
| `15min` | 15分足 |
| `30min` | 30分足 |
| `1hour` | 1時間足 |
| `4hour` | 4時間足 |
| `8hour` | 8時間足 |
| `12hour` | 12時間足 |
| `1day` | 日足 |
| `1week` | 週足 |
| `1month` | 月足 |

日付は `YYYYMMDD` 形式で指定（例: `"20240101"`）。

## Error Handling

`BitbankError` で定義されるエラー:

| エラー | 説明 |
|--------|------|
| `HttpClientNotConfigured` | HTTP クライアント未設定 |
| `Http` | HTTP リクエスト失敗 |
| `ApiError` | bitbank API がエラーを返した（success != 1） |
| `MissingField` | 必須フィールドが存在しない |
| `InsufficientArrayLength` | 配列の要素数が不足 |
| `Conversion` | 数値変換失敗 |
