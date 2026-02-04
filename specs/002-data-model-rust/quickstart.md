# Quickstart: Rust Data Model

**Feature Branch**: `002-data-model-rust`
**Date**: 2026-02-03

## Prerequisites

- Rust (latest stable)
- Node.js / npm（スキーマバンドル用）
- cargo-typify（`cargo install cargo-typify`）

## Installation

### As a Dependency

```toml
[dependencies]
marketschema = { git = "https://github.com/drillan/marketschema", branch = "002-data-model-rust" }
```

### From Source

```bash
git clone https://github.com/drillan/marketschema
cd marketschema/rust
cargo build
```

## Basic Usage

### Deserialize JSON Data

```rust
use marketschema::{Quote, Trade, Ohlcv, OrderBook, Instrument};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Quote (最良気配値)
    let quote_json = r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-03T09:00:00Z",
        "bid": 2850.0,
        "ask": 2851.0,
        "bid_size": 1000.0,
        "ask_size": 500.0
    }"#;
    let quote: Quote = serde_json::from_str(quote_json)?;
    println!("Quote: {} bid={} ask={}", *quote.symbol, quote.bid, quote.ask);

    // Trade (約定)
    let trade_json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-03T14:30:00.123Z",
        "price": 175.50,
        "size": 100.0,
        "side": "buy"
    }"#;
    let trade: Trade = serde_json::from_str(trade_json)?;
    println!("Trade: {} price={} size={}", *trade.symbol, trade.price, trade.size);

    Ok(())
}
```

### Builder Pattern

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

### Serialize to JSON

```rust
use marketschema::Quote;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quote_json = r#"{
        "symbol": "BTC-USD",
        "timestamp": "2026-02-03T00:00:00Z",
        "bid": 50000.0,
        "ask": 50010.0
    }"#;
    let quote: Quote = serde_json::from_str(quote_json)?;

    // Serialize back to JSON
    let serialized = serde_json::to_string_pretty(&quote)?;
    println!("{}", serialized);

    Ok(())
}
```

### Working with OrderBook

```rust
use marketschema::OrderBook;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orderbook_json = r#"{
        "symbol": "USDJPY",
        "timestamp": "2026-02-03T09:00:00Z",
        "bids": [
            { "price": 149.50, "size": 1000000.0 },
            { "price": 149.49, "size": 2000000.0 }
        ],
        "asks": [
            { "price": 149.51, "size": 1500000.0 },
            { "price": 149.52, "size": 3000000.0 }
        ]
    }"#;
    let orderbook: OrderBook = serde_json::from_str(orderbook_json)?;

    println!("OrderBook: {} with {} bids, {} asks",
             *orderbook.symbol,
             orderbook.bids.len(),
             orderbook.asks.len());

    // Access best bid/ask
    if let Some(best_bid) = orderbook.bids.first() {
        println!("Best bid: {} @ {}", best_bid.size, best_bid.price);
    }
    if let Some(best_ask) = orderbook.asks.first() {
        println!("Best ask: {} @ {}", best_ask.size, best_ask.price);
    }

    Ok(())
}
```

## Code Generation

### Regenerate Rust Types

```bash
# 1. Bundle schemas (resolve $ref and convert unevaluatedProperties)
./scripts/bundle_schemas.sh

# 2. Generate Rust code
./scripts/generate_rust.sh

# Or use make
make generate-rust
```

### Manual Generation

```bash
# Bundle a single schema
cd schemas
npx json-refs resolve quote.json | \
    jq 'walk(if type == "object" and has("unevaluatedProperties")
        then .additionalProperties = .unevaluatedProperties | del(.unevaluatedProperties)
        else . end)' > ../../../rust/bundled/quote.json

# Generate Rust code
cargo typify rust/bundled/quote.json --output rust/src/types/quote.rs
```

## Validation

### Compile Check

```bash
cd rust
cargo check
```

### Run Tests

```bash
cd rust
cargo test
```

### Clippy Lint

```bash
cd rust
cargo clippy
```

## Error Handling

### Deserialization Errors

```rust
use marketschema::Quote;

fn main() {
    // Missing required field
    let invalid_json = r#"{"symbol": "AAPL"}"#;
    let result: Result<Quote, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());

    // Unknown field (rejected by deny_unknown_fields)
    let unknown_field_json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-03T00:00:00Z",
        "bid": 175.0,
        "ask": 175.50,
        "unknown_field": "value"
    }"#;
    let result: Result<Quote, _> = serde_json::from_str(unknown_field_json);
    assert!(result.is_err());
}
```

### Symbol Validation

```rust
use marketschema::types::quote::QuoteSymbol;
use std::str::FromStr;

fn main() {
    // Valid symbol
    let symbol = QuoteSymbol::from_str("7203.T");
    assert!(symbol.is_ok());

    // Invalid symbol (empty string)
    let empty = QuoteSymbol::from_str("");
    assert!(empty.is_err());
}
```

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
regress = "0.10"  # For pattern validation
```

## Related Documents

- [Code Generation Guide](../../docs/code-generation.md)
- [Parent Spec: 002-data-model](../002-data-model/spec.md)
- [ADR-001: unevaluatedProperties 対応策](../../docs/adr/codegen/001-unevaluated-properties-workaround.md)
