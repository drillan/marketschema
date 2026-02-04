# StockAnalysis Adapter

stockanalysis.com から米国株式の履歴データ（HTML テーブル形式）をスクレイピングし、marketschema の OHLCV / ExtendedOHLCV モデルに変換するアダプター。

## Usage

### ビルド

```bash
cargo build -p marketschema --example stockanalysis_demo
```

ビルド後のバイナリ:

```
target/debug/examples/stockanalysis_demo
```

### 実行

cargo run を使用:

```bash
cargo run -p marketschema --example stockanalysis_demo -- tsla
```

または直接実行:

```bash
./target/debug/examples/stockanalysis_demo tsla
```

シンボルを省略すると `tsla` がデフォルトで使用される。

## API

### StockAnalysisAdapter

主要なメソッド:

| メソッド | 説明 |
|----------|------|
| `new()` | HTTP クライアントなしでアダプターを作成 |
| `with_default_http_client()` | デフォルト HTTP クライアントで作成 |
| `with_http_client(client)` | カスタム HTTP クライアントで作成（テスト用） |
| `fetch_and_parse(symbol)` | データ取得と標準 OHLCV パースを一括実行 |
| `fetch_and_parse_extended(symbol)` | データ取得と ExtendedOhlcv パースを一括実行 |
| `fetch_history(symbol)` | HTML データのみ取得 |
| `parse_html(content, symbol)` | HTML 文字列を標準 OHLCV にパース |
| `parse_html_extended(content, symbol)` | HTML 文字列を ExtendedOhlcv にパース |
| `parse_date(date_str)` | "Feb 2, 2026" 形式を ISO 8601 に変換 |
| `parse_volume(volume_str)` | カンマ区切りの出来高を変換 |

### 使用例

```rust
mod stockanalysis;

use stockanalysis::{StockAnalysisAdapter, StockAnalysisError};

#[tokio::main]
async fn main() -> Result<(), StockAnalysisError> {
    let adapter = StockAnalysisAdapter::with_default_http_client()?;

    // 調整後終値を含む ExtendedOhlcv を取得
    let ohlcv_data = adapter.fetch_and_parse_extended("tsla").await?;

    for record in ohlcv_data.iter().take(5) {
        println!(
            "{}: O={:.2} H={:.2} L={:.2} C={:.2} AC={:.2} V={}",
            record.timestamp,
            record.open,
            record.high,
            record.low,
            record.close,
            record.adj_close,
            record.volume
        );
    }
    Ok(())
}
```

## Output

### Ohlcv

`fetch_and_parse` / `parse_html` メソッドは `Vec<Ohlcv>` を返す。各 `Ohlcv` レコードは以下のフィールドを持つ:

- `symbol`: 銘柄シンボル（例: "TSLA"）
- `timestamp`: ISO 8601 形式（例: "2026-02-02T00:00:00Z"）
- `open`, `high`, `low`, `close`: 価格（f64）
- `volume`: 出来高（f64）

### ExtendedOhlcv

`fetch_and_parse_extended` / `parse_html_extended` メソッドは `Vec<ExtendedOhlcv>` を返す。`Ohlcv` に加えて:

- `adj_close`: 調整後終値（f64）- 株式分割・配当調整済み

## Symbol Format

stockanalysis.com のシンボル形式:

| 種類 | 形式 | 例 |
|------|------|------|
| 米国株 | `{ticker}` | `tsla`, `aapl`, `msft` |

大文字・小文字は区別されない（内部で小文字に変換される）。

## HTML Table Format

stockanalysis.com の履歴データテーブル構造:

| Date | Open | High | Low | Close | Adj Close | Change | Volume |
|------|------|------|-----|-------|-----------|--------|--------|
| Feb 2, 2026 | 260.03 | 270.49 | 259.21 | 269.96 | 269.96 | 4.04% | 73,368,699 |

- **Date**: "MMM D, YYYY" 形式（例: "Feb 2, 2026"）
- **Volume**: カンマ区切り（例: "73,368,699"）
- **Change**: パーセント値（パース対象外）

## Error Handling

`StockAnalysisError` で定義されるエラー:

| エラー | 説明 |
|--------|------|
| `HttpClientNotConfigured` | HTTP クライアント未設定 |
| `Http` | HTTP リクエスト失敗 |
| `EmptyHtml` | HTML が空 |
| `NoTableFound` | HTML 内にテーブルが見つからない |
| `TableStructureError` | テーブル構造が不正（tbody なし等） |
| `InvalidDateFormat` | 日付形式が不正（"MMM D, YYYY" 以外） |
| `InvalidMonth` | 月の略称が不正（"Jan"〜"Dec" 以外） |
| `InsufficientColumns` | テーブル列数が不足（8 列必要） |
| `EmptyVolume` | 出来高が空文字 |
| `Conversion` | 数値変換失敗 |

## Notes

- stockanalysis.com はブラウザ検証が必要な場合があるため、カスタム User-Agent ヘッダーを使用
- HTML 構造が変更された場合、パースが失敗する可能性がある
- 大量のリクエストを送信するとレート制限される可能性がある
