//! Quickstart example - validates code snippets from quickstart.md

use marketschema::{Ohlcv, OrderBook, Quote, Trade};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Deserialize JSON Data ===\n");
    deserialize_examples()?;

    println!("\n=== Builder Pattern ===\n");
    builder_example()?;

    println!("\n=== Serialize to JSON ===\n");
    serialize_example()?;

    println!("\n=== Working with OrderBook ===\n");
    orderbook_example()?;

    println!("\n=== Error Handling ===\n");
    error_handling_examples();

    println!("\n=== Symbol Validation ===\n");
    symbol_validation_example();

    println!("\nAll examples passed!");
    Ok(())
}

fn deserialize_examples() -> Result<(), Box<dyn std::error::Error>> {
    // Quote
    let quote_json = r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-03T09:00:00Z",
        "bid": 2850.0,
        "ask": 2851.0,
        "bid_size": 1000.0,
        "ask_size": 500.0
    }"#;
    let quote: Quote = serde_json::from_str(quote_json)?;
    println!(
        "Quote: {} bid={:?} ask={:?}",
        *quote.symbol, quote.bid, quote.ask
    );

    // Trade
    let trade_json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-03T14:30:00.123Z",
        "price": 175.50,
        "size": 100.0,
        "side": "buy"
    }"#;
    let trade: Trade = serde_json::from_str(trade_json)?;
    println!(
        "Trade: {} price={} size={}",
        *trade.symbol, trade.price, trade.size
    );

    // OHLCV
    let ohlcv_json = r#"{
        "symbol": "BTC-USD",
        "timestamp": "2026-02-03T00:00:00Z",
        "open": 50000.0,
        "high": 51000.0,
        "low": 49000.0,
        "close": 50500.0,
        "volume": 1000.0
    }"#;
    let ohlcv: Ohlcv = serde_json::from_str(ohlcv_json)?;
    println!(
        "OHLCV: {} O={:?} H={:?} L={:?} C={:?} V={:?}",
        *ohlcv.symbol, ohlcv.open, ohlcv.high, ohlcv.low, ohlcv.close, ohlcv.volume
    );

    Ok(())
}

fn builder_example() -> Result<(), Box<dyn std::error::Error>> {
    use chrono::Utc;
    use marketschema::types::quote::QuoteSymbol;

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

fn serialize_example() -> Result<(), Box<dyn std::error::Error>> {
    let quote_json = r#"{
        "symbol": "BTC-USD",
        "timestamp": "2026-02-03T00:00:00Z",
        "bid": 50000.0,
        "ask": 50010.0
    }"#;
    let quote: Quote = serde_json::from_str(quote_json)?;

    let serialized = serde_json::to_string_pretty(&quote)?;
    println!("{}", serialized);

    Ok(())
}

fn orderbook_example() -> Result<(), Box<dyn std::error::Error>> {
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

    println!(
        "OrderBook: {} with {} bids, {} asks",
        *orderbook.symbol,
        orderbook.bids.len(),
        orderbook.asks.len()
    );

    if let Some(best_bid) = orderbook.bids.first() {
        println!("Best bid: {} @ {}", best_bid.size, best_bid.price);
    }
    if let Some(best_ask) = orderbook.asks.first() {
        println!("Best ask: {} @ {}", best_ask.size, best_ask.price);
    }

    Ok(())
}

fn error_handling_examples() {
    // Missing required field
    let invalid_json = r#"{"symbol": "AAPL"}"#;
    let result: Result<Quote, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
    println!("Missing field test: PASSED (error as expected)");

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
    println!("Unknown field test: PASSED (error as expected)");
}

fn symbol_validation_example() {
    use marketschema::types::quote::QuoteSymbol;
    use std::str::FromStr;

    // Valid symbol
    let symbol = QuoteSymbol::from_str("7203.T");
    assert!(symbol.is_ok());
    println!("Valid symbol test: PASSED");

    // Invalid symbol (empty string)
    let empty = QuoteSymbol::from_str("");
    assert!(empty.is_err());
    println!("Empty symbol test: PASSED (error as expected)");
}
