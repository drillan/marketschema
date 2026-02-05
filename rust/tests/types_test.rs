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
    assert_eq!(quote.bid, Some(2850.0));
    assert_eq!(quote.ask, Some(2851.0));
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
    assert_eq!(ohlcv.open, Some(50000.0));
    assert_eq!(ohlcv.volume, Some(12345.67));
    assert_eq!(ohlcv.quote_volume, None);
}

#[test]
fn test_ohlcv_deserialization_with_quote_volume() {
    let json = r#"{
        "symbol": "ETHUSDT",
        "timestamp": "2026-02-02T08:00:00Z",
        "open": 3200.0,
        "high": 3350.0,
        "low": 3150.0,
        "close": 3280.0,
        "volume": 50000.0,
        "quote_volume": 162500000.0
    }"#;

    let ohlcv: Ohlcv = serde_json::from_str(json).expect("Failed to deserialize OHLCV");
    assert_eq!(*ohlcv.symbol, "ETHUSDT");
    assert_eq!(ohlcv.open, Some(3200.0));
    assert_eq!(ohlcv.high, Some(3350.0));
    assert_eq!(ohlcv.low, Some(3150.0));
    assert_eq!(ohlcv.close, Some(3280.0));
    assert_eq!(ohlcv.volume, Some(50000.0));
    assert_eq!(ohlcv.quote_volume, Some(162500000.0));
}

#[test]
fn test_ohlcv_deserialization_equity() {
    // Test with equity market data (stock)
    let json = r#"{
        "symbol": "7203.T",
        "timestamp": "2026-02-02T06:00:00Z",
        "open": 2800.0,
        "high": 2850.0,
        "low": 2780.0,
        "close": 2820.0,
        "volume": 1500000.0
    }"#;

    let ohlcv: Ohlcv = serde_json::from_str(json).expect("Failed to deserialize OHLCV");
    assert_eq!(*ohlcv.symbol, "7203.T");
    assert_eq!(ohlcv.close, Some(2820.0));
    assert_eq!(ohlcv.volume, Some(1500000.0));
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
fn test_orderbook_deserialization_empty_arrays() {
    // T027: OrderBook with empty bids/asks arrays
    let json = r#"{
        "symbol": "EURUSD",
        "timestamp": "2026-02-02T12:00:00Z",
        "bids": [],
        "asks": []
    }"#;

    let orderbook: OrderBook = serde_json::from_str(json).expect("Failed to deserialize OrderBook");
    assert_eq!(*orderbook.symbol, "EURUSD");
    assert!(orderbook.bids.is_empty());
    assert!(orderbook.asks.is_empty());
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
// Tests for deny_unknown_fields (FR-R016 compliance)
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
// Tests for VolumeInfo
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

#[test]
fn test_volume_info_with_open_interest() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "volume": 12345.67,
        "open_interest": 125000.0
    }"#;

    let volume_info: VolumeInfo =
        serde_json::from_str(json).expect("Failed to deserialize VolumeInfo");
    assert_eq!(volume_info.open_interest, Some(125000.0));
}

#[test]
fn test_volume_info_open_interest_optional() {
    let json = r#"{
        "symbol": "BTCUSDT",
        "timestamp": "2026-02-02T00:00:00Z",
        "volume": 12345.67
    }"#;

    let volume_info: VolumeInfo =
        serde_json::from_str(json).expect("Failed to deserialize VolumeInfo");
    assert!(volume_info.open_interest.is_none());
}

// =============================================================================
// Tests for ExpiryInfo
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
// Tests for OptionInfo
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
// Tests for DerivativeInfo
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

#[test]
fn test_derivative_info_with_settlement_price() {
    let json = r#"{
        "underlying_symbol": "NK225",
        "underlying_type": "index",
        "multiplier": 1000.0,
        "tick_size": 5.0,
        "settlement_method": "cash",
        "settlement_price": 39850.0
    }"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize DerivativeInfo");
    assert_eq!(derivative_info.settlement_price, Some(39850.0));
}

#[test]
fn test_derivative_info_settlement_price_optional() {
    let json = r#"{
        "underlying_symbol": "NK225",
        "underlying_type": "index",
        "multiplier": 1000.0,
        "tick_size": 5.0
    }"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize DerivativeInfo");
    assert!(derivative_info.settlement_price.is_none());
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

// =============================================================================
// Tests for required field validation
// Verify that missing required fields cause deserialization errors
// =============================================================================

#[test]
fn test_volume_info_rejects_missing_required_fields() {
    // symbol missing
    let json = r#"{"timestamp": "2026-02-02T00:00:00Z", "volume": 100.0}"#;
    let result: Result<VolumeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // timestamp missing
    let json = r#"{"symbol": "TEST", "volume": 100.0}"#;
    let result: Result<VolumeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // volume missing
    let json = r#"{"symbol": "TEST", "timestamp": "2026-02-02T00:00:00Z"}"#;
    let result: Result<VolumeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_expiry_info_rejects_missing_required_fields() {
    // expiration_date missing
    let json = r#"{"expiry": "2026-03"}"#;
    let result: Result<ExpiryInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_option_info_rejects_missing_required_fields() {
    // strike_price missing
    let json = r#"{"option_type": "call"}"#;
    let result: Result<OptionInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // option_type missing
    let json = r#"{"strike_price": 100.0}"#;
    let result: Result<OptionInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_derivative_info_rejects_missing_required_fields() {
    // underlying_symbol missing
    let json = r#"{"underlying_type": "index", "multiplier": 1.0, "tick_size": 0.01}"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // underlying_type missing
    let json = r#"{"underlying_symbol": "TEST", "multiplier": 1.0, "tick_size": 0.01}"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // multiplier missing
    let json = r#"{"underlying_symbol": "TEST", "underlying_type": "index", "tick_size": 0.01}"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // tick_size missing
    let json = r#"{"underlying_symbol": "TEST", "underlying_type": "index", "multiplier": 1.0}"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for enum value validation
// Verify that invalid enum values are rejected
// =============================================================================

#[test]
fn test_option_info_rejects_invalid_option_type() {
    let json = r#"{"strike_price": 100.0, "option_type": "invalid"}"#;
    let result: Result<OptionInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_option_info_rejects_invalid_exercise_style() {
    let json = r#"{"strike_price": 100.0, "option_type": "call", "exercise_style": "invalid"}"#;
    let result: Result<OptionInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_derivative_info_rejects_invalid_underlying_type() {
    let json = r#"{
        "underlying_symbol": "TEST",
        "underlying_type": "invalid_type",
        "multiplier": 1.0,
        "tick_size": 0.01
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_derivative_info_rejects_invalid_settlement_method() {
    let json = r#"{
        "underlying_symbol": "TEST",
        "underlying_type": "index",
        "multiplier": 1.0,
        "tick_size": 0.01,
        "settlement_method": "invalid"
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for pattern validation
// Verify that invalid formats are rejected
// =============================================================================

#[test]
fn test_expiry_info_rejects_invalid_date_format() {
    // Slash format instead of dash
    let json = r#"{"expiration_date": "2026/03/20"}"#;
    let result: Result<ExpiryInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // No separators
    let json = r#"{"expiration_date": "20260320"}"#;
    let result: Result<ExpiryInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Wrong number of digits
    let json = r#"{"expiration_date": "26-03-20"}"#;
    let result: Result<ExpiryInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_derivative_info_rejects_invalid_currency_code() {
    // Lowercase
    let json = r#"{
        "underlying_symbol": "TEST",
        "underlying_type": "index",
        "multiplier": 1.0,
        "tick_size": 0.01,
        "settlement_currency": "jpy"
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Wrong length (2 characters)
    let json = r#"{
        "underlying_symbol": "TEST",
        "underlying_type": "index",
        "multiplier": 1.0,
        "tick_size": 0.01,
        "settlement_currency": "JP"
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Wrong length (4 characters)
    let json = r#"{
        "underlying_symbol": "TEST",
        "underlying_type": "index",
        "multiplier": 1.0,
        "tick_size": 0.01,
        "settlement_currency": "JPYY"
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_instrument_rejects_invalid_currency_pattern() {
    // T029: Instrument - invalid currency pattern (lowercase)
    let json = r#"{
        "symbol": "7203.T",
        "asset_class": "equity",
        "currency": "jpy"
    }"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // invalid currency pattern (wrong length)
    let json = r#"{
        "symbol": "7203.T",
        "asset_class": "equity",
        "currency": "JP"
    }"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // invalid exchange pattern (3 characters instead of 4)
    let json = r#"{
        "symbol": "7203.T",
        "asset_class": "equity",
        "exchange": "XJP"
    }"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for minLength validation
// Verify that empty strings are rejected for fields with minLength: 1
// =============================================================================

#[test]
fn test_volume_info_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "timestamp": "2026-02-02T00:00:00Z", "volume": 100.0}"#;
    let result: Result<VolumeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_derivative_info_rejects_empty_underlying_symbol() {
    let json = r#"{
        "underlying_symbol": "",
        "underlying_type": "index",
        "multiplier": 1.0,
        "tick_size": 0.01
    }"#;
    let result: Result<DerivativeInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_quote_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "timestamp": "2026-02-02T09:00:00Z", "bid": 100.0, "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_trade_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "timestamp": "2026-02-02T14:30:00Z", "price": 175.50, "size": 100.0, "side": "buy"}"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_ohlcv_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "timestamp": "2026-02-02T00:00:00Z", "open": 100.0, "high": 105.0, "low": 99.0, "close": 103.0, "volume": 1000.0}"#;
    let result: Result<Ohlcv, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_orderbook_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "timestamp": "2026-02-02T09:00:00Z", "bids": [], "asks": []}"#;
    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_instrument_rejects_empty_symbol() {
    let json = r#"{"symbol": "", "asset_class": "equity"}"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for enum validation
// Verify that invalid enum values are rejected
// =============================================================================

#[test]
fn test_instrument_rejects_invalid_asset_class() {
    let json = r#"{"symbol": "TEST", "asset_class": "invalid"}"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for timestamp format validation
// Verify that invalid timestamp formats are rejected
// =============================================================================

#[test]
fn test_quote_rejects_invalid_timestamp_format() {
    // Unix timestamp instead of ISO 8601
    let json = r#"{"symbol": "TEST", "timestamp": 1706857200, "bid": 100.0, "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // Invalid date format
    let json =
        r#"{"symbol": "TEST", "timestamp": "2026/02/02 09:00:00", "bid": 100.0, "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Tests for required field validation (SC-R004)
// Verify that missing required fields cause deserialization errors
// =============================================================================

#[test]
fn test_quote_rejects_missing_required_fields() {
    // T020: Quote - missing symbol
    let json = r#"{"timestamp": "2026-02-02T09:00:00Z", "bid": 100.0, "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing timestamp
    let json = r#"{"symbol": "TEST", "bid": 100.0, "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // bid and ask are now optional (nullable), so missing is allowed
    let json = r#"{"symbol": "TEST", "timestamp": "2026-02-02T09:00:00Z", "ask": 101.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_ok());

    let json = r#"{"symbol": "TEST", "timestamp": "2026-02-02T09:00:00Z", "bid": 100.0}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_ok());

    // Both bid and ask missing is also valid
    let json = r#"{"symbol": "TEST", "timestamp": "2026-02-02T09:00:00Z"}"#;
    let result: Result<Quote, _> = serde_json::from_str(json);
    assert!(result.is_ok());
}

#[test]
fn test_ohlcv_rejects_missing_required_fields() {
    // T022: Ohlcv - missing symbol (still required)
    let json = r#"{
        "timestamp": "2026-02-02T00:00:00Z",
        "open": 100.0, "high": 105.0, "low": 99.0, "close": 103.0, "volume": 1000.0
    }"#;
    let result: Result<Ohlcv, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing timestamp (still required)
    let json = r#"{
        "symbol": "TEST",
        "open": 100.0, "high": 105.0, "low": 99.0, "close": 103.0, "volume": 1000.0
    }"#;
    let result: Result<Ohlcv, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // open, high, low, close, volume are now optional (nullable), so missing is allowed
    let json = r#"{
        "symbol": "TEST", "timestamp": "2026-02-02T00:00:00Z"
    }"#;
    let result: Result<Ohlcv, _> = serde_json::from_str(json);
    assert!(result.is_ok());
}

#[test]
fn test_trade_rejects_missing_required_fields() {
    // T024: Trade - missing side
    let json = r#"{
        "symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z",
        "price": 175.50, "size": 100.0
    }"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing price
    let json = r#"{
        "symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z",
        "size": 100.0, "side": "buy"
    }"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing symbol
    let json = r#"{
        "timestamp": "2026-02-02T14:30:00Z",
        "price": 175.50, "size": 100.0, "side": "buy"
    }"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_trade_rejects_invalid_side_value() {
    // T025: Trade - invalid side type (not "buy" or "sell")
    let json = r#"{
        "symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z",
        "price": 175.50, "size": 100.0, "side": "invalid"
    }"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // wrong type (number instead of string)
    let json = r#"{
        "symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z",
        "price": 175.50, "size": 100.0, "side": 1
    }"#;
    let result: Result<Trade, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_instrument_rejects_missing_required_fields() {
    // missing symbol
    let json = r#"{"asset_class": "equity"}"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing asset_class
    let json = r#"{"symbol": "7203.T"}"#;
    let result: Result<Instrument, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_orderbook_rejects_missing_required_fields() {
    // missing symbol
    let json = r#"{
        "timestamp": "2026-02-02T09:00:00Z",
        "bids": [], "asks": []
    }"#;
    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing bids
    let json = r#"{
        "symbol": "USDJPY", "timestamp": "2026-02-02T09:00:00Z",
        "asks": []
    }"#;
    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());

    // missing asks
    let json = r#"{
        "symbol": "USDJPY", "timestamp": "2026-02-02T09:00:00Z",
        "bids": []
    }"#;
    let result: Result<OrderBook, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// =============================================================================
// Serialization roundtrip tests (SC-R006)
// Verify that serialize -> deserialize produces equivalent data
// =============================================================================

#[test]
fn test_ohlcv_serialization_roundtrip() {
    // T031: Ohlcv roundtrip
    let json = r#"{"close":51200.0,"high":51500.0,"low":49800.0,"open":50000.0,"symbol":"BTCUSDT","timestamp":"2026-02-02T00:00:00Z","volume":12345.67}"#;

    let ohlcv: Ohlcv = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&ohlcv).expect("Failed to serialize");
    let roundtrip: Ohlcv =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*ohlcv.symbol, *roundtrip.symbol);
    assert_eq!(ohlcv.open, roundtrip.open);
    assert_eq!(ohlcv.high, roundtrip.high);
    assert_eq!(ohlcv.low, roundtrip.low);
    assert_eq!(ohlcv.close, roundtrip.close);
    assert_eq!(ohlcv.volume, roundtrip.volume);
}

#[test]
fn test_trade_serialization_roundtrip() {
    // T032: Trade roundtrip
    let json = r#"{"price":175.5,"side":"buy","size":100.0,"symbol":"AAPL","timestamp":"2026-02-02T14:30:00Z"}"#;

    let trade: Trade = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&trade).expect("Failed to serialize");
    let roundtrip: Trade =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*trade.symbol, *roundtrip.symbol);
    assert_eq!(trade.price, roundtrip.price);
    assert_eq!(trade.size, roundtrip.size);
    assert_eq!(trade.side.to_string(), roundtrip.side.to_string());
    assert_eq!(trade.timestamp, roundtrip.timestamp);
}

#[test]
fn test_orderbook_serialization_roundtrip() {
    // T033: OrderBook roundtrip
    let json = r#"{"asks":[{"price":149.51,"size":1500000.0}],"bids":[{"price":149.5,"size":1000000.0}],"symbol":"USDJPY","timestamp":"2026-02-02T09:00:00Z"}"#;

    let orderbook: OrderBook = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&orderbook).expect("Failed to serialize");
    let roundtrip: OrderBook =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*orderbook.symbol, *roundtrip.symbol);
    assert_eq!(orderbook.bids.len(), roundtrip.bids.len());
    assert_eq!(orderbook.asks.len(), roundtrip.asks.len());
    assert_eq!(orderbook.timestamp, roundtrip.timestamp);
}

#[test]
fn test_instrument_serialization_roundtrip() {
    // T034: Instrument roundtrip
    let json = r#"{"asset_class":"equity","currency":"JPY","exchange":"XJPX","symbol":"7203.T"}"#;

    let instrument: Instrument = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&instrument).expect("Failed to serialize");
    let roundtrip: Instrument =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*instrument.symbol, *roundtrip.symbol);
    assert_eq!(
        instrument.asset_class.to_string(),
        roundtrip.asset_class.to_string()
    );
}

#[test]
fn test_volume_info_serialization_roundtrip() {
    // T051: VolumeInfo roundtrip
    let json = r#"{"quote_volume":617283500.0,"symbol":"BTCUSDT","timestamp":"2026-02-02T00:00:00Z","volume":12345.67}"#;

    let volume_info: VolumeInfo = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&volume_info).expect("Failed to serialize");
    let roundtrip: VolumeInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*volume_info.symbol, *roundtrip.symbol);
    assert_eq!(volume_info.volume, roundtrip.volume);
    assert_eq!(volume_info.quote_volume, roundtrip.quote_volume);
    assert_eq!(volume_info.timestamp, roundtrip.timestamp);
}

#[test]
fn test_expiry_info_serialization_roundtrip() {
    // T052: ExpiryInfo roundtrip
    let json =
        r#"{"expiration_date":"2026-03-20","expiry":"2026-03","last_trading_day":"2026-03-19"}"#;

    let expiry_info: ExpiryInfo = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&expiry_info).expect("Failed to serialize");
    let roundtrip: ExpiryInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(*expiry_info.expiration_date, *roundtrip.expiration_date);
}

#[test]
fn test_option_info_serialization_roundtrip() {
    // T053: OptionInfo roundtrip
    let json = r#"{"exercise_style":"european","option_type":"call","strike_price":30000.0}"#;

    let option_info: OptionInfo = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&option_info).expect("Failed to serialize");
    let roundtrip: OptionInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(option_info.strike_price, roundtrip.strike_price);
    assert_eq!(
        option_info.option_type.to_string(),
        roundtrip.option_type.to_string()
    );
}

#[test]
fn test_derivative_info_serialization_roundtrip() {
    // T054: DerivativeInfo roundtrip
    let json = r#"{"multiplier":1000.0,"settlement_currency":"JPY","settlement_method":"cash","tick_size":5.0,"underlying_symbol":"NK225","underlying_type":"index"}"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&derivative_info).expect("Failed to serialize");
    let roundtrip: DerivativeInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(
        *derivative_info.underlying_symbol,
        *roundtrip.underlying_symbol
    );
    assert_eq!(derivative_info.multiplier, roundtrip.multiplier);
}

#[test]
fn test_derivative_info_with_settlement_price_roundtrip() {
    // T055: DerivativeInfo with settlement_price roundtrip
    let json = r#"{"multiplier":1000.0,"settlement_price":39850.0,"tick_size":5.0,"underlying_symbol":"NK225","underlying_type":"index"}"#;

    let derivative_info: DerivativeInfo =
        serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&derivative_info).expect("Failed to serialize");
    let roundtrip: DerivativeInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(derivative_info.settlement_price, roundtrip.settlement_price);
}

#[test]
fn test_volume_info_with_open_interest_roundtrip() {
    // T056: VolumeInfo with open_interest roundtrip
    let json = r#"{"open_interest":125000.0,"symbol":"BTCUSDT","timestamp":"2026-02-02T00:00:00Z","volume":12345.67}"#;

    let volume_info: VolumeInfo = serde_json::from_str(json).expect("Failed to deserialize");
    let serialized = serde_json::to_string(&volume_info).expect("Failed to serialize");
    let roundtrip: VolumeInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize roundtrip");

    assert_eq!(volume_info.open_interest, roundtrip.open_interest);
}
