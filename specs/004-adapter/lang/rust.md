# Rust Implementation Guide

**Feature**: 004-adapter
**Date**: 2026-02-03
**Status**: Planned

## Overview

本ドキュメントは 004-adapter の Rust 実装ガイドを提供する。
現時点では設計方針のみを記載し、実装は将来対応とする。

## Design Direction

### Module Structure (Proposed)

```
crates/marketschema-adapters/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public exports
│   ├── adapter.rs       # BaseAdapter trait
│   ├── mapping.rs       # ModelMapping struct
│   ├── registry.rs      # AdapterRegistry
│   └── transforms.rs    # Transform functions
```

### BaseAdapter Trait

```rust
use async_trait::async_trait;
use crate::mapping::ModelMapping;
use crate::transforms::Transforms;
use marketschema_models::{Quote, OHLCV, Trade, OrderBook, Instrument};

/// Base trait for data source adapters.
#[async_trait]
pub trait BaseAdapter: Send + Sync {
    /// Returns the data source identifier.
    fn source_name(&self) -> &'static str;

    /// Returns field mappings for Quote model.
    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns field mappings for OHLCV model.
    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns field mappings for Trade model.
    fn get_trade_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns field mappings for OrderBook model.
    fn get_orderbook_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns field mappings for Instrument model.
    fn get_instrument_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }
}
```

### ModelMapping Struct

```rust
use std::sync::Arc;

/// Transform function type alias.
pub type TransformFn = Arc<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, TransformError> + Send + Sync>;

/// Defines how to map a source field to a target field.
#[derive(Clone)]
pub struct ModelMapping {
    /// Name of the field in the target model.
    pub target_field: String,
    /// Path to the field in the source data (supports dot notation).
    pub source_field: String,
    /// Optional transform function.
    pub transform: Option<TransformFn>,
    /// Optional default value.
    pub default: Option<serde_json::Value>,
    /// Whether the field is required.
    pub required: bool,
}

impl ModelMapping {
    /// Create a new required mapping without transform.
    pub fn new(target_field: impl Into<String>, source_field: impl Into<String>) -> Self {
        Self {
            target_field: target_field.into(),
            source_field: source_field.into(),
            transform: None,
            default: None,
            required: true,
        }
    }

    /// Add a transform function.
    pub fn with_transform(mut self, transform: TransformFn) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Set default value.
    pub fn with_default(mut self, default: serde_json::Value) -> Self {
        self.default = Some(default);
        self
    }

    /// Mark as optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}
```

### Transforms Module

```rust
use chrono::{DateTime, Utc, TimeZone, FixedOffset};
use crate::error::TransformError;

/// Collection of common transform functions.
pub struct Transforms;

impl Transforms {
    /// Convert value to f64.
    pub fn to_float(value: &serde_json::Value) -> Result<f64, TransformError> {
        match value {
            serde_json::Value::Number(n) => n.as_f64()
                .ok_or_else(|| TransformError::new(format!("Cannot convert {:?} to float", value))),
            serde_json::Value::String(s) => s.parse::<f64>()
                .map_err(|_| TransformError::new(format!("Cannot convert {:?} to float", value))),
            _ => Err(TransformError::new(format!("Cannot convert {:?} to float", value))),
        }
    }

    /// Convert Unix milliseconds to ISO 8601.
    pub fn unix_timestamp_ms(value: &serde_json::Value) -> Result<String, TransformError> {
        let ms = Self::to_float(value)? as i64;
        let secs = ms / 1000;
        let nsecs = ((ms % 1000) * 1_000_000) as u32;

        Utc.timestamp_opt(secs, nsecs)
            .single()
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .ok_or_else(|| TransformError::new(format!("Invalid timestamp: {}", ms)))
    }

    /// Normalize side string to "buy" or "sell".
    pub fn side_from_string(value: &serde_json::Value) -> Result<String, TransformError> {
        let s = value.as_str()
            .ok_or_else(|| TransformError::new(format!("Expected string, got {:?}", value)))?;

        let normalized = s.to_lowercase();
        match normalized.as_str() {
            "buy" | "bid" | "b" => Ok("buy".to_string()),
            "sell" | "ask" | "offer" | "s" | "a" => Ok("sell".to_string()),
            _ => Err(TransformError::new(format!("Cannot normalize side value: {:?}", s))),
        }
    }
}
```

### AdapterRegistry

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

/// Type for adapter factory functions.
pub type AdapterFactory = Arc<dyn Fn() -> Box<dyn BaseAdapter> + Send + Sync>;

/// Global adapter registry.
static REGISTRY: Lazy<RwLock<HashMap<String, AdapterFactory>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Adapter registry for managing adapters by source name.
pub struct AdapterRegistry;

impl AdapterRegistry {
    /// Register an adapter factory.
    pub fn register<F>(source_name: &str, factory: F)
    where
        F: Fn() -> Box<dyn BaseAdapter> + Send + Sync + 'static,
    {
        let mut registry = REGISTRY.write().unwrap();
        registry.insert(source_name.to_string(), Arc::new(factory));
    }

    /// Get an adapter instance by source name.
    pub fn get(source_name: &str) -> Option<Box<dyn BaseAdapter>> {
        let registry = REGISTRY.read().unwrap();
        registry.get(source_name).map(|factory| factory())
    }

    /// List all registered adapter names.
    pub fn list_adapters() -> Vec<String> {
        let registry = REGISTRY.read().unwrap();
        registry.keys().cloned().collect()
    }
}
```

### Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Adapter error: {0}")]
    General(String),

    #[error("Mapping error: {0}")]
    Mapping(#[from] MappingError),

    #[error("Transform error: {0}")]
    Transform(#[from] TransformError),
}

#[derive(Error, Debug)]
#[error("Mapping error: {message}")]
pub struct MappingError {
    pub message: String,
}

#[derive(Error, Debug)]
#[error("Transform error: {message}")]
pub struct TransformError {
    pub message: String,
}

impl TransformError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}
```

## Dependencies (Proposed)

```toml
# Cargo.toml
[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"

# For HTTP client
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Macro for Boilerplate Reduction (Future)

```rust
/// Proposed derive macro for adapter registration.
/// Not yet implemented.
#[derive(Adapter)]
#[adapter(source_name = "binance")]
struct BinanceAdapter {
    // ...
}
```

## Implementation Priorities

1. **Phase 1**: Core traits and structs (ModelMapping, Transforms)
2. **Phase 2**: BaseAdapter trait with serde integration
3. **Phase 3**: AdapterRegistry with thread-safe global state
4. **Phase 4**: Derive macro for boilerplate reduction
5. **Phase 5**: Integration with marketschema-models crate

## Notes

- Rust 実装は Python 実装の成熟後に着手予定
- serde との統合を重視し、JSON/MessagePack/CBOR 等をサポート予定
- async-trait を使用して非同期サポートを提供
- エラーハンドリングは thiserror を使用

## Reference

- [002-data-model spec](../../002-data-model/spec.md) - User Story 3 (Rust struct generation)
- [Rust typify](https://github.com/oxidecomputer/typify) - JSON Schema to Rust code generation
