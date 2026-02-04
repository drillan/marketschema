(python-models-guide)=
# Python モデル実装ガイド

marketschema の pydantic モデルを使用してマーケットデータを扱う方法を解説する。

## 概要

marketschema は金融マーケットデータを統一的に扱うための pydantic モデルを提供する。JSON Schema から自動生成されたモデルを使用することで、異なるデータソースからのデータを一貫した形式で扱える。

このガイドの対象読者:

- マーケットデータを扱うアプリケーションの開発者
- 複数の取引所やデータプロバイダを統合するシステムの開発者
- 型安全なデータ処理を求める開発者

## クイックスタート

### インストール

```bash
pip install marketschema
```

### 最初のモデル作成

気配値（Quote）モデルを作成する例:

```python
from datetime import datetime, timezone
from marketschema.models import Quote, Symbol, Timestamp, Price, Size

quote = Quote(
    symbol=Symbol("AAPL"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    bid=Price(150.00),
    ask=Price(150.05),
    bid_size=Size(100.0),
    ask_size=Size(200.0),
)

print(quote.model_dump_json(indent=2))
```

出力:

```json
{
  "symbol": "AAPL",
  "timestamp": "2025-01-15T10:30:00Z",
  "bid": 150.0,
  "ask": 150.05,
  "bid_size": 100.0,
  "ask_size": 200.0
}
```

## マーケットデータモデル

### Quote（気配値）

最良気配値（BBO: Best Bid/Offer）を表現する。

```python
from datetime import datetime, timezone
from marketschema.models import Quote, Symbol, Timestamp, Price, Size

# 必須フィールドのみ
quote = Quote(
    symbol=Symbol("BTC-USD"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    bid=Price(42000.00),
    ask=Price(42001.50),
)

# オプションフィールドを含む
quote_with_size = Quote(
    symbol=Symbol("BTC-USD"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    bid=Price(42000.00),
    ask=Price(42001.50),
    bid_size=Size(1.5),
    ask_size=Size(2.0),
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `timestamp` | Timestamp | Yes | 気配値取得時刻 |
| `bid` | Price | Yes | 買い気配値 |
| `ask` | Price | Yes | 売り気配値 |
| `bid_size` | Size | No | 買い気配の数量 |
| `ask_size` | Size | No | 売り気配の数量 |

### OHLCV（ローソク足）

ローソク足データ（始値、高値、安値、終値、出来高）を表現する。

```python
from datetime import datetime, timezone
from marketschema.models import OHLCV, Symbol, Timestamp, Price, Size

ohlcv = OHLCV(
    symbol=Symbol("ETH-USD"),
    timestamp=Timestamp(datetime(2025, 1, 15, 0, 0, 0, tzinfo=timezone.utc)),
    open=Price(2500.00),
    high=Price(2550.00),
    low=Price(2480.00),
    close=Price(2530.00),
    volume=Size(10000.0),
    quote_volume=Size(25000000.0),
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `timestamp` | Timestamp | Yes | 足の開始時刻 |
| `open` | Price | Yes | 始値 |
| `high` | Price | Yes | 高値 |
| `low` | Price | Yes | 安値 |
| `close` | Price | Yes | 終値 |
| `volume` | Size | Yes | 出来高 |
| `quote_volume` | Size | No | 売買代金（決済通貨建て） |

### Trade（約定）

個別約定（歩み値 / Time & Sales）を表現する。

```python
from datetime import datetime, timezone
from marketschema.models import Trade, Symbol, Timestamp, Price, Size, Side

trade = Trade(
    symbol=Symbol("AAPL"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    price=Price(150.25),
    size=Size(100.0),
    side=Side.buy,
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `timestamp` | Timestamp | Yes | 約定時刻 |
| `price` | Price | Yes | 約定価格 |
| `size` | Size | Yes | 約定数量 |
| `side` | Side | Yes | 売買方向（buy/sell） |

### OrderBook（板情報）

複数レベルの板情報を表現する。

```python
from datetime import datetime, timezone
from marketschema.models import OrderBook, Symbol, Timestamp, PriceLevel, Price, Size

orderbook = OrderBook(
    symbol=Symbol("BTC-USD"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    bids=[
        PriceLevel(price=Price(42000.0), size=Size(1.5)),
        PriceLevel(price=Price(41999.0), size=Size(2.0)),
        PriceLevel(price=Price(41998.0), size=Size(3.5)),
    ],
    asks=[
        PriceLevel(price=Price(42001.0), size=Size(1.0)),
        PriceLevel(price=Price(42002.0), size=Size(2.5)),
        PriceLevel(price=Price(42003.0), size=Size(4.0)),
    ],
)

# 最良気配を取得
best_bid = orderbook.bids[0]
best_ask = orderbook.asks[0]
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `timestamp` | Timestamp | Yes | 板情報取得時刻 |
| `bids` | list[PriceLevel] | Yes | 買い板（価格降順） |
| `asks` | list[PriceLevel] | Yes | 売り板（価格昇順） |

### VolumeInfo（出来高・売買代金）

出来高と売買代金を表現する。

```python
from datetime import datetime, timezone
from marketschema.models import VolumeInfo, Symbol, Timestamp, Size

volume_info = VolumeInfo(
    symbol=Symbol("AAPL"),
    timestamp=Timestamp(datetime.now(timezone.utc)),
    volume=Size(1000000.0),
    quote_volume=Size(150000000.0),
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `timestamp` | Timestamp | Yes | データ取得時刻 |
| `volume` | Size | Yes | 出来高（数量ベース） |
| `quote_volume` | Size | No | 売買代金（決済通貨建て） |

## 銘柄情報モデル

### Instrument（銘柄情報）

銘柄識別情報を表現する。

```python
from marketschema.models import Instrument, Symbol, AssetClass, Currency, Exchange

# 株式
stock = Instrument(
    symbol=Symbol("AAPL"),
    asset_class=AssetClass.equity,
    currency=Currency("USD"),
    exchange=Exchange("XNAS"),
)

# 暗号資産ペア
crypto = Instrument(
    symbol=Symbol("BTC-USD"),
    asset_class=AssetClass.crypto,
    base_currency=Currency("BTC"),
    quote_currency=Currency("USD"),
)

# FX ペア
fx = Instrument(
    symbol=Symbol("USD/JPY"),
    asset_class=AssetClass.fx,
    base_currency=Currency("USD"),
    quote_currency=Currency("JPY"),
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | Symbol | Yes | 銘柄識別子 |
| `asset_class` | AssetClass | Yes | 資産クラス |
| `currency` | Currency | No | 単一通貨（株式・債券等） |
| `exchange` | Exchange | No | 上場取引所（ISO 10383 MIC） |
| `base_currency` | Currency | No | 基軸通貨（FX・暗号資産） |
| `quote_currency` | Currency | No | 決済通貨（FX・暗号資産） |

### DerivativeInfo（デリバティブ共通）

先物・オプション共通の情報を表現する。

```python
from marketschema.models import (
    DerivativeInfo,
    Symbol,
    Currency,
    UnderlyingType,
    SettlementMethod,
)

# 必須フィールドのみ
derivative = DerivativeInfo(
    multiplier=100.0,
    tick_size=0.01,
    underlying_symbol=Symbol("SPX"),
    underlying_type=UnderlyingType.index_,  # Python では index_ を使用
)

# オプションフィールドを含む
derivative_full = DerivativeInfo(
    multiplier=100.0,
    tick_size=0.01,
    tick_value=1.0,  # オプション
    contract_value=100.0,  # オプション
    contract_value_currency=Currency("USD"),  # オプション
    lot_size=1.0,  # オプション
    min_order_size=1.0,  # オプション
    max_order_size=1000.0,  # オプション
    underlying_symbol=Symbol("SPX"),
    underlying_type=UnderlyingType.index_,
    is_perpetual=False,  # オプション
    is_inverse=False,  # オプション
    settlement_method=SettlementMethod.cash,  # オプション
    settlement_currency=Currency("USD"),  # オプション
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `multiplier` | float | Yes | 契約乗数（1契約あたりの乗数） |
| `tick_size` | float | Yes | 呼値単位（最小価格変動） |
| `underlying_symbol` | Symbol | Yes | 原資産のシンボル |
| `underlying_type` | UnderlyingType | Yes | 原資産タイプ |
| `tick_value` | float | No | ティック価値（1ティックあたりの金額変動） |
| `contract_value` | float | No | 契約基本価値 |
| `contract_value_currency` | Currency | No | 契約価値の通貨 |
| `lot_size` | float | No | 取引単位（注文可能な最小数量単位） |
| `min_order_size` | float | No | 最小注文数量 |
| `max_order_size` | float | No | 最大注文数量 |
| `is_perpetual` | bool | No | 無期限契約か否か（暗号資産デリバティブ向け） |
| `is_inverse` | bool | No | インバース契約か否か（暗号資産デリバティブ向け） |
| `settlement_method` | SettlementMethod | No | 決済方法 |
| `settlement_currency` | Currency | No | 決済通貨 |

### ExpiryInfo（満期情報）

先物・オプションの満期関連情報を表現する。

```python
from marketschema.models import ExpiryInfo
from marketschema.models.definitions import Date, ExpirySeries

# 必須フィールドのみ
expiry_minimal = ExpiryInfo(
    expiration_date=Date("2025-03-21"),
)

# オプションフィールドを含む
expiry_full = ExpiryInfo(
    expiry=ExpirySeries("2025-03"),  # オプション
    last_trading_day=Date("2025-03-20"),  # オプション
    expiration_date=Date("2025-03-21"),
    settlement_date=Date("2025-03-21"),  # オプション
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `expiration_date` | Date | Yes | 満期日/SQ日 |
| `expiry` | ExpirySeries | No | 満期系列識別子（YYYY-MM, YYYY-Www, YYYY-MM-DD形式） |
| `last_trading_day` | Date | No | 取引可能な最終日 |
| `settlement_date` | Date | No | 決済日 |

### OptionInfo（オプション）

オプション固有の情報を表現する。

```python
from marketschema.models import OptionInfo, Price, OptionType, ExerciseStyle

# 必須フィールドのみ
option = OptionInfo(
    strike_price=Price(5000.0),
    option_type=OptionType.call,
)

# オプションフィールドを含む
option_with_style = OptionInfo(
    strike_price=Price(5000.0),
    option_type=OptionType.call,
    exercise_style=ExerciseStyle.european,  # オプション
)
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `strike_price` | Price | Yes | 権利行使価格 |
| `option_type` | OptionType | Yes | オプションタイプ（call/put） |
| `exercise_style` | ExerciseStyle | No | 行使スタイル（american/european/bermudan） |

## 共通型定義

marketschema では、フィールドの意味を明確にするために専用の型を定義している。

### 基本型

| 型 | 説明 | 例 |
|----|------|-----|
| `Timestamp` | ISO 8601形式のタイムスタンプ (UTC) | `Timestamp(datetime.now(timezone.utc))` |
| `Symbol` | 銘柄識別子（1文字以上の文字列） | `Symbol("AAPL")` |
| `Price` | 価格 | `Price(150.00)` |
| `Size` | 数量 | `Size(100.0)` |

### 列挙型

| 型 | 説明 | 値 |
|----|------|-----|
| `Side` | 売買方向 | `buy`, `sell` |
| `AssetClass` | 資産クラス | `equity`, `fund`, `bond`, `future`, `option`, `fx`, `crypto`, `cfd` |
| `OptionType` | オプションタイプ | `call`, `put` |
| `ExerciseStyle` | 行使スタイル | `american`, `european`, `bermudan` |
| `SettlementMethod` | 決済方法 | `cash`, `physical` |
| `UnderlyingType` | 原資産タイプ | `stock`, `index_`*, `etf`, `commodity`, `currency`, `crypto` |

\* Python では予約語 `index` との衝突を避けるため `UnderlyingType.index_` を使用。シリアライズ時は `"index"` となる。

### 文字列パターン型

| 型 | 説明 | パターン | 例 |
|----|------|---------|-----|
| `Currency` | ISO 4217通貨コード | `^[A-Z]{3}$` | `Currency("USD")`, `Currency("JPY")` |
| `Exchange` | ISO 10383市場識別コード | `^[A-Z]{4}$` | `Exchange("XNYS")`, `Exchange("XJPX")` |
| `Date` | 日付 | `^\\d{4}-\\d{2}-\\d{2}$` | `Date("2025-03-21")` |
| `ExpirySeries` | 満期系列識別子 | `^\\d{4}(-\\d{2}\|-W\\d{2}\|-\\d{2}-\\d{2})$` | `ExpirySeries("2025-03")` |

### PriceLevel

板情報の気配レベルを表現する。

```python
from marketschema.models import PriceLevel, Price, Size

level = PriceLevel(price=Price(100.0), size=Size(50.0))
```

## バリデーション

marketschema のモデルは pydantic による自動バリデーションを提供する。

### バリデーションエラーの例

```python
from pydantic import ValidationError
from marketschema.models import Quote, Symbol, Timestamp, Price

try:
    # timestamp が欠落
    quote = Quote(
        symbol=Symbol("AAPL"),
        bid=Price(150.00),
        ask=Price(150.05),
    )
except ValidationError as e:
    print(e)
```

出力:

```text
1 validation error for Quote
timestamp
  Field required [type=missing, input_value={'symbol': 'AAPL', 'bid': 150.0, 'ask': 150.05}, input_type=dict]
```

### 追加フィールドの禁止

すべてのモデルは `extra="forbid"` で設定されており、スキーマに定義されていないフィールドは拒否される。

```python
from datetime import datetime, timezone
from pydantic import ValidationError
from marketschema.models import Quote, Symbol, Timestamp, Price

try:
    quote = Quote(
        symbol=Symbol("AAPL"),
        timestamp=Timestamp(datetime.now(timezone.utc)),
        bid=Price(150.00),
        ask=Price(150.05),
        unknown_field="value",  # スキーマにないフィールド
    )
except ValidationError as e:
    print(e)
```

出力:

```text
1 validation error for Quote
unknown_field
  Extra inputs are not permitted [type=extra_forbidden, ...]
```

### パターンバリデーション

文字列パターン型は自動的にパターンバリデーションが行われる。

```python
from pydantic import ValidationError
from marketschema.models import Currency

try:
    # 小文字は不可
    currency = Currency("usd")
except ValidationError as e:
    print(e)
```

出力:

```text
1 validation error for Currency
  String should match pattern '^[A-Z]{3}$' [type=string_pattern_mismatch, ...]
```

## JSON Schema との関係

marketschema は Schema First アプローチを採用している。

### アプローチ

1. JSON Schema でデータ構造を定義
2. スキーマから Python (pydantic) および Rust (serde) のコードを自動生成
3. スキーマを Single Source of Truth として保持

### スキーマファイルの場所

スキーマファイルは `schemas/` に配置されている。主要なスキーマには以下が含まれる（新規追加により増える可能性あり）:

```text
schemas/
├── definitions.json   # 共通型定義
├── quote.json         # Quote モデル
├── ohlcv.json         # OHLCV モデル
├── trade.json         # Trade モデル
├── orderbook.json     # OrderBook モデル
├── volume_info.json   # VolumeInfo モデル
├── instrument.json    # Instrument モデル
├── derivative_info.json  # DerivativeInfo モデル
├── expiry_info.json   # ExpiryInfo モデル
└── option_info.json   # OptionInfo モデル
```

### コード生成

Python モデルは datamodel-codegen を使用して自動生成される。詳細は [コード生成ガイド](../code-generation.md) を参照。

---

(rust-models-guide)=
# Rust モデル実装ガイド

marketschema の Rust モデルを使用してマーケットデータを扱う方法を解説する。

## 概要

marketschema は金融マーケットデータを統一的に扱うための Rust 構造体を提供する。JSON Schema から自動生成された serde 対応の構造体を使用することで、異なるデータソースからのデータを一貫した形式で扱える。

このガイドの対象読者:

- マーケットデータを扱う Rust アプリケーションの開発者
- 複数の取引所やデータプロバイダを統合するシステムの開発者
- 型安全なデータ処理を求める開発者

## クイックスタート

### インストール

```toml
[dependencies]
marketschema = { git = "https://github.com/drillan/marketschema" }
serde_json = "1.0"
```

### 最初のモデル作成

気配値（Quote）モデルを作成する例:

```rust
use marketschema::Quote;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quote: Quote = serde_json::from_str(r#"{
        "symbol": "AAPL",
        "timestamp": "2025-01-15T10:30:00Z",
        "bid": 150.0,
        "ask": 150.05,
        "bid_size": 100.0,
        "ask_size": 200.0
    }"#)?;

    println!("{}", serde_json::to_string_pretty(&quote)?);
    Ok(())
}
```

出力:

```json
{
  "symbol": "AAPL",
  "timestamp": "2025-01-15T10:30:00Z",
  "bid": 150.0,
  "ask": 150.05,
  "bid_size": 100.0,
  "ask_size": 200.0
}
```

## マーケットデータモデル

### Quote（気配値）

最良気配値（BBO: Best Bid/Offer）を表現する。

```rust
use marketschema::Quote;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 必須フィールドのみ
    let quote: Quote = serde_json::from_str(r#"{
        "symbol": "BTC-USD",
        "timestamp": "2025-01-15T00:00:00Z",
        "bid": 42000.00,
        "ask": 42001.50
    }"#)?;

    // オプションフィールドを含む
    let quote_with_size: Quote = serde_json::from_str(r#"{
        "symbol": "BTC-USD",
        "timestamp": "2025-01-15T00:00:00Z",
        "bid": 42000.00,
        "ask": 42001.50,
        "bid_size": 1.5,
        "ask_size": 2.0
    }"#)?;

    Ok(())
}
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | QuoteSymbol | Yes | 銘柄識別子 |
| `timestamp` | DateTime<Utc> | Yes | 気配値取得時刻 |
| `bid` | f64 | Yes | 買い気配値 |
| `ask` | f64 | Yes | 売り気配値 |
| `bid_size` | Option<f64> | No | 買い気配の数量 |
| `ask_size` | Option<f64> | No | 売り気配の数量 |

### Ohlcv（ローソク足）

ローソク足データ（始値、高値、安値、終値、出来高）を表現する。

```rust
use marketschema::Ohlcv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ohlcv: Ohlcv = serde_json::from_str(r#"{
        "symbol": "ETH-USD",
        "timestamp": "2025-01-15T00:00:00Z",
        "open": 2500.00,
        "high": 2550.00,
        "low": 2480.00,
        "close": 2530.00,
        "volume": 10000.0,
        "quote_volume": 25000000.0
    }"#)?;

    Ok(())
}
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | OhlcvSymbol | Yes | 銘柄識別子 |
| `timestamp` | DateTime<Utc> | Yes | 足の開始時刻 |
| `open` | f64 | Yes | 始値 |
| `high` | f64 | Yes | 高値 |
| `low` | f64 | Yes | 安値 |
| `close` | f64 | Yes | 終値 |
| `volume` | f64 | Yes | 出来高 |
| `quote_volume` | Option<f64> | No | 売買代金（決済通貨建て） |

### Trade（約定）

個別約定（歩み値 / Time & Sales）を表現する。

```rust
use marketschema::Trade;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let trade: Trade = serde_json::from_str(r#"{
        "symbol": "AAPL",
        "timestamp": "2025-01-15T10:30:00Z",
        "price": 150.25,
        "size": 100.0,
        "side": "buy"
    }"#)?;

    Ok(())
}
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | TradeSymbol | Yes | 銘柄識別子 |
| `timestamp` | DateTime<Utc> | Yes | 約定時刻 |
| `price` | f64 | Yes | 約定価格 |
| `size` | f64 | Yes | 約定数量 |
| `side` | TradeSide | Yes | 売買方向（buy/sell） |

### OrderBook（板情報）

複数レベルの板情報を表現する。

```rust
use marketschema::OrderBook;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orderbook: OrderBook = serde_json::from_str(r#"{
        "symbol": "BTC-USD",
        "timestamp": "2025-01-15T00:00:00Z",
        "bids": [
            { "price": 42000.0, "size": 1.5 },
            { "price": 41999.0, "size": 2.0 },
            { "price": 41998.0, "size": 3.5 }
        ],
        "asks": [
            { "price": 42001.0, "size": 1.0 },
            { "price": 42002.0, "size": 2.5 },
            { "price": 42003.0, "size": 4.0 }
        ]
    }"#)?;

    // 最良気配を取得
    if let Some(best_bid) = orderbook.bids.first() {
        println!("Best bid: {} @ {}", best_bid.size, best_bid.price);
    }
    if let Some(best_ask) = orderbook.asks.first() {
        println!("Best ask: {} @ {}", best_ask.size, best_ask.price);
    }

    Ok(())
}
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | OrderBookSymbol | Yes | 銘柄識別子 |
| `timestamp` | DateTime<Utc> | Yes | 板情報取得時刻 |
| `bids` | Vec<PriceLevel> | Yes | 買い板（価格降順） |
| `asks` | Vec<PriceLevel> | Yes | 売り板（価格昇順） |

### VolumeInfo（出来高・売買代金）

出来高と売買代金を表現する。

```rust
use marketschema::VolumeInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let volume_info: VolumeInfo = serde_json::from_str(r#"{
        "symbol": "AAPL",
        "timestamp": "2025-01-15T00:00:00Z",
        "volume": 1000000.0,
        "quote_volume": 150000000.0
    }"#)?;

    Ok(())
}
```

フィールド:

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `symbol` | VolumeInfoSymbol | Yes | 銘柄識別子 |
| `timestamp` | DateTime<Utc> | Yes | データ取得時刻 |
| `volume` | f64 | Yes | 出来高（数量ベース） |
| `quote_volume` | Option<f64> | No | 売買代金（決済通貨建て） |

## 銘柄情報モデル

### Instrument（銘柄情報）

銘柄識別情報を表現する。

```rust
use marketschema::Instrument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 株式
    let stock: Instrument = serde_json::from_str(r#"{
        "symbol": "AAPL",
        "asset_class": "equity",
        "currency": "USD",
        "exchange": "XNAS"
    }"#)?;

    // 暗号資産ペア
    let crypto: Instrument = serde_json::from_str(r#"{
        "symbol": "BTC-USD",
        "asset_class": "crypto",
        "base_currency": "BTC",
        "quote_currency": "USD"
    }"#)?;

    // FX ペア
    let fx: Instrument = serde_json::from_str(r#"{
        "symbol": "USD/JPY",
        "asset_class": "fx",
        "base_currency": "USD",
        "quote_currency": "JPY"
    }"#)?;

    Ok(())
}
```

### DerivativeInfo（デリバティブ共通）

先物・オプション共通の情報を表現する。

```rust
use marketschema::DerivativeInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let derivative: DerivativeInfo = serde_json::from_str(r#"{
        "multiplier": 100.0,
        "tick_size": 0.01,
        "underlying_symbol": "SPX",
        "underlying_type": "index"
    }"#)?;

    Ok(())
}
```

### ExpiryInfo（満期情報）

先物・オプションの満期関連情報を表現する。

```rust
use marketschema::ExpiryInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expiry: ExpiryInfo = serde_json::from_str(r#"{
        "expiry": "2025-03",
        "last_trading_day": "2025-03-20",
        "expiration_date": "2025-03-21",
        "settlement_date": "2025-03-21"
    }"#)?;

    Ok(())
}
```

### OptionInfo（オプション）

オプション固有の情報を表現する。

```rust
use marketschema::OptionInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let option: OptionInfo = serde_json::from_str(r#"{
        "strike_price": 5000.0,
        "option_type": "call",
        "exercise_style": "european"
    }"#)?;

    Ok(())
}
```

## ビルダーパターン

Rust モデルはビルダーパターンによる構築をサポートする。

```rust
use marketschema::Quote;
use marketschema::types::quote::QuoteSymbol;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quote: Quote = Quote::builder()
        .symbol("7203.T".parse::<QuoteSymbol>()?)
        .timestamp(Utc::now())
        .bid(2850.0)
        .ask(2851.0)
        .bid_size(Some(1000.0))
        .ask_size(Some(500.0))
        .try_into()?;

    println!("{:?}", quote);
    Ok(())
}
```

## バリデーション

marketschema のモデルは serde による自動バリデーションを提供する。

### デシリアライズエラーの例

```rust
use marketschema::Quote;

fn main() {
    // timestamp が欠落
    let invalid_json = r#"{"symbol": "AAPL", "bid": 150.0, "ask": 150.05}"#;
    let result: Result<Quote, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}
```

### 追加フィールドの禁止

すべてのモデルは `#[serde(deny_unknown_fields)]` で設定されており、スキーマに定義されていないフィールドは拒否される。

```rust
use marketschema::Quote;

fn main() {
    let unknown_field_json = r#"{
        "symbol": "AAPL",
        "timestamp": "2025-01-15T00:00:00Z",
        "bid": 150.0,
        "ask": 150.05,
        "unknown_field": "value"
    }"#;
    let result: Result<Quote, _> = serde_json::from_str(unknown_field_json);
    assert!(result.is_err());
}
```

### Symbol バリデーション

各 Symbol ニュータイプは最小長1文字のバリデーションを持つ。

```rust
use marketschema::types::quote::QuoteSymbol;
use std::str::FromStr;

fn main() {
    // 有効な symbol
    let symbol = QuoteSymbol::from_str("7203.T");
    assert!(symbol.is_ok());

    // 無効な symbol（空文字列）
    let empty = QuoteSymbol::from_str("");
    assert!(empty.is_err());
}
```

## JSON Schema との関係

marketschema は Schema First アプローチを採用している。

### アプローチ

1. JSON Schema でデータ構造を定義
2. スキーマから Python (pydantic) および Rust (serde) のコードを自動生成
3. スキーマを Single Source of Truth として保持

### コード生成

Rust モデルは typify を使用して自動生成される。詳細は [コード生成ガイド](../code-generation.md) を参照。

## 参照

- [クイックスタートガイド](https://github.com/drillan/marketschema/blob/main/specs/002-data-model-rust/quickstart.md) - Rust 向けクイックスタート
- [用語集](../glossary.md) - 標準フィールド名の定義
- [コード生成ガイド](../code-generation.md) - JSON Schema からのコード生成方法
- [ADR: フィールド名](../adr/index.md) - フィールド名の決定経緯
