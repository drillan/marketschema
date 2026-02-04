//! Stooq.com stock data adapter.
//!
//! This module provides an adapter for fetching and parsing stock data from stooq.com.
//!
//! # Usage
//!
//! See the `stooq_demo` example for a complete usage demonstration:
//! ```bash
//! cargo run -p marketschema --example stooq_demo -- spy.us
//! ```

mod adapter;
mod constants;
mod error;

pub use adapter::StooqAdapter;
// Re-exported for tests that use #[path] to include this module.
// These constants are not directly used in the example demo but are needed for testing.
#[allow(unused_imports)]
pub use constants::{STOOQ_BASE_URL, STOOQ_EXPECTED_HEADER, STOOQ_INTERVAL_DAILY};
pub use error::StooqError;
