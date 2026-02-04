//! Stooq.com stock data adapter for testing.
//!
//! This module provides an adapter for fetching and parsing stock data from stooq.com.

mod adapter;
mod constants;
mod error;

pub use adapter::StooqAdapter;
pub use constants::{STOOQ_BASE_URL, STOOQ_EXPECTED_HEADER, STOOQ_INTERVAL_DAILY};
pub use error::StooqError;
