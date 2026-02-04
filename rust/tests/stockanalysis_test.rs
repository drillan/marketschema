//! Tests for the StockAnalysis adapter.

// Use #[path] to reference the example module directly, avoiding code duplication.
// This allows tests to validate the exact same code that the examples demonstrate.
#[path = "../examples/stockanalysis/mod.rs"]
mod stockanalysis;

use marketschema_adapters::BaseAdapter;
use stockanalysis::{
    StockAnalysisAdapter, StockAnalysisError, STOCKANALYSIS_BASE_URL,
    STOCKANALYSIS_EXPECTED_COLUMN_COUNT, STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE,
    STOCKANALYSIS_HTML_INDEX_CLOSE, STOCKANALYSIS_HTML_INDEX_DATE, STOCKANALYSIS_HTML_INDEX_HIGH,
    STOCKANALYSIS_HTML_INDEX_LOW, STOCKANALYSIS_HTML_INDEX_OPEN, STOCKANALYSIS_HTML_INDEX_VOLUME,
    STOCKANALYSIS_MONTH_MAP, STOCKANALYSIS_USER_AGENT,
};

mod parse_date_tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let result = StockAnalysisAdapter::parse_date("Feb 2, 2026").unwrap();
        assert_eq!(result, "2026-02-02T00:00:00Z");
    }

    #[test]
    fn test_valid_date_double_digit_day() {
        let result = StockAnalysisAdapter::parse_date("Jan 15, 2024").unwrap();
        assert_eq!(result, "2024-01-15T00:00:00Z");
    }

    #[test]
    fn test_valid_date_all_months() {
        // Test all 12 months
        let test_cases = [
            ("Jan 1, 2024", "2024-01-01T00:00:00Z"),
            ("Feb 1, 2024", "2024-02-01T00:00:00Z"),
            ("Mar 1, 2024", "2024-03-01T00:00:00Z"),
            ("Apr 1, 2024", "2024-04-01T00:00:00Z"),
            ("May 1, 2024", "2024-05-01T00:00:00Z"),
            ("Jun 1, 2024", "2024-06-01T00:00:00Z"),
            ("Jul 1, 2024", "2024-07-01T00:00:00Z"),
            ("Aug 1, 2024", "2024-08-01T00:00:00Z"),
            ("Sep 1, 2024", "2024-09-01T00:00:00Z"),
            ("Oct 1, 2024", "2024-10-01T00:00:00Z"),
            ("Nov 1, 2024", "2024-11-01T00:00:00Z"),
            ("Dec 1, 2024", "2024-12-01T00:00:00Z"),
        ];

        for (input, expected) in test_cases {
            let result = StockAnalysisAdapter::parse_date(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_invalid_format_missing_parts() {
        let result = StockAnalysisAdapter::parse_date("Feb 2024");
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidDateFormat { .. })
        ));
    }

    #[test]
    fn test_invalid_format_extra_parts() {
        let result = StockAnalysisAdapter::parse_date("Feb 2, 2024 Extra");
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidDateFormat { .. })
        ));
    }

    #[test]
    fn test_invalid_month_abbreviation() {
        let result = StockAnalysisAdapter::parse_date("Xyz 2, 2024");
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidMonth { .. })
        ));
    }

    #[test]
    fn test_invalid_day_not_numeric() {
        let result = StockAnalysisAdapter::parse_date("Feb abc, 2024");
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidDateFormat { .. })
        ));
    }

    #[test]
    fn test_invalid_year_not_numeric() {
        let result = StockAnalysisAdapter::parse_date("Feb 2, abcd");
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidDateFormat { .. })
        ));
    }
}

mod parse_volume_tests {
    use super::*;

    #[test]
    fn test_volume_with_commas() {
        let result = StockAnalysisAdapter::parse_volume("73,368,699").unwrap();
        assert_eq!(result, "73368699");
    }

    #[test]
    fn test_volume_without_commas() {
        let result = StockAnalysisAdapter::parse_volume("1000000").unwrap();
        assert_eq!(result, "1000000");
    }

    #[test]
    fn test_small_volume() {
        let result = StockAnalysisAdapter::parse_volume("100").unwrap();
        assert_eq!(result, "100");
    }

    #[test]
    fn test_empty_volume() {
        let result = StockAnalysisAdapter::parse_volume("");
        assert!(matches!(result, Err(StockAnalysisError::EmptyVolume)));
    }
}

mod parse_html_row_tests {
    use super::*;

    fn create_valid_row() -> Vec<String> {
        vec![
            "Feb 2, 2026".to_string(),  // Date
            "260.03".to_string(),       // Open
            "270.49".to_string(),       // High
            "259.21".to_string(),       // Low
            "269.96".to_string(),       // Close
            "269.96".to_string(),       // Adj Close
            "4.04%".to_string(),        // Change
            "73,368,699".to_string(),   // Volume
        ]
    }

    #[test]
    fn test_valid_row() {
        let adapter = StockAnalysisAdapter::new();
        let row = create_valid_row();
        let symbol = "TSLA";

        let result = adapter.parse_html_row(&row, symbol, 1).unwrap();

        assert_eq!(result.symbol, "TSLA");
        assert_eq!(result.timestamp, "2026-02-02T00:00:00Z");
        assert!((result.open - 260.03).abs() < f64::EPSILON);
        assert!((result.high - 270.49).abs() < f64::EPSILON);
        assert!((result.low - 259.21).abs() < f64::EPSILON);
        assert!((result.close - 269.96).abs() < f64::EPSILON);
        assert!((result.volume - 73_368_699.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_insufficient_columns() {
        let adapter = StockAnalysisAdapter::new();
        let row = vec![
            "Feb 2, 2026".to_string(),
            "260.03".to_string(),
            "270.49".to_string(),
        ];
        let symbol = "TSLA";

        let result = adapter.parse_html_row(&row, symbol, 1);
        assert!(matches!(
            result,
            Err(StockAnalysisError::InsufficientColumns {
                expected: 8,
                actual: 3
            })
        ));
    }

    #[test]
    fn test_invalid_date_in_row() {
        let adapter = StockAnalysisAdapter::new();
        let mut row = create_valid_row();
        row[0] = "invalid-date".to_string();
        let symbol = "TSLA";

        let result = adapter.parse_html_row(&row, symbol, 1);
        assert!(matches!(
            result,
            Err(StockAnalysisError::InvalidDateFormat { .. })
        ));
    }

    #[test]
    fn test_invalid_numeric_value() {
        let adapter = StockAnalysisAdapter::new();
        let mut row = create_valid_row();
        row[1] = "not_a_number".to_string(); // Invalid open price
        let symbol = "TSLA";

        let result = adapter.parse_html_row(&row, symbol, 1);
        assert!(matches!(result, Err(StockAnalysisError::Conversion { .. })));
    }

    #[test]
    fn test_conversion_error_includes_row_index() {
        let adapter = StockAnalysisAdapter::new();
        let mut row = create_valid_row();
        row[1] = "not_a_number".to_string();
        let symbol = "TSLA";

        let result = adapter.parse_html_row(&row, symbol, 42);
        match result {
            Err(StockAnalysisError::Conversion { row_index, .. }) => {
                assert_eq!(row_index, 42);
            }
            _ => panic!("Expected Conversion error"),
        }
    }
}

mod parse_html_row_extended_tests {
    use super::*;

    fn create_valid_row() -> Vec<String> {
        vec![
            "Feb 2, 2026".to_string(),  // Date
            "260.03".to_string(),       // Open
            "270.49".to_string(),       // High
            "259.21".to_string(),       // Low
            "269.96".to_string(),       // Close
            "268.50".to_string(),       // Adj Close (different from Close)
            "4.04%".to_string(),        // Change
            "73,368,699".to_string(),   // Volume
        ]
    }

    #[test]
    fn test_valid_row_extended() {
        let adapter = StockAnalysisAdapter::new();
        let row = create_valid_row();
        let symbol = "TSLA";

        let result = adapter.parse_html_row_extended(&row, symbol, 1).unwrap();

        assert_eq!(result.symbol, "TSLA");
        assert_eq!(result.timestamp, "2026-02-02T00:00:00Z");
        assert!((result.open - 260.03).abs() < f64::EPSILON);
        assert!((result.high - 270.49).abs() < f64::EPSILON);
        assert!((result.low - 259.21).abs() < f64::EPSILON);
        assert!((result.close - 269.96).abs() < f64::EPSILON);
        assert!((result.adj_close - 268.50).abs() < f64::EPSILON);
        assert!((result.volume - 73_368_699.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_adj_close_equals_close() {
        let adapter = StockAnalysisAdapter::new();
        let mut row = create_valid_row();
        row[5] = "269.96".to_string(); // Same as Close
        let symbol = "TSLA";

        let result = adapter.parse_html_row_extended(&row, symbol, 1).unwrap();
        assert!((result.close - result.adj_close).abs() < f64::EPSILON);
    }
}

mod parse_html_tests {
    use super::*;

    fn create_valid_html() -> String {
        r#"
        <html>
        <body>
            <table>
                <thead>
                    <tr><th>Date</th><th>Open</th><th>High</th><th>Low</th><th>Close</th><th>Adj Close</th><th>Change</th><th>Volume</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Feb 2, 2026</td><td>260.03</td><td>270.49</td><td>259.21</td><td>269.96</td><td>269.96</td><td>4.04%</td><td>73,368,699</td>
                    </tr>
                    <tr>
                        <td>Feb 1, 2026</td><td>255.00</td><td>262.50</td><td>254.00</td><td>259.47</td><td>259.47</td><td>1.80%</td><td>50,000,000</td>
                    </tr>
                </tbody>
            </table>
        </body>
        </html>
        "#.to_string()
    }

    #[test]
    fn test_valid_html() {
        let adapter = StockAnalysisAdapter::new();
        let html = create_valid_html();
        let symbol = "TSLA";

        let result = adapter.parse_html(&html, symbol).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].timestamp, "2026-02-02T00:00:00Z");
        assert_eq!(result[1].timestamp, "2026-02-01T00:00:00Z");
    }

    #[test]
    fn test_empty_html() {
        let adapter = StockAnalysisAdapter::new();
        let html = "";
        let symbol = "TSLA";

        let result = adapter.parse_html(html, symbol);
        assert!(matches!(result, Err(StockAnalysisError::EmptyHtml)));
    }

    #[test]
    fn test_whitespace_only_html() {
        let adapter = StockAnalysisAdapter::new();
        let html = "   \n\t  ";
        let symbol = "TSLA";

        let result = adapter.parse_html(html, symbol);
        assert!(matches!(result, Err(StockAnalysisError::EmptyHtml)));
    }

    #[test]
    fn test_no_table_found() {
        let adapter = StockAnalysisAdapter::new();
        let html = "<html><body><div>No table here</div></body></html>";
        let symbol = "TSLA";

        let result = adapter.parse_html(html, symbol);
        assert!(matches!(result, Err(StockAnalysisError::NoTableFound)));
    }

    #[test]
    fn test_no_tbody_found() {
        // Note: HTML5 parsers (like html5ever used by scraper) automatically wrap
        // `tr` elements in a `tbody` even when not explicitly present in the source.
        // This test verifies parsing behavior when the HTML is malformed in a way
        // that the parser cannot auto-correct (table with only thead, no tbody).
        let adapter = StockAnalysisAdapter::new();
        let html = r#"
        <html>
        <body>
            <table>
                <thead>
                    <tr><th>Header only</th></tr>
                </thead>
            </table>
        </body>
        </html>
        "#;
        let symbol = "TSLA";

        let result = adapter.parse_html(html, symbol);
        assert!(matches!(
            result,
            Err(StockAnalysisError::TableStructureError { .. })
        ));
    }

    #[test]
    fn test_empty_tbody() {
        let adapter = StockAnalysisAdapter::new();
        let html = r#"
        <html>
        <body>
            <table>
                <tbody>
                </tbody>
            </table>
        </body>
        </html>
        "#;
        let symbol = "TSLA";

        let result = adapter.parse_html(html, symbol).unwrap();
        assert!(result.is_empty());
    }
}

mod parse_html_extended_tests {
    use super::*;

    fn create_valid_html_with_different_adj_close() -> String {
        r#"
        <html>
        <body>
            <table>
                <thead>
                    <tr><th>Date</th><th>Open</th><th>High</th><th>Low</th><th>Close</th><th>Adj Close</th><th>Change</th><th>Volume</th></tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Feb 2, 2026</td><td>260.03</td><td>270.49</td><td>259.21</td><td>269.96</td><td>265.00</td><td>4.04%</td><td>73,368,699</td>
                    </tr>
                </tbody>
            </table>
        </body>
        </html>
        "#.to_string()
    }

    #[test]
    fn test_valid_html_extended() {
        let adapter = StockAnalysisAdapter::new();
        let html = create_valid_html_with_different_adj_close();
        let symbol = "TSLA";

        let result = adapter.parse_html_extended(&html, symbol).unwrap();

        assert_eq!(result.len(), 1);
        assert!((result[0].close - 269.96).abs() < f64::EPSILON);
        assert!((result[0].adj_close - 265.00).abs() < f64::EPSILON);
    }
}

mod constants_tests {
    use super::*;

    #[test]
    fn test_base_url() {
        assert_eq!(STOCKANALYSIS_BASE_URL, "https://stockanalysis.com/stocks");
    }

    #[test]
    fn test_user_agent_contains_browser_info() {
        assert!(STOCKANALYSIS_USER_AGENT.contains("Mozilla"));
        assert!(STOCKANALYSIS_USER_AGENT.contains("Chrome"));
    }

    #[test]
    fn test_expected_column_count() {
        assert_eq!(STOCKANALYSIS_EXPECTED_COLUMN_COUNT, 8);
    }

    #[test]
    fn test_column_indices() {
        assert_eq!(STOCKANALYSIS_HTML_INDEX_DATE, 0);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_OPEN, 1);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_HIGH, 2);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_LOW, 3);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_CLOSE, 4);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_ADJ_CLOSE, 5);
        assert_eq!(STOCKANALYSIS_HTML_INDEX_VOLUME, 7);
    }

    #[test]
    fn test_month_map_contains_all_months() {
        assert_eq!(STOCKANALYSIS_MONTH_MAP.len(), 12);
        assert_eq!(STOCKANALYSIS_MONTH_MAP.get("Jan"), Some(&"01"));
        assert_eq!(STOCKANALYSIS_MONTH_MAP.get("Dec"), Some(&"12"));
    }
}

mod http_client_tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_history_without_http_client_returns_error() {
        let adapter = StockAnalysisAdapter::new();
        let result = adapter.fetch_history("tsla").await;
        assert!(matches!(
            result,
            Err(StockAnalysisError::HttpClientNotConfigured)
        ));
    }

    #[tokio::test]
    async fn test_fetch_and_parse_without_http_client_returns_error() {
        let adapter = StockAnalysisAdapter::new();
        let result = adapter.fetch_and_parse("tsla").await;
        assert!(matches!(
            result,
            Err(StockAnalysisError::HttpClientNotConfigured)
        ));
    }

    #[tokio::test]
    async fn test_fetch_and_parse_extended_without_http_client_returns_error() {
        let adapter = StockAnalysisAdapter::new();
        let result = adapter.fetch_and_parse_extended("tsla").await;
        assert!(matches!(
            result,
            Err(StockAnalysisError::HttpClientNotConfigured)
        ));
    }

    #[test]
    fn test_with_default_http_client() {
        let result = StockAnalysisAdapter::with_default_http_client();
        assert!(result.is_ok());
    }
}

mod base_adapter_tests {
    use super::*;

    #[test]
    fn test_source_name() {
        let adapter = StockAnalysisAdapter::new();
        assert_eq!(adapter.source_name(), "stockanalysis");
    }

    #[test]
    fn test_get_ohlcv_mapping_contains_required_fields() {
        let adapter = StockAnalysisAdapter::new();
        let mappings = adapter.get_ohlcv_mapping();

        let source_fields: Vec<&str> = mappings.iter().map(|m| m.source_field.as_str()).collect();

        assert!(source_fields.contains(&"open"));
        assert!(source_fields.contains(&"high"));
        assert!(source_fields.contains(&"low"));
        assert!(source_fields.contains(&"close"));
        assert!(source_fields.contains(&"volume"));
        assert!(source_fields.contains(&"timestamp"));
        assert!(source_fields.contains(&"symbol"));
    }
}
