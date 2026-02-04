//! Tests for the Stooq adapter.

use marketschema_examples::stooq::{
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
    fn test_invalid_format_wrong_year_length() {
        let result = StooqAdapter::date_to_iso_timestamp("24-01-15");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
    }

    #[test]
    fn test_invalid_format_wrong_month_length() {
        let result = StooqAdapter::date_to_iso_timestamp("2024-1-15");
        assert!(matches!(result, Err(StooqError::InvalidDateFormat { .. })));
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
            Err(StooqError::InsufficientColumns { expected: 6, actual: 3 })
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
