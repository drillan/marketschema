//! Demo script for Stooq adapter.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p marketschema-examples --example stooq_demo -- spy.us
//! ```

use std::env;
use std::process::ExitCode;

use marketschema_examples::stooq::{StooqAdapter, StooqError};
use marketschema_http::HttpError;

/// Default symbol to fetch if not provided.
const DEFAULT_SYMBOL: &str = "spy.us";

/// Number of records to display in the demo.
const DEMO_DISPLAY_COUNT: usize = 5;

/// Separator line width.
const SEPARATOR_WIDTH: usize = 60;

fn print_separator() {
    println!("{}", "=".repeat(SEPARATOR_WIDTH));
}

#[tokio::main]
async fn main() -> ExitCode {
    print_separator();
    println!("Stooq Adapter Demo");
    print_separator();

    let symbol = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_SYMBOL.to_string());
    println!("Symbol: {}", symbol);

    let adapter = match StooqAdapter::with_default_http_client() {
        Ok(adapter) => adapter,
        Err(e) => {
            eprintln!("\nError: Failed to create HTTP client: {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("\nFetching data from stooq.com...\n");

    match adapter.fetch_and_parse(&symbol).await {
        Ok(ohlcv_data) => {
            println!("Received {} records\n", ohlcv_data.len());
            print_separator();
            println!("Latest {} records:", DEMO_DISPLAY_COUNT);
            print_separator();

            // Show the most recent records (data is usually in chronological order)
            let display_data: Vec<_> = ohlcv_data.iter().rev().take(DEMO_DISPLAY_COUNT).collect();

            for ohlcv in display_data {
                println!(
                    "{}: O={:.2} H={:.2} L={:.2} C={:.2} V={}",
                    ohlcv.timestamp, ohlcv.open, ohlcv.high, ohlcv.low, ohlcv.close, ohlcv.volume
                );
            }

            print_separator();
            println!("Demo completed!");
            print_separator();

            ExitCode::SUCCESS
        }
        Err(e) => {
            match &e {
                StooqError::Http(HttpError::Status {
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
                StooqError::Http(HttpError::Timeout { message, .. }) => {
                    eprintln!("\nError: Request timed out: {}", message);
                }
                StooqError::Http(HttpError::Connection { message, .. }) => {
                    eprintln!("\nError: Connection failed: {}", message);
                }
                _ => {
                    eprintln!("\nError: Failed to fetch data: {}", e);
                }
            }
            ExitCode::FAILURE
        }
    }
}
