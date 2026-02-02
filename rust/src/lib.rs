//! marketschema - Unified market data schema for financial applications
//!
//! This crate provides Rust struct definitions generated from JSON Schema
//! for market data types like Quote, OHLCV, Trade, OrderBook, and Instrument.

pub mod types;

// Re-export commonly used types at crate root
pub use types::quote::Quote;
pub use types::ohlcv::Ohlcv;
pub use types::trade::Trade;
pub use types::orderbook::OrderBook;
pub use types::instrument::Instrument;
