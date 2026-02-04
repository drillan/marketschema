//! Tests for BaseAdapter trait.

use marketschema_adapters::{AdapterError, AdapterRegistry, BaseAdapter, ModelMapping, Transforms};
use serde_json::json;
use std::sync::Arc;

// ====================
// Test Constants
// ====================

/// 2024-01-01 00:00:00 UTC in milliseconds
const TEST_TIMESTAMP_MS: i64 = 1704067200000;

/// Expected ISO 8601 formatted timestamp for TEST_TIMESTAMP_MS
const TEST_TIMESTAMP_ISO: &str = "2024-01-01T00:00:00Z";

// ====================
// Test Adapters
// ====================

/// Minimal adapter that only overrides the required `source_name()` method,
/// relying on default implementations for all mapping methods.
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
                "time": TEST_TIMESTAMP_MS,
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
        assert_eq!(ts_value.as_str().unwrap(), TEST_TIMESTAMP_ISO);

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
                "time": TEST_TIMESTAMP_MS
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
                "time": TEST_TIMESTAMP_MS
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
                "t": TEST_TIMESTAMP_MS
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

    /// Generate a unique test ID for isolation
    fn unique_id() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }

    #[test]
    fn register_and_get_adapter() {
        let id = unique_id();
        let name = format!("sample_exchange_{}", id);

        // Ignore duplicate error in case of parallel runs
        let _ = AdapterRegistry::register(name.clone(), || Box::new(SampleAdapter));

        let adapter = AdapterRegistry::get(&name)
            .expect("Registry lock should not be poisoned")
            .expect("Adapter should be registered");
        assert_eq!(adapter.source_name(), "sample_exchange");
    }

    #[test]
    fn get_nonexistent_adapter_returns_none() {
        let id = unique_id();
        let name = format!("nonexistent_{}", id);

        let result = AdapterRegistry::get(&name).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn duplicate_registration_returns_error() {
        let id = unique_id();
        let name = format!("duplicate_test_{}", id);

        // First registration should succeed
        let result1 = AdapterRegistry::register(name.clone(), || Box::new(MinimalAdapter));
        assert!(result1.is_ok(), "First registration should succeed");

        // Second registration with same name should fail
        let result2 = AdapterRegistry::register(name.clone(), || Box::new(MinimalAdapter));
        match result2 {
            Err(AdapterError::DuplicateRegistration(dup_name)) => {
                assert_eq!(dup_name, name);
            }
            Err(other) => panic!("Expected DuplicateRegistration, got {:?}", other),
            Ok(_) => panic!("Expected error, got Ok"),
        }
    }

    #[test]
    fn list_adapters_returns_registered_names() {
        let id = unique_id();
        let name_a = format!("list_adapter_a_{}", id);
        let name_b = format!("list_adapter_b_{}", id);

        // Register adapters (ignore duplicates from parallel runs)
        let _ = AdapterRegistry::register(name_a.clone(), || Box::new(MinimalAdapter));
        let _ = AdapterRegistry::register(name_b.clone(), || Box::new(SampleAdapter));

        let names = AdapterRegistry::list_adapters().unwrap();
        assert!(names.contains(&name_a));
        assert!(names.contains(&name_b));
    }

    #[test]
    fn is_registered_returns_correct_status() {
        let id = unique_id();
        let name = format!("is_registered_test_{}", id);
        let nonexistent_name = format!("not_registered_{}", id);

        // Register adapter (ignore duplicates)
        let _ = AdapterRegistry::register(name.clone(), || Box::new(MinimalAdapter));

        assert!(AdapterRegistry::is_registered(&name).unwrap());
        assert!(!AdapterRegistry::is_registered(&nonexistent_name).unwrap());
    }

    #[test]
    fn adapter_from_registry_has_working_mappings() {
        let id = unique_id();
        let name = format!("full_adapter_{}", id);

        // Register adapter (ignore duplicates)
        let _ = AdapterRegistry::register(name.clone(), || Box::new(SampleAdapter));

        let adapter = AdapterRegistry::get(&name)
            .expect("Registry lock should not be poisoned")
            .expect("Adapter should be registered");

        // Verify mappings work through the registry
        let quote_mappings = adapter.get_quote_mapping();
        assert!(!quote_mappings.is_empty());

        let source = json!({
            "quote": {
                "bid_price": "50000.00",
                "ask_price": "50001.00",
                "time": TEST_TIMESTAMP_MS,
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
    use std::sync::atomic::{AtomicUsize, Ordering};
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
                "time": TEST_TIMESTAMP_MS,
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

    #[test]
    fn concurrent_registry_read_operations() {
        // Use a unique prefix to avoid interference with other tests
        let test_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Pre-register some adapters with unique names
        for i in 0..5 {
            let name = format!("concurrent_read_{}_{}", test_id, i);
            // Ignore DuplicateRegistration errors (adapter may already exist from parallel runs)
            let _ = AdapterRegistry::register(name, || Box::new(MinimalAdapter));
        }

        // Spawn multiple threads that concurrently read from the registry
        const NUM_THREADS: usize = 8;
        const READS_PER_THREAD: usize = 100;

        let completed_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..NUM_THREADS)
            .map(|_thread_id| {
                let completed_count = Arc::clone(&completed_count);
                thread::spawn(move || {
                    for i in 0..READS_PER_THREAD {
                        // Read operations: get, is_registered, list_adapters
                        let adapter_id = i % 5;
                        let name = format!("concurrent_read_{}_{}", test_id, adapter_id);

                        // Test get() - should not panic
                        let _ = AdapterRegistry::get(&name).unwrap();

                        // Test is_registered() - should not panic
                        let _ = AdapterRegistry::is_registered(&name).unwrap();

                        // Test list_adapters() - should not panic
                        let _ = AdapterRegistry::list_adapters().unwrap();

                        completed_count.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should not panic");
        }

        // Verify all iterations completed without panic
        let expected_completions = NUM_THREADS * READS_PER_THREAD;
        assert_eq!(
            completed_count.load(Ordering::Relaxed),
            expected_completions,
            "All concurrent read operations should complete without panic"
        );
    }

    #[test]
    fn concurrent_registry_write_operations() {
        // Use a unique prefix to avoid interference with other tests
        let test_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Spawn multiple threads that concurrently register adapters with unique names
        const NUM_THREADS: usize = 8;
        const REGISTRATIONS_PER_THREAD: usize = 10;

        let success_count = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..NUM_THREADS)
            .map(|thread_id| {
                let success_count = Arc::clone(&success_count);
                thread::spawn(move || {
                    for i in 0..REGISTRATIONS_PER_THREAD {
                        // Each thread registers uniquely named adapters
                        let name =
                            format!("write_{}__thread_{}__adapter_{}", test_id, thread_id, i);
                        match AdapterRegistry::register(name, || Box::new(MinimalAdapter)) {
                            Ok(()) => {
                                success_count.fetch_add(1, Ordering::Relaxed);
                            }
                            Err(AdapterError::DuplicateRegistration(_)) => {
                                // This should never happen with unique names
                                panic!("Unexpected duplicate registration");
                            }
                            Err(e) => {
                                panic!("Unexpected error: {:?}", e);
                            }
                        }
                    }
                    thread_id
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread should not panic");
        }

        // Verify all registrations succeeded
        let expected_registrations = NUM_THREADS * REGISTRATIONS_PER_THREAD;
        assert_eq!(
            success_count.load(Ordering::Relaxed),
            expected_registrations,
            "All concurrent registrations should succeed"
        );

        // Verify all adapters we registered are actually there
        for thread_id in 0..NUM_THREADS {
            for i in 0..REGISTRATIONS_PER_THREAD {
                let name = format!("write_{}__thread_{}__adapter_{}", test_id, thread_id, i);
                assert!(
                    AdapterRegistry::is_registered(&name).unwrap(),
                    "Adapter {} should be registered",
                    name
                );
            }
        }
    }

    #[test]
    fn concurrent_mixed_read_write_operations() {
        // Use a unique prefix to avoid interference with other tests
        let test_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Pre-register a base set of adapters with unique names
        for i in 0..3 {
            let name = format!("mixed_base_{}_{}", test_id, i);
            // Ignore DuplicateRegistration errors
            let _ = AdapterRegistry::register(name, || Box::new(MinimalAdapter));
        }

        const NUM_READER_THREADS: usize = 4;
        const NUM_WRITER_THREADS: usize = 4;
        const OPS_PER_THREAD: usize = 50;

        let read_completed = Arc::new(AtomicUsize::new(0));
        let write_success = Arc::new(AtomicUsize::new(0));

        // Spawn reader threads
        let reader_handles: Vec<_> = (0..NUM_READER_THREADS)
            .map(|_thread_id| {
                let read_completed = Arc::clone(&read_completed);
                thread::spawn(move || {
                    for i in 0..OPS_PER_THREAD {
                        let name = format!("mixed_base_{}_{}", test_id, i % 3);
                        // Perform read operations - should not panic
                        let _ = AdapterRegistry::is_registered(&name).unwrap();
                        read_completed.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        // Spawn writer threads
        let writer_handles: Vec<_> = (0..NUM_WRITER_THREADS)
            .map(|thread_id| {
                let write_success = Arc::clone(&write_success);
                thread::spawn(move || {
                    for i in 0..OPS_PER_THREAD {
                        let name = format!(
                            "mixed_writer_{}__thread_{}__adapter_{}",
                            test_id, thread_id, i
                        );
                        if AdapterRegistry::register(name, || Box::new(MinimalAdapter)).is_ok() {
                            write_success.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                })
            })
            .collect();

        // Wait for all threads
        for handle in reader_handles {
            handle.join().expect("Reader thread should not panic");
        }
        for handle in writer_handles {
            handle.join().expect("Writer thread should not panic");
        }

        // Verify all read operations completed without panic
        let expected_reads = NUM_READER_THREADS * OPS_PER_THREAD;
        assert_eq!(
            read_completed.load(Ordering::Relaxed),
            expected_reads,
            "All read operations should complete"
        );

        // Verify all write operations succeeded
        let expected_writes = NUM_WRITER_THREADS * OPS_PER_THREAD;
        assert_eq!(
            write_success.load(Ordering::Relaxed),
            expected_writes,
            "All write operations should succeed"
        );
    }

    #[test]
    fn clear_provides_test_isolation() {
        // First, register an adapter
        AdapterRegistry::clear().unwrap();
        AdapterRegistry::register("isolation_test_adapter", || Box::new(MinimalAdapter)).unwrap();
        assert!(AdapterRegistry::is_registered("isolation_test_adapter").unwrap());

        // Clear and verify it's gone
        AdapterRegistry::clear().unwrap();
        assert!(!AdapterRegistry::is_registered("isolation_test_adapter").unwrap());

        // Verify we can re-register the same name
        let result =
            AdapterRegistry::register("isolation_test_adapter", || Box::new(MinimalAdapter));
        assert!(result.is_ok(), "Should be able to re-register after clear");
    }
}

// ====================
// Error Path Tests
// ====================

mod mapping_error_paths {
    use super::*;

    #[test]
    fn quote_mapping_returns_error_when_required_field_missing() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();

        // Missing bid_price field
        let source = json!({
            "quote": {
                "ask_price": "123.50",
                "time": TEST_TIMESTAMP_MS,
                "symbol": "BTC/USDT"
            }
        });

        let bid_mapping = mappings
            .iter()
            .find(|m| m.target_field == "bid")
            .expect("bid mapping should exist");
        let result = bid_mapping.apply(&source);
        assert!(result.is_err(), "Missing field should return error");
    }

    #[test]
    fn trade_mapping_returns_error_for_invalid_side_value() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_trade_mapping();

        // "long" is not a valid side value (only "buy"/"sell" variants are supported)
        let source = json!({
            "trade": {
                "price": "100.0",
                "qty": "1.5",
                "side": "long",
                "time": TEST_TIMESTAMP_MS
            }
        });

        let side_mapping = mappings
            .iter()
            .find(|m| m.target_field == "side")
            .expect("side mapping should exist");
        let result = side_mapping.apply(&source);
        assert!(result.is_err(), "Invalid side value should return error");
    }

    #[test]
    fn float_conversion_returns_error_for_invalid_string() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();

        // "not_a_number" cannot be parsed as float
        let source = json!({
            "quote": {
                "bid_price": "not_a_number",
                "ask_price": "123.50",
                "time": TEST_TIMESTAMP_MS,
                "symbol": "BTC/USDT"
            }
        });

        let bid_mapping = mappings
            .iter()
            .find(|m| m.target_field == "bid")
            .expect("bid mapping should exist");
        let result = bid_mapping.apply(&source);
        assert!(result.is_err(), "Invalid float string should return error");
    }

    #[test]
    fn nested_path_returns_error_when_parent_missing() {
        let adapter = SampleAdapter;
        let mappings = adapter.get_quote_mapping();

        // "quote" parent object is entirely missing
        let source = json!({
            "other": {
                "bid_price": "123.45"
            }
        });

        let bid_mapping = mappings
            .iter()
            .find(|m| m.target_field == "bid")
            .expect("bid mapping should exist");
        let result = bid_mapping.apply(&source);
        assert!(result.is_err(), "Missing parent object should return error");
    }
}
