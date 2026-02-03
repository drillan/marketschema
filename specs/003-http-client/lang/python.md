# Python Implementation Guide

**Feature**: 003-http-client
**Date**: 2026-02-03
**Status**: Implemented

## Overview

本ドキュメントは 003-http-client の Python 実装ガイドを提供する。
型シグネチャは [contracts/python-api.md](../contracts/python-api.md) を参照。

## Module Structure

```
src/marketschema/http/
├── __init__.py          # Public exports
├── client.py            # AsyncHttpClient implementation
├── exceptions.py        # HTTP exception classes
├── middleware/          # Middleware implementations
│   ├── __init__.py
│   ├── retry.py         # RetryMiddleware
│   └── rate_limit.py    # RateLimitMiddleware
└── cache.py             # ResponseCache implementation
```

## Dependencies

```toml
# pyproject.toml
[project]
requires-python = ">=3.13"
dependencies = [
    "httpx>=0.27.0",
    "pydantic>=2.0.0",
]

[project.optional-dependencies]
dev = [
    "respx>=0.21.0",  # For mocking httpx
    "pytest>=8.0.0",
    "pytest-asyncio>=0.23.0",
]
```

## Library Selection

### HTTP Client: httpx

[httpx](https://www.python-httpx.org/) を選択した理由:

- **async/await ネイティブサポート**: asyncio との統合が優れている
- **requests 互換 API**: 広く知られた API パターン
- **HTTP/2 サポート**: 将来の最適化に対応
- **タイムアウト設定の柔軟性**: connect/read/write/pool 個別設定可能
- **コネクションプーリング**: デフォルトで有効

### Testing: respx

[respx](https://github.com/lundberg/respx) を選択した理由:

- **httpx 専用**: httpx のモックライブラリとして設計
- **シンプルな API**: 直感的なルーティングとレスポンス定義
- **非同期対応**: pytest-asyncio との統合が容易

## Async Model

```python
import asyncio
from marketschema.http import AsyncHttpClient

async def main() -> None:
    async with AsyncHttpClient() as client:
        data = await client.get_json("https://api.example.com/data")
        print(data)

if __name__ == "__main__":
    asyncio.run(main())
```

### Context Manager Pattern

Python では `async with` を使用してリソース管理を行う:

```python
async with AsyncHttpClient() as client:
    # client は自動的に初期化される
    result = await client.get_json(url)
# ブロックを抜けると自動的に close() が呼ばれる
```

### Exception Chaining

Python では `__cause__` を使用して例外チェインを実装:

```python
try:
    await client.get_json(url)
except HttpTimeoutError as e:
    # e.__cause__ で元の httpx.TimeoutException にアクセス可能
    original = e.__cause__
```

## Implementation Details

### Error Mapping

| httpx Exception | marketschema Exception |
|-----------------|------------------------|
| `httpx.TimeoutException` | `HttpTimeoutError` |
| `httpx.ConnectError` | `HttpConnectionError` |
| `httpx.HTTPStatusError` (4xx/5xx) | `HttpStatusError` |
| `httpx.HTTPStatusError` (429) | `HttpRateLimitError` |

### Retry Configuration

```python
from marketschema.http import AsyncHttpClient, RetryConfig

client = AsyncHttpClient(
    retry=RetryConfig(
        max_retries=3,
        backoff_factor=0.5,
        retry_statuses={429, 500, 502, 503, 504},
    )
)
```

### Rate Limiting

```python
from marketschema.http import AsyncHttpClient, RateLimitConfig

client = AsyncHttpClient(
    rate_limit=RateLimitConfig(
        requests_per_second=10.0,
        burst_size=5,
    )
)
```

### Caching

```python
from marketschema.http import AsyncHttpClient, CacheConfig

CACHE_TTL_SECONDS = 300  # 5 minutes
CACHE_MAX_SIZE = 1000

client = AsyncHttpClient(
    cache=CacheConfig(
        ttl=CACHE_TTL_SECONDS,
        max_size=CACHE_MAX_SIZE,
    )
)
```

## Type Checking

```bash
# Run mypy
uv run mypy src/marketschema/http/

# Expected: no errors
```

## Testing Guidelines

### Unit Test Structure

```python
import pytest
import respx
from httpx import Response

from marketschema.http import AsyncHttpClient
from marketschema.http.exceptions import HttpTimeoutError, HttpStatusError


@pytest.fixture
def mock_api() -> respx.MockRouter:
    with respx.mock(assert_all_called=False) as router:
        yield router


class TestAsyncHttpClient:
    @pytest.mark.asyncio
    async def test_get_json_success(self, mock_api: respx.MockRouter) -> None:
        mock_api.get("https://api.example.com/data").mock(
            return_value=Response(200, json={"key": "value"})
        )

        async with AsyncHttpClient() as client:
            result = await client.get_json("https://api.example.com/data")

        assert result == {"key": "value"}

    @pytest.mark.asyncio
    async def test_get_json_timeout(self, mock_api: respx.MockRouter) -> None:
        import httpx

        mock_api.get("https://api.example.com/data").mock(
            side_effect=httpx.TimeoutException("Timeout")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpTimeoutError):
                await client.get_json("https://api.example.com/data")

    @pytest.mark.asyncio
    async def test_get_json_status_error(self, mock_api: respx.MockRouter) -> None:
        mock_api.get("https://api.example.com/data").mock(
            return_value=Response(404, text="Not Found")
        )

        async with AsyncHttpClient() as client:
            with pytest.raises(HttpStatusError) as exc_info:
                await client.get_json("https://api.example.com/data")

        assert exc_info.value.status_code == 404
```

## Existing Implementation

現在の実装は以下のファイルに存在する:

- `src/marketschema/http/__init__.py` - Public exports
- `src/marketschema/http/client.py` - AsyncHttpClient
- `src/marketschema/http/exceptions.py` - Exception classes
- `src/marketschema/http/middleware/` - Middleware implementations
- `src/marketschema/http/cache.py` - ResponseCache

## Reference

- [HTTP Client Guide](../../../docs/guides/http-client.md) - 実践的チュートリアル
- [Python API Contract](../contracts/python-api.md) - API 契約
- [Error Taxonomy](../contracts/error-taxonomy.md) - エラー分類
- [httpx Documentation](https://www.python-httpx.org/) - httpx 公式ドキュメント
- [respx Documentation](https://github.com/lundberg/respx) - respx 公式ドキュメント
