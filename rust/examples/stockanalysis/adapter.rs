//! StockAnalysis adapter implementation.
//!
//! This adapter transforms HTML table data from stockanalysis.com into marketschema OHLCV models.
//!
//! # Data Source
//!
//! - URL: `https://stockanalysis.com/stocks/{symbol}/history/`
//!
//! # HTML Table Format
//!
//! ```text
//! Date       | Open   | High   | Low    | Close  | Adj Close | Change | Volume
//! Feb 2, 2026| 260.03 | 270.49 | 259.21 | 269.96 | 269.96    | 4.04%  | 73,368,699
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};
use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};

use super::constants::{
    STOCKANALYSIS_BASE_URL, STOCKANALYSIS_EXPECTED_COLUMN_COUNT, STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE,
    STOCKANALYSIS_HTML_INDEX_CLOSE, STOCKANALYSIS_HTML_INDEX_DATE, STOCKANALYSIS_HTML_INDEX_HIGH,
    STOCKANALYSIS_HTML_INDEX_LOW, STOCKANALYSIS_HTML_INDEX_OPEN, STOCKANALYSIS_HTML_INDEX_VOLUME,
    STOCKANALYSIS_MONTH_MAP, STOCKANALYSIS_USER_AGENT,
};
use super::error::StockAnalysisError;
use super::models::ExtendedOhlcv;

/// Standard OHLCV data structure for StockAnalysis adapter.
///
/// This is a lightweight intermediate representation used during HTML parsing,
/// separate from the library's `marketschema::Ohlcv` type. This allows the
/// adapter to perform initial data transformation without depending on the
/// full schema type system. The data can be converted to `marketschema::Ohlcv`
/// for downstream processing if needed.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Ohlcv {
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
    /// Trading volume.
    pub volume: f64,
}

/// Adapter for stockanalysis.com stock data.
///
/// Transforms HTML table data from stockanalysis.com into standardized OHLCV models.
///
/// # Note
///
/// stockanalysis.com HTML does not include symbol information.
/// Symbol must be provided as a parameter to parse methods.
pub struct StockAnalysisAdapter {
    http_client: Option<Arc<AsyncHttpClient>>,
}

impl StockAnalysisAdapter {
    /// Create a new `StockAnalysisAdapter` without HTTP client.
    ///
    /// Use [`StockAnalysisAdapter::with_http_client`] to create an adapter with HTTP capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self { http_client: None }
    }

    /// Create a new `StockAnalysisAdapter` with the given HTTP client.
    #[must_use]
    // Unused in the current crate but provided as a public API for external callers
    // who want to inject a custom HTTP client (e.g., for testing with mocks).
    #[allow(dead_code)]
    pub fn with_http_client(http_client: Arc<AsyncHttpClient>) -> Self {
        Self {
            http_client: Some(http_client),
        }
    }

    /// Create a new `StockAnalysisAdapter` with a default HTTP client.
    ///
    /// The client is configured with a custom User-Agent header to avoid being blocked
    /// by the server's bot detection.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    pub fn with_default_http_client() -> Result<Self, StockAnalysisError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(STOCKANALYSIS_USER_AGENT),
        );

        let client = AsyncHttpClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(StockAnalysisError::Http)?;
        Ok(Self {
            http_client: Some(Arc::new(client)),
        })
    }

    /// Convert date string to ISO 8601 timestamp.
    ///
    /// # Arguments
    ///
    /// * `date_str` - Date in "MMM D, YYYY" format (e.g., "Feb 2, 2026").
    ///
    /// # Returns
    ///
    /// ISO 8601 timestamp string (UTC midnight).
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::InvalidDateFormat`] - If the date format is invalid.
    /// * [`StockAnalysisError::InvalidMonth`] - If the month abbreviation is not recognized.
    pub fn parse_date(date_str: &str) -> Result<String, StockAnalysisError> {
        // Expected format: "MMM D, YYYY" (e.g., "Feb 2, 2026")
        let parts: Vec<&str> = date_str.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(StockAnalysisError::InvalidDateFormat {
                value: date_str.to_string(),
                reason: format!("expected 3 parts, got {}", parts.len()),
            });
        }

        let month_abbr = parts[0];
        let day_str = parts[1];
        let year_str = parts[2];

        // Validate and convert month
        let month = STOCKANALYSIS_MONTH_MAP
            .get(month_abbr)
            .ok_or_else(|| StockAnalysisError::InvalidMonth {
                value: month_abbr.to_string(),
            })?;

        // Remove comma from day and validate
        let day_str = day_str.trim_end_matches(',');
        let day: u32 = day_str.parse().map_err(|_| StockAnalysisError::InvalidDateFormat {
            value: date_str.to_string(),
            reason: format!("invalid day: {day_str:?}"),
        })?;

        // Validate year
        let year: u32 = year_str.parse().map_err(|_| StockAnalysisError::InvalidDateFormat {
            value: date_str.to_string(),
            reason: format!("invalid year: {year_str:?}"),
        })?;

        Ok(format!("{year:04}-{month}-{day:02}T00:00:00Z"))
    }

    /// Remove commas from volume string.
    ///
    /// # Arguments
    ///
    /// * `volume_str` - Volume with commas (e.g., "73,368,699").
    ///
    /// # Returns
    ///
    /// Volume without commas (e.g., "73368699").
    ///
    /// # Errors
    ///
    /// Returns [`StockAnalysisError::EmptyVolume`] if the volume string is empty.
    pub fn parse_volume(volume_str: &str) -> Result<String, StockAnalysisError> {
        if volume_str.is_empty() {
            return Err(StockAnalysisError::EmptyVolume);
        }
        Ok(volume_str.replace(',', ""))
    }

    /// Parse a string to f64.
    fn parse_float(
        value: &str,
        field_name: &str,
        row_index: usize,
    ) -> Result<f64, StockAnalysisError> {
        value
            .parse::<f64>()
            .map_err(|e| StockAnalysisError::Conversion {
                message: format!(
                    "Failed to parse '{}' as float for field '{}': {}",
                    value, field_name, e
                ),
                row_index,
            })
    }

    /// Parse a single HTML table row into OHLCV model.
    ///
    /// # Arguments
    ///
    /// * `row_data` - List of string values from HTML table row.
    /// * `symbol` - Stock symbol (e.g., "TSLA").
    /// * `row_index` - 1-based row index for error reporting.
    ///
    /// # Returns
    ///
    /// OHLCV model instance.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::InsufficientColumns`] - If row has insufficient columns.
    /// * [`StockAnalysisError::InvalidDateFormat`] - If date format is invalid.
    /// * [`StockAnalysisError::Conversion`] - If numeric conversion fails.
    #[allow(dead_code)]
    pub fn parse_html_row(
        &self,
        row_data: &[String],
        symbol: &str,
        row_index: usize,
    ) -> Result<Ohlcv, StockAnalysisError> {
        if row_data.len() < STOCKANALYSIS_EXPECTED_COLUMN_COUNT {
            return Err(StockAnalysisError::InsufficientColumns {
                expected: STOCKANALYSIS_EXPECTED_COLUMN_COUNT,
                actual: row_data.len(),
            });
        }

        let timestamp = Self::parse_date(&row_data[STOCKANALYSIS_HTML_INDEX_DATE])?;
        let volume_str = Self::parse_volume(&row_data[STOCKANALYSIS_HTML_INDEX_VOLUME])?;

        let open = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_OPEN], "open", row_index)?;
        let high = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_HIGH], "high", row_index)?;
        let low = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_LOW], "low", row_index)?;
        let close =
            Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_CLOSE], "close", row_index)?;
        let volume = Self::parse_float(&volume_str, "volume", row_index)?;

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

    /// Parse a single HTML table row into ExtendedOhlcv model (includes adj_close).
    ///
    /// # Arguments
    ///
    /// * `row_data` - List of string values from HTML table row.
    /// * `symbol` - Stock symbol (e.g., "TSLA").
    /// * `row_index` - 1-based row index for error reporting.
    ///
    /// # Returns
    ///
    /// ExtendedOhlcv model instance with adj_close field.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::InsufficientColumns`] - If row has insufficient columns.
    /// * [`StockAnalysisError::InvalidDateFormat`] - If date format is invalid.
    /// * [`StockAnalysisError::Conversion`] - If numeric conversion fails.
    pub fn parse_html_row_extended(
        &self,
        row_data: &[String],
        symbol: &str,
        row_index: usize,
    ) -> Result<ExtendedOhlcv, StockAnalysisError> {
        if row_data.len() < STOCKANALYSIS_EXPECTED_COLUMN_COUNT {
            return Err(StockAnalysisError::InsufficientColumns {
                expected: STOCKANALYSIS_EXPECTED_COLUMN_COUNT,
                actual: row_data.len(),
            });
        }

        let timestamp = Self::parse_date(&row_data[STOCKANALYSIS_HTML_INDEX_DATE])?;
        let volume_str = Self::parse_volume(&row_data[STOCKANALYSIS_HTML_INDEX_VOLUME])?;

        let open = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_OPEN], "open", row_index)?;
        let high = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_HIGH], "high", row_index)?;
        let low = Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_LOW], "low", row_index)?;
        let close =
            Self::parse_float(&row_data[STOCKANALYSIS_HTML_INDEX_CLOSE], "close", row_index)?;
        let adj_close = Self::parse_float(
            &row_data[STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE],
            "adj_close",
            row_index,
        )?;
        let volume = Self::parse_float(&volume_str, "volume", row_index)?;

        Ok(ExtendedOhlcv {
            symbol: symbol.to_string(),
            timestamp,
            open,
            high,
            low,
            close,
            adj_close,
            volume,
        })
    }

    /// Parse HTML content into list of OHLCV models.
    ///
    /// # Arguments
    ///
    /// * `html_content` - Full HTML content as string.
    /// * `symbol` - Stock symbol (e.g., "TSLA").
    ///
    /// # Returns
    ///
    /// List of OHLCV model instances.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::EmptyHtml`] - If HTML content is empty.
    /// * [`StockAnalysisError::NoTableFound`] - If no table element found.
    /// * [`StockAnalysisError::TableStructureError`] - If table structure is invalid.
    /// * Other errors from [`parse_html_row`].
    #[allow(dead_code)]
    pub fn parse_html(&self, html_content: &str, symbol: &str) -> Result<Vec<Ohlcv>, StockAnalysisError> {
        if html_content.is_empty() || html_content.trim().is_empty() {
            return Err(StockAnalysisError::EmptyHtml);
        }

        let document = Html::parse_document(html_content);

        // Find the table element
        let table_selector =
            Selector::parse("table").expect("static selector should always be valid");
        let table = document
            .select(&table_selector)
            .next()
            .ok_or(StockAnalysisError::NoTableFound)?;

        // Find tbody and extract data rows
        let tbody_selector =
            Selector::parse("tbody").expect("static selector should always be valid");
        let tbody = table.select(&tbody_selector).next().ok_or_else(|| {
            StockAnalysisError::TableStructureError {
                message: "<tbody> element not found. The page structure may have changed."
                    .to_string(),
            }
        })?;

        // Find all tr elements in tbody
        let tr_selector = Selector::parse("tr").expect("static selector should always be valid");
        let td_selector = Selector::parse("td").expect("static selector should always be valid");

        let mut results: Vec<Ohlcv> = Vec::new();
        let mut row_index: usize = 0;

        for row in tbody.select(&tr_selector) {
            row_index += 1;

            // Extract cell text content
            let cells: Vec<String> = row
                .select(&td_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();

            // Skip empty rows
            if cells.is_empty() {
                continue;
            }

            results.push(self.parse_html_row(&cells, symbol, row_index)?);
        }

        Ok(results)
    }

    /// Parse HTML content into list of ExtendedOhlcv models (includes adj_close).
    ///
    /// # Arguments
    ///
    /// * `html_content` - Full HTML content as string.
    /// * `symbol` - Stock symbol (e.g., "TSLA").
    ///
    /// # Returns
    ///
    /// List of ExtendedOhlcv model instances with adj_close field.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::EmptyHtml`] - If HTML content is empty.
    /// * [`StockAnalysisError::NoTableFound`] - If no table element found.
    /// * [`StockAnalysisError::TableStructureError`] - If table structure is invalid.
    /// * Other errors from [`parse_html_row_extended`].
    pub fn parse_html_extended(
        &self,
        html_content: &str,
        symbol: &str,
    ) -> Result<Vec<ExtendedOhlcv>, StockAnalysisError> {
        if html_content.is_empty() || html_content.trim().is_empty() {
            return Err(StockAnalysisError::EmptyHtml);
        }

        let document = Html::parse_document(html_content);

        // Find the table element
        let table_selector =
            Selector::parse("table").expect("static selector should always be valid");
        let table = document
            .select(&table_selector)
            .next()
            .ok_or(StockAnalysisError::NoTableFound)?;

        // Find tbody and extract data rows
        let tbody_selector =
            Selector::parse("tbody").expect("static selector should always be valid");
        let tbody = table.select(&tbody_selector).next().ok_or_else(|| {
            StockAnalysisError::TableStructureError {
                message: "<tbody> element not found. The page structure may have changed."
                    .to_string(),
            }
        })?;

        // Find all tr elements in tbody
        let tr_selector = Selector::parse("tr").expect("static selector should always be valid");
        let td_selector = Selector::parse("td").expect("static selector should always be valid");

        let mut results: Vec<ExtendedOhlcv> = Vec::new();
        let mut row_index: usize = 0;

        for row in tbody.select(&tr_selector) {
            row_index += 1;

            // Extract cell text content
            let cells: Vec<String> = row
                .select(&td_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();

            // Skip empty rows
            if cells.is_empty() {
                continue;
            }

            results.push(self.parse_html_row_extended(&cells, symbol, row_index)?);
        }

        Ok(results)
    }

    /// Fetch historical data HTML from stockanalysis.com.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "TSLA", "AAPL").
    ///
    /// # Returns
    ///
    /// HTML content as string.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`StockAnalysisError::Http`] - If HTTP request fails.
    pub async fn fetch_history(&self, symbol: &str) -> Result<String, StockAnalysisError> {
        let client = self
            .http_client
            .as_ref()
            .ok_or(StockAnalysisError::HttpClientNotConfigured)?;

        let url = format!("{}/{}/history/", STOCKANALYSIS_BASE_URL, symbol.to_lowercase());

        let html_content = client
            .get_text(&url)
            .await
            .map_err(StockAnalysisError::Http)?;

        Ok(html_content)
    }

    /// Fetch historical data from stockanalysis.com and parse into OHLCV models.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "TSLA", "AAPL").
    ///
    /// # Returns
    ///
    /// List of OHLCV model instances.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`StockAnalysisError::Http`] - If HTTP request fails.
    /// * Other errors from [`parse_html`].
    #[allow(dead_code)]
    pub async fn fetch_and_parse(&self, symbol: &str) -> Result<Vec<Ohlcv>, StockAnalysisError> {
        let html_content = self.fetch_history(symbol).await?;
        self.parse_html(&html_content, symbol)
    }

    /// Fetch historical data from stockanalysis.com and parse into ExtendedOhlcv models.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Stock symbol (e.g., "TSLA", "AAPL").
    ///
    /// # Returns
    ///
    /// List of ExtendedOhlcv model instances with adj_close field.
    ///
    /// # Errors
    ///
    /// * [`StockAnalysisError::HttpClientNotConfigured`] - If HTTP client is not configured.
    /// * [`StockAnalysisError::Http`] - If HTTP request fails.
    /// * Other errors from [`parse_html_extended`].
    pub async fn fetch_and_parse_extended(
        &self,
        symbol: &str,
    ) -> Result<Vec<ExtendedOhlcv>, StockAnalysisError> {
        let html_content = self.fetch_history(symbol).await?;
        self.parse_html_extended(&html_content, symbol)
    }
}

impl Default for StockAnalysisAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BaseAdapter for StockAnalysisAdapter {
    fn source_name(&self) -> &'static str {
        "stockanalysis"
    }

    /// Returns the OHLCV field mapping for this adapter.
    ///
    /// This mapping defines how source fields map to the standard OHLCV model.
    /// It serves as metadata for generic data transformation pipelines.
    ///
    /// Note: Currently `parse_html_row` constructs `Ohlcv` directly without using
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
