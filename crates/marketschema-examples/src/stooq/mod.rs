//! Stooq.com stock data adapter.
//!
//! This module provides an adapter for fetching and parsing stock data from stooq.com.
//!
//! # Example
//!
//! ```rust,no_run
//! use marketschema_examples::stooq::StooqAdapter;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let adapter = StooqAdapter::with_default_http_client()?;
//!     let ohlcv_data = adapter.fetch_and_parse("spy.us").await?;
//!
//!     for ohlcv in ohlcv_data.iter().take(5) {
//!         println!("{}: O={} H={} L={} C={} V={}",
//!             ohlcv.timestamp, ohlcv.open, ohlcv.high, ohlcv.low, ohlcv.close, ohlcv.volume);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod adapter;
mod constants;
mod error;

pub use adapter::{Ohlcv, StooqAdapter};
pub use constants::{
    STOOQ_BASE_URL, STOOQ_CSV_INDEX_CLOSE, STOOQ_CSV_INDEX_DATE, STOOQ_CSV_INDEX_HIGH,
    STOOQ_CSV_INDEX_LOW, STOOQ_CSV_INDEX_OPEN, STOOQ_CSV_INDEX_VOLUME, STOOQ_EXPECTED_COLUMN_COUNT,
    STOOQ_EXPECTED_HEADER, STOOQ_INTERVAL_DAILY,
};
pub use error::StooqError;
