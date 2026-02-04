//! stockanalysis.com stock data adapter.
//!
//! This module provides an adapter for fetching and parsing stock data from stockanalysis.com.
//! The adapter handles HTML table scraping for US stock historical data.
//!
//! # Usage
//!
//! See the `stockanalysis_demo` example for a complete usage demonstration:
//! ```bash
//! cargo run -p marketschema --example stockanalysis_demo -- tsla
//! ```

mod adapter;
mod constants;
mod error;
mod models;

pub use adapter::StockAnalysisAdapter;
// Re-exported for tests that use #[path] to include this module.
// These constants and types are not directly used in the example demo but are needed for testing.
#[allow(unused_imports)]
pub use adapter::Ohlcv;
#[allow(unused_imports)]
pub use constants::{
    STOCKANALYSIS_BASE_URL, STOCKANALYSIS_EXPECTED_COLUMN_COUNT, STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE,
    STOCKANALYSIS_HTML_INDEX_CLOSE, STOCKANALYSIS_HTML_INDEX_DATE, STOCKANALYSIS_HTML_INDEX_HIGH,
    STOCKANALYSIS_HTML_INDEX_LOW, STOCKANALYSIS_HTML_INDEX_OPEN, STOCKANALYSIS_HTML_INDEX_VOLUME,
    STOCKANALYSIS_MONTH_MAP, STOCKANALYSIS_USER_AGENT,
};
pub use error::StockAnalysisError;
#[allow(unused_imports)]
pub use models::ExtendedOhlcv;
