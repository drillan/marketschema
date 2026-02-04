//! Adapter framework for marketschema data transformations.
//!
//! This crate provides the infrastructure for building adapters that transform
//! data from external sources into marketschema's standardized data models.
//!
//! # Main Components
//!
//! - [`BaseAdapter`] - The core trait that all adapters must implement
//! - [`ModelMapping`] - Field mapping configuration with builder pattern
//! - [`Transforms`] - Common transformation functions for data conversion
//! - [`AdapterRegistry`] - Thread-safe global registry for adapter discovery
//!
//! # Quick Start
//!
//! ## Creating an Adapter
//!
//! ```
//! use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
//! use async_trait::async_trait;
//!
//! struct MyApiAdapter;
//!
//! #[async_trait]
//! impl BaseAdapter for MyApiAdapter {
//!     fn source_name(&self) -> &'static str {
//!         "myapi"
//!     }
//!
//!     fn get_quote_mapping(&self) -> Vec<ModelMapping> {
//!         vec![
//!             ModelMapping::new("bid", "ticker.bid")
//!                 .with_transform(Transforms::to_float_fn()),
//!             ModelMapping::new("ask", "ticker.ask")
//!                 .with_transform(Transforms::to_float_fn()),
//!         ]
//!     }
//! }
//! ```
//!
//! ## Registering and Using Adapters
//!
//! ```
//! use marketschema_adapters::{BaseAdapter, AdapterRegistry, ModelMapping};
//! use async_trait::async_trait;
//! use serde_json::json;
//! # struct MyApiAdapter;
//! # #[async_trait]
//! # impl BaseAdapter for MyApiAdapter {
//! #     fn source_name(&self) -> &'static str { "myapi" }
//! # }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Register the adapter
//! AdapterRegistry::register("myapi", || Box::new(MyApiAdapter))?;
//!
//! // List registered adapters
//! let adapters = AdapterRegistry::list_adapters()?;
//! assert!(adapters.contains(&"myapi".to_string()));
//!
//! // Retrieve and use an adapter
//! match AdapterRegistry::get("myapi")? {
//!     Some(adapter) => println!("Source: {}", adapter.source_name()),
//!     None => eprintln!("Warning: Adapter 'myapi' not found"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Transform Functions
//!
//! All transform functions return results wrapped in `serde_json::Value`.
//!
//! | Function | Description |
//! |----------|-------------|
//! | [`Transforms::to_float_fn`] | Convert string/number to `f64` |
//! | [`Transforms::to_int_fn`] | Convert string/number to `i64` |
//! | [`Transforms::iso_timestamp_fn`] | Parse RFC 3339 timestamps and normalize to UTC with Z suffix |
//! | [`Transforms::unix_timestamp_ms_fn`] | Convert Unix milliseconds to ISO 8601 (second precision) |
//! | [`Transforms::unix_timestamp_sec_fn`] | Convert Unix seconds to ISO 8601 |
//! | [`Transforms::jst_to_utc_fn`] | Convert JST timestamps to UTC (accepts RFC 3339 or naive datetime) |
//! | [`Transforms::side_from_string_fn`] | Normalize side values: buy/bid/b → "buy", sell/ask/offer/s/a → "sell" |
//! | [`Transforms::uppercase_fn`] | Convert to uppercase |
//! | [`Transforms::lowercase_fn`] | Convert to lowercase |

mod adapter;
mod error;
mod mapping;
mod registry;
mod transforms;

pub use adapter::BaseAdapter;
pub use error::{AdapterError, MappingError, TransformError};
pub use mapping::{ModelMapping, TransformFn};
pub use registry::{AdapterFactory, AdapterRegistry};
pub use transforms::{
    JST_UTC_OFFSET_HOURS, MS_PER_SECOND, NS_PER_MS, SECONDS_PER_HOUR, Transforms,
};
