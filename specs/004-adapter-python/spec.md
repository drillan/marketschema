# Python Implementation Specification: Adapter Interface

**Feature Branch**: `004-adapter-python`
**Parent Spec**: [004-adapter](../004-adapter/spec.md)
**Status**: Implemented

## Overview

Python でのアダプターインターフェース実装仕様。
型シグネチャは [contracts/](contracts/) を参照。

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

## Language-Specific Considerations

- Python 3.13+ required
- `async/await` for asynchronous processing
- `async with` for resource management
- `dataclass(frozen=True, slots=True)` for ModelMapping
- Type generics with `[T: BaseAdapter]` syntax (Python 3.12+)
- Exception chaining via `from e`

## Contracts

- [Adapter Interface](contracts/adapter-interface.md) - BaseAdapter, ModelMapping, AdapterRegistry の Python 型シグネチャ
- [Transform Functions](contracts/transforms.md) - 変換関数の Python 型シグネチャ

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

## Type Checking

```bash
# Run mypy
uv run mypy src/marketschema/adapters/

# Expected: no errors
```

## Reference

- [Adapter Development Guide](../../docs/guides/adapter-development.md) - 実践的チュートリアル
- [HTTP Client Guide](../../docs/guides/http-client.md) - HTTP クライアント使用方法
- [003-http-client-python](../003-http-client-python/spec.md) - HTTP クライアント Python 実装
