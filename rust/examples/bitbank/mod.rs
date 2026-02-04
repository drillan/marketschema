//! bitbank Public API adapter.
//!
//! This module provides an adapter for fetching and parsing cryptocurrency data
//! from bitbank's Public API.
//!
//! # Usage
//!
//! See the `bitbank_demo` example for a complete usage demonstration:
//! ```bash
//! cargo run -p marketschema --example bitbank_demo -- btc_jpy
//! ```

mod adapter;
mod constants;
mod error;

pub use adapter::BitbankAdapter;
// Re-exported for tests that use #[path] to include this module.
#[allow(unused_imports)]
pub use constants::{
    BITBANK_BASE_URL, BITBANK_OHLCV_EXPECTED_LENGTH, BITBANK_OHLCV_INDEX_CLOSE,
    BITBANK_OHLCV_INDEX_HIGH, BITBANK_OHLCV_INDEX_LOW, BITBANK_OHLCV_INDEX_OPEN,
    BITBANK_OHLCV_INDEX_TIMESTAMP, BITBANK_OHLCV_INDEX_VOLUME, BITBANK_PRICE_LEVEL_EXPECTED_LENGTH,
    BITBANK_PRICE_LEVEL_INDEX_PRICE, BITBANK_PRICE_LEVEL_INDEX_SIZE, BITBANK_SUCCESS_CODE,
};
pub use error::BitbankError;
