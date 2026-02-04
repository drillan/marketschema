//! marketschema - Unified market data schema for financial applications
//!
//! This crate provides Rust struct definitions generated from JSON Schema
//! for market data types like Quote, OHLCV, Trade, OrderBook, and Instrument.

pub mod types;

// Re-export commonly used types at crate root
pub use types::derivative_info::DerivativeInfo;
pub use types::expiry_info::ExpiryInfo;
pub use types::instrument::Instrument;
pub use types::ohlcv::Ohlcv;
pub use types::option_info::OptionInfo;
pub use types::orderbook::OrderBook;
pub use types::quote::Quote;
pub use types::trade::Trade;
pub use types::volume_info::VolumeInfo;
