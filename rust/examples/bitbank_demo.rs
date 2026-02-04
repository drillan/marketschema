//! Demo script for bitbank adapter.
//!
//! # Usage
//!
//! ```bash
//! # Default symbol (btc_jpy)
//! cargo run -p marketschema --example bitbank_demo
//!
//! # Custom symbol
//! cargo run -p marketschema --example bitbank_demo -- eth_jpy
//! ```
//!
//! # Supported Endpoints
//!
//! This demo fetches data from all four bitbank Public API endpoints:
//! - Ticker → Quote
//! - Transactions → Trade[]
//! - Candlestick → OHLCV[]
//! - Depth → OrderBook

mod bitbank;

use std::env;
use std::process::ExitCode;

use bitbank::{BitbankAdapter, BitbankError};
use chrono::Utc;
use marketschema_http::HttpError;

/// Default trading pair if not provided.
const DEFAULT_PAIR: &str = "btc_jpy";

/// Number of items to display for list data.
const DEMO_DISPLAY_COUNT: usize = 5;

/// Number of order book levels to display.
const ORDERBOOK_DISPLAY_LEVELS: usize = 5;

/// Separator line width.
const SEPARATOR_WIDTH: usize = 60;

fn print_separator() {
    println!("{}", "=".repeat(SEPARATOR_WIDTH));
}

fn print_section(title: &str) {
    println!();
    print_separator();
    println!("{}", title);
    print_separator();
}

#[tokio::main]
async fn main() -> ExitCode {
    print_separator();
    println!("bitbank Adapter Demo");
    print_separator();

    let pair = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_PAIR.to_string());
    println!("Trading Pair: {}", pair);

    let adapter = match BitbankAdapter::with_default_http_client() {
        Ok(adapter) => adapter,
        Err(e) => {
            eprintln!("\nError: Failed to create HTTP client: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Fetch Ticker (Quote)
    print_section("Ticker (Quote)");
    match adapter.fetch_ticker(&pair).await {
        Ok(quote) => {
            println!("Symbol:    {}", quote.symbol);
            println!("Bid:       {:.2}", quote.bid);
            println!("Ask:       {:.2}", quote.ask);
            println!("Timestamp: {}", quote.timestamp);
        }
        Err(e) => {
            print_error(&e, &pair);
            return ExitCode::FAILURE;
        }
    }

    // Fetch Transactions (Trade)
    print_section("Transactions (Trade)");
    match adapter.fetch_transactions(&pair).await {
        Ok(trades) => {
            println!("Total trades: {}\n", trades.len());
            println!("Latest {} trades:", DEMO_DISPLAY_COUNT.min(trades.len()));
            for trade in trades.iter().take(DEMO_DISPLAY_COUNT) {
                println!(
                    "  {} | {:>4} | Price: {:>12.2} | Size: {}",
                    trade.timestamp, trade.side, trade.price, trade.size
                );
            }
        }
        Err(e) => {
            print_error(&e, &pair);
            return ExitCode::FAILURE;
        }
    }

    // Fetch Candlestick (OHLCV)
    print_section("Candlestick (OHLCV)");
    let today = Utc::now().format("%Y%m%d").to_string();
    match adapter.fetch_candlestick(&pair, "1hour", &today).await {
        Ok(ohlcv_data) => {
            println!("Date: {} | Candle type: 1hour", today);
            println!("Total records: {}\n", ohlcv_data.len());

            if ohlcv_data.is_empty() {
                println!("No candlestick data available for today");
            } else {
                println!(
                    "Latest {} records:",
                    DEMO_DISPLAY_COUNT.min(ohlcv_data.len())
                );
                for ohlcv in ohlcv_data.iter().rev().take(DEMO_DISPLAY_COUNT) {
                    println!(
                        "  {} | O={:.2} H={:.2} L={:.2} C={:.2} V={:.4}",
                        ohlcv.timestamp,
                        ohlcv.open,
                        ohlcv.high,
                        ohlcv.low,
                        ohlcv.close,
                        ohlcv.volume
                    );
                }
            }
        }
        Err(e) => {
            print_error(&e, &pair);
            return ExitCode::FAILURE;
        }
    }

    // Fetch Depth (OrderBook)
    print_section("Depth (OrderBook)");
    match adapter.fetch_depth(&pair).await {
        Ok(orderbook) => {
            println!("Symbol:    {}", orderbook.symbol);
            println!("Timestamp: {}", orderbook.timestamp);
            println!(
                "Ask levels: {} | Bid levels: {}\n",
                orderbook.asks.len(),
                orderbook.bids.len()
            );

            println!(
                "Top {} ask levels (sell orders):",
                ORDERBOOK_DISPLAY_LEVELS.min(orderbook.asks.len())
            );
            for level in orderbook.asks.iter().take(ORDERBOOK_DISPLAY_LEVELS) {
                println!("  Price: {:>12.2} | Size: {}", level.price, level.size);
            }

            println!(
                "\nTop {} bid levels (buy orders):",
                ORDERBOOK_DISPLAY_LEVELS.min(orderbook.bids.len())
            );
            for level in orderbook.bids.iter().take(ORDERBOOK_DISPLAY_LEVELS) {
                println!("  Price: {:>12.2} | Size: {}", level.price, level.size);
            }
        }
        Err(e) => {
            print_error(&e, &pair);
            return ExitCode::FAILURE;
        }
    }

    print_section("Demo completed!");

    ExitCode::SUCCESS
}

fn print_error(e: &BitbankError, pair: &str) {
    match e {
        BitbankError::Http(HttpError::Status {
            status_code,
            message,
            ..
        }) => {
            if *status_code == 404 {
                eprintln!("Error: Trading pair '{}' not found", pair);
            } else {
                eprintln!("Error: HTTP {} - {}", status_code, message);
            }
        }
        BitbankError::Http(HttpError::Timeout { message, .. }) => {
            eprintln!("Error: Request timed out: {}", message);
        }
        BitbankError::Http(HttpError::Connection { message, .. }) => {
            eprintln!("Error: Connection failed: {}", message);
        }
        BitbankError::ApiError { success_code, .. } => {
            eprintln!(
                "Error: bitbank API returned error (success={})",
                success_code
            );
        }
        _ => {
            eprintln!("Error: {}", e);
        }
    }
}
