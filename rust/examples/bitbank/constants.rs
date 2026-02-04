//! Constants for the bitbank adapter.
//!
//! All constants use the `BITBANK_` prefix following the naming convention
//! specified in CLAUDE.md.

/// Base URL for bitbank Public API.
pub const BITBANK_BASE_URL: &str = "https://public.bitbank.cc";

/// Success code returned by bitbank API when request is successful.
pub const BITBANK_SUCCESS_CODE: i64 = 1;

/// Index constants for bitbank candlestick array [open, high, low, close, volume, timestamp].
pub const BITBANK_OHLCV_INDEX_OPEN: usize = 0;
/// Index for high price in OHLCV array.
pub const BITBANK_OHLCV_INDEX_HIGH: usize = 1;
/// Index for low price in OHLCV array.
pub const BITBANK_OHLCV_INDEX_LOW: usize = 2;
/// Index for close price in OHLCV array.
pub const BITBANK_OHLCV_INDEX_CLOSE: usize = 3;
/// Index for volume in OHLCV array.
pub const BITBANK_OHLCV_INDEX_VOLUME: usize = 4;
/// Index for timestamp in OHLCV array.
pub const BITBANK_OHLCV_INDEX_TIMESTAMP: usize = 5;
/// Expected number of elements in OHLCV array.
pub const BITBANK_OHLCV_EXPECTED_LENGTH: usize = 6;

/// Index constants for bitbank price level array [price, size].
pub const BITBANK_PRICE_LEVEL_INDEX_PRICE: usize = 0;
/// Index for size in price level array.
pub const BITBANK_PRICE_LEVEL_INDEX_SIZE: usize = 1;
/// Expected number of elements in price level array.
pub const BITBANK_PRICE_LEVEL_EXPECTED_LENGTH: usize = 2;
