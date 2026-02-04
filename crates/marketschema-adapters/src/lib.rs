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
//! # Example
//!
//! ```ignore
//! use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
//! use async_trait::async_trait;
//!
//! struct MyAdapter;
//!
//! #[async_trait]
//! impl BaseAdapter for MyAdapter {
//!     fn source_name(&self) -> &'static str {
//!         "myapi"
//!     }
//! }
//! ```

mod adapter;
mod error;
mod mapping;
mod registry;
mod transforms;

pub use adapter::BaseAdapter;
pub use error::{AdapterError, MappingError, TransformError};
pub use mapping::{ModelMapping, TransformFn};
pub use registry::AdapterRegistry;
pub use transforms::{
    JST_UTC_OFFSET_HOURS, MS_PER_SECOND, NS_PER_MS, SECONDS_PER_HOUR, Transforms,
};
