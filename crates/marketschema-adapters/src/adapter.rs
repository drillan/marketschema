//! Base adapter trait definition.

use crate::mapping::ModelMapping;
use async_trait::async_trait;

/// Base trait for all adapters.
///
/// Adapters transform data from external sources into marketschema's
/// standardized data models (Quote, OHLCV, Trade, OrderBook, Instrument).
///
/// # Example
///
/// ```ignore
/// use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
/// use async_trait::async_trait;
///
/// struct MyAdapter;
///
/// #[async_trait]
/// impl BaseAdapter for MyAdapter {
///     fn source_name(&self) -> &'static str {
///         "myapi"
///     }
///
///     fn get_quote_mapping(&self) -> Vec<ModelMapping> {
///         vec![
///             ModelMapping::new("bid", "data.bid")
///                 .with_transform(Transforms::to_float_fn()),
///             ModelMapping::new("ask", "data.ask")
///                 .with_transform(Transforms::to_float_fn()),
///         ]
///     }
/// }
/// ```
#[async_trait]
pub trait BaseAdapter: Send + Sync {
    /// Returns the unique identifier for this data source.
    fn source_name(&self) -> &'static str;

    /// Returns the field mappings for Quote model.
    ///
    /// The default implementation returns an empty vector, indicating
    /// that this adapter does not support Quote model transformation.
    /// Override this method to provide Quote model support.
    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns the field mappings for OHLCV model.
    ///
    /// The default implementation returns an empty vector, indicating
    /// that this adapter does not support OHLCV model transformation.
    /// Override this method to provide OHLCV model support.
    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns the field mappings for Trade model.
    ///
    /// The default implementation returns an empty vector, indicating
    /// that this adapter does not support Trade model transformation.
    /// Override this method to provide Trade model support.
    fn get_trade_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns the field mappings for OrderBook model.
    ///
    /// The default implementation returns an empty vector, indicating
    /// that this adapter does not support OrderBook model transformation.
    /// Override this method to provide OrderBook model support.
    fn get_orderbook_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }

    /// Returns the field mappings for Instrument model.
    ///
    /// The default implementation returns an empty vector, indicating
    /// that this adapter does not support Instrument model transformation.
    /// Override this method to provide Instrument model support.
    fn get_instrument_mapping(&self) -> Vec<ModelMapping> {
        Vec::new()
    }
}
