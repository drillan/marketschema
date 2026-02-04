//! Constants for the Stooq adapter.
//!
//! All constants use the `STOOQ_` prefix following the naming convention
//! specified in CLAUDE.md.

/// Base URL for Stooq CSV API.
pub const STOOQ_BASE_URL: &str = "https://stooq.com/q/d/l/";

/// Interval parameter for daily data.
pub const STOOQ_INTERVAL_DAILY: &str = "d";

/// Expected number of columns in Stooq CSV data.
pub const STOOQ_EXPECTED_COLUMN_COUNT: usize = 6;

/// CSV column index for Date.
pub const STOOQ_CSV_INDEX_DATE: usize = 0;

/// CSV column index for Open price.
pub const STOOQ_CSV_INDEX_OPEN: usize = 1;

/// CSV column index for High price.
pub const STOOQ_CSV_INDEX_HIGH: usize = 2;

/// CSV column index for Low price.
pub const STOOQ_CSV_INDEX_LOW: usize = 3;

/// CSV column index for Close price.
pub const STOOQ_CSV_INDEX_CLOSE: usize = 4;

/// CSV column index for Volume.
pub const STOOQ_CSV_INDEX_VOLUME: usize = 5;

/// Expected CSV header columns.
pub const STOOQ_EXPECTED_HEADER: [&str; 6] = ["Date", "Open", "High", "Low", "Close", "Volume"];

/// Number of date parts (YYYY-MM-DD split by '-').
pub const STOOQ_DATE_PARTS_COUNT: usize = 3;

/// Expected length for year part in date.
pub const STOOQ_DATE_YEAR_LENGTH: usize = 4;

/// Expected length for month part in date.
pub const STOOQ_DATE_MONTH_LENGTH: usize = 2;

/// Expected length for day part in date.
pub const STOOQ_DATE_DAY_LENGTH: usize = 2;
