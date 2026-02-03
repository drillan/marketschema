# Adapter Contract (Rust)

**Feature**: 004-adapter-rust
**Parent Spec**: [004-adapter-rust](../spec.md)
**Date**: 2026-02-03
**Status**: Planned

## BaseAdapter Trait

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

## ModelMapping Struct

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

    /// Apply the mapping to source data.
    pub fn apply(&self, source_data: &serde_json::Value) -> Result<serde_json::Value, MappingError> {
        // Implementation: get nested value, apply transform, handle defaults
        ...
    }
}
```

## AdapterRegistry

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
    pub fn register<F>(source_name: &str, factory: F) -> Result<(), AdapterError>
    where
        F: Fn() -> Box<dyn BaseAdapter> + Send + Sync + 'static,
    {
        let mut registry = REGISTRY.write().unwrap();
        if registry.contains_key(source_name) {
            return Err(AdapterError::DuplicateRegistration(source_name.to_string()));
        }
        registry.insert(source_name.to_string(), Arc::new(factory));
        Ok(())
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

    /// Check if an adapter is registered.
    pub fn is_registered(source_name: &str) -> bool {
        let registry = REGISTRY.read().unwrap();
        registry.contains_key(source_name)
    }

    /// Clear all registered adapters (for testing).
    pub fn clear() {
        let mut registry = REGISTRY.write().unwrap();
        registry.clear();
    }
}
```

## Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Adapter error: {0}")]
    General(String),

    #[error("Duplicate registration for source: {0}")]
    DuplicateRegistration(String),

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

impl MappingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
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

## Usage Example

```rust
use marketschema_adapters::{BaseAdapter, ModelMapping, AdapterRegistry, Transforms};
use async_trait::async_trait;

struct MyApiAdapter;

#[async_trait]
impl BaseAdapter for MyApiAdapter {
    fn source_name(&self) -> &'static str {
        "myapi"
    }

    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bid", "bid_price")
                .with_transform(Transforms::to_float()),
            ModelMapping::new("ask", "ask_price")
                .with_transform(Transforms::to_float()),
            ModelMapping::new("timestamp", "time")
                .with_transform(Transforms::unix_timestamp_ms()),
        ]
    }
}

// Register the adapter
fn register_adapters() {
    AdapterRegistry::register("myapi", || Box::new(MyApiAdapter)).unwrap();
}

// Get and use the adapter
fn main() {
    register_adapters();

    if let Some(adapter) = AdapterRegistry::get("myapi") {
        println!("Using adapter: {}", adapter.source_name());
    }
}
```
