//! Tests for BaseAdapter trait.

use marketschema_adapters::{AdapterRegistry, BaseAdapter, ModelMapping, Transforms};
use serde_json::json;
use std::sync::Arc;

// ====================
// Test Adapters
// ====================

/// Minimal adapter that only implements required method.
struct MinimalAdapter;

impl BaseAdapter for MinimalAdapter {
    fn source_name(&self) -> &'static str {
        "minimal"
    }
}

/// Sample adapter demonstrating full mapping implementation.
struct SampleAdapter;

impl BaseAdapter for SampleAdapter {
    fn source_name(&self) -> &'static str {
        "sample_exchange"
    }

    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bid", "quote.bid_price").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("ask", "quote.ask_price").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "quote.time")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
            ModelMapping::new("symbol", "quote.symbol"),
        ]
    }

    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("open", "candle.o").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("high", "candle.h").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("low", "candle.l").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("close", "candle.c").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("volume", "candle.v").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "candle.t")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    fn get_trade_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("price", "trade.price").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("size", "trade.qty").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("side", "trade.side")
                .with_transform(Transforms::side_from_string_fn()),
            ModelMapping::new("timestamp", "trade.time")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    fn get_orderbook_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bids", "depth.bids"),
            ModelMapping::new("asks", "depth.asks"),
            ModelMapping::new("timestamp", "depth.time")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    fn get_instrument_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "info.symbol"),
            ModelMapping::new("base_asset", "info.base"),
            ModelMapping::new("quote_asset", "info.quote"),
        ]
    }
}

// ====================
// Unit Tests: BaseAdapter Trait
// ====================

mod base_adapter_trait {
    use super::*;

    #[test]
    fn minimal_adapter_returns_source_name() {
        let adapter = MinimalAdapter;
        assert_eq!(adapter.source_name(), "minimal");
    }

    #[test]
    fn minimal_adapter_default_mappings_return_empty_vec() {
        let adapter = MinimalAdapter;

        assert!(
            adapter.get_quote_mapping().is_empty(),
            "Default quote mapping should be empty"
        );
        assert!(
            adapter.get_ohlcv_mapping().is_empty(),
            "Default OHLCV mapping should be empty"
        );
        assert!(
            adapter.get_trade_mapping().is_empty(),
            "Default trade mapping should be empty"
        );
        assert!(
            adapter.get_orderbook_mapping().is_empty(),
            "Default orderbook mapping should be empty"
        );
        assert!(
            adapter.get_instrument_mapping().is_empty(),
            "Default instrument mapping should be empty"
        );
    }

    #[test]
    fn adapter_trait_object_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Box<dyn BaseAdapter>>();
    }

    #[test]
    fn adapter_trait_object_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Box<dyn BaseAdapter>>();
    }
}

// ====================
// Unit Tests: Sample Adapter Mappings
// ====================

mod sample_adapter_mappings {
    use super::*;

    #[test]
    fn sample_adapter_returns_source_name() {
        let adapter = SampleAdapter;
        assert_eq!(adapter.source_name(), "sample_exchange");
    }

    #[test]
    fn quote_mapping_returns_expected_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();

        assert_eq!(mappings.len(), 4);

        let targets: Vec<&str> = mappings.iter().map(|m| m.target_field.as_str()).collect();
        assert!(targets.contains(&"bid"));
        assert!(targets.contains(&"ask"));
        assert!(targets.contains(&"timestamp"));
        assert!(targets.contains(&"symbol"));
    }

    #[test]
    fn ohlcv_mapping_returns_expected_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_ohlcv_mapping();

        assert_eq!(mappings.len(), 6);

        let targets: Vec<&str> = mappings.iter().map(|m| m.target_field.as_str()).collect();
        assert!(targets.contains(&"open"));
        assert!(targets.contains(&"high"));
        assert!(targets.contains(&"low"));
        assert!(targets.contains(&"close"));
        assert!(targets.contains(&"volume"));
        assert!(targets.contains(&"timestamp"));
    }

    #[test]
    fn trade_mapping_returns_expected_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_trade_mapping();

        assert_eq!(mappings.len(), 4);

        let targets: Vec<&str> = mappings.iter().map(|m| m.target_field.as_str()).collect();
        assert!(targets.contains(&"price"));
        assert!(targets.contains(&"size"));
        assert!(targets.contains(&"side"));
        assert!(targets.contains(&"timestamp"));
    }

    #[test]
    fn orderbook_mapping_returns_expected_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_orderbook_mapping();

        assert_eq!(mappings.len(), 3);

        let targets: Vec<&str> = mappings.iter().map(|m| m.target_field.as_str()).collect();
        assert!(targets.contains(&"bids"));
        assert!(targets.contains(&"asks"));
        assert!(targets.contains(&"timestamp"));
    }

    #[test]
    fn instrument_mapping_returns_expected_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_instrument_mapping();

        assert_eq!(mappings.len(), 3);

        let targets: Vec<&str> = mappings.iter().map(|m| m.target_field.as_str()).collect();
        assert!(targets.contains(&"symbol"));
        assert!(targets.contains(&"base_asset"));
        assert!(targets.contains(&"quote_asset"));
    }
}

// ====================
// Integration Tests: Mapping Apply
// ====================

mod mapping_apply_integration {
    use super::*;

    #[test]
    fn quote_mapping_transforms_source_data() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();

        let source = json!({
            "quote": {
                "bid_price": "123.45",
                "ask_price": "123.50",
                "time": 1704067200000_i64,
                "symbol": "BTC/USDT"
            }
        });

        // Apply bid mapping
        let bid_mapping = mappings.iter().find(|m| m.target_field == "bid").unwrap();
        let bid_value = bid_mapping.apply(&source).unwrap();
        assert_eq!(bid_value.as_f64().unwrap(), 123.45);

        // Apply ask mapping
        let ask_mapping = mappings.iter().find(|m| m.target_field == "ask").unwrap();
        let ask_value = ask_mapping.apply(&source).unwrap();
        assert_eq!(ask_value.as_f64().unwrap(), 123.50);

        // Apply timestamp mapping
        let ts_mapping = mappings
            .iter()
            .find(|m| m.target_field == "timestamp")
            .unwrap();
        let ts_value = ts_mapping.apply(&source).unwrap();
        assert!(ts_value.as_str().unwrap().contains("2024-01-01"));

        // Apply symbol mapping (no transform)
        let symbol_mapping = mappings
            .iter()
            .find(|m| m.target_field == "symbol")
            .unwrap();
        let symbol_value = symbol_mapping.apply(&source).unwrap();
        assert_eq!(symbol_value.as_str().unwrap(), "BTC/USDT");
    }

    #[test]
    fn trade_mapping_transforms_side_correctly() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_trade_mapping();

        let buy_source = json!({
            "trade": {
                "price": "100.0",
                "qty": "1.5",
                "side": "BUY",
                "time": 1704067200000_i64
            }
        });

        let side_mapping = mappings.iter().find(|m| m.target_field == "side").unwrap();
        let side_value = side_mapping.apply(&buy_source).unwrap();
        assert_eq!(side_value.as_str().unwrap(), "buy");

        let sell_source = json!({
            "trade": {
                "price": "100.0",
                "qty": "1.5",
                "side": "sell",
                "time": 1704067200000_i64
            }
        });

        let side_value = side_mapping.apply(&sell_source).unwrap();
        assert_eq!(side_value.as_str().unwrap(), "sell");
    }

    #[test]
    fn ohlcv_mapping_transforms_all_fields() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_ohlcv_mapping();

        let source = json!({
            "candle": {
                "o": "100.00",
                "h": "105.00",
                "l": "99.00",
                "c": "104.50",
                "v": "1000.5",
                "t": 1704067200000_i64
            }
        });

        // Verify all OHLCV values are transformed
        let test_cases = [
            ("open", 100.0),
            ("high", 105.0),
            ("low", 99.0),
            ("close", 104.5),
            ("volume", 1000.5),
        ];

        for (field, expected) in test_cases {
            let mapping = mappings.iter().find(|m| m.target_field == field).unwrap();
            let value = mapping.apply(&source).unwrap();
            assert_eq!(
                value.as_f64().unwrap(),
                expected,
                "Field {} should be {}",
                field,
                expected
            );
        }
    }
}

// ====================
// Integration Tests: AdapterRegistry with BaseAdapter
// ====================

mod registry_integration {
    use super::*;

    fn setup() {
        #[cfg(any(test, feature = "test-utils"))]
        AdapterRegistry::clear().unwrap();
    }

    #[test]
    fn register_and_get_adapter() {
        setup();

        AdapterRegistry::register("sample_exchange", || Box::new(SampleAdapter)).unwrap();

        let adapter = AdapterRegistry::get("sample_exchange").unwrap().unwrap();
        assert_eq!(adapter.source_name(), "sample_exchange");
    }

    #[test]
    fn get_nonexistent_adapter_returns_none() {
        setup();

        let result = AdapterRegistry::get("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn duplicate_registration_returns_error() {
        setup();

        AdapterRegistry::register("sample", || Box::new(MinimalAdapter)).unwrap();
        let result = AdapterRegistry::register("sample", || Box::new(MinimalAdapter));

        assert!(result.is_err());
    }

    #[test]
    fn list_adapters_returns_registered_names() {
        setup();

        AdapterRegistry::register("adapter_a", || Box::new(MinimalAdapter)).unwrap();
        AdapterRegistry::register("adapter_b", || Box::new(SampleAdapter)).unwrap();

        let names = AdapterRegistry::list_adapters().unwrap();
        assert!(names.contains(&"adapter_a".to_string()));
        assert!(names.contains(&"adapter_b".to_string()));
    }

    #[test]
    fn is_registered_returns_correct_status() {
        setup();

        AdapterRegistry::register("registered_one", || Box::new(MinimalAdapter)).unwrap();

        assert!(AdapterRegistry::is_registered("registered_one").unwrap());
        assert!(!AdapterRegistry::is_registered("not_registered").unwrap());
    }

    #[test]
    fn adapter_from_registry_has_working_mappings() {
        setup();

        AdapterRegistry::register("full_adapter", || Box::new(SampleAdapter)).unwrap();

        let adapter = AdapterRegistry::get("full_adapter").unwrap().unwrap();

        // Verify mappings work through the registry
        let quote_mappings = adapter.get_quote_mapping();
        assert!(!quote_mappings.is_empty());

        let source = json!({
            "quote": {
                "bid_price": "50000.00",
                "ask_price": "50001.00",
                "time": 1704067200000_i64,
                "symbol": "ETH/USDT"
            }
        });

        let bid_mapping = quote_mappings
            .iter()
            .find(|m| m.target_field == "bid")
            .unwrap();
        let bid_value = bid_mapping.apply(&source).unwrap();
        assert_eq!(bid_value.as_f64().unwrap(), 50000.0);
    }
}

// ====================
// Thread Safety Tests
// ====================

mod thread_safety {
    use super::*;
    use std::thread;

    #[test]
    fn adapter_can_be_shared_across_threads() {
        let adapter: Box<dyn BaseAdapter> = Box::new(SampleAdapter);
        let adapter = Arc::new(adapter);

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let adapter = Arc::clone(&adapter);
                thread::spawn(move || {
                    assert_eq!(adapter.source_name(), "sample_exchange");
                    let mappings = adapter.get_quote_mapping();
                    assert!(!mappings.is_empty());
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn mappings_can_be_applied_from_multiple_threads() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();
        let mappings = Arc::new(mappings);

        let source = json!({
            "quote": {
                "bid_price": "123.45",
                "ask_price": "123.50",
                "time": 1704067200000_i64,
                "symbol": "TEST"
            }
        });
        let source = Arc::new(source);

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let mappings = Arc::clone(&mappings);
                let source = Arc::clone(&source);
                thread::spawn(move || {
                    let bid_mapping = mappings.iter().find(|m| m.target_field == "bid").unwrap();
                    let value = bid_mapping.apply(&source).unwrap();
                    assert_eq!(value.as_f64().unwrap(), 123.45);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
