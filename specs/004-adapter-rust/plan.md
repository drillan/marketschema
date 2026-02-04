# Implementation Plan: Adapter Interface Rust Implementation

**Branch**: `004-adapter-rust` | **Date**: 2026-02-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-adapter-rust/spec.md`

## Summary

marketschema ライブラリの Rust 実装におけるアダプターフレームワーク（BaseAdapter trait, ModelMapping, Transforms, AdapterRegistry）を提供する。`async_trait` による非同期 trait メソッド、`Arc<dyn Fn + Send + Sync>` によるスレッドセーフな transform 関数、`once_cell::sync::Lazy` によるグローバルレジストリを実装する。

## Technical Context

**Language/Version**: Rust 2024 edition (latest stable, MSRV 1.71.1 for moka compatibility)
**Primary Dependencies**: async-trait 0.1, chrono 0.4, once_cell 1.19, serde 1.0, serde_json 1.0, thiserror 2.0, tokio 1.0
**Storage**: N/A（インメモリ処理のみ）
**Testing**: cargo test + wiremock（HTTP モック）
**Target Platform**: Linux/macOS/Windows（クロスプラットフォーム）
**Project Type**: Rust library crate（ワークスペース内）
**Performance Goals**: 変換処理において 10,000 レコード/秒以上
**Constraints**: `Send + Sync` 境界によるスレッドセーフ性必須
**Scale/Scope**: 5 データモデル（Quote, OHLCV, Trade, OrderBook, Instrument）、9 変換関数

## Constitution Check

### Pre-Design Gate (Phase 0)

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Schema First | ✅ PASS | BaseAdapter trait はスキーマ生成モデル（Quote, OHLCV 等）への変換を提供 |
| II. 軽量コア | ✅ PASS | アダプター基盤はコアに含まれるべき共通インフラストラクチャ |
| III. シンプルさ優先 | ✅ PASS | ビルダーパターン、trait デフォルト実装で 80% ユースケースをカバー |
| IV. 言語非依存 | ✅ PASS | 親仕様 004-adapter の Rust 実装として言語固有最適化を行う |
| V. エコシステム拡張 | ✅ PASS | 個別アダプター（bitbank 等）は別 crate として実装予定 |
| 命名の揺れ禁止 | ✅ PASS | 業界標準名（bid, ask, open, high, low, close, volume, price, side, size）を使用 |
| 暗黙的フォールバック禁止 | ✅ PASS | すべての変換関数は Result<T, TransformError> を返す |
| ハードコード禁止 | ✅ PASS | MS_PER_SECOND 等の定数を定義 |

**Gate Result**: ✅ PASS

### Post-Design Re-check (Phase 1)

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Schema First | ✅ PASS | data-model.md で 9 エンティティを定義、スキーマモデルへの変換パス確立 |
| II. 軽量コア | ✅ PASS | 単一 crate（marketschema-adapters）、最小限の依存（6 crates） |
| III. シンプルさ優先 | ✅ PASS | contracts/ で明確な API、quickstart.md で即座に使用可能 |
| IV. 言語非依存 | ✅ PASS | 親仕様 004-adapter のインターフェース契約に準拠 |
| V. エコシステム拡張 | ✅ PASS | AdapterRegistry で外部 crate からのアダプター登録をサポート |
| 命名の揺れ禁止 | ✅ PASS | contracts/transforms.md で side_from_string は "buy"/"sell" のみ返す |
| 暗黙的フォールバック禁止 | ✅ PASS | ModelMapping.apply() は Option/Result で明示的エラー、Value::Null 返却は required=false の場合のみ |
| ハードコード禁止 | ✅ PASS | contracts/transforms.md で MS_PER_SECOND, JST_UTC_OFFSET_HOURS 定数を定義 |

**Final Gate Result**: ✅ PASS - すべての原則を遵守、実装準備完了

## Project Structure

### Documentation (this feature)

```text
specs/004-adapter-rust/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── adapter.md       # BaseAdapter, ModelMapping, AdapterRegistry の Rust 型定義
│   └── transforms.md    # 変換関数の Rust 型定義
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
crates/
├── marketschema-http/          # 既存: HTTP クライアント
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── client.rs
│       ├── error.rs
│       ├── retry.rs
│       ├── rate_limit.rs
│       └── cache.rs
│
└── marketschema-adapters/      # 新規: アダプターフレームワーク
    ├── Cargo.toml
    └── src/
        ├── lib.rs              # Public exports
        ├── adapter.rs          # BaseAdapter trait
        ├── mapping.rs          # ModelMapping struct
        ├── registry.rs         # AdapterRegistry
        ├── transforms.rs       # Transforms functions
        └── error.rs            # Error types

tests/
└── unit/
    └── adapters/               # アダプターテスト
        ├── adapter_test.rs
        ├── mapping_test.rs
        ├── registry_test.rs
        └── transforms_test.rs
```

**Structure Decision**: 単一 crate 構造（`marketschema-adapters`）を採用。既存の `marketschema-http` とは別 crate として、ワークスペースに追加。将来的に `marketschema-http` への依存を追加可能。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - すべての Constitution Check がパス

## Dependencies

### Direct Dependencies

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| async-trait | 0.1 | 非同期 trait メソッド | BaseAdapter trait に必須 |
| chrono | 0.4 | タイムスタンプ処理 | features = ["serde"] |
| once_cell | 1.19 | グローバルシングルトン | AdapterRegistry に必須 |
| serde | 1.0 | シリアライズ | features = ["derive"] |
| serde_json | 1.0 | JSON 処理 | ModelMapping.apply() に必須 |
| thiserror | 2.0 | エラー型 derive | 既存の marketschema-http と同じバージョン |

### Dev Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.0 | テスト用非同期ランタイム | features = ["rt-multi-thread", "macros"] |

### Optional Dependencies (Future)

| Crate | Version | Purpose |
|-------|---------|---------|
| marketschema-http | workspace | HTTP クライアント統合 |
| marketschema-models | workspace | 型付きモデル（typify 生成） |

## Implementation Phases

仕様の Implementation Priorities に従い、以下のフェーズで実装：

1. **Phase 1**: Core types and errors (error.rs) - AdapterError, MappingError, TransformError
2. **Phase 2**: Transform functions (transforms.rs) - Transforms struct と 9 つの変換関数
3. **Phase 3**: ModelMapping with builder pattern (mapping.rs) - ビルダーパターン、apply() メソッド
4. **Phase 4**: BaseAdapter trait with async_trait (adapter.rs) - trait 定義、デフォルト実装
5. **Phase 5**: AdapterRegistry with thread-safe global state (registry.rs) - シングルトン、RwLock
6. **Phase 6**: Integration with marketschema-models crate (将来)

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| async_trait のオーバーヘッド | 低 | Rust 2024 edition で native async trait 検討 |
| グローバル状態のテスト干渉 | 中 | clear() メソッドでテスト間隔離 |
| typify 生成モデルとの互換性 | 中 | serde_json::Value で抽象化し、将来的に型付きモデル対応 |
