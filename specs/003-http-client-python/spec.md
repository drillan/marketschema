# Python Implementation Specification: HTTP Client

**Feature Branch**: `003-http-client-python`
**Parent Spec**: [003-http-client](../003-http-client/spec.md)
**Status**: Implemented

## Overview

Python での HTTP クライアント実装仕様。

## Library Selection

- **HTTP Client**: [httpx](https://www.python-httpx.org/) - Async support, complete type hints
- **Testing**: [respx](https://github.com/lundberg/respx) - httpx mocking

## Language-Specific Considerations

- Python 3.13+ required
- `async/await` for asynchronous processing
- Context manager (`async with`) for resource management
- Exception chaining via `__cause__` for accessing original exception

## Module Structure

```
src/marketschema/http/
├── __init__.py       # Public exports
├── client.py         # AsyncHttpClient
├── exceptions.py     # Exception classes
├── middleware.py     # RetryMiddleware, RateLimitMiddleware
└── cache.py          # ResponseCache
```

## Contracts

- [Python API Contract](contracts/python-api.md) - API contract with Python type signatures

## Reference

- [HTTP Client Guide](../../docs/guides/http-client.md) - Implementation guide with code examples
- [Error Taxonomy](../003-http-client/contracts/error-taxonomy.md) - Language-independent error classification
