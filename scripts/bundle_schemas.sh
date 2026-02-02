#!/bin/bash
# Bundle JSON Schema files by resolving $ref references
# This is required for typify which doesn't support external $ref
# Usage: ./scripts/bundle_schemas.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SCHEMAS_DIR="$PROJECT_ROOT/src/marketschema/schemas"
OUTPUT_DIR="$PROJECT_ROOT/rust/bundled"

echo "Bundling JSON Schema files..."
echo "  Source: $SCHEMAS_DIR"
echo "  Output: $OUTPUT_DIR"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# List of schemas to bundle (excluding definitions.json which is the source of $refs)
SCHEMAS=(
    "quote.json"
    "ohlcv.json"
    "trade.json"
    "orderbook.json"
    "instrument.json"
    "derivative_info.json"
    "expiry_info.json"
    "option_info.json"
    "volume_info.json"
)

# Also bundle definitions.json directly (no $refs to resolve)
cp "$SCHEMAS_DIR/definitions.json" "$OUTPUT_DIR/definitions.json"
echo "  Copied: definitions.json"

# Bundle each schema by resolving $ref
for schema in "${SCHEMAS[@]}"; do
    echo "  Bundling: $schema"
    cd "$SCHEMAS_DIR"
    npx json-refs resolve "$schema" > "$OUTPUT_DIR/$schema"
done

echo "Done! Bundled schemas in $OUTPUT_DIR"
