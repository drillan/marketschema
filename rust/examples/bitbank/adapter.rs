//! bitbank adapter implementation.
//!
//! This adapter transforms JSON data from bitbank's Public API into marketschema models.
//!
//! # API Documentation
//!
//! - <https://github.com/bitbankinc/bitbank-api-docs/blob/master/public-api.md>
//!
//! # Supported Endpoints
//!
//! | Endpoint                                | Output Model |
//! |-----------------------------------------|--------------|
//! | `/{pair}/ticker`                        | Quote        |
//! | `/{pair}/transactions`                  | Trade[]      |
//! | `/{pair}/candlestick/{type}/{yyyymmdd}` | OHLCV[]      |
//! | `/{pair}/depth`                         | OrderBook    |
//!
//! # Design Note: Adapter-local types
//!
//! This adapter defines its own `Quote`, `Trade`, `Ohlcv`, `OrderBook`, and `PriceLevel`
//! types rather than using types from the `marketschema` crate. This is intentional:
//!
//! - **Lightweight intermediate representation**: These types serve as a simple,
//!   dependency-free output format for the adapter.
//! - **Decoupling**: Keeps the example adapter independent of schema evolution
//!   in the core `marketschema` crate.
//! - **Flexibility**: Users can easily map these types to their own domain models
//!   or directly to `marketschema` types if desired.
//!
//! This follows the same pattern used by the Stooq adapter example.

use std::sync::Arc;

use async_trait::async_trait;
use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
use serde_json::Value;

use super::constants::{
    BITBANK_BASE_URL, BITBANK_OHLCV_EXPECTED_LENGTH, BITBANK_OHLCV_INDEX_CLOSE,
    BITBANK_OHLCV_INDEX_HIGH, BITBANK_OHLCV_INDEX_LOW, BITBANK_OHLCV_INDEX_OPEN,
    BITBANK_OHLCV_INDEX_TIMESTAMP, BITBANK_OHLCV_INDEX_VOLUME, BITBANK_PRICE_LEVEL_EXPECTED_LENGTH,
    BITBANK_PRICE_LEVEL_INDEX_PRICE, BITBANK_PRICE_LEVEL_INDEX_SIZE, BITBANK_SUCCESS_CODE,
};
use super::error::BitbankError;

/// Quote data structure for bitbank adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct Quote {
    /// Trading pair symbol (e.g., "btc_jpy").
    pub symbol: String,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Bid (buy) price.
    pub bid: f64,
    /// Ask (sell) price.
    pub ask: f64,
}

/// Trade data structure for bitbank adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct Trade {
    /// Trading pair symbol (e.g., "btc_jpy").
    pub symbol: String,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Trade price.
    pub price: f64,
    /// Trade size.
    pub size: f64,
    /// Trade side ("buy" or "sell").
    pub side: String,
}

/// OHLCV data structure for bitbank adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct Ohlcv {
    /// Trading pair symbol (e.g., "btc_jpy").
    pub symbol: String,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Opening price.
    pub open: f64,
    /// Highest price.
    pub high: f64,
    /// Lowest price.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Trading volume.
    pub volume: f64,
}

/// Price level in order book.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceLevel {
    /// Price.
    pub price: f64,
    /// Size.
    pub size: f64,
}

/// OrderBook data structure for bitbank adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBook {
    /// Trading pair symbol (e.g., "btc_jpy").
    pub symbol: String,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Ask (sell) side price levels, sorted by price ascending.
    pub asks: Vec<PriceLevel>,
    /// Bid (buy) side price levels, sorted by price descending.
    pub bids: Vec<PriceLevel>,
}

/// Adapter for bitbank Public API.
///
/// Transforms JSON data from bitbank's Public API endpoints into standardized
/// market data models.
///
/// # Note
///
/// bitbank API responses do not include symbol information in the data payload.
/// Symbol must be provided as a parameter to parse methods.
pub struct BitbankAdapter {
    http_client: Option<Arc<AsyncHttpClient>>,
}

impl BitbankAdapter {
    /// Create a new `BitbankAdapter` without HTTP client.
    ///
    /// Use [`BitbankAdapter::with_http_client`] to create an adapter with HTTP capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self { http_client: None }
    }

    /// Create a new `BitbankAdapter` with the given HTTP client.
    #[must_use]
    #[allow(dead_code)]
    pub fn with_http_client(http_client: Arc<AsyncHttpClient>) -> Self {
        Self {
            http_client: Some(http_client),
        }
    }

    /// Create a new `BitbankAdapter` with a default HTTP client.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    pub fn with_default_http_client() -> Result<Self, BitbankError> {
        let client = AsyncHttpClientBuilder::new()
            .build()
            .map_err(BitbankError::Http)?;
        Ok(Self {
            http_client: Some(Arc::new(client)),
        })
    }

    /// Validate bitbank API response.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw JSON response from bitbank API.
    ///
    /// # Errors
    ///
    /// Returns [`BitbankError::ApiError`] if response indicates an error (success != 1).
    /// Returns [`BitbankError::MissingField`] if "success" field is missing.
    pub fn validate_response(&self, data: &Value) -> Result<(), BitbankError> {
        let success = data
            .get("success")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| BitbankError::MissingField {
                field: "success".to_string(),
                context: "API response".to_string(),
            })?;

        if success != BITBANK_SUCCESS_CODE {
            return Err(BitbankError::ApiError {
                success_code: success,
                response: data.to_string(),
            });
        }

        Ok(())
    }

    /// Extract "data" field from API response.
    fn extract_data<'a>(
        &self,
        response: &'a Value,
        context: &str,
    ) -> Result<&'a Value, BitbankError> {
        response
            .get("data")
            .ok_or_else(|| BitbankError::MissingField {
                field: "data".to_string(),
                context: context.to_string(),
            })
    }

    /// Get value type description for error messages.
    fn value_type_name(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }

    /// Validate that all elements in a slice are arrays and return them.
    ///
    /// Unlike `filter_map`, this method returns an error if any element
    /// is not an array, following the "no implicit fallback" rule.
    fn validate_array_elements<'a>(
        &self,
        values: &'a [Value],
        context: &str,
    ) -> Result<Vec<&'a Vec<Value>>, BitbankError> {
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                value.as_array().ok_or_else(|| BitbankError::UnexpectedType {
                    index,
                    context: context.to_string(),
                    actual_type: Self::value_type_name(value).to_string(),
                })
            })
            .collect()
    }

    /// Parse a JSON value to f64.
    fn parse_float(&self, value: &Value, field_name: &str) -> Result<f64, BitbankError> {
        Transforms::to_float(value).map_err(|e| BitbankError::Conversion {
            message: format!("Failed to parse '{}': {}", field_name, e),
        })
    }

    /// Convert Unix milliseconds timestamp to ISO 8601 string.
    fn unix_ms_to_iso(&self, value: &Value) -> Result<String, BitbankError> {
        Transforms::unix_timestamp_ms(value).map_err(|e| BitbankError::Conversion {
            message: format!("Failed to convert timestamp: {}", e),
        })
    }

    /// Parse bitbank ticker data into Quote model.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Ticker response "data" field content.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if required fields are missing or cannot be converted.
    pub fn parse_quote(&self, raw_data: &Value, symbol: &str) -> Result<Quote, BitbankError> {
        let buy = raw_data
            .get("buy")
            .ok_or_else(|| BitbankError::MissingField {
                field: "buy".to_string(),
                context: "ticker response".to_string(),
            })?;
        let sell = raw_data
            .get("sell")
            .ok_or_else(|| BitbankError::MissingField {
                field: "sell".to_string(),
                context: "ticker response".to_string(),
            })?;
        let timestamp_raw =
            raw_data
                .get("timestamp")
                .ok_or_else(|| BitbankError::MissingField {
                    field: "timestamp".to_string(),
                    context: "ticker response".to_string(),
                })?;

        Ok(Quote {
            symbol: symbol.to_string(),
            bid: self.parse_float(buy, "buy")?,
            ask: self.parse_float(sell, "sell")?,
            timestamp: self.unix_ms_to_iso(timestamp_raw)?,
        })
    }

    /// Parse bitbank transaction data into Trade model.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Single transaction object from transactions response.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if required fields are missing or cannot be converted.
    pub fn parse_trade(&self, raw_data: &Value, symbol: &str) -> Result<Trade, BitbankError> {
        let price = raw_data
            .get("price")
            .ok_or_else(|| BitbankError::MissingField {
                field: "price".to_string(),
                context: "transaction".to_string(),
            })?;
        let amount = raw_data
            .get("amount")
            .ok_or_else(|| BitbankError::MissingField {
                field: "amount".to_string(),
                context: "transaction".to_string(),
            })?;
        let side_raw = raw_data
            .get("side")
            .ok_or_else(|| BitbankError::MissingField {
                field: "side".to_string(),
                context: "transaction".to_string(),
            })?;
        let executed_at =
            raw_data
                .get("executed_at")
                .ok_or_else(|| BitbankError::MissingField {
                    field: "executed_at".to_string(),
                    context: "transaction".to_string(),
                })?;

        let side =
            Transforms::side_from_string(side_raw).map_err(|e| BitbankError::Conversion {
                message: format!("Failed to parse side: {}", e),
            })?;

        Ok(Trade {
            symbol: symbol.to_string(),
            price: self.parse_float(price, "price")?,
            size: self.parse_float(amount, "amount")?,
            side,
            timestamp: self.unix_ms_to_iso(executed_at)?,
        })
    }

    /// Parse multiple bitbank transactions into Trade models.
    ///
    /// # Arguments
    ///
    /// * `transactions` - Array of transaction objects.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if any transaction fails to parse.
    pub fn parse_trades(
        &self,
        transactions: &[Value],
        symbol: &str,
    ) -> Result<Vec<Trade>, BitbankError> {
        transactions
            .iter()
            .map(|tx| self.parse_trade(tx, symbol))
            .collect()
    }

    /// Parse bitbank candlestick array into OHLCV model.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Candlestick array `[open, high, low, close, volume, timestamp]`.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if array has insufficient elements or values cannot be converted.
    pub fn parse_ohlcv(&self, raw_data: &[Value], symbol: &str) -> Result<Ohlcv, BitbankError> {
        if raw_data.len() < BITBANK_OHLCV_EXPECTED_LENGTH {
            return Err(BitbankError::InsufficientArrayLength {
                expected: BITBANK_OHLCV_EXPECTED_LENGTH,
                actual: raw_data.len(),
                context: "OHLCV data".to_string(),
            });
        }

        Ok(Ohlcv {
            symbol: symbol.to_string(),
            open: self.parse_float(&raw_data[BITBANK_OHLCV_INDEX_OPEN], "open")?,
            high: self.parse_float(&raw_data[BITBANK_OHLCV_INDEX_HIGH], "high")?,
            low: self.parse_float(&raw_data[BITBANK_OHLCV_INDEX_LOW], "low")?,
            close: self.parse_float(&raw_data[BITBANK_OHLCV_INDEX_CLOSE], "close")?,
            volume: self.parse_float(&raw_data[BITBANK_OHLCV_INDEX_VOLUME], "volume")?,
            timestamp: self.unix_ms_to_iso(&raw_data[BITBANK_OHLCV_INDEX_TIMESTAMP])?,
        })
    }

    /// Parse multiple bitbank candlestick arrays into OHLCV models.
    ///
    /// # Arguments
    ///
    /// * `ohlcv_arrays` - Array of candlestick arrays.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if any candlestick fails to parse.
    pub fn parse_ohlcv_batch(
        &self,
        ohlcv_arrays: &[Value],
        symbol: &str,
    ) -> Result<Vec<Ohlcv>, BitbankError> {
        let arrays = self.validate_array_elements(ohlcv_arrays, "OHLCV data")?;
        arrays
            .into_iter()
            .map(|arr| self.parse_ohlcv(arr, symbol))
            .collect()
    }

    /// Parse a single price level array into PriceLevel.
    fn parse_price_level(
        &self,
        level: &[Value],
        context: &str,
    ) -> Result<PriceLevel, BitbankError> {
        if level.len() < BITBANK_PRICE_LEVEL_EXPECTED_LENGTH {
            return Err(BitbankError::InsufficientArrayLength {
                expected: BITBANK_PRICE_LEVEL_EXPECTED_LENGTH,
                actual: level.len(),
                context: context.to_string(),
            });
        }

        Ok(PriceLevel {
            price: self.parse_float(&level[BITBANK_PRICE_LEVEL_INDEX_PRICE], "price")?,
            size: self.parse_float(&level[BITBANK_PRICE_LEVEL_INDEX_SIZE], "size")?,
        })
    }

    /// Parse bitbank depth data into OrderBook model.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Depth response "data" field content.
    /// * `symbol` - Trading pair symbol (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// Returns error if required fields are missing or cannot be converted.
    pub fn parse_orderbook(
        &self,
        raw_data: &Value,
        symbol: &str,
    ) -> Result<OrderBook, BitbankError> {
        let asks_raw = raw_data
            .get("asks")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BitbankError::MissingField {
                field: "asks".to_string(),
                context: "depth response".to_string(),
            })?;

        let bids_raw = raw_data
            .get("bids")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BitbankError::MissingField {
                field: "bids".to_string(),
                context: "depth response".to_string(),
            })?;

        let timestamp_raw =
            raw_data
                .get("timestamp")
                .ok_or_else(|| BitbankError::MissingField {
                    field: "timestamp".to_string(),
                    context: "depth response".to_string(),
                })?;

        let ask_arrays = self.validate_array_elements(asks_raw, "ask levels")?;
        let asks: Result<Vec<PriceLevel>, BitbankError> = ask_arrays
            .into_iter()
            .map(|arr| self.parse_price_level(arr, "ask level"))
            .collect();

        let bid_arrays = self.validate_array_elements(bids_raw, "bid levels")?;
        let bids: Result<Vec<PriceLevel>, BitbankError> = bid_arrays
            .into_iter()
            .map(|arr| self.parse_price_level(arr, "bid level"))
            .collect();

        Ok(OrderBook {
            symbol: symbol.to_string(),
            timestamp: self.unix_ms_to_iso(timestamp_raw)?,
            asks: asks?,
            bids: bids?,
        })
    }

    /// Fetch ticker data and return Quote.
    ///
    /// # Arguments
    ///
    /// * `pair` - Trading pair (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// * [`BitbankError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`BitbankError::Http`] - If HTTP request fails.
    /// * [`BitbankError::ApiError`] - If API returns error.
    /// * [`BitbankError::MissingField`] - If response format is invalid.
    pub async fn fetch_ticker(&self, pair: &str) -> Result<Quote, BitbankError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(BitbankError::HttpClientNotConfigured)?;

        let url = format!("{}/{}/ticker", BITBANK_BASE_URL, pair);
        let response = client.get_json(&url).await.map_err(BitbankError::Http)?;

        self.validate_response(&response)?;
        let data = self.extract_data(&response, "ticker response")?;
        self.parse_quote(data, pair)
    }

    /// Fetch transactions and return list of Trade.
    ///
    /// # Arguments
    ///
    /// * `pair` - Trading pair (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// * [`BitbankError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`BitbankError::Http`] - If HTTP request fails.
    /// * [`BitbankError::ApiError`] - If API returns error.
    /// * [`BitbankError::MissingField`] - If response format is invalid.
    pub async fn fetch_transactions(&self, pair: &str) -> Result<Vec<Trade>, BitbankError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(BitbankError::HttpClientNotConfigured)?;

        let url = format!("{}/{}/transactions", BITBANK_BASE_URL, pair);
        let response = client.get_json(&url).await.map_err(BitbankError::Http)?;

        self.validate_response(&response)?;
        let data = self.extract_data(&response, "transactions response")?;

        let transactions = data
            .get("transactions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BitbankError::MissingField {
                field: "transactions".to_string(),
                context: "transactions response".to_string(),
            })?;

        self.parse_trades(transactions, pair)
    }

    /// Fetch candlestick data and return list of OHLCV.
    ///
    /// # Arguments
    ///
    /// * `pair` - Trading pair (e.g., "btc_jpy").
    /// * `candle_type` - Candle type (e.g., "1hour", "1day").
    /// * `date` - Date string in YYYYMMDD format.
    ///
    /// # Errors
    ///
    /// * [`BitbankError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`BitbankError::Http`] - If HTTP request fails.
    /// * [`BitbankError::ApiError`] - If API returns error.
    /// * [`BitbankError::MissingField`] - If response format is invalid.
    pub async fn fetch_candlestick(
        &self,
        pair: &str,
        candle_type: &str,
        date: &str,
    ) -> Result<Vec<Ohlcv>, BitbankError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(BitbankError::HttpClientNotConfigured)?;

        let url = format!(
            "{}/{}/candlestick/{}/{}",
            BITBANK_BASE_URL, pair, candle_type, date
        );
        let response = client.get_json(&url).await.map_err(BitbankError::Http)?;

        self.validate_response(&response)?;
        let data = self.extract_data(&response, "candlestick response")?;

        let candlestick_list = data
            .get("candlestick")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BitbankError::MissingField {
                field: "candlestick".to_string(),
                context: "candlestick response".to_string(),
            })?;

        if candlestick_list.is_empty() {
            return Ok(Vec::new());
        }

        let ohlcv_arrays = candlestick_list[0]
            .get("ohlcv")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BitbankError::MissingField {
                field: "ohlcv".to_string(),
                context: "candlestick response".to_string(),
            })?;

        self.parse_ohlcv_batch(ohlcv_arrays, pair)
    }

    /// Fetch depth data and return OrderBook.
    ///
    /// # Arguments
    ///
    /// * `pair` - Trading pair (e.g., "btc_jpy").
    ///
    /// # Errors
    ///
    /// * [`BitbankError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`BitbankError::Http`] - If HTTP request fails.
    /// * [`BitbankError::ApiError`] - If API returns error.
    /// * [`BitbankError::MissingField`] - If response format is invalid.
    pub async fn fetch_depth(&self, pair: &str) -> Result<OrderBook, BitbankError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(BitbankError::HttpClientNotConfigured)?;

        let url = format!("{}/{}/depth", BITBANK_BASE_URL, pair);
        let response = client.get_json(&url).await.map_err(BitbankError::Http)?;

        self.validate_response(&response)?;
        let data = self.extract_data(&response, "depth response")?;
        self.parse_orderbook(data, pair)
    }
}

impl Default for BitbankAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BaseAdapter for BitbankAdapter {
    fn source_name(&self) -> &'static str {
        "bitbank"
    }

    /// Returns the Quote field mapping for this adapter.
    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "symbol"),
            ModelMapping::new("bid", "buy").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("ask", "sell").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "timestamp")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    /// Returns the Trade field mapping for this adapter.
    fn get_trade_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "symbol"),
            ModelMapping::new("price", "price").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("size", "amount").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("side", "side").with_transform(Transforms::side_from_string_fn()),
            ModelMapping::new("timestamp", "executed_at")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    /// Returns the OHLCV field mapping for this adapter.
    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "symbol"),
            ModelMapping::new("open", "open").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("high", "high").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("low", "low").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("close", "close").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("volume", "volume").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "timestamp")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }

    /// Returns the OrderBook field mapping for this adapter.
    fn get_orderbook_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "symbol"),
            ModelMapping::new("timestamp", "timestamp")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
            // Note: asks and bids are handled separately as nested arrays
        ]
    }
}
