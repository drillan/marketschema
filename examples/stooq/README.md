# stooq.com Stock Data Adapter

stooq.com から取得した CSV 形式の株価データを marketschema モデルに変換するアダプターです。

## 対応データ形式

| Data Source | Model | Description |
|-------------|-------|-------------|
| CSV (`/q/d/l/?s={symbol}&i=d`) | `OHLCV` | 日足株価データ |

## インストール

このアダプターは marketschema の examples として含まれています。

```bash
pip install marketschema
```

## 使用方法

### 基本的な使用例

```python
import urllib.request

from examples.stooq.adapter import StooqAdapter

# アダプターのインスタンス化
adapter = StooqAdapter()

# CSV データの取得
url = "https://stooq.com/q/d/l/?s=spy.us&i=d"
with urllib.request.urlopen(url) as resp:
    csv_content = resp.read().decode("utf-8")

# CSV → OHLCV
ohlcvs = adapter.parse_csv(csv_content, symbol="spy.us")
for ohlcv in ohlcvs[-5:]:  # 直近5件
    print(f"Date: {ohlcv.timestamp.root.date()}")
    print(f"Close: {ohlcv.close.root}")
```

### AdapterRegistry を使った使用例

```python
from marketschema.adapters.registry import AdapterRegistry
from examples.stooq.adapter import StooqAdapter  # 登録のためにインポート

# レジストリからアダプターを取得
adapter = AdapterRegistry.get("stooq")

# 以降は通常通り使用
ohlcvs = adapter.parse_csv(csv_content, symbol="spy.us")
```

### 単一行のパース

```python
from examples.stooq.adapter import StooqAdapter

adapter = StooqAdapter()

# CSV 行を直接パース
row = ["2025-01-15", "100.50", "105.25", "99.75", "104.00", "1234567"]
ohlcv = adapter.parse_csv_row(row, symbol="spy.us")
print(f"Close: {ohlcv.close.root}")
```

## フィールドマッピング

### CSV → OHLCV

| stooq CSV | marketschema | Transform |
|-----------|--------------|-----------|
| `Date` | `timestamp` | `YYYY-MM-DD` → `YYYY-MM-DDT00:00:00Z` |
| `Open` | `open` | `to_float` |
| `High` | `high` | `to_float` |
| `Low` | `low` | `to_float` |
| `Close` | `close` | `to_float` |
| `Volume` | `volume` | `to_float` |

## デモ実行

```bash
# デフォルト (SPY ETF)
uv run python examples/stooq/demo.py

# シンボル指定
uv run python examples/stooq/demo.py aapl.us
uv run python examples/stooq/demo.py ^spx
```

## 対応シンボル例

| Symbol | Description |
|--------|-------------|
| `spy.us` | SPDR S&P 500 ETF |
| `aapl.us` | Apple Inc. |
| `msft.us` | Microsoft Corp. |
| `^spx` | S&P 500 Index |
| `^dji` | Dow Jones Industrial Average |

## API リファレンス

- [stooq.com](https://stooq.com/) - 株価データ提供元

## 注意事項

- stooq.com の CSV には `symbol` が含まれないため、`parse_*` メソッドで `symbol` を引数として渡す必要があります
- 日付は UTC 深夜 (00:00:00Z) として変換されます
- API のレート制限に注意してください
