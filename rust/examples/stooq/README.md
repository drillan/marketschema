# Stooq Adapter

stooq.com から株価データ（CSV 形式）を取得し、marketschema の OHLCV モデルに変換するアダプター。

## Usage

### ビルド

```bash
cargo build -p marketschema --example stooq_demo
```

ビルド後のバイナリ:

```
target/debug/examples/stooq_demo
```

### 実行

cargo run を使用:

```bash
cargo run -p marketschema --example stooq_demo -- spy.us
```

または直接実行:

```bash
./target/debug/examples/stooq_demo spy.us
```

シンボルを省略すると `spy.us` がデフォルトで使用される。

## API

### StooqAdapter

主要なメソッド:

| メソッド | 説明 |
|----------|------|
| `new()` | HTTP クライアントなしでアダプターを作成 |
| `with_default_http_client()` | デフォルト HTTP クライアントで作成 |
| `with_http_client(client)` | カスタム HTTP クライアントで作成（テスト用） |
| `fetch_and_parse(symbol)` | データ取得とパースを一括実行 |
| `fetch_csv(symbol)` | CSV データのみ取得 |
| `parse_csv(content, symbol)` | CSV 文字列をパース |

### 使用例

```rust
use stooq::{StooqAdapter, StooqError};

#[tokio::main]
async fn main() -> Result<(), StooqError> {
    let adapter = StooqAdapter::with_default_http_client()?;
    let ohlcv_data = adapter.fetch_and_parse("spy.us").await?;

    for record in ohlcv_data.iter().rev().take(5) {
        println!(
            "{}: O={:.2} H={:.2} L={:.2} C={:.2} V={}",
            record.timestamp,
            record.open,
            record.high,
            record.low,
            record.close,
            record.volume
        );
    }
    Ok(())
}
```

## Data Structure

### Ohlcv

| フィールド | 型 | 説明 |
|------------|------|------|
| `symbol` | `String` | 銘柄シンボル（例: "spy.us"） |
| `timestamp` | `String` | ISO 8601 形式（例: "2024-01-15T00:00:00Z"） |
| `open` | `f64` | 始値 |
| `high` | `f64` | 高値 |
| `low` | `f64` | 安値 |
| `close` | `f64` | 終値 |
| `volume` | `f64` | 出来高 |

## Symbol Format

stooq.com のシンボル形式:

| 種類 | 形式 | 例 |
|------|------|------|
| 米国株 | `{ticker}.us` | `spy.us`, `aapl.us` |
| 日本株 | `{code}.jp` | `7203.jp`（トヨタ） |
| 指数 | `^{symbol}` | `^spx`, `^dji` |

## Error Handling

`StooqError` で定義されるエラー:

| エラー | 説明 |
|--------|------|
| `HttpClientNotConfigured` | HTTP クライアント未設定 |
| `Http` | HTTP リクエスト失敗 |
| `EmptyCsv` | CSV が空（ヘッダーなし） |
| `InvalidHeader` | CSV ヘッダーが期待と不一致 |
| `InvalidDateFormat` | 日付形式が不正 |
| `InsufficientColumns` | CSV 列数が不足 |
| `Conversion` | 数値変換失敗 |
| `CsvParse` | CSV パースエラー |
