# Implementation Plan: HTTP Client Rust Implementation

**Branch**: `003-http-client-rust` | **Date**: 2026-02-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-http-client-rust/spec.md`

## Summary

Rust 言語での marketschema HTTP クライアントレイヤーを実装する。親仕様 [003-http-client](../003-http-client/spec.md) で定義された機能要件を、reqwest + tokio ベースの非同期実装として提供する。`Send + Sync` 境界、`thiserror` によるエラー処理、ビルダーパターンなど Rust の慣用的なパターンを採用する。

## Technical Context

**Language/Version**: Rust 1.71.1+ (MSRV: 1.71.1 - moka 0.12 の要求による)
**Primary Dependencies**: reqwest (HTTP), tokio (async runtime), thiserror (errors), serde/serde_json (serialization)
**Storage**: N/A（インメモリキャッシュのみ）
**Testing**: cargo test + wiremock (HTTP mocking)
**Target Platform**: Linux/macOS/Windows (tokio 対応プラットフォーム)
**Project Type**: Single library crate (`marketschema-http`)
**Performance Goals**: reqwest/tokio のデフォルト性能に依存。明示的な最適化は不要
**Constraints**: `Send + Sync` 境界を満たすこと、非同期 API のみ提供
**Scale/Scope**: ライブラリクレート、約10ファイル、6つの主要型

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Schema First | ✅ Pass | HTTP クライアントはスキーマ定義ではなく、インフラストラクチャ層。スキーマとの整合性は不要 |
| II. 軽量コア | ✅ Pass | Constitution v0.5.0 で「共通 HTTP クライアント」はコアスコープに明記。オプショナル依存として提供 |
| III. シンプルさ優先 | ✅ Pass | 80% のユースケースに最適化。`get_json()`, `get_text()` で大半をカバー |
| IV. 言語非依存 | ✅ Pass | 親仕様で言語非依存の契約を定義済み。本 spec は Rust 固有の実装詳細 |
| V. エコシステム拡張 | ✅ Pass | HTTP クライアントはコアインフラ。個別データソースは対象外 |
| 禁止事項: 命名の揺れ | ✅ Pass | 業界標準名を採用（`timeout`, `retry`, `rate_limit`） |
| 禁止事項: 暗黙的フォールバック | ✅ Pass | すべてのエラーは `Result<T, HttpError>` として明示的に返す |
| 禁止事項: ハードコード | ✅ Pass | デフォルト値は定数として定義（`DEFAULT_TIMEOUT`, `DEFAULT_MAX_RETRIES` 等） |
| Quality: HTTP Client | ✅ Pass | 非同期必須、タイムアウト必須、リトライはべき等操作のみ |

**Gate Result**: ✅ All gates passed

## Project Structure

### Documentation (this feature)

```text
specs/003-http-client-rust/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── rust-api.md      # Rust API contract (existing)
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
crates/
└── marketschema-http/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs           # Public exports
    │   ├── client.rs        # AsyncHttpClient, AsyncHttpClientBuilder
    │   ├── error.rs         # HttpError enum
    │   ├── retry.rs         # RetryConfig
    │   ├── rate_limit.rs    # RateLimiter
    │   └── cache.rs         # ResponseCache
    └── tests/
        ├── client_tests.rs
        ├── error_tests.rs
        └── integration_tests.rs

rust/
├── Cargo.toml           # Workspace root (update to include new crate)
└── src/
    └── lib.rs           # Existing marketschema crate
```

**Structure Decision**: `crates/marketschema-http/` に独立したクレートとして実装。既存の `rust/` ディレクトリはワークスペースルートとして機能させ、新しいクレートを workspace members に追加する。これにより、将来的な言語固有のクレート追加も容易になる。

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | - | - |

追加の複雑さは不要。Constitution に準拠した設計で実装可能。

## Phase Completion Status

### Phase 0: Research ✅

**Completed**: 2026-02-03

**Outputs**:
- [research.md](./research.md) - 技術調査結果

**Key Decisions**:
1. **HTTP クライアント**: reqwest + tokio（業界標準、非同期対応）
2. **レート制限**: 手動実装（`Mutex` + トークンバケット）で外部依存を削減
3. **キャッシュ**: moka クレート（並行安全、TinyLFU、非同期対応）
4. **テスト**: wiremock（非同期対応、テスト分離）
5. **プロジェクト構造**: `crates/` サブディレクトリによるワークスペース構成

### Phase 1: Design ✅

**Completed**: 2026-02-03

**Outputs**:
- [data-model.md](./data-model.md) - データモデル定義
- [quickstart.md](./quickstart.md) - クイックスタートガイド
- [contracts/rust-api.md](./contracts/rust-api.md) - API コントラクト（既存）

**Constitution Re-check (Post-Design)**:

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Schema First | ✅ Pass | 変更なし |
| II. 軽量コア | ✅ Pass | 変更なし |
| III. シンプルさ優先 | ✅ Pass | 6つの主要型のみ。過度な抽象化なし |
| IV. 言語非依存 | ✅ Pass | 親仕様の契約に準拠 |
| V. エコシステム拡張 | ✅ Pass | 変更なし |
| 禁止事項: 命名の揺れ | ✅ Pass | data-model.md で統一命名を確認 |
| 禁止事項: 暗黙的フォールバック | ✅ Pass | `Result<T, HttpError>` で明示的エラー |
| 禁止事項: ハードコード | ✅ Pass | 定数として定義（data-model.md 参照） |
| Quality: HTTP Client | ✅ Pass | 設計が要件を満たす |

**Gate Result (Post-Design)**: ✅ All gates passed

### Phase 2: Tasks

**Status**: Pending（`/speckit.tasks` で生成）

---

## Next Steps

1. `/speckit.tasks` を実行して tasks.md を生成
2. `/speckit.implement` を実行して実装を開始
