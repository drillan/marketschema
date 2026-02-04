//! Constants for the StockAnalysis adapter.
//!
//! All constants use the `STOCKANALYSIS_` prefix following the naming convention
//! specified in CLAUDE.md.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Base URL for stockanalysis.com stock history page.
pub const STOCKANALYSIS_BASE_URL: &str = "https://stockanalysis.com/stocks";

/// User-Agent header for HTTP requests.
///
/// A browser-like User-Agent is required to avoid being blocked by the server.
pub const STOCKANALYSIS_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Expected minimum number of columns in HTML table row.
///
/// Columns: Date, Open, High, Low, Close, Adj Close, Change, Volume
pub const STOCKANALYSIS_EXPECTED_COLUMN_COUNT: usize = 8;

/// HTML column index for Date.
pub const STOCKANALYSIS_HTML_INDEX_DATE: usize = 0;

/// HTML column index for Open price.
pub const STOCKANALYSIS_HTML_INDEX_OPEN: usize = 1;

/// HTML column index for High price.
pub const STOCKANALYSIS_HTML_INDEX_HIGH: usize = 2;

/// HTML column index for Low price.
pub const STOCKANALYSIS_HTML_INDEX_LOW: usize = 3;

/// HTML column index for Close price.
pub const STOCKANALYSIS_HTML_INDEX_CLOSE: usize = 4;

/// HTML column index for Adjusted Close price.
pub const STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE: usize = 5;

// Index 6 is Change (%) - not used

/// HTML column index for Volume.
pub const STOCKANALYSIS_HTML_INDEX_VOLUME: usize = 7;

/// Month abbreviation mapping (Jan → "01", Feb → "02", etc.).
///
/// Used to convert date format "Feb 2, 2026" to ISO 8601.
pub static STOCKANALYSIS_MONTH_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("Jan", "01"),
            ("Feb", "02"),
            ("Mar", "03"),
            ("Apr", "04"),
            ("May", "05"),
            ("Jun", "06"),
            ("Jul", "07"),
            ("Aug", "08"),
            ("Sep", "09"),
            ("Oct", "10"),
            ("Nov", "11"),
            ("Dec", "12"),
        ])
    });
