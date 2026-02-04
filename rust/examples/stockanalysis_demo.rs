//! Demo script for StockAnalysis adapter.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p marketschema --example stockanalysis_demo -- tsla
//! ```

mod stockanalysis;

use std::env;
use std::process::ExitCode;

use marketschema_http::HttpError;
use stockanalysis::{StockAnalysisAdapter, StockAnalysisError};

/// Default symbol to fetch if not provided.
const DEFAULT_SYMBOL: &str = "tsla";

/// Number of records to display in the demo.
const DEMO_DISPLAY_COUNT: usize = 5;

/// Separator line width.
const SEPARATOR_WIDTH: usize = 70;

fn print_separator() {
    println!("{}", "=".repeat(SEPARATOR_WIDTH));
}

#[tokio::main]
async fn main() -> ExitCode {
    print_separator();
    println!("StockAnalysis Adapter Demo");
    print_separator();

    let symbol = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_SYMBOL.to_string());
    println!("Symbol: {}", symbol.to_uppercase());

    let adapter = match StockAnalysisAdapter::with_default_http_client() {
        Ok(adapter) => adapter,
        Err(e) => {
            eprintln!("\nError: Failed to create HTTP client: {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("\nFetching data from stockanalysis.com...\n");

    // Fetch extended OHLCV data (includes adj_close)
    match adapter.fetch_and_parse_extended(&symbol).await {
        Ok(ohlcv_data) => {
            println!("Received {} records\n", ohlcv_data.len());
            print_separator();
            println!("Latest {} records (with adjusted close):", DEMO_DISPLAY_COUNT);
            print_separator();

            // Show the most recent records (data is in reverse chronological order)
            let display_data: Vec<_> = ohlcv_data.iter().take(DEMO_DISPLAY_COUNT).collect();

            for ohlcv in display_data {
                println!(
                    "{}: O={:.2} H={:.2} L={:.2} C={:.2} AC={:.2} V={}",
                    ohlcv.timestamp,
                    ohlcv.open,
                    ohlcv.high,
                    ohlcv.low,
                    ohlcv.close,
                    ohlcv.adj_close,
                    ohlcv.volume
                );
            }

            print_separator();
            println!("Demo completed!");
            print_separator();

            ExitCode::SUCCESS
        }
        Err(e) => {
            match &e {
                StockAnalysisError::Http(HttpError::Status {
                    status_code,
                    message,
                    ..
                }) => {
                    if *status_code == 404 {
                        eprintln!("\nError: Symbol '{}' not found", symbol);
                    } else {
                        eprintln!("\nError: HTTP {} - {}", status_code, message);
                    }
                }
                StockAnalysisError::Http(HttpError::Timeout { message, .. }) => {
                    eprintln!("\nError: Request timed out: {}", message);
                }
                StockAnalysisError::Http(HttpError::Connection { message, .. }) => {
                    eprintln!("\nError: Connection failed: {}", message);
                }
                StockAnalysisError::Http(HttpError::RateLimit { retry_after, .. }) => {
                    if let Some(delay) = retry_after {
                        eprintln!("\nError: Rate limited. Please retry after {:?}", delay);
                    } else {
                        eprintln!("\nError: Rate limited. Please wait before retrying.");
                    }
                }
                StockAnalysisError::NoTableFound => {
                    eprintln!(
                        "\nError: No data table found. The page structure may have changed, \
                         or the symbol '{}' may not have historical data.",
                        symbol
                    );
                }
                _ => {
                    eprintln!("\nError: Failed to fetch data: {}", e);
                }
            }
            ExitCode::FAILURE
        }
    }
}
