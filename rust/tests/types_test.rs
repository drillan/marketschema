//! Tests for generated types

use marketschema::{Quote, Trade, Ohlcv, OrderBook, Instrument};

#[test]
fn test_quote_deserialization() {
    let json = r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-02T09:00:00Z",
        "bid": 2850.0,
        "ask": 2851.0,
        "bid_size": 1000.0,
        "ask_size": 500.0
    }"#;

    let quote: Quote = serde_json::from_str(json).expect("Failed to deserialize Quote");
    assert_eq!(*quote.symbol, "7203.T");
    assert_eq!(quote.bid, 2850.0);
    assert_eq!(quote.ask, 2851.0);
    assert_eq!(quote.bid_size, Some(1000.0));
    assert_eq!(quote.ask_size, Some(500.0));
}

#[test]
fn test_quote_without_optional_fields() {
    let json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-02T14:30:00Z",
        "bid": 175.0,
        "ask": 175.50
    }"#;

    let quote: Quote = serde_json::from_str(json).expect("Failed to deserialize Quote");
    assert_eq!(quote.bid_size, None);
    assert_eq!(quote.ask_size, None);
}

#[test]
fn test_trade_deserialization() {
    let json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-02T14:30:00.123Z",
        "price": 175.50,
        "size": 100.0,
        "side": "buy"
    }"#;

    let trade: Trade = serde_json::from_str(json).expect("Failed to deserialize Trade");
    assert_eq!(*trade.symbol, "AAPL");
    assert_eq!(trade.price, 175.50);
    assert_eq!(trade.size, 100.0);
}

#[test]
fn test_ohlcv_deserialization() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "open": 50000.0,
        "high": 51500.0,
        "low": 49800.0,
        "close": 51200.0,
        "volume": 12345.67
    }"#;

    let ohlcv: Ohlcv = serde_json::from_str(json).expect("Failed to deserialize OHLCV");
    assert_eq!(*ohlcv.symbol, "BTCUSDT");
    assert_eq!(ohlcv.open, 50000.0);
    assert_eq!(ohlcv.volume, 12345.67);
}

#[test]
fn test_orderbook_deserialization() {
    let json = r#"{
        "symbol": "USDJPY",
        "timestamp": "2026-02-02T09:00:00Z",
        "bids": [
            { "price": 149.50, "size": 1000000.0 },
            { "price": 149.49, "size": 2000000.0 }
        ],
        "asks": [
            { "price": 149.51, "size": 1500000.0 },
            { "price": 149.52, "size": 3000000.0 }
        ]
    }"#;

    let orderbook: OrderBook = serde_json::from_str(json).expect("Failed to deserialize OrderBook");
    assert_eq!(*orderbook.symbol, "USDJPY");
    assert_eq!(orderbook.bids.len(), 2);
    assert_eq!(orderbook.asks.len(), 2);
}

#[test]
fn test_instrument_deserialization() {
    let json = r#"{
        "symbol": "7203.T",
        "asset_class": "equity",
        "currency": "JPY",
        "exchange": "XJPX"
    }"#;

    let instrument: Instrument = serde_json::from_str(json).expect("Failed to deserialize Instrument");
    assert_eq!(*instrument.symbol, "7203.T");
}

#[test]
fn test_quote_serialization_roundtrip() {
    let json = r#"{"ask":2851.0,"bid":2850.0,"symbol":"7203.T","timestamp":"2026-02-02T09:00:00Z"}"#;

    let quote: Quote = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&quote).expect("Failed to serialize");

    // Verify we can deserialize the serialized version
    let _: Quote = serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");
}
