# bitbank Public API Adapter

bitbank の Public API から取得したデータを marketschema モデルに変換するアダプターです。

## 対応エンドポイント

| Endpoint | Model | Description |
|----------|-------|-------------|
| `/{pair}/ticker` | `Quote` | 最良気配値 (BBO) |
| `/{pair}/transactions` | `Trade` | 約定履歴 |
| `/{pair}/candlestick/{type}/{date}` | `OHLCV` | ローソク足 |
| `/{pair}/depth` | `OrderBook` | 板情報 |

## インストール

このアダプターは marketschema の examples として含まれています。

```bash
pip install marketschema
```

## 使用方法

### 基本的な使用例

```python
import json
import urllib.request

from examples.bitbank.adapter import BitbankAdapter

# アダプターのインスタンス化
adapter = BitbankAdapter()

# Ticker → Quote
with urllib.request.urlopen("https://public.bitbank.cc/btc_jpy/ticker") as resp:
    data = json.loads(resp.read())["data"]
    quote = adapter.parse_quote(data, symbol="btc_jpy")
    print(f"Bid: {quote.bid.root}, Ask: {quote.ask.root}")

# Transactions → Trade
with urllib.request.urlopen("https://public.bitbank.cc/btc_jpy/transactions") as resp:
    data = json.loads(resp.read())["data"]
    trades = adapter.parse_trades(data["transactions"], symbol="btc_jpy")
    for trade in trades[:5]:
        print(f"Price: {trade.price.root}, Size: {trade.size.root}")

# Depth → OrderBook
with urllib.request.urlopen("https://public.bitbank.cc/btc_jpy/depth") as resp:
    data = json.loads(resp.read())["data"]
    orderbook = adapter.parse_orderbook(data, symbol="btc_jpy")
    print(f"Best bid: {orderbook.bids[0].price.root}")
    print(f"Best ask: {orderbook.asks[0].price.root}")
```

### AdapterRegistry を使った使用例

```python
from marketschema.adapters.registry import AdapterRegistry
from examples.bitbank.adapter import BitbankAdapter  # 登録のためにインポート

# レジストリからアダプターを取得
adapter = AdapterRegistry.get("bitbank")

# 以降は通常通り使用
quote = adapter.parse_quote(ticker_data, symbol="btc_jpy")
```

## フィールドマッピング

### Ticker → Quote

| bitbank | marketschema | Transform |
|---------|--------------|-----------|
| `buy` | `bid` | `to_float` |
| `sell` | `ask` | `to_float` |
| `timestamp` | `timestamp` | `unix_timestamp_ms` |

### Transaction → Trade

| bitbank | marketschema | Transform |
|---------|--------------|-----------|
| `price` | `price` | `to_float` |
| `amount` | `size` | `to_float` |
| `side` | `side` | `side_from_string` |
| `executed_at` | `timestamp` | `unix_timestamp_ms` |

### Candlestick → OHLCV

bitbank の candlestick は配列形式 `[open, high, low, close, volume, timestamp]` で返されます。

| Index | marketschema | Transform |
|-------|--------------|-----------|
| 0 | `open` | `to_float` |
| 1 | `high` | `to_float` |
| 2 | `low` | `to_float` |
| 3 | `close` | `to_float` |
| 4 | `volume` | `to_float` |
| 5 | `timestamp` | `unix_timestamp_ms` |

### Depth → OrderBook

| bitbank | marketschema | Transform |
|---------|--------------|-----------|
| `asks` | `asks` | `[[price, size], ...]` → `list[PriceLevel]` |
| `bids` | `bids` | `[[price, size], ...]` → `list[PriceLevel]` |
| `timestamp` | `timestamp` | `unix_timestamp_ms` |

## デモ実行

```bash
uv run python examples/bitbank/demo.py
```

## API リファレンス

- [bitbank API Documentation](https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md)

## 注意事項

- bitbank API レスポンスには `symbol` が含まれないため、`parse_*` メソッドで `symbol` を引数として渡す必要があります
- API レスポンスの `success` フィールドのチェックは呼び出し側で行ってください。アダプターは `data` フィールドの内容のみを扱います
