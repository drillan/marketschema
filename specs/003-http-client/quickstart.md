# Quickstart: HTTP Client Layer

**Feature**: 003-http-client
**Date**: 2026-02-03

## Installation

```bash
# Install marketschema with HTTP support
pip install marketschema[http]

# Or with uv
uv add marketschema[http]
```

## Basic Usage

### Direct HTTP Client Usage

```python
import asyncio
from marketschema.http import AsyncHttpClient

async def main():
    async with AsyncHttpClient() as client:
        # Get JSON response
        data = await client.get_json("https://api.example.com/data")
        print(data)

        # Get text response
        html = await client.get_text("https://example.com")
        print(html)

        # Get raw response
        response = await client.get("https://api.example.com/data")
        print(response.status_code)

asyncio.run(main())
```

### Using with BaseAdapter

```python
import asyncio
from marketschema.adapters.base import BaseAdapter
from marketschema.models import Quote

class MyExchangeAdapter(BaseAdapter):
    source_name = "myexchange"

    async def fetch_quote(self, symbol: str) -> Quote:
        url = f"https://api.myexchange.com/ticker/{symbol}"
        data = await self.http_client.get_json(url)

        return Quote(
            symbol=symbol,
            timestamp=data["timestamp"],
            bid=float(data["bid"]),
            ask=float(data["ask"]),
        )

async def main():
    async with MyExchangeAdapter() as adapter:
        quote = await adapter.fetch_quote("BTC-USD")
        print(f"BTC/USD: bid={quote.bid}, ask={quote.ask}")

asyncio.run(main())
```

## Error Handling

```python
from marketschema.http import (
    AsyncHttpClient,
    HttpError,
    HttpTimeoutError,
    HttpConnectionError,
    HttpStatusError,
    HttpRateLimitError,
)

async def fetch_with_error_handling(url: str) -> dict:
    async with AsyncHttpClient(timeout=10.0) as client:
        try:
            return await client.get_json(url)
        except HttpTimeoutError:
            print(f"Request to {url} timed out")
            raise
        except HttpConnectionError:
            print(f"Failed to connect to {url}")
            raise
        except HttpRateLimitError as e:
            print(f"Rate limited (429). Retry after: {e.retry_after}")
            raise
        except HttpStatusError as e:
            print(f"HTTP error {e.status_code} from {url}")
            raise
        except HttpError as e:
            print(f"HTTP error: {e}")
            raise
```

## Configuration Options

### Timeout and Connections

```python
client = AsyncHttpClient(
    timeout=60.0,         # Request timeout in seconds (default: 30)
    max_connections=200,  # Max concurrent connections (default: 100)
    headers={             # Default headers for all requests
        "User-Agent": "MyApp/1.0",
        "Accept": "application/json",
    },
)
```

### With Retry (Phase 2)

```python
from marketschema.http import AsyncHttpClient, RetryMiddleware

client = AsyncHttpClient(
    retry=RetryMiddleware(
        max_retries=5,            # Maximum retry attempts (default: 3)
        backoff_factor=1.0,       # Backoff multiplier (default: 0.5)
        retry_statuses={503, 504}, # Status codes to retry
    )
)
```

### With Rate Limiting (Phase 2)

```python
from marketschema.http import AsyncHttpClient, RateLimitMiddleware

client = AsyncHttpClient(
    rate_limit=RateLimitMiddleware(
        requests_per_second=10.0,  # Max requests per second
        burst_size=20,             # Allow burst of 20 requests
    )
)
```

### With Caching (Phase 3)

```python
from datetime import timedelta
from marketschema.http import AsyncHttpClient, ResponseCache

client = AsyncHttpClient(
    cache=ResponseCache(
        max_size=500,                     # Max cached entries (default: 1000)
        default_ttl=timedelta(minutes=1), # Cache TTL (default: 5 min)
    )
)
```

## Common Patterns

### Parallel Requests

```python
import asyncio
from marketschema.http import AsyncHttpClient

async def fetch_multiple_symbols():
    symbols = ["BTC-USD", "ETH-USD", "SOL-USD"]

    async with AsyncHttpClient() as client:
        tasks = [
            client.get_json(f"https://api.example.com/ticker/{s}")
            for s in symbols
        ]
        results = await asyncio.gather(*tasks, return_exceptions=True)

    for symbol, result in zip(symbols, results):
        if isinstance(result, Exception):
            print(f"{symbol}: Error - {result}")
        else:
            print(f"{symbol}: {result}")
```

### Custom Headers per Request

```python
async with AsyncHttpClient() as client:
    # Default headers are set in constructor
    # Override or add headers per request
    data = await client.get_json(
        "https://api.example.com/data",
        headers={"Authorization": "Bearer token123"},
    )
```

### Reusing Client Across Requests

```python
class MyService:
    def __init__(self):
        self._client: AsyncHttpClient | None = None

    @property
    def client(self) -> AsyncHttpClient:
        if self._client is None:
            self._client = AsyncHttpClient()
        return self._client

    async def close(self):
        if self._client:
            await self._client.close()

    async def fetch_data(self, endpoint: str) -> dict:
        return await self.client.get_json(f"https://api.example.com/{endpoint}")
```

## Testing

### Mocking HTTP Requests

```python
import pytest
import respx
import httpx
from marketschema.http import AsyncHttpClient

@pytest.mark.asyncio
@respx.mock
async def test_get_json():
    # Setup mock
    respx.get("https://api.example.com/data").mock(
        return_value=httpx.Response(200, json={"key": "value"})
    )

    # Test
    async with AsyncHttpClient() as client:
        result = await client.get_json("https://api.example.com/data")

    assert result == {"key": "value"}
```

### Mocking Errors

```python
import pytest
import respx
import httpx
from marketschema.http import AsyncHttpClient, HttpTimeoutError

@pytest.mark.asyncio
@respx.mock
async def test_timeout_error():
    respx.get("https://api.example.com/data").mock(
        side_effect=httpx.TimeoutException("Timeout")
    )

    async with AsyncHttpClient() as client:
        with pytest.raises(HttpTimeoutError):
            await client.get_json("https://api.example.com/data")
```

## Migration from urllib

### Before (urllib)

```python
import json
import urllib.request

def fetch_ticker(symbol: str) -> dict:
    url = f"https://api.example.com/ticker/{symbol}"
    with urllib.request.urlopen(url, timeout=30) as response:
        return json.loads(response.read().decode("utf-8"))
```

### After (AsyncHttpClient)

```python
from marketschema.http import AsyncHttpClient

async def fetch_ticker(symbol: str) -> dict:
    async with AsyncHttpClient() as client:
        return await client.get_json(
            f"https://api.example.com/ticker/{symbol}"
        )
```

## Next Steps

- [Full API Reference](./contracts/) (coming soon)
- [Architecture Design](./features/http-client-layer/architecture.md)
- [Implementation Plan](./features/http-client-layer/plan.md)
