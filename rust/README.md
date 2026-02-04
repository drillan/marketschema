# marketschema

Unified market data schema for financial applications in Rust.

This crate provides Rust struct definitions generated from JSON Schema for market data types.

## Types

### Market Data Types

| Type | Description |
|------|-------------|
| `Quote` | Best bid/offer (BBO) |
| `Trade` | Trade execution |
| `Ohlcv` | OHLCV candlestick |
| `OrderBook` | Order book with price levels |
| `VolumeInfo` | Volume and quote volume |

### Instrument Types

| Type | Description |
|------|-------------|
| `Instrument` | Instrument metadata |
| `DerivativeInfo` | Derivative contract information (futures/options common) |
| `ExpiryInfo` | Expiration information for derivatives |
| `OptionInfo` | Option-specific information (strike, type, exercise style) |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
marketschema = { git = "https://github.com/drillan/marketschema" }
serde_json = "1.0"
```

## Usage

```rust
use marketschema::{Quote, Trade, OrderBook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deserialize Quote
    let quote: Quote = serde_json::from_str(r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-03T09:00:00Z",
        "bid": 2850.0,
        "ask": 2851.0
    }"#)?;
    println!("Quote: {} bid={} ask={}", *quote.symbol, quote.bid, quote.ask);

    // Deserialize Trade
    let trade: Trade = serde_json::from_str(r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-03T14:30:00Z",
        "price": 175.50,
        "size": 100.0,
        "side": "buy"
    }"#)?;
    println!("Trade: {} @ {}", *trade.symbol, trade.price);

    Ok(())
}
```

## Features

- Type-safe deserialization with serde
- Unknown fields are rejected (`#[serde(deny_unknown_fields)]`)
- Builder pattern for struct construction
- String newtypes with validation (each type has its own Symbol newtype, e.g., `QuoteSymbol`, `TradeSymbol`)

## Documentation

- [Quickstart Guide](https://github.com/drillan/marketschema/blob/main/specs/002-data-model-rust/quickstart.md)
- [Code Generation Guide](https://github.com/drillan/marketschema/blob/main/docs/code-generation.md)

## License

MIT
