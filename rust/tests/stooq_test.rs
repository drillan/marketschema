//! Tests for the Stooq adapter.

mod stooq;

use marketschema_adapters::BaseAdapter;
use stooq::{
    StooqAdapter, StooqError, STOOQ_BASE_URL, STOOQ_EXPECTED_HEADER, STOOQ_INTERVAL_DAILY,
};

mod date_to_iso_timestamp_tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-01-15").unwrap();
        assert_eq!(result, "2024-01-15T00:00:00Z");
    }

    #[test]
    fn test_valid_date_early() {
        let result = StooqAdapter::date_to_iso_timestamp("1999-04-06").unwrap();
        assert_eq!(result, "1999-04-06T00:00:00Z");
    }

    #[test]
    fn test_invalid_format_missing_parts() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-01");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_invalid_format_wrong_separator() {
        let result = StooqAdapter::date_to_iso_timestamp("2024/01/15");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_short_year_parsed_as_year_24() {
        // chrono parses "24-01-15" as year 24, which is a valid (ancient) date
        let result = StooqAdapter::date_to_iso_timestamp("24-01-15").unwrap();
        assert_eq!(result, "0024-01-15T00:00:00Z");
    }

    #[test]
    fn test_single_digit_month_is_valid() {
        // chrono's %m accepts single digit months
        let result = StooqAdapter::date_to_iso_timestamp("2024-1-15").unwrap();
        assert_eq!(result, "2024-01-15T00:00:00Z");
    }

    #[test]
    fn test_invalid_format_non_numeric() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-ab-15");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }
}

mod parse_csv_row_tests {
    use super::*;

    #[test]
    fn test_valid_row() {
        let adapter = StooqAdapter::new();
        let row = vec![
            "2024-01-15".to_string(),
            "100.50".to_string(),
            "105.00".to_string(),
            "99.00".to_string(),
            "103.25".to_string(),
            "1000000".to_string(),
        ];
        let symbol = "spy.us";

        let result = adapter.parse_csv_row(&row, symbol).unwrap();

        assert_eq!(result.symbol, symbol);
        assert_eq!(result.timestamp, "2024-01-15T00:00:00Z");
        assert!((result.open - 100.50).abs() < f64::EPSILON);
        assert!((result.high - 105.00).abs() < f64::EPSILON);
        assert!((result.low - 99.00).abs() < f64::EPSILON);
        assert!((result.close - 103.25).abs() < f64::EPSILON);
        assert!((result.volume - 1_000_000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_insufficient_columns() {
        let adapter = StooqAdapter::new();
        let row = vec![
            "2024-01-15".to_string(),
            "100.50".to_string(),
            "105.00".to_string(),
        ];
        let symbol = "spy.us";

        let result = adapter.parse_csv_row(&row, symbol);
        assert!(matches!(
            result,
            Err(StooqError::InsufficientColumns {
                expected: 6,
                actual: 3
            })
        ));
    }

    #[test]
    fn test_invalid_date_in_row() {
        let adapter = StooqAdapter::new();
        let row = vec![
            "invalid-date".to_string(),
            "100.50".to_string(),
            "105.00".to_string(),
            "99.00".to_string(),
            "103.25".to_string(),
            "1000000".to_string(),
        ];
        let symbol = "spy.us";

        let result = adapter.parse_csv_row(&row, symbol);
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_invalid_numeric_value() {
        let adapter = StooqAdapter::new();
        let row = vec![
            "2024-01-15".to_string(),
            "not_a_number".to_string(),
            "105.00".to_string(),
            "99.00".to_string(),
            "103.25".to_string(),
            "1000000".to_string(),
        ];
        let symbol = "spy.us";

        let result = adapter.parse_csv_row(&row, symbol);
        assert!(matches!(result, Err(StooqError::Conversion { .. })));
    }
}

mod parse_csv_tests {
    use super::*;

    #[test]
    fn test_valid_csv() {
        let adapter = StooqAdapter::new();
        let csv_content = "\
Date,Open,High,Low,Close,Volume
2024-01-15,100.50,105.00,99.00,103.25,1000000
2024-01-16,103.50,108.00,102.00,106.75,1200000
";
        let symbol = "spy.us";

        let result = adapter.parse_csv(csv_content, symbol).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].timestamp, "2024-01-15T00:00:00Z");
        assert_eq!(result[1].timestamp, "2024-01-16T00:00:00Z");
    }

    #[test]
    fn test_empty_csv() {
        let adapter = StooqAdapter::new();
        let csv_content = "";
        let symbol = "spy.us";

        let result = adapter.parse_csv(csv_content, symbol);
        assert!(matches!(result, Err(StooqError::EmptyCsv)));
    }

    #[test]
    fn test_invalid_header() {
        let adapter = StooqAdapter::new();
        let csv_content = "\
WrongHeader,Open,High,Low,Close,Volume
2024-01-15,100.50,105.00,99.00,103.25,1000000
";
        let symbol = "spy.us";

        let result = adapter.parse_csv(csv_content, symbol);
        assert!(matches!(result, Err(StooqError::InvalidHeader { .. })));
    }

    #[test]
    fn test_header_only() {
        let adapter = StooqAdapter::new();
        let csv_content = "Date,Open,High,Low,Close,Volume\n";
        let symbol = "spy.us";

        let result = adapter.parse_csv(csv_content, symbol).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_skips_empty_rows() {
        let adapter = StooqAdapter::new();
        let csv_content = "\
Date,Open,High,Low,Close,Volume
2024-01-15,100.50,105.00,99.00,103.25,1000000

2024-01-16,103.50,108.00,102.00,106.75,1200000
";
        let symbol = "spy.us";

        let result = adapter.parse_csv(csv_content, symbol).unwrap();
        assert_eq!(result.len(), 2);
    }
}

mod constants_tests {
    use super::*;

    #[test]
    fn test_base_url() {
        assert_eq!(STOOQ_BASE_URL, "https://stooq.com/q/d/l/");
    }

    #[test]
    fn test_interval_daily() {
        assert_eq!(STOOQ_INTERVAL_DAILY, "d");
    }

    #[test]
    fn test_expected_header() {
        assert_eq!(
            STOOQ_EXPECTED_HEADER,
            ["Date", "Open", "High", "Low", "Close", "Volume"]
        );
    }
}

mod http_client_tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_csv_without_http_client_returns_error() {
        let adapter = StooqAdapter::new();
        let result = adapter.fetch_csv("spy.us").await;
        assert!(matches!(result, Err(StooqError::HttpClientNotConfigured)));
    }

    #[tokio::test]
    async fn test_fetch_and_parse_without_http_client_returns_error() {
        let adapter = StooqAdapter::new();
        let result = adapter.fetch_and_parse("spy.us").await;
        assert!(matches!(result, Err(StooqError::HttpClientNotConfigured)));
    }

    #[test]
    fn test_with_default_http_client() {
        let result = StooqAdapter::with_default_http_client();
        assert!(result.is_ok());
    }
}

mod base_adapter_tests {
    use super::*;

    #[test]
    fn test_source_name() {
        let adapter = StooqAdapter::new();
        assert_eq!(adapter.source_name(), "stooq");
    }

    #[test]
    fn test_get_ohlcv_mapping_contains_required_fields() {
        let adapter = StooqAdapter::new();
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

mod date_validation_tests {
    use super::*;

    #[test]
    fn test_invalid_month_out_of_range() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-13-01");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_invalid_day_out_of_range() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-01-32");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_invalid_february_30th() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-02-30");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_valid_leap_year_february_29th() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-02-29").unwrap();
        assert_eq!(result, "2024-02-29T00:00:00Z");
    }

    #[test]
    fn test_invalid_non_leap_year_february_29th() {
        let result = StooqAdapter::date_to_iso_timestamp("2023-02-29");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }
}
