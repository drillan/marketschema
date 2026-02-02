# marketschema

Unified market data schema for financial applications.

## Overview

marketschema provides standardized data models for financial market data:
- **Quote** - Best bid/offer (BBO) prices
- **OHLCV** - Candlestick/bar data
- **Trade** - Individual trades (time & sales)
- **OrderBook** - Multi-level order book
- **Instrument** - Security/asset information

## Features

- JSON Schema definitions (Draft 2020-12)
- Python pydantic v2 models (auto-generated)
- Rust structs (auto-generated)
- Adapter framework for data source integration

## Installation

```bash
# Clone the repository
git clone https://github.com/example/marketschema.git
cd marketschema

# Install Python dependencies
uv sync --group dev

# Install JSON Schema validation tools
npm install
```

## Quick Start

### Python

```python
from marketschema import Quote, Trade, OHLCV, Side

# Create a quote
quote = Quote(
    symbol="AAPL",
    timestamp="2026-02-02T14:30:00Z",
    bid=175.00,
    ask=175.50,
)

# Create a trade
trade = Trade(
    symbol="AAPL",
    timestamp="2026-02-02T14:30:00.123Z",
    price=175.25,
    size=100,
    side=Side.buy,
)
```

### Using Adapters

```python
from marketschema import BaseAdapter, ModelMapping, register, AdapterRegistry

@register
class MyExchangeAdapter(BaseAdapter):
    source_name = "my_exchange"

    def get_quote_mapping(self):
        return [
            ModelMapping("symbol", "ticker"),
            ModelMapping("timestamp", "time", transform=self.transforms.unix_timestamp_ms),
            ModelMapping("bid", "best_bid", transform=self.transforms.to_float),
            ModelMapping("ask", "best_ask", transform=self.transforms.to_float),
        ]

# Get adapter from registry
adapter = AdapterRegistry.get("my_exchange")
```

### Rust

```rust
use marketschema::Quote;

let json = r#"{"symbol": "AAPL", "timestamp": "2026-02-02T14:30:00Z", "bid": 175.00, "ask": 175.50}"#;
let quote: Quote = serde_json::from_str(json)?;
```

## Code Generation

### Python Models

```bash
# Generate pydantic models from JSON Schema
./scripts/generate_models.sh
# or
make generate-models
```

### Rust Structs

```bash
# Bundle schemas and generate Rust code
./scripts/bundle_schemas.sh
./scripts/generate_rust.sh
# or
make generate-rust
```

## Development

```bash
# Run linter
make lint

# Run type checker
make typecheck

# Run tests
make test

# Run all checks
make all
```

## Project Structure

```
marketschema/
├── src/marketschema/
│   ├── schemas/          # JSON Schema files
│   ├── models/           # Generated pydantic models
│   ├── adapters/         # Adapter framework
│   └── exceptions.py     # Custom exceptions
├── rust/
│   ├── src/types/        # Generated Rust structs
│   └── tests/            # Rust tests
├── tests/
│   ├── unit/             # Unit tests
│   ├── integration/      # Integration tests
│   └── contract/         # Schema compliance tests
└── scripts/              # Code generation scripts
```

## License

MIT
