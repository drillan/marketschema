# Python Implementation Guide

**Feature**: 003-http-client
**Status**: Implemented

## Library Selection

- **HTTP Client**: [httpx](https://www.python-httpx.org/) - 非同期対応、型ヒント完備
- **Testing**: [respx](https://github.com/lundberg/respx) - httpx モック

## Language-Specific Considerations

- Python 3.13 以上が必要
- `async/await` による非同期処理
- コンテキストマネージャ（`async with`）でリソース管理
- 例外チェインは `__cause__` で元の例外にアクセス可能

## Module Structure

```
src/marketschema/http/
├── __init__.py       # Public exports
├── client.py         # AsyncHttpClient
├── exceptions.py     # Exception classes
├── middleware.py     # RetryMiddleware, RateLimitMiddleware
└── cache.py          # ResponseCache
```

## Reference

- [HTTP Client Guide](../../../docs/guides/http-client.md) - 実装ガイド・コード例
- [Python API Contract](../contracts/python-api.md) - API 契約
- [Error Taxonomy](../contracts/error-taxonomy.md) - エラー分類
