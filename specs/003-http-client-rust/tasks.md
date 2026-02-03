# Tasks: HTTP Client Rust Implementation

**Input**: Design documents from `/specs/003-http-client-rust/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/rust-api.md ✅

**Tests**: TDD 必須（Constitution で HTTP クライアントは TDD 対象と明記）

**Organization**: ユーザーストーリーごとにグループ化。各ストーリーは独立して実装・テスト可能。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能（異なるファイル、依存関係なし）
- **[Story]**: ユーザーストーリーラベル（US1, US2, US3, US4, US5, US6）
- ファイルパスを含む

## Path Conventions

```
crates/marketschema-http/
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
```

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: プロジェクト初期化と基本構造

- [ ] T001 Create crates/marketschema-http/ directory structure per plan.md
- [ ] T002 Create Cargo.toml with dependencies (reqwest, tokio, thiserror, serde, serde_json, moka) in crates/marketschema-http/Cargo.toml
- [ ] T003 Create Cargo.toml at repository root as workspace root with members = ["crates/marketschema-http", "rust"]
- [ ] T004 [P] Update rust/Cargo.toml to reference workspace dependencies (if applicable)
- [ ] T005 [P] Create crates/marketschema-http/src/lib.rs with module declarations and public exports
- [ ] T006 Configure clippy and rustfmt in crates/marketschema-http/

**Checkpoint**: `cargo check -p marketschema-http` が通る

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: すべてのユーザーストーリーで必要な共通インフラ

**⚠️ CRITICAL**: US1/US2 は同時に必要（エラー型なしにクライアントを実装できない）

- [ ] T007 Define constants (DEFAULT_TIMEOUT_SECS, DEFAULT_MAX_CONNECTIONS, etc.) in crates/marketschema-http/src/lib.rs
- [ ] T008 Add wiremock to dev-dependencies in crates/marketschema-http/Cargo.toml

**Checkpoint**: Foundation ready - ユーザーストーリー実装開始可能

---

## Phase 3: User Story 1 - 非同期 HTTP リクエストの実行 (Priority: P1) 🎯 MVP

**Goal**: reqwest ベースの非同期 HTTP クライアントで JSON/テキストレスポンスを取得

**Independent Test**: モック API に対して `get_json()` を呼び出し、正しいレスポンスを取得できれば成功

### Tests for User Story 1 (TDD) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T009 [P] [US1] Create test file crates/marketschema-http/tests/client_tests.rs with test module structure
- [ ] T010 [P] [US1] Write failing test for AsyncHttpClientBuilder::new() and build() in tests/client_tests.rs
- [ ] T011 [P] [US1] Write failing test for AsyncHttpClient::get_json() returning serde_json::Value in tests/client_tests.rs
- [ ] T012 [P] [US1] Write failing test for AsyncHttpClient::get_text() returning String in tests/client_tests.rs
- [ ] T013 [P] [US1] Write failing test for AsyncHttpClient::get() returning reqwest::Response in tests/client_tests.rs
- [ ] T014 [US1] Write failing test for query parameters with get_json_with_params() in tests/client_tests.rs
- [ ] T015 [US1] Write failing test for custom headers via builder in tests/client_tests.rs
- [ ] T016 [US1] Write failing test for Arc<AsyncHttpClient> thread-safety (spawn multiple tasks) in tests/client_tests.rs

### Implementation for User Story 1

- [ ] T017 [P] [US1] Create crates/marketschema-http/src/error.rs with HttpError enum stub (Build variant only)
- [ ] T018 [US1] Implement AsyncHttpClientBuilder struct in crates/marketschema-http/src/client.rs
- [ ] T019 [US1] Implement AsyncHttpClientBuilder::new() with default values in src/client.rs
- [ ] T020 [US1] Implement AsyncHttpClientBuilder::timeout(), max_connections(), default_headers() in src/client.rs
- [ ] T021 [US1] Implement AsyncHttpClientBuilder::build() creating reqwest::Client in src/client.rs
- [ ] T022 [US1] Implement AsyncHttpClient struct with inner: reqwest::Client in src/client.rs
- [ ] T023 [US1] Implement AsyncHttpClient::get() method in src/client.rs
- [ ] T024 [US1] Implement AsyncHttpClient::get_with_params() method in src/client.rs
- [ ] T025 [US1] Implement AsyncHttpClient::get_json() method in src/client.rs
- [ ] T026 [US1] Implement AsyncHttpClient::get_json_with_params() method in src/client.rs
- [ ] T027 [US1] Implement AsyncHttpClient::get_text() method in src/client.rs
- [ ] T028 [US1] Implement AsyncHttpClient::get_text_with_params() method in src/client.rs
- [ ] T029 [US1] Verify Send + Sync bounds for AsyncHttpClient in src/client.rs
- [ ] T030 [US1] Export AsyncHttpClient and AsyncHttpClientBuilder from src/lib.rs

**Checkpoint**: `cargo test -p marketschema-http` で US1 テストが全て通る

---

## Phase 4: User Story 2 - Result 型による HTTP エラーの処理 (Priority: P1)

**Goal**: タイムアウト、接続エラー、HTTP ステータスエラーを `Result<T, HttpError>` で処理

**Independent Test**: 各種エラー状況をシミュレートし、適切なエラーが返されることを確認

### Tests for User Story 2 (TDD) ⚠️

- [ ] T031 [P] [US2] Create test file crates/marketschema-http/tests/error_tests.rs
- [ ] T032 [P] [US2] Write failing test for HttpError::Timeout when request times out in tests/error_tests.rs
- [ ] T033 [P] [US2] Write failing test for HttpError::Connection when connection fails in tests/error_tests.rs
- [ ] T034 [P] [US2] Write failing test for HttpError::Status with 404 response in tests/error_tests.rs
- [ ] T035 [P] [US2] Write failing test for HttpError::RateLimit with 429 response in tests/error_tests.rs
- [ ] T036 [US2] Write failing test for HttpError::Parse with invalid JSON in tests/error_tests.rs
- [ ] T037 [US2] Write failing test for std::error::Error::source() returning original reqwest error in tests/error_tests.rs
- [ ] T038 [US2] Write failing test for HttpError::is_retryable() method in tests/error_tests.rs

### Implementation for User Story 2

- [ ] T039 [US2] Implement HttpError enum with all variants (Timeout, Connection, Status, RateLimit, Parse, Build) in src/error.rs
- [ ] T040 [US2] Add thiserror derive and #[error] attributes to HttpError in src/error.rs
- [ ] T041 [US2] Add #[source] attributes for exception chaining in src/error.rs
- [ ] T042 [US2] Implement HttpError::url() method in src/error.rs
- [ ] T043 [US2] Implement HttpError::status_code() method in src/error.rs
- [ ] T044 [US2] Implement HttpError::is_retryable() method in src/error.rs
- [ ] T045 [US2] Implement error conversion from reqwest::Error to HttpError in src/error.rs
- [ ] T046 [US2] Update AsyncHttpClient methods to return proper HttpError variants in src/client.rs
- [ ] T047 [US2] Handle Retry-After header parsing for RateLimit error in src/client.rs
- [ ] T048 [US2] Export HttpError from src/lib.rs

**Checkpoint**: `cargo test -p marketschema-http` で US1 + US2 テストが全て通る

---

## Phase 5: User Story 3 - 指数バックオフによる自動リトライ (Priority: P2)

**Goal**: 一時的なエラーに対して自動リトライを実行

**Independent Test**: wiremock でモックした API が一時的にエラーを返した後に成功する場合、自動リトライが機能することを確認

### Tests for User Story 3 (TDD) ⚠️

- [ ] T049 [P] [US3] Write failing test for RetryConfig::new() with default values in tests/client_tests.rs
- [ ] T050 [P] [US3] Write failing test for RetryConfig builder methods in tests/client_tests.rs
- [ ] T051 [US3] Write failing test for RetryConfig::should_retry() logic in tests/client_tests.rs
- [ ] T052 [US3] Write failing test for RetryConfig::get_delay() exponential backoff in tests/client_tests.rs
- [ ] T053 [US3] Write failing test for automatic retry on 503 then success in tests/client_tests.rs
- [ ] T054 [US3] Write failing test for no retry on 400/401/403/404 in tests/client_tests.rs
- [ ] T055 [US3] Write failing test for max_retries exceeded in tests/client_tests.rs

### Implementation for User Story 3

- [ ] T056 [P] [US3] Create crates/marketschema-http/src/retry.rs with RetryConfig struct
- [ ] T057 [US3] Implement RetryConfig::new() with defaults (max_retries: 3, backoff_factor: 0.5, jitter: 0.1) in src/retry.rs
- [ ] T058 [US3] Implement RetryConfig builder methods (max_retries, backoff_factor, jitter, retry_statuses) in src/retry.rs
- [ ] T059 [US3] Implement RetryConfig::should_retry() method in src/retry.rs
- [ ] T060 [US3] Implement RetryConfig::get_delay() with exponential backoff and jitter in src/retry.rs
- [ ] T061 [US3] Add retry_config field to AsyncHttpClient struct in src/client.rs
- [ ] T062 [US3] Add retry() method to AsyncHttpClientBuilder in src/client.rs
- [ ] T063 [US3] Implement retry loop in AsyncHttpClient request methods in src/client.rs
- [ ] T064 [US3] Export RetryConfig from src/lib.rs

**Checkpoint**: `cargo test -p marketschema-http` で US1 + US2 + US3 テストが全て通る

---

## Phase 6: User Story 4 - トークンバケットによるレート制限 (Priority: P2)

**Goal**: API のレート制限を遵守し、429 エラーを未然に防止

**Independent Test**: レート制限設定で高速リクエストを送信し、適切な間隔で送信されることを確認

### Tests for User Story 4 (TDD) ⚠️

- [ ] T065 [P] [US4] Write failing test for RateLimiter::new() in tests/client_tests.rs
- [ ] T066 [P] [US4] Write failing test for RateLimiter::try_acquire() in tests/client_tests.rs
- [ ] T067 [US4] Write failing test for RateLimiter::acquire() async waiting in tests/client_tests.rs
- [ ] T068 [US4] Write failing test for burst behavior in tests/client_tests.rs
- [ ] T069 [US4] Write failing test for token replenishment over time in tests/client_tests.rs
- [ ] T070 [US4] Write failing test for RateLimiter Send + Sync bounds in tests/client_tests.rs

### Implementation for User Story 4

- [ ] T071 [P] [US4] Create crates/marketschema-http/src/rate_limit.rs with RateLimiter struct
- [ ] T072 [US4] Implement RateLimiter::new() with requests_per_second and burst_size in src/rate_limit.rs
- [ ] T073 [US4] Implement token replenishment logic in src/rate_limit.rs
- [ ] T074 [US4] Implement RateLimiter::try_acquire() method in src/rate_limit.rs
- [ ] T075 [US4] Implement RateLimiter::acquire() async method in src/rate_limit.rs
- [ ] T076 [US4] Verify Send + Sync bounds for RateLimiter in src/rate_limit.rs
- [ ] T077 [US4] Add rate_limiter field to AsyncHttpClient struct in src/client.rs
- [ ] T078 [US4] Add rate_limit() method to AsyncHttpClientBuilder in src/client.rs
- [ ] T079 [US4] Integrate rate limiter into AsyncHttpClient request methods in src/client.rs
- [ ] T080 [US4] Export RateLimiter from src/lib.rs

**Checkpoint**: `cargo test -p marketschema-http` で US1 + US2 + US3 + US4 テストが全て通る

---

## Phase 7: User Story 5 - LRU キャッシュによるレスポンスキャッシュ (Priority: P3)

**Goal**: 同一 URL への繰り返しリクエストに対してキャッシュレスポンスを返す

**Independent Test**: 同一 URL に2回リクエストし、2回目がキャッシュから返されることを確認

### Tests for User Story 5 (TDD) ⚠️

- [ ] T081 [P] [US5] Write failing test for ResponseCache::new() in tests/client_tests.rs
- [ ] T082 [P] [US5] Write failing test for ResponseCache::get() and set() in tests/client_tests.rs
- [ ] T083 [US5] Write failing test for cache TTL expiration in tests/client_tests.rs
- [ ] T084 [US5] Write failing test for cache max_size LRU eviction in tests/client_tests.rs
- [ ] T085 [US5] Write failing test for ResponseCache::delete() and clear() in tests/client_tests.rs
- [ ] T086 [US5] Write failing test for cache integration with AsyncHttpClient in tests/client_tests.rs

### Implementation for User Story 5

- [ ] T087 [P] [US5] Create crates/marketschema-http/src/cache.rs with ResponseCache struct using moka
- [ ] T088 [US5] Implement ResponseCache::new() with max_size and default_ttl in src/cache.rs
- [ ] T089 [US5] Implement ResponseCache::get() method in src/cache.rs
- [ ] T090 [US5] Implement ResponseCache::set() method with optional TTL in src/cache.rs
- [ ] T091 [US5] Implement ResponseCache::delete() method in src/cache.rs
- [ ] T092 [US5] Implement ResponseCache::clear() method in src/cache.rs
- [ ] T093 [US5] Verify Send + Sync bounds for ResponseCache in src/cache.rs
- [ ] T094 [US5] Add cache field to AsyncHttpClient struct in src/client.rs
- [ ] T095 [US5] Add cache() method to AsyncHttpClientBuilder in src/client.rs
- [ ] T096 [US5] Integrate cache lookup/storage into AsyncHttpClient get methods in src/client.rs
- [ ] T097 [US5] Export ResponseCache from src/lib.rs

**Checkpoint**: `cargo test -p marketschema-http` で全 US テストが通る

---

## Phase 8: User Story 6 - BaseAdapter トレイトとの統合 (Priority: P2)

**Goal**: BaseAdapter トレイトを実装した構造体で HTTP クライアントを簡単に利用

**Independent Test**: BaseAdapter トレイトを実装した構造体で `http_client()` にアクセスし、HTTP リクエストを実行できれば成功

### Tests for User Story 6 (TDD) ⚠️

- [ ] T098 [P] [US6] Write failing test for BaseAdapter trait definition in tests/integration_tests.rs
- [ ] T099 [US6] Write failing test for OnceCell lazy initialization in tests/integration_tests.rs
- [ ] T100 [US6] Write failing test for custom AsyncHttpClient injection in tests/integration_tests.rs
- [ ] T101 [US6] Write failing test for Drop behavior in tests/integration_tests.rs

### Implementation for User Story 6

- [ ] T102 [US6] Define BaseAdapter trait with http_client() method in crates/marketschema-http/src/adapter.rs
- [ ] T103 [US6] Create example adapter struct implementing BaseAdapter in tests/integration_tests.rs
- [ ] T104 [US6] Implement OnceCell-based lazy initialization pattern in example
- [ ] T105 [US6] Document BaseAdapter usage pattern in quickstart.md

**Checkpoint**: 全てのユーザーストーリーが独立して機能

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: 複数のユーザーストーリーに影響する改善

- [ ] T106 [P] Run cargo clippy -p marketschema-http and fix all warnings
- [ ] T107 [P] Run cargo fmt --check -p marketschema-http and format code
- [ ] T108 [P] Add doc comments to all public types and methods in src/
- [ ] T109 Run cargo test -p marketschema-http --all-features and verify all tests pass
- [ ] T110 Create integration test with real-world-like scenario in tests/integration_tests.rs
- [ ] T111 Update quickstart.md with verified examples
- [ ] T112 Final verification: confirm T106, T107, T109 results satisfy SC-R006, SC-R007, SC-R008

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies - 即座に開始可能
- **Phase 2 (Foundational)**: Phase 1 完了後
- **Phase 3 (US1) + Phase 4 (US2)**: Phase 2 完了後、並列実行可能だが US1 は US2 のエラー型に依存
- **Phase 5 (US3)**: Phase 3 + Phase 4 完了後
- **Phase 6 (US4)**: Phase 3 + Phase 4 完了後（US3 と並列可能）
- **Phase 7 (US5)**: Phase 3 + Phase 4 完了後（US3/US4 と並列可能）
- **Phase 8 (US6)**: Phase 3 + Phase 4 完了後（US3/US4/US5 と並列可能）
- **Phase 9 (Polish)**: 全 US 完了後

### User Story Dependencies

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundational)
    │
    ├─────────────────────────────────────┐
    ▼                                     │
Phase 3 (US1) ◄───────────────────────────┤
    │                                     │
    ▼                                     │
Phase 4 (US2) ◄───────────────────────────┘
    │
    ├────────────┬────────────┬────────────┐
    ▼            ▼            ▼            ▼
Phase 5      Phase 6      Phase 7      Phase 8
(US3)        (US4)        (US5)        (US6)
    │            │            │            │
    └────────────┴────────────┴────────────┘
                        │
                        ▼
                Phase 9 (Polish)
```

### Parallel Opportunities

**Phase 1 内**:
- T003, T004, T005, T006 は並列実行可能

**Phase 3 (US1) テスト**:
- T009, T010, T011, T012, T013 は並列実行可能

**Phase 4 (US2) テスト**:
- T031, T032, T033, T034, T035 は並列実行可能

**Phase 5-8 (US3-US6)**:
- 各フェーズは独立しており、並列実行可能

**Phase 9 内**:
- T106, T107, T108 は並列実行可能

---

## Parallel Example: User Story 1

```bash
# Launch all US1 tests together:
Task: "Write failing test for AsyncHttpClientBuilder::new() and build() in tests/client_tests.rs"
Task: "Write failing test for AsyncHttpClient::get_json() returning serde_json::Value in tests/client_tests.rs"
Task: "Write failing test for AsyncHttpClient::get_text() returning String in tests/client_tests.rs"
Task: "Write failing test for AsyncHttpClient::get() returning reqwest::Response in tests/client_tests.rs"

# After tests exist, launch parallel model implementations:
Task: "Create crates/marketschema-http/src/error.rs with HttpError enum stub"
# (error.rs は client.rs が依存するため先に作成)
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (Basic HTTP)
4. Complete Phase 4: User Story 2 (Error Handling)
5. **STOP and VALIDATE**: `cargo test -p marketschema-http` で全テスト通過
6. MVP として使用可能（リトライ、レート制限、キャッシュなしでも基本機能は動作）

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add US1 + US2 → Test → MVP!（基本 HTTP クライアント）
3. Add US3 → Test → リトライ機能追加
4. Add US4 → Test → レート制限機能追加
5. Add US5 → Test → キャッシュ機能追加
6. Add US6 → Test → BaseAdapter 統合
7. Each story adds value without breaking previous stories

### Quality Gates

各フェーズ完了後に確認:

```bash
cargo check -p marketschema-http
cargo clippy -p marketschema-http
cargo fmt --check -p marketschema-http
cargo test -p marketschema-http
```

---

## Notes

- [P] tasks = 異なるファイル、依存関係なし
- [Story] label = 特定のユーザーストーリーにマッピング
- 各ユーザーストーリーは独立して完了・テスト可能
- テストが失敗することを確認してから実装
- 各タスクまたは論理グループ後にコミット
- 任意のチェックポイントで停止してストーリーを独立検証可能
- 避けるべき: 曖昧なタスク、同一ファイルの競合、ストーリー間の独立性を壊す依存関係

---

## Summary

| Metric | Value |
|--------|-------|
| Total Tasks | 112 |
| Phase 1 (Setup) | 6 tasks |
| Phase 2 (Foundational) | 2 tasks |
| Phase 3 (US1 - P1 MVP) | 22 tasks (8 tests + 14 impl) |
| Phase 4 (US2 - P1) | 18 tasks (8 tests + 10 impl) |
| Phase 5 (US3 - P2) | 16 tasks (7 tests + 9 impl) |
| Phase 6 (US4 - P2) | 16 tasks (6 tests + 10 impl) |
| Phase 7 (US5 - P3) | 17 tasks (6 tests + 11 impl) |
| Phase 8 (US6 - P2) | 8 tasks (4 tests + 4 impl) |
| Phase 9 (Polish) | 7 tasks |
| Parallel Opportunities | ~40% of tasks can run in parallel |
| MVP Scope | US1 + US2 (Phase 1-4, 48 tasks) |
