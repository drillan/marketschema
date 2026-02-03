# Implementation Plan: Rust Data Model Implementation

**Branch**: `002-data-model-rust` | **Date**: 2026-02-03 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-data-model-rust/spec.md`

## Summary

JSON Schema から cargo-typify を使用して Rust struct を自動生成する。typify の `unevaluatedProperties` 未対応問題に対し、バンドル時に `additionalProperties` へ変換することで `#[serde(deny_unknown_fields)]` の自動生成を実現する。

## Technical Context

**Language/Version**: Rust (latest stable)
**Primary Dependencies**: serde, serde_json, chrono, regress
**Build Tool**: cargo-typify (cargo install cargo-typify)
**Schema Bundling**: json-refs (npm), jq
**Storage**: N/A
**Testing**: cargo test
**Target Platform**: Cross-platform (Linux, macOS, Windows)
**Project Type**: Library crate
**Performance Goals**: コンパイル時の型安全性を保証、実行時オーバーヘッドなし
**Constraints**: typify の JSON Schema Draft 2020-12 サポートが部分的（unevaluatedProperties 未対応）
**Scale/Scope**: 10 スキーマファイル（Quote, OHLCV, Trade, OrderBook, Instrument, VolumeInfo, ExpiryInfo, OptionInfo, DerivativeInfo, definitions）

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Schema First | ✅ PASS | JSON Schema から自動生成、手動編集なし |
| II. 軽量コア | ✅ PASS | 生成 struct のみ、外部依存は serde/chrono のみ |
| III. シンプルさ優先 | ✅ PASS | typify のデフォルト動作を活用、最小限のカスタマイズ |
| IV. 言語非依存 | ✅ PASS | 親仕様（002-data-model）の JSON Schema に準拠 |
| V. エコシステム拡張 | ✅ PASS | 生成 struct は derive(Serialize, Deserialize) で拡張可能 |
| Naming Consistency | ✅ PASS | フィールド名は親仕様（ADR準拠）に従う |
| TDD Application | ✅ PASS | コード生成は推奨、デシリアライズテストを実装 |

## Project Structure

### Documentation (this feature)

```text
specs/002-data-model-rust/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (生成 struct の概要)
├── quickstart.md        # Phase 1 output (使用方法)
└── contracts/           # N/A (API契約なし、struct定義のみ)
```

### Source Code (repository root)

```text
rust/
├── Cargo.toml           # パッケージ定義
├── bundled/             # バンドル済みスキーマ（$ref解決済み）
│   ├── definitions.json
│   ├── quote.json
│   ├── ohlcv.json
│   ├── trade.json
│   ├── orderbook.json
│   ├── instrument.json
│   ├── derivative_info.json
│   ├── expiry_info.json
│   ├── option_info.json
│   └── volume_info.json
├── src/
│   ├── lib.rs           # Crate root, re-exports
│   └── types/           # 生成コード配置先
│       ├── mod.rs
│       ├── definitions.rs
│       ├── quote.rs
│       ├── ohlcv.rs
│       ├── trade.rs
│       ├── orderbook.rs
│       ├── instrument.rs
│       ├── derivative_info.rs
│       ├── expiry_info.rs
│       ├── option_info.rs
│       └── volume_info.rs
└── tests/
    └── types_test.rs    # デシリアライズテスト

scripts/
├── bundle_schemas.sh    # スキーマバンドル（jq変換含む）
└── generate_rust.sh     # Rust コード生成
```

**Structure Decision**: 単一の library crate として構成。`rust/` ディレクトリをサブプロジェクトとして配置し、Python コードと共存させる。

## Post-Design Constitution Check

*Re-evaluation after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Schema First | ✅ PASS | data-model.md で定義した全 struct は JSON Schema から生成 |
| II. 軽量コア | ✅ PASS | 依存は serde/chrono/regress のみ、最小限 |
| III. シンプルさ優先 | ✅ PASS | typify のデフォルト生成を活用、カスタム実装なし |
| IV. 言語非依存 | ✅ PASS | 親仕様の JSON Schema をそのまま使用 |
| V. エコシステム拡張 | ✅ PASS | derive マクロにより拡張容易 |
| Naming Consistency | ✅ PASS | フィールド名は ADR 準拠（bid, ask, timestamp, etc.） |
| TDD Application | ✅ PASS | tests/types_test.rs でデシリアライズテスト実装済み |

**Gate Status**: ✅ ALL PASS - 実装フェーズに進行可能

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | - | - |

## Implementation Tasks (High-Level)

> **Note**: 詳細なタスク分解は `/speckit.tasks` コマンドで `tasks.md` に生成する。

### Phase 1: バンドルスクリプト修正

1. `scripts/bundle_schemas.sh` を修正し、`unevaluatedProperties` → `additionalProperties` 変換を追加（FR-R002, FR-R003）
2. バンドル済みスキーマを再生成

### Phase 2: Rust コード再生成

1. `scripts/generate_rust.sh` でコード再生成
2. `cargo check` でコンパイルエラー 0 件を確認（FR-R020）
3. `cargo fmt` でフォーマット（FR-R021）

### Phase 3: テスト拡充

1. 各 struct に対して正常系テスト 3 件以上（SC-R003）
2. 各 struct に対して異常系テスト 2 件以上（SC-R004）
3. ラウンドトリップテスト（SC-R006）
4. `cargo clippy` で警告 0 件（SC-R005）

### Phase 4: ドキュメント更新

1. `docs/code-generation.md` の Rust セクション更新
2. `rust/README.md` 作成（オプション）

## Generated Artifacts

| Artifact | Path | Status |
|----------|------|--------|
| Research | specs/002-data-model-rust/research.md | ✅ Complete |
| Data Model | specs/002-data-model-rust/data-model.md | ✅ Complete |
| Quickstart | specs/002-data-model-rust/quickstart.md | ✅ Complete |
| Contracts | N/A (不要) | N/A |
