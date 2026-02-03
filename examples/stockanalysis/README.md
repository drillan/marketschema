# stockanalysis.com Stock Data Adapter

stockanalysis.com から取得した HTML テーブル形式の株価データを marketschema モデルに変換するアダプターです。

## 対応データ形式

| Data Source | Model | Description |
|-------------|-------|-------------|
| HTML (`/stocks/{symbol}/history/`) | `OHLCV` | 日足株価データ |

## インストール

このアダプターは marketschema の examples として含まれています。

```bash
pip install marketschema
```

追加で `beautifulsoup4` が必要です：

```bash
pip install beautifulsoup4
```

## 使用方法

### 基本的な使用例

```python
import urllib.request

from examples.stockanalysis.adapter import StockAnalysisAdapter

# アダプターのインスタンス化
adapter = StockAnalysisAdapter()

# HTML データの取得
url = "https://stockanalysis.com/stocks/tsla/history/"
request = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
with urllib.request.urlopen(request) as resp:
    html_content = resp.read().decode("utf-8")

# HTML → OHLCV
ohlcvs = adapter.parse_html(html_content, symbol="TSLA")
for ohlcv in ohlcvs[:5]:  # 直近5件
    print(f"Date: {ohlcv.timestamp.root.date()}")
    print(f"Close: {ohlcv.close.root}")
```

### AdapterRegistry を使った使用例

```python
from marketschema.adapters.registry import AdapterRegistry
from examples.stockanalysis.adapter import StockAnalysisAdapter  # 登録のためにインポート

# レジストリからアダプターを取得
adapter = AdapterRegistry.get("stockanalysis")

# 以降は通常通り使用
ohlcvs = adapter.parse_html(html_content, symbol="TSLA")
```

### 単一行のパース

```python
from examples.stockanalysis.adapter import StockAnalysisAdapter

adapter = StockAnalysisAdapter()

# HTML テーブル行を直接パース
row = ["Feb 2, 2026", "260.03", "270.49", "259.21", "269.96", "269.96", "4.04%", "73,368,699"]
ohlcv = adapter.parse_html_row(row, symbol="TSLA")
print(f"Close: {ohlcv.close.root}")
```

## フィールドマッピング

### HTML → OHLCV

| stockanalysis HTML | marketschema | Transform |
|--------------------|--------------|-----------|
| `Date` | `timestamp` | `MMM D, YYYY` → `YYYY-MM-DDT00:00:00Z` |
| `Open` | `open` | `to_float` |
| `High` | `high` | `to_float` |
| `Low` | `low` | `to_float` |
| `Close` | `close` | `to_float` |
| `Volume` | `volume` | カンマ除去 → `to_float` |

**Note**: `Adj Close` と `Change` カラムは OHLCV モデルに含まれないため無視されます。

## デモ実行

```bash
# デフォルト (TSLA)
uv run python examples/stockanalysis/demo.py

# シンボル指定
uv run python examples/stockanalysis/demo.py aapl
uv run python examples/stockanalysis/demo.py msft
```

## 対応シンボル例

| Symbol | Description |
|--------|-------------|
| `tsla` | Tesla, Inc. |
| `aapl` | Apple Inc. |
| `msft` | Microsoft Corp. |
| `googl` | Alphabet Inc. |
| `amzn` | Amazon.com, Inc. |

## API リファレンス

- [stockanalysis.com](https://stockanalysis.com/) - 株価データ提供元

## 注意事項

- stockanalysis.com の HTML には `symbol` が含まれないため、`parse_*` メソッドで `symbol` を引数として渡す必要があります
- 日付は UTC 深夜 (00:00:00Z) として変換されます
- User-Agent ヘッダーが必要な場合があります
- API のレート制限に注意してください
- HTML 構造は変更される可能性があるため、定期的なメンテナンスが必要です
