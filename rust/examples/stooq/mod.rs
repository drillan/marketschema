//! Stooq.com stock data adapter.
//!
//! This module provides an adapter for fetching and parsing stock data from stooq.com.
//!
//! # Example
//!
//! ```rust,no_run
//! # // This example is disabled because examples cannot be directly imported
//! # // Use the stooq_demo example to see usage
//! ```

mod adapter;
mod constants;
mod error;

pub use adapter::StooqAdapter;
pub use error::StooqError;
