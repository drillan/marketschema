//! Tests for the bitbank adapter.

// Use #[path] to reference the example module directly, avoiding code duplication.
#[path = "../examples/bitbank/mod.rs"]
mod bitbank;

use bitbank::{
    BitbankAdapter, BitbankError, BITBANK_BASE_URL, BITBANK_OHLCV_EXPECTED_LENGTH,
    BITBANK_PRICE_LEVEL_EXPECTED_LENGTH, BITBANK_SUCCESS_CODE,
};
use marketschema_adapters::BaseAdapter;
use serde_json::json;

mod constants_tests {
    use super::*;

    #[test]
    fn test_base_url() {
        assert_eq!(BITBANK_BASE_URL, "https://public.bitbank.cc");
    }

    #[test]
    fn test_success_code() {
        assert_eq!(BITBANK_SUCCESS_CODE, 1);
    }

    #[test]
    fn test_ohlcv_expected_length() {
        assert_eq!(BITBANK_OHLCV_EXPECTED_LENGTH, 6);
    }

    #[test]
    fn test_price_level_expected_length() {
        assert_eq!(BITBANK_PRICE_LEVEL_EXPECTED_LENGTH, 2);
    }
}

mod validate_response_tests {
    use super::*;

    #[test]
    fn test_valid_response() {
        let adapter = BitbankAdapter::new();
        let response = json!({
            "success": 1,
            "data": {}
        });
        assert!(adapter.validate_response(&response).is_ok());
    }

    #[test]
    fn test_api_error_response() {
        let adapter = BitbankAdapter::new();
        let response = json!({
            "success": 0,
            "data": {
                "code": 10001
            }
        });
        let result = adapter.validate_response(&response);
        assert!(matches!(
            result,
            Err(BitbankError::ApiError {
                success_code: 0,
                ..
            })
        ));
    }

    #[test]
    fn test_missing_success_field() {
        let adapter = BitbankAdapter::new();
        let response = json!({
            "data": {}
        });
        let result = adapter.validate_response(&response);
        assert!(
            matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "success")
        );
    }
}

mod parse_quote_tests {
    use super::*;

    #[test]
    fn test_valid_ticker() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "sell": "14878899",
            "buy": "14878898",
            "high": "15000000",
            "low": "14500000",
            "open": "14750000",
            "last": "14878899",
            "vol": "123.4567",
            "timestamp": 1704067200000_i64
        });

        let quote = adapter.parse_quote(&data, "btc_jpy").unwrap();

        assert_eq!(quote.symbol, "btc_jpy");
        assert!((quote.bid - 14878898.0).abs() < f64::EPSILON);
        assert!((quote.ask - 14878899.0).abs() < f64::EPSILON);
        assert_eq!(quote.timestamp, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_missing_buy_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "sell": "14878899",
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_quote(&data, "btc_jpy");
        assert!(matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "buy"));
    }

    #[test]
    fn test_missing_sell_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "buy": "14878898",
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_quote(&data, "btc_jpy");
        assert!(matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "sell"));
    }

    #[test]
    fn test_missing_timestamp_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "buy": "14878898",
            "sell": "14878899"
        });

        let result = adapter.parse_quote(&data, "btc_jpy");
        assert!(
            matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "timestamp")
        );
    }
}

mod parse_trade_tests {
    use super::*;

    #[test]
    fn test_valid_transaction() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "transaction_id": 123456,
            "side": "buy",
            "price": "14878899",
            "amount": "0.01",
            "executed_at": 1704067200000_i64
        });

        let trade = adapter.parse_trade(&data, "btc_jpy").unwrap();

        assert_eq!(trade.symbol, "btc_jpy");
        assert!((trade.price - 14878899.0).abs() < f64::EPSILON);
        assert!((trade.size - 0.01).abs() < f64::EPSILON);
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.timestamp, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_sell_side() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "side": "sell",
            "price": "14878899",
            "amount": "0.01",
            "executed_at": 1704067200000_i64
        });

        let trade = adapter.parse_trade(&data, "btc_jpy").unwrap();
        assert_eq!(trade.side, "sell");
    }

    #[test]
    fn test_missing_price_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "side": "buy",
            "amount": "0.01",
            "executed_at": 1704067200000_i64
        });

        let result = adapter.parse_trade(&data, "btc_jpy");
        assert!(
            matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "price")
        );
    }

    #[test]
    fn test_missing_amount_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "side": "buy",
            "price": "14878899",
            "executed_at": 1704067200000_i64
        });

        let result = adapter.parse_trade(&data, "btc_jpy");
        assert!(
            matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "amount")
        );
    }

    #[test]
    fn test_missing_side_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "price": "14878899",
            "amount": "0.01",
            "executed_at": 1704067200000_i64
        });

        let result = adapter.parse_trade(&data, "btc_jpy");
        assert!(matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "side"));
    }
}

mod parse_trades_tests {
    use super::*;

    #[test]
    fn test_multiple_transactions() {
        let adapter = BitbankAdapter::new();
        let transactions = vec![
            json!({
                "side": "buy",
                "price": "14878899",
                "amount": "0.01",
                "executed_at": 1704067200000_i64
            }),
            json!({
                "side": "sell",
                "price": "14878900",
                "amount": "0.02",
                "executed_at": 1704067201000_i64
            }),
        ];

        let trades = adapter.parse_trades(&transactions, "btc_jpy").unwrap();

        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].side, "buy");
        assert_eq!(trades[1].side, "sell");
    }

    #[test]
    fn test_empty_transactions() {
        let adapter = BitbankAdapter::new();
        let transactions: Vec<serde_json::Value> = vec![];

        let trades = adapter.parse_trades(&transactions, "btc_jpy").unwrap();
        assert!(trades.is_empty());
    }
}

mod parse_ohlcv_tests {
    use super::*;

    #[test]
    fn test_valid_ohlcv() {
        let adapter = BitbankAdapter::new();
        let ohlcv_array = vec![
            json!("14500000"),        // open
            json!("15000000"),        // high
            json!("14000000"),        // low
            json!("14750000"),        // close
            json!("123.4567"),        // volume
            json!(1704067200000_i64), // timestamp
        ];

        let ohlcv = adapter.parse_ohlcv(&ohlcv_array, "btc_jpy").unwrap();

        assert_eq!(ohlcv.symbol, "btc_jpy");
        assert!((ohlcv.open - 14500000.0).abs() < f64::EPSILON);
        assert!((ohlcv.high - 15000000.0).abs() < f64::EPSILON);
        assert!((ohlcv.low - 14000000.0).abs() < f64::EPSILON);
        assert!((ohlcv.close - 14750000.0).abs() < f64::EPSILON);
        assert!((ohlcv.volume - 123.4567).abs() < f64::EPSILON);
        assert_eq!(ohlcv.timestamp, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_insufficient_array_length() {
        let adapter = BitbankAdapter::new();
        let ohlcv_array = vec![json!("14500000"), json!("15000000"), json!("14000000")];

        let result = adapter.parse_ohlcv(&ohlcv_array, "btc_jpy");
        assert!(matches!(
            result,
            Err(BitbankError::InsufficientArrayLength {
                expected: 6,
                actual: 3,
                ..
            })
        ));
    }
}

mod parse_ohlcv_batch_tests {
    use super::*;

    #[test]
    fn test_multiple_ohlcv() {
        let adapter = BitbankAdapter::new();
        let ohlcv_arrays = vec![
            json!([
                "14500000",
                "15000000",
                "14000000",
                "14750000",
                "123.4567",
                1704067200000_i64
            ]),
            json!([
                "14750000",
                "15500000",
                "14500000",
                "15000000",
                "234.5678",
                1704153600000_i64
            ]),
        ];

        let ohlcvs = adapter.parse_ohlcv_batch(&ohlcv_arrays, "btc_jpy").unwrap();

        assert_eq!(ohlcvs.len(), 2);
        assert_eq!(ohlcvs[0].timestamp, "2024-01-01T00:00:00Z");
        assert_eq!(ohlcvs[1].timestamp, "2024-01-02T00:00:00Z");
    }

    #[test]
    fn test_empty_batch() {
        let adapter = BitbankAdapter::new();
        let ohlcv_arrays: Vec<serde_json::Value> = vec![];

        let ohlcvs = adapter.parse_ohlcv_batch(&ohlcv_arrays, "btc_jpy").unwrap();
        assert!(ohlcvs.is_empty());
    }

    #[test]
    fn test_non_array_element_returns_error() {
        let adapter = BitbankAdapter::new();
        let ohlcv_arrays = vec![
            json!([
                "14500000",
                "15000000",
                "14000000",
                "14750000",
                "123.4567",
                1704067200000_i64
            ]),
            json!("not an array"), // This should cause an error
            json!([
                "14750000",
                "15500000",
                "14500000",
                "15000000",
                "234.5678",
                1704153600000_i64
            ]),
        ];

        let result = adapter.parse_ohlcv_batch(&ohlcv_arrays, "btc_jpy");
        assert!(matches!(
            result,
            Err(BitbankError::UnexpectedType {
                index: 1,
                context,
                actual_type
            }) if context == "OHLCV data" && actual_type == "string"
        ));
    }
}

mod parse_orderbook_tests {
    use super::*;

    #[test]
    fn test_valid_orderbook() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "asks": [
                ["14878899", "0.1"],
                ["14878900", "0.2"]
            ],
            "bids": [
                ["14878898", "0.3"],
                ["14878897", "0.4"]
            ],
            "timestamp": 1704067200000_i64
        });

        let orderbook = adapter.parse_orderbook(&data, "btc_jpy").unwrap();

        assert_eq!(orderbook.symbol, "btc_jpy");
        assert_eq!(orderbook.timestamp, "2024-01-01T00:00:00Z");

        assert_eq!(orderbook.asks.len(), 2);
        assert!((orderbook.asks[0].price - 14878899.0).abs() < f64::EPSILON);
        assert!((orderbook.asks[0].size - 0.1).abs() < f64::EPSILON);

        assert_eq!(orderbook.bids.len(), 2);
        assert!((orderbook.bids[0].price - 14878898.0).abs() < f64::EPSILON);
        assert!((orderbook.bids[0].size - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_missing_asks_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "bids": [["14878898", "0.3"]],
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_orderbook(&data, "btc_jpy");
        assert!(matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "asks"));
    }

    #[test]
    fn test_missing_bids_field() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "asks": [["14878899", "0.1"]],
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_orderbook(&data, "btc_jpy");
        assert!(matches!(result, Err(BitbankError::MissingField { field, .. }) if field == "bids"));
    }

    #[test]
    fn test_empty_orderbook() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "asks": [],
            "bids": [],
            "timestamp": 1704067200000_i64
        });

        let orderbook = adapter.parse_orderbook(&data, "btc_jpy").unwrap();

        assert!(orderbook.asks.is_empty());
        assert!(orderbook.bids.is_empty());
    }

    #[test]
    fn test_non_array_ask_element_returns_error() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "asks": [
                ["14878899", "0.1"],
                "not an array"  // This should cause an error
            ],
            "bids": [["14878898", "0.3"]],
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_orderbook(&data, "btc_jpy");
        assert!(matches!(
            result,
            Err(BitbankError::UnexpectedType {
                index: 1,
                context,
                actual_type
            }) if context == "ask levels" && actual_type == "string"
        ));
    }

    #[test]
    fn test_non_array_bid_element_returns_error() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "asks": [["14878899", "0.1"]],
            "bids": [
                123,  // This should cause an error (number instead of array)
                ["14878897", "0.4"]
            ],
            "timestamp": 1704067200000_i64
        });

        let result = adapter.parse_orderbook(&data, "btc_jpy");
        assert!(matches!(
            result,
            Err(BitbankError::UnexpectedType {
                index: 0,
                context,
                actual_type
            }) if context == "bid levels" && actual_type == "number"
        ));
    }
}

mod http_client_tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_ticker_without_http_client_returns_error() {
        let adapter = BitbankAdapter::new();
        let result = adapter.fetch_ticker("btc_jpy").await;
        assert!(matches!(result, Err(BitbankError::HttpClientNotConfigured)));
    }

    #[tokio::test]
    async fn test_fetch_transactions_without_http_client_returns_error() {
        let adapter = BitbankAdapter::new();
        let result = adapter.fetch_transactions("btc_jpy").await;
        assert!(matches!(result, Err(BitbankError::HttpClientNotConfigured)));
    }

    #[tokio::test]
    async fn test_fetch_candlestick_without_http_client_returns_error() {
        let adapter = BitbankAdapter::new();
        let result = adapter
            .fetch_candlestick("btc_jpy", "1hour", "20240101")
            .await;
        assert!(matches!(result, Err(BitbankError::HttpClientNotConfigured)));
    }

    #[tokio::test]
    async fn test_fetch_depth_without_http_client_returns_error() {
        let adapter = BitbankAdapter::new();
        let result = adapter.fetch_depth("btc_jpy").await;
        assert!(matches!(result, Err(BitbankError::HttpClientNotConfigured)));
    }

    #[test]
    fn test_with_default_http_client() {
        let result = BitbankAdapter::with_default_http_client();
        assert!(result.is_ok());
    }
}

mod base_adapter_tests {
    use super::*;

    #[test]
    fn test_source_name() {
        let adapter = BitbankAdapter::new();
        assert_eq!(adapter.source_name(), "bitbank");
    }

    #[test]
    fn test_get_quote_mapping_contains_required_fields() {
        let adapter = BitbankAdapter::new();
        let mappings = adapter.get_quote_mapping();

        let source_fields: Vec<&str> = mappings.iter().map(|m| m.source_field.as_str()).collect();

        assert!(source_fields.contains(&"buy"));
        assert!(source_fields.contains(&"sell"));
        assert!(source_fields.contains(&"timestamp"));
        assert!(source_fields.contains(&"symbol"));
    }

    #[test]
    fn test_get_trade_mapping_contains_required_fields() {
        let adapter = BitbankAdapter::new();
        let mappings = adapter.get_trade_mapping();

        let source_fields: Vec<&str> = mappings.iter().map(|m| m.source_field.as_str()).collect();

        assert!(source_fields.contains(&"price"));
        assert!(source_fields.contains(&"amount"));
        assert!(source_fields.contains(&"side"));
        assert!(source_fields.contains(&"executed_at"));
        assert!(source_fields.contains(&"symbol"));
    }

    #[test]
    fn test_get_ohlcv_mapping_contains_required_fields() {
        let adapter = BitbankAdapter::new();
        let mappings = adapter.get_ohlcv_mapping();

        let source_fields: Vec<&str> = mappings.iter().map(|m| m.source_field.as_str()).collect();

        assert!(source_fields.contains(&"open"));
        assert!(source_fields.contains(&"high"));
        assert!(source_fields.contains(&"low"));
        assert!(source_fields.contains(&"close"));
        assert!(source_fields.contains(&"volume"));
        assert!(source_fields.contains(&"timestamp"));
        assert!(source_fields.contains(&"symbol"));
    }

    #[test]
    fn test_get_orderbook_mapping_contains_required_fields() {
        let adapter = BitbankAdapter::new();
        let mappings = adapter.get_orderbook_mapping();

        let source_fields: Vec<&str> = mappings.iter().map(|m| m.source_field.as_str()).collect();

        assert!(source_fields.contains(&"timestamp"));
        assert!(source_fields.contains(&"symbol"));
    }
}

mod conversion_tests {
    use super::*;

    #[test]
    fn test_numeric_string_conversion() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "sell": "14878899.12345",
            "buy": "14878898.54321",
            "timestamp": 1704067200000_i64
        });

        let quote = adapter.parse_quote(&data, "btc_jpy").unwrap();

        assert!((quote.bid - 14878898.54321).abs() < 1e-5);
        assert!((quote.ask - 14878899.12345).abs() < 1e-5);
    }

    #[test]
    fn test_numeric_value_conversion() {
        let adapter = BitbankAdapter::new();
        let data = json!({
            "sell": 14878899,
            "buy": 14878898,
            "timestamp": 1704067200000_i64
        });

        let quote = adapter.parse_quote(&data, "btc_jpy").unwrap();

        assert!((quote.bid - 14878898.0).abs() < f64::EPSILON);
        assert!((quote.ask - 14878899.0).abs() < f64::EPSILON);
    }
}
