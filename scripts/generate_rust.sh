#!/bin/bash
# Generate Rust structs from bundled JSON Schema files using typify
# Prerequisites: cargo install cargo-typify
# Usage: ./scripts/generate_rust.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUNDLED_DIR="$PROJECT_ROOT/rust/bundled"
RUST_TYPES_DIR="$PROJECT_ROOT/rust/src/types"

echo "Generating Rust structs from JSON Schema..."
echo "  Source: $BUNDLED_DIR"
echo "  Output: $RUST_TYPES_DIR"

# Ensure output directory exists
mkdir -p "$RUST_TYPES_DIR"

# First, bundle schemas if not already done
if [ ! -d "$BUNDLED_DIR" ] || [ -z "$(ls -A $BUNDLED_DIR 2>/dev/null)" ]; then
    echo "Bundled schemas not found. Running bundle_schemas.sh first..."
    "$SCRIPT_DIR/bundle_schemas.sh"
fi

# Generate Rust code for each bundled schema
SCHEMAS=(
    "definitions"
    "quote"
    "ohlcv"
    "trade"
    "orderbook"
    "instrument"
    "derivative_info"
    "expiry_info"
    "option_info"
    "volume_info"
)

for schema in "${SCHEMAS[@]}"; do
    echo "  Generating: ${schema}.rs"
    cargo typify "$BUNDLED_DIR/${schema}.json" --output "$RUST_TYPES_DIR/${schema}.rs" 2>/dev/null || {
        echo "  Warning: typify failed for ${schema}.json, skipping..."
    }
done

# Create mod.rs to export all modules (alphabetically sorted)
echo "Creating mod.rs..."
cat > "$RUST_TYPES_DIR/mod.rs" << 'EOF'
//! Generated types from JSON Schema

pub mod definitions;
pub mod derivative_info;
pub mod expiry_info;
pub mod instrument;
pub mod ohlcv;
pub mod option_info;
pub mod orderbook;
pub mod quote;
pub mod trade;
pub mod volume_info;

// Re-export commonly used types
pub use definitions::*;
pub use instrument::Instrument;
pub use ohlcv::Ohlcv;
pub use orderbook::OrderBook;
pub use quote::Quote;
pub use trade::Trade;
EOF

echo "Done! Rust types generated in $RUST_TYPES_DIR"
