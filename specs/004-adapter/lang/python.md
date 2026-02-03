# Python Implementation Guide

**Feature**: 004-adapter
**Date**: 2026-02-03
**Status**: Implemented

## Overview

本ドキュメントは 004-adapter の Python 実装ガイドを提供する。
型シグネチャは [contracts/adapter-interface.md](../contracts/adapter-interface.md) および [contracts/transforms.md](../contracts/transforms.md) を参照。

## Module Structure

```
src/marketschema/adapters/
├── __init__.py          # Public exports
├── base.py              # BaseAdapter implementation
├── mapping.py           # ModelMapping implementation
├── registry.py          # AdapterRegistry and @register decorator
└── transforms.py        # Transforms static methods
```

## Dependencies

```toml
# pyproject.toml
[project]
requires-python = ">=3.13"
dependencies = [
    "pydantic>=2.0.0",
    "httpx>=0.27.0",
]
```

## Implementation Requirements

### BaseAdapter

#### Class Attributes

```python
class BaseAdapter:
    # Required: Must be overridden by subclasses
    source_name: str = ""

    # Default transforms class (can be overridden)
    transforms: type[Transforms] = Transforms
```

#### Constructor

```python
def __init__(
    self,
    http_client: AsyncHttpClient | None = None,
) -> None:
    """Initialize the adapter.

    Raises AdapterError if source_name is empty.
    """
    if not self.source_name:
        raise AdapterError(f"{self.__class__.__name__} must define source_name")

    self._http_client: AsyncHttpClient | None = http_client
    self._owns_http_client = http_client is None
```

#### HTTP Client Integration

```python
@property
def http_client(self) -> AsyncHttpClient:
    """Lazy initialization of HTTP client."""
    if self._http_client is None:
        from marketschema.http import AsyncHttpClient
        self._http_client = AsyncHttpClient()
        self._owns_http_client = True
    return self._http_client
```

#### Async Context Manager

```python
async def __aenter__(self) -> Self:
    return self

async def __aexit__(
    self,
    exc_type: type[BaseException] | None,
    exc_val: BaseException | None,
    exc_tb: TracebackType | None,
) -> None:
    await self.close()
```

### ModelMapping

#### Dataclass Definition

```python
from dataclasses import dataclass

@dataclass(frozen=True, slots=True)
class ModelMapping:
    """Immutable mapping definition.

    frozen=True: Prevents modification after creation
    slots=True: Memory-efficient, faster attribute access
    """
    target_field: str
    source_field: str
    transform: Callable[[Any], Any] | None = None
    default: Any | None = None
    required: bool = True
```

#### apply() Method Logic

```python
def apply(self, source_data: dict[str, Any]) -> Any:
    value = self._get_nested_value(source_data, self.source_field)

    if value is None:
        if self.default is not None:
            return self.default
        if self.required:
            raise MappingError(
                f"Required field '{self.source_field}' is missing from source data"
            )
        return None

    if self.transform is not None:
        return self.transform(value)

    return value
```

### AdapterRegistry

#### Singleton Pattern

```python
class AdapterRegistry:
    _instance: "AdapterRegistry | None" = None
    _adapters: dict[str, type[BaseAdapter]]

    def __new__(cls) -> "AdapterRegistry":
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._adapters = {}
        return cls._instance
```

#### Generic Type Parameter

```python
@classmethod
def register[T: BaseAdapter](cls, adapter_class: type[T]) -> type[T]:
    """Python 3.12+ generic syntax for type-safe registration."""
    ...
```

### Transforms

#### Static Method Pattern

```python
class Transforms:
    """All methods are static and stateless."""

    @staticmethod
    def to_float(value: Any) -> float:
        try:
            return float(value)
        except (TypeError, ValueError) as e:
            raise TransformError(f"Cannot convert {value!r} to float") from e
```

## Usage Examples

### Basic Adapter Implementation

```python
from marketschema.adapters.base import BaseAdapter
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.registry import register
from marketschema.models import Quote

# Constants with source prefix
MYAPI_BASE_URL = "https://api.example.com"
MYAPI_SUCCESS_CODE = 0

@register
class MyApiAdapter(BaseAdapter):
    """Example adapter implementation."""

    source_name = "myapi"

    def get_quote_mapping(self) -> list[ModelMapping]:
        return [
            ModelMapping("bid", "bid_price", transform=self.transforms.to_float),
            ModelMapping("ask", "ask_price", transform=self.transforms.to_float),
            ModelMapping(
                "timestamp",
                "time",
                transform=self.transforms.unix_timestamp_ms,
            ),
        ]

    async def fetch_quote(self, symbol: str) -> Quote:
        """Fetch quote data from API."""
        url = f"{MYAPI_BASE_URL}/ticker"
        data = await self.http_client.get_json(url, params={"symbol": symbol})
        return self._parse_quote(data, symbol=symbol)

    def _parse_quote(self, raw_data: dict, *, symbol: str) -> Quote:
        """Parse raw data into Quote model."""
        data_with_symbol = {**raw_data, "symbol": symbol}
        mappings = self.get_quote_mapping() + [ModelMapping("symbol", "symbol")]
        return self._apply_mapping(data_with_symbol, mappings, Quote)
```

### Using Adapter with Context Manager

```python
async def main() -> None:
    async with MyApiAdapter() as adapter:
        quote = await adapter.fetch_quote("btc_jpy")
        print(f"Bid: {quote.bid}, Ask: {quote.ask}")
```

### Using AdapterRegistry

```python
from marketschema.adapters.registry import AdapterRegistry

# Get adapter by source name
adapter = AdapterRegistry.get("myapi")

# List all registered adapters
names = AdapterRegistry.list_adapters()

# Check if registered
if AdapterRegistry.is_registered("myapi"):
    print("myapi adapter is available")
```

### Custom Transform Function

```python
def my_custom_transform(value: str) -> str:
    """Custom transform that must follow the contract."""
    try:
        # Custom logic
        return value.strip().upper()
    except Exception as e:
        raise TransformError(f"Custom transform failed: {value!r}") from e

# Use in mapping
ModelMapping("symbol", "s", transform=my_custom_transform)
```

## Type Checking

```bash
# Run mypy
uv run mypy src/marketschema/adapters/

# Expected: no errors
```

## Testing Guidelines

### Unit Test Structure

```python
import pytest
from marketschema.adapters.mapping import ModelMapping
from marketschema.adapters.transforms import Transforms
from marketschema.exceptions import MappingError, TransformError


class TestModelMapping:
    def test_simple_mapping(self) -> None:
        mapping = ModelMapping("target", "source")
        result = mapping.apply({"source": "value"})
        assert result == "value"

    def test_nested_path(self) -> None:
        mapping = ModelMapping("target", "outer.inner")
        result = mapping.apply({"outer": {"inner": "value"}})
        assert result == "value"

    def test_required_field_missing(self) -> None:
        mapping = ModelMapping("target", "missing", required=True)
        with pytest.raises(MappingError):
            mapping.apply({})


class TestTransforms:
    def test_to_float_string(self) -> None:
        assert Transforms.to_float("123.45") == 123.45

    def test_to_float_invalid(self) -> None:
        with pytest.raises(TransformError):
            Transforms.to_float("invalid")
```

## Existing Implementation

現在の実装は以下のファイルに存在する:

- `src/marketschema/adapters/base.py` - BaseAdapter
- `src/marketschema/adapters/mapping.py` - ModelMapping
- `src/marketschema/adapters/registry.py` - AdapterRegistry, register
- `src/marketschema/adapters/transforms.py` - Transforms

サンプルアダプターは `examples/` ディレクトリに存在する:

- `examples/bitbank/` - bitbank API adapter
- `examples/stooq/` - stooq CSV adapter
- `examples/stockanalysis/` - stockanalysis HTML scraper

## Reference

- [Adapter Development Guide](../../../docs/guides/adapter-development.md) - 実践的チュートリアル
- [HTTP Client Guide](../../../docs/guides/http-client.md) - HTTP クライアント使用方法
- [003-http-client contracts](../../003-http-client/contracts/python-api.md) - HTTP クライアント契約
