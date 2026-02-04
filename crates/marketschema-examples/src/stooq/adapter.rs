//! Stooq adapter implementation.
//!
//! This adapter transforms CSV data from stooq.com into marketschema OHLCV models.
//!
//! # Data Source
//!
//! - URL: `https://stooq.com/q/d/l/?s={symbol}&i=d`
//!
//! # CSV Format
//!
//! ```text
//! Date,Open,High,Low,Close,Volume
//! 1999-04-06,898.471,919.49,879.213,919.49,890722
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};

use super::constants::{
    STOOQ_BASE_URL, STOOQ_CSV_INDEX_CLOSE, STOOQ_CSV_INDEX_DATE, STOOQ_CSV_INDEX_HIGH,
    STOOQ_CSV_INDEX_LOW, STOOQ_CSV_INDEX_OPEN, STOOQ_CSV_INDEX_VOLUME, STOOQ_EXPECTED_COLUMN_COUNT,
    STOOQ_EXPECTED_HEADER, STOOQ_INTERVAL_DAILY,
};
use super::error::StooqError;

/// OHLCV data structure for Stooq adapter.
#[derive(Debug, Clone, PartialEq)]
pub struct Ohlcv {
    /// Stock symbol (e.g., "spy.us").
    pub symbol: String,
    /// ISO 8601 timestamp (e.g., "2024-01-15T00:00:00Z").
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

/// Adapter for stooq.com stock data.
///
/// Transforms CSV data from stooq.com into standardized OHLCV models.
///
/// # Note
///
/// stooq.com CSV does not include symbol information.
/// Symbol must be provided as a parameter to parse methods.
pub struct StooqAdapter {
    http_client: Option<Arc<AsyncHttpClient>>,
}

impl StooqAdapter {
    /// Create a new `StooqAdapter` without HTTP client.
    ///
    /// Use [`StooqAdapter::with_http_client`] to create an adapter with HTTP capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self { http_client: None }
    }

    /// Create a new `StooqAdapter` with the given HTTP client.
    #[must_use]
    pub fn with_http_client(http_client: Arc<AsyncHttpClient>) -> Self {
        Self {
            http_client: Some(http_client),
        }
    }

    /// Create a new `StooqAdapter` with a default HTTP client.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    pub fn with_default_http_client() -> Result<Self, StooqError> {
        let client = AsyncHttpClientBuilder::new()
            .build()
            .map_err(StooqError::Http)?;
        Ok(Self {
            http_client: Some(Arc::new(client)),
        })
    }

    /// Convert date string to ISO 8601 timestamp.
    ///
    /// # Arguments
    ///
    /// * `date_str` - Date in YYYY-MM-DD format.
    ///
    /// # Returns
    ///
    /// ISO 8601 timestamp string (UTC midnight).
    ///
    /// # Errors
    ///
    /// Returns [`StooqError::InvalidDateFormat`] if the date format is invalid
    /// or represents an invalid calendar date (e.g., "2024-02-30").
    pub fn date_to_iso_timestamp(date_str: &str) -> Result<String, StooqError> {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
            StooqError::InvalidDateFormat {
                value: date_str.to_string(),
            }
        })?;
        Ok(format!("{}T00:00:00Z", date.format("%Y-%m-%d")))
    }

    /// Parse a single CSV row into OHLCV model.
    ///
    /// # Arguments
    ///
    /// * `row` - List of string values from CSV row.
    /// * `symbol` - Stock symbol (e.g., "spy.us").
    ///
    /// # Returns
    ///
    /// OHLCV model instance.
    ///
    /// # Errors
    ///
    /// * [`StooqError::InsufficientColumns`] - If row has insufficient columns.
    /// * [`StooqError::InvalidDateFormat`] - If date format is invalid.
    /// * [`StooqError::Conversion`] - If numeric conversion fails.
    pub fn parse_csv_row(&self, row: &[String], symbol: &str) -> Result<Ohlcv, StooqError> {
        if row.len() < STOOQ_EXPECTED_COLUMN_COUNT {
            return Err(StooqError::InsufficientColumns {
                expected: STOOQ_EXPECTED_COLUMN_COUNT,
                actual: row.len(),
            });
        }

        let timestamp = Self::date_to_iso_timestamp(&row[STOOQ_CSV_INDEX_DATE])?;

        let open = self.parse_float(&row[STOOQ_CSV_INDEX_OPEN], "open")?;
        let high = self.parse_float(&row[STOOQ_CSV_INDEX_HIGH], "high")?;
        let low = self.parse_float(&row[STOOQ_CSV_INDEX_LOW], "low")?;
        let close = self.parse_float(&row[STOOQ_CSV_INDEX_CLOSE], "close")?;
        let volume = self.parse_float(&row[STOOQ_CSV_INDEX_VOLUME], "volume")?;

        Ok(Ohlcv {
            symbol: symbol.to_string(),
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        })
    }

    /// Parse a string to f64.
    fn parse_float(&self, value: &str, field_name: &str) -> Result<f64, StooqError> {
        value.parse::<f64>().map_err(|e| StooqError::Conversion {
            message: format!(
                "Failed to parse '{}' as float for field '{}': {}",
                value, field_name, e
            ),
        })
    }

    /// Parse CSV content into list of OHLCV models.
    ///
    /// # Arguments
    ///
    /// * `csv_content` - Full CSV content as string.
    /// * `symbol` - Stock symbol (e.g., "spy.us").
    ///
    /// # Returns
    ///
    /// List of OHLCV model instances.
    ///
    /// # Behavior
    ///
    /// - Empty rows (rows with no data or all empty fields) are silently skipped.
    ///   This is common in CSV files that may have trailing newlines.
    ///
    /// # Errors
    ///
    /// * [`StooqError::EmptyCsv`] - If CSV has no header row.
    /// * [`StooqError::InvalidHeader`] - If CSV header is invalid.
    /// * [`StooqError::CsvParse`] - If CSV parsing fails.
    /// * Other errors from [`parse_csv_row`].
    pub fn parse_csv(&self, csv_content: &str, symbol: &str) -> Result<Vec<Ohlcv>, StooqError> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_content.as_bytes());

        let mut records = reader.records();

        // Read and validate header
        let header_record =
            records
                .next()
                .ok_or(StooqError::EmptyCsv)?
                .map_err(|e| StooqError::CsvParse {
                    message: e.to_string(),
                })?;

        let header: Vec<&str> = header_record.iter().collect();
        let expected_header: Vec<&str> = STOOQ_EXPECTED_HEADER.to_vec();

        if header != expected_header {
            return Err(StooqError::InvalidHeader {
                expected: expected_header.iter().map(|s| s.to_string()).collect(),
                actual: header.iter().map(|s| s.to_string()).collect(),
            });
        }

        // Parse data rows
        let mut results: Vec<Ohlcv> = Vec::new();

        for record_result in records {
            let record = record_result.map_err(|e| StooqError::CsvParse {
                message: e.to_string(),
            })?;

            // Skip empty rows
            if record.is_empty() || record.iter().all(|s| s.is_empty()) {
                continue;
            }

            let row: Vec<String> = record.iter().map(String::from).collect();
            results.push(self.parse_csv_row(&row, symbol)?);
        }

        Ok(results)
    }

    /// Fetch CSV data from stooq.com.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "spy.us", "aapl.us", "^spx").
    ///
    /// # Returns
    ///
    /// CSV content as string.
    ///
    /// # Errors
    ///
    /// * [`StooqError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`StooqError::Http`] - If HTTP request fails.
    pub async fn fetch_csv(&self, symbol: &str) -> Result<String, StooqError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(StooqError::HttpClientNotConfigured)?;

        let csv_content = client
            .get_text_with_params(
                STOOQ_BASE_URL,
                &[("s", symbol), ("i", STOOQ_INTERVAL_DAILY)],
            )
            .await
            .map_err(StooqError::Http)?;

        Ok(csv_content)
    }

    /// Fetch CSV from stooq.com and parse into OHLCV models.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "spy.us", "aapl.us", "^spx").
    ///
    /// # Returns
    ///
    /// List of OHLCV model instances.
    ///
    /// # Errors
    ///
    /// * [`StooqError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`StooqError::Http`] - If HTTP request fails.
    /// * Other errors from [`parse_csv`].
    pub async fn fetch_and_parse(&self, symbol: &str) -> Result<Vec<Ohlcv>, StooqError> {
        let csv_content = self.fetch_csv(symbol).await?;
        self.parse_csv(&csv_content, symbol)
    }
}

impl Default for StooqAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BaseAdapter for StooqAdapter {
    fn source_name(&self) -> &'static str {
        "stooq"
    }

    /// Returns the OHLCV field mapping for this adapter.
    ///
    /// This mapping defines how source fields map to the standard OHLCV model.
    /// It serves as metadata for generic data transformation pipelines.
    ///
    /// Note: Currently `parse_csv_row` constructs `Ohlcv` directly without using
    /// this mapping. This is intentional for performance and type safety. The mapping
    /// is provided for future integration with generic transformation utilities.
    fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("symbol", "symbol"),
            ModelMapping::new("timestamp", "timestamp")
                .with_transform(Transforms::iso_timestamp_fn()),
            ModelMapping::new("open", "open").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("high", "high").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("low", "low").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("close", "close").with_transform(Transforms::to_float_fn()),
            ModelMapping::new("volume", "volume").with_transform(Transforms::to_float_fn()),
        ]
    }
}
