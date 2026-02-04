//! Models for the StockAnalysis adapter.

/// Extended OHLCV data structure with adjusted close price.
///
/// This model extends the standard OHLCV structure with an adjusted close price field,
/// which accounts for stock splits and dividend adjustments. This data is specifically
/// available from stockanalysis.com historical data tables.
///
/// This is a lightweight intermediate representation used during HTML parsing,
/// separate from the library's `marketschema::Ohlcv` type. This allows the
/// adapter to perform initial data transformation without depending on the
/// full schema type system.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedOhlcv {
    /// Stock symbol (e.g., "TSLA", "AAPL").
    pub symbol: String,
    /// ISO 8601 timestamp (e.g., "2026-02-02T00:00:00Z").
    pub timestamp: String,
    /// Opening price.
    pub open: f64,
    /// Highest price.
    pub high: f64,
    /// Lowest price.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Adjusted close price (accounts for stock splits and dividend adjustments).
    pub adj_close: f64,
    /// Trading volume.
    pub volume: f64,
}
