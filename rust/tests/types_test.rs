//! Tests for generated types

use marketschema::{Instrument, Ohlcv, OrderBook, Quote, Trade};

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

    let instrument: Instrument =
        serde_json::from_str(json).expect("Failed to deserialize Instrument");
    assert_eq!(*instrument.symbol, "7203.T");
}

#[test]
fn test_quote_serialization_roundtrip() {
    let json =
        r#"{"ask":2851.0,"bid":2850.0,"symbol":"7203.T","timestamp":"2026-02-02T09:00:00Z"}"#;

    let quote: Quote = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&quote).expect("Failed to serialize");

    // Verify we can deserialize the serialized version
    let _: Quote = serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");
}

// =============================================================================
// Tests for deny_unknown_fields (FR-010 compliance)
// These tests verify that unknown fields are rejected during deserialization
// =============================================================================

#[test]
fn test_quote_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-02T09:00:00Z",
        "bid": 2850.0,
        "ask": 2851.0,
        "unknown_field": "should_cause_error"
    }"#;

    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

#[test]
fn test_trade_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "AAPL",
        "timestamp": "2026-02-02T14:30:00Z",
        "price": 175.50,
        "size": 100.0,
        "side": "buy",
        "extra_field": 123
    }"#;

    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

#[test]
fn test_ohlcv_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "open": 50000.0,
        "high": 51500.0,
        "low": 49800.0,
        "close": 51200.0,
        "volume": 12345.67,
        "invalid_field": true
    }"#;

    let result: Result<Ohlcv, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

#[test]
fn test_orderbook_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "USDJPY",
        "timestamp": "2026-02-02T09:00:00Z",
        "bids": [{ "price": 149.50, "size": 1000000.0 }],
        "asks": [{ "price": 149.51, "size": 1500000.0 }],
        "unknown": "value"
    }"#;

    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

#[test]
fn test_orderbook_nested_rejects_unknown_fields() {
    // Test that unknown fields in nested structures (bids/asks items) are also rejected
    let json = r#"{
        "symbol": "USDJPY",
        "timestamp": "2026-02-02T09:00:00Z",
        "bids": [{ "price": 149.50, "size": 1000000.0, "extra": "field" }],
        "asks": [{ "price": 149.51, "size": 1500000.0 }]
    }"#;

    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

#[test]
fn test_instrument_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "7203.T",
        "asset_class": "equity",
        "currency": "JPY",
        "exchange": "XJPX",
        "not_a_field": "invalid"
    }"#;

    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

// =============================================================================
// Tests for VolumeInfo (T055)
// =============================================================================

use marketschema::types::volume_info::VolumeInfo;

#[test]
fn test_volume_info_deserialization() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "volume": 12345.67,
        "quote_volume": 617283500.0
    }"#;

    let volume_info: VolumeInfo =
        serde_json::from_str(json).expect("Failed to deserialize VolumeInfo");
    assert_eq!(*volume_info.symbol, "BTCUSDT");
    assert_eq!(volume_info.volume, 12345.67);
    assert_eq!(volume_info.quote_volume, Some(617283500.0));
}

#[test]
fn test_volume_info_without_optional_fields() {
    let json = r#"{
        "symbol": "ETHUSDT",
        "timestamp": "2026-02-02T12:00:00Z",
        "volume": 5000.0
    }"#;

    let volume_info: VolumeInfo =
        serde_json::from_str(json).expect("Failed to deserialize VolumeInfo");
    assert_eq!(volume_info.quote_volume, None);
}

#[test]
fn test_volume_info_rejects_unknown_fields() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "volume": 12345.67,
        "unknown_field": "invalid"
    }"#;

    let result: Result<VolumeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

// =============================================================================
// Tests for ExpiryInfo (T056)
// =============================================================================

use marketschema::types::expiry_info::ExpiryInfo;

#[test]
fn test_expiry_info_deserialization() {
    let json = r#"{
        "expiration_date": "2026-03-20",
        "expiry": "2026-03",
        "last_trading_day": "2026-03-19",
        "settlement_date": "2026-03-20"
    }"#;

    let expiry_info: ExpiryInfo =
        serde_json::from_str(json).expect("Failed to deserialize ExpiryInfo");
    assert_eq!(*expiry_info.expiration_date, "2026-03-20");
    assert_eq!(
        expiry_info.expiry.as_ref().map(|e| e.as_str()),
        Some("2026-03")
    );
    assert_eq!(
        expiry_info.last_trading_day.as_ref().map(|d| d.as_str()),
        Some("2026-03-19")
    );
    assert_eq!(
        expiry_info.settlement_date.as_ref().map(|d| d.as_str()),
        Some("2026-03-20")
    );
}

#[test]
fn test_expiry_info_without_optional_fields() {
    let json = r#"{
        "expiration_date": "2026-06-18"
    }"#;

    let expiry_info: ExpiryInfo =
        serde_json::from_str(json).expect("Failed to deserialize ExpiryInfo");
    assert_eq!(*expiry_info.expiration_date, "2026-06-18");
    assert!(expiry_info.expiry.is_none());
    assert!(expiry_info.last_trading_day.is_none());
    assert!(expiry_info.settlement_date.is_none());
}

#[test]
fn test_expiry_info_rejects_unknown_fields() {
    let json = r#"{
        "expiration_date": "2026-03-20",
        "invalid_field": "test"
    }"#;

    let result: Result<ExpiryInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

// =============================================================================
// Tests for OptionInfo (T057)
// =============================================================================

use marketschema::types::option_info::OptionInfo;

#[test]
fn test_option_info_call_deserialization() {
    let json = r#"{
        "strike_price": 30000.0,
        "option_type": "call",
        "exercise_style": "european"
    }"#;

    let option_info: OptionInfo =
        serde_json::from_str(json).expect("Failed to deserialize OptionInfo");
    assert_eq!(option_info.strike_price, 30000.0);
    assert_eq!(option_info.option_type.to_string(), "call");
    assert_eq!(
        option_info.exercise_style.as_ref().map(|s| s.to_string()),
        Some("european".to_string())
    );
}

#[test]
fn test_option_info_put_deserialization() {
    let json = r#"{
        "strike_price": 25000.0,
        "option_type": "put"
    }"#;

    let option_info: OptionInfo =
        serde_json::from_str(json).expect("Failed to deserialize OptionInfo");
    assert_eq!(option_info.strike_price, 25000.0);
    assert_eq!(option_info.option_type.to_string(), "put");
    assert!(option_info.exercise_style.is_none());
}

#[test]
fn test_option_info_rejects_unknown_fields() {
    let json = r#"{
        "strike_price": 30000.0,
        "option_type": "call",
        "invalid_field": true
    }"#;

    let result: Result<OptionInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

// =============================================================================
// Tests for DerivativeInfo (T058)
// =============================================================================

use marketschema::types::derivative_info::DerivativeInfo;

#[test]
fn test_derivative_info_deserialization() {
    let json = r#"{
        "underlying_symbol": "NK225",
        "underlying_type": "index",
        "multiplier": 1000.0,
        "tick_size": 5.0,
        "lot_size": 1.0,
        "settlement_method": "cash",
        "settlement_currency": "JPY"
    }"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize DerivativeInfo");
    assert_eq!(*derivative_info.underlying_symbol, "NK225");
    assert_eq!(derivative_info.underlying_type.to_string(), "index");
    assert_eq!(derivative_info.multiplier, 1000.0);
    assert_eq!(derivative_info.tick_size, 5.0);
    assert_eq!(derivative_info.lot_size, Some(1.0));
    assert_eq!(
        derivative_info
            .settlement_method
            .as_ref()
            .map(|m| m.to_string()),
        Some("cash".to_string())
    );
}

#[test]
fn test_derivative_info_without_optional_fields() {
    let json = r#"{
        "underlying_symbol": "BTCUSD",
        "underlying_type": "crypto",
        "multiplier": 1.0,
        "tick_size": 0.5
    }"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize DerivativeInfo");
    assert_eq!(*derivative_info.underlying_symbol, "BTCUSD");
    assert!(derivative_info.lot_size.is_none());
    assert!(derivative_info.settlement_method.is_none());
    assert!(derivative_info.settlement_currency.is_none());
    assert!(derivative_info.is_perpetual.is_none());
    assert!(derivative_info.is_inverse.is_none());
}

#[test]
fn test_derivative_info_rejects_unknown_fields() {
    let json = r#"{
        "underlying_symbol": "NK225",
        "underlying_type": "index",
        "multiplier": 1000.0,
        "tick_size": 5.0,
        "unknown": "field"
    }"#;

    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("unknown field"));
}

// =============================================================================
// Tests for re-export verification (T015)
// =============================================================================

#[test]
fn test_types_module_reexport() {
    // Verify that types can be accessed via types:: module path
    // This test verifies T015: lib.rs で全型が re-export されている

    // Major types are re-exported at crate root
    let _: Quote = serde_json::from_str(
        r#"{"symbol":"TEST","timestamp":"2026-02-02T00:00:00Z","bid":100.0,"ask":101.0}"#,
    )
    .unwrap();
    let _: Trade = serde_json::from_str(
        r#"{"symbol":"TEST","timestamp":"2026-02-02T00:00:00Z","price":100.0,"size":10.0,"side":"buy"}"#,
    )
    .unwrap();
    let _: Ohlcv = serde_json::from_str(
        r#"{"symbol":"TEST","timestamp":"2026-02-02T00:00:00Z","open":100.0,"high":105.0,"low":99.0,"close":103.0,"volume":1000.0}"#,
    )
    .unwrap();
    let _: OrderBook = serde_json::from_str(
        r#"{"symbol":"TEST","timestamp":"2026-02-02T00:00:00Z","bids":[{"price":100.0,"size":10.0}],"asks":[{"price":101.0,"size":10.0}]}"#,
    )
    .unwrap();
    let _: Instrument =
        serde_json::from_str(r#"{"symbol":"TEST","asset_class":"equity"}"#).unwrap();

    // Additional types are accessible via types:: module
    let _: VolumeInfo = serde_json::from_str(
        r#"{"symbol":"TEST","timestamp":"2026-02-02T00:00:00Z","volume":1000.0}"#,
    )
    .unwrap();
    let _: ExpiryInfo = serde_json::from_str(r#"{"expiration_date":"2026-03-20"}"#).unwrap();
    let _: OptionInfo =
        serde_json::from_str(r#"{"strike_price":100.0,"option_type":"call"}"#).unwrap();
    let _: DerivativeInfo = serde_json::from_str(
        r#"{"underlying_symbol":"TEST","underlying_type":"index","multiplier":1.0,"tick_size":0.01}"#,
    )
    .unwrap();
}
