# Rust Implementation Specification: Adapter Interface

**Feature Branch**: `004-adapter-rust`
**Parent Spec**: [004-adapter](../004-adapter/spec.md)
**Status**: Planned

## Overview

Rust でのアダプターインターフェース実装仕様。
現時点では設計方針のみを記載し、実装は将来対応とする。

## Module Structure (Proposed)

```
crates/marketschema-adapters/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public exports
│   ├── adapter.rs       # BaseAdapter trait
│   ├── mapping.rs       # ModelMapping struct
│   ├── registry.rs      # AdapterRegistry
│   └── transforms.rs    # Transform functions
```

## Dependencies (Proposed)

```toml
# Cargo.toml
[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"

# For HTTP client
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Language-Specific Considerations

- `async_trait` for async trait methods
- `Arc<dyn Fn(...) + Send + Sync>` for transform functions
- `once_cell::sync::Lazy` for global registry
- `RwLock` for thread-safe registry access
- `thiserror` for error type derivation
- `#[source]` attribute for error chaining

## Contracts

- [Adapter Contract](contracts/adapter.md) - BaseAdapter trait, ModelMapping, AdapterRegistry の Rust 定義
- [Transform Functions](contracts/transforms.md) - 変換関数の Rust 定義

## Implementation Priorities

1. **Phase 1**: Core traits and structs (ModelMapping, Transforms)
2. **Phase 2**: BaseAdapter trait with serde integration
3. **Phase 3**: AdapterRegistry with thread-safe global state
4. **Phase 4**: Derive macro for boilerplate reduction
5. **Phase 5**: Integration with marketschema-models crate

## Notes

- Rust 実装は Python 実装の成熟後に着手予定
- serde との統合を重視し、JSON/MessagePack/CBOR 等をサポート予定
- async-trait を使用して非同期サポートを提供
- エラーハンドリングは thiserror を使用

## Reference

- [002-data-model-rust](../002-data-model-rust/spec.md) - Rust struct generation
- [003-http-client-rust](../003-http-client-rust/spec.md) - Rust HTTP client
- [Rust typify](https://github.com/oxidecomputer/typify) - JSON Schema to Rust code generation
