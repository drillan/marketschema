# Rust Implementation Guide

**Feature**: 003-http-client
**Status**: Planned

## Library Selection (Proposed)

- **HTTP Client**: [reqwest](https://docs.rs/reqwest/) - 非同期対応、広く採用
- **Async Runtime**: [tokio](https://tokio.rs/) - デファクト標準
- **Error Handling**: [thiserror](https://docs.rs/thiserror/) - エラー型導出

## Language-Specific Considerations

- 所有権とライフタイムの考慮（クライアント共有時は `Arc<Client>`）
- `Send + Sync` 境界が必要（マルチスレッド環境）
- エラー型は `thiserror` で `#[source]` 属性を使用
- リソース管理は `Drop` トレイトと RAII パターン

## Reference

- [Error Taxonomy](../contracts/error-taxonomy.md) - エラー分類契約
- Rust 実装ガイドは実装時に `docs/guides/http-client-rust.md` として作成予定
