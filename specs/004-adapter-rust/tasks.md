# Tasks: Adapter Interface Rust Implementation

**Input**: Design documents from `/specs/004-adapter-rust/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Included per Success Criteria SC-R001〜SC-R008 in spec.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Rust crate**: `crates/marketschema-adapters/src/` for source
- **Tests**: `crates/marketschema-adapters/src/` (inline tests) + `crates/marketschema-adapters/tests/` for integration tests

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and crate structure

- [ ] T001 Create crate directory structure per plan.md in crates/marketschema-adapters/
- [ ] T002 Initialize Cargo.toml with dependencies (async-trait, chrono, once_cell, serde, serde_json, thiserror) in crates/marketschema-adapters/Cargo.toml
- [ ] T003 [P] Add crate to workspace in Cargo.toml at repository root
- [ ] T004 [P] Create lib.rs with module declarations in crates/marketschema-adapters/src/lib.rs

---

## Phase 2: Foundational (Error Types)

**Purpose**: Core error types that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Implement TransformError struct with new() constructor in crates/marketschema-adapters/src/error.rs
- [ ] T006 Implement MappingError struct with new() constructor in crates/marketschema-adapters/src/error.rs
- [ ] T007 Implement AdapterError enum with General, DuplicateRegistration, Mapping, Transform variants in crates/marketschema-adapters/src/error.rs
- [ ] T008 Add #[from] conversions for MappingError and TransformError in crates/marketschema-adapters/src/error.rs
- [ ] T009 Export error types from lib.rs in crates/marketschema-adapters/src/lib.rs

**Checkpoint**: Error types ready - User Story 3 (Transforms) can now begin

---

## Phase 3: User Story 3 - Transforms 静的メソッドによる型変換 (Priority: P1) 🎯 MVP

**Goal**: Rust アダプター開発者が Transforms の関連関数を使用して型変換を行える

**Independent Test**: 各変換関数に対して正常系・異常系の入力でテストを実行し、すべて通過する

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation (TDD: Red → Green → Refactor)**

- [ ] T010 [P] [US3] Write unit tests for to_float() in crates/marketschema-adapters/src/transforms.rs (mod tests)
- [ ] T011 [P] [US3] Write unit tests for to_int() in crates/marketschema-adapters/src/transforms.rs (mod tests)
- [ ] T012 [P] [US3] Write unit tests for timestamp functions (iso_timestamp, unix_timestamp_ms, unix_timestamp_sec) in crates/marketschema-adapters/src/transforms.rs
- [ ] T013 [P] [US3] Write unit tests for jst_to_utc() in crates/marketschema-adapters/src/transforms.rs
- [ ] T014 [P] [US3] Write unit tests for side_from_string() in crates/marketschema-adapters/src/transforms.rs
- [ ] T015 [P] [US3] Write unit tests for uppercase() and lowercase() in crates/marketschema-adapters/src/transforms.rs

### Implementation for User Story 3

- [ ] T016 [US3] Define TransformFn type alias and MS_PER_SECOND, JST_UTC_OFFSET_HOURS constants in crates/marketschema-adapters/src/transforms.rs
- [ ] T017 [P] [US3] Implement to_float() and to_float_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T018 [P] [US3] Implement to_int() and to_int_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T019 [P] [US3] Implement iso_timestamp() and iso_timestamp_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T020 [P] [US3] Implement unix_timestamp_ms() and unix_timestamp_ms_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T021 [P] [US3] Implement unix_timestamp_sec() and unix_timestamp_sec_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T022 [P] [US3] Implement jst_to_utc() and jst_to_utc_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T023 [P] [US3] Implement side_from_string() and side_from_string_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T024 [P] [US3] Implement uppercase(), uppercase_fn(), lowercase(), lowercase_fn() in crates/marketschema-adapters/src/transforms.rs
- [ ] T025 [US3] Export Transforms and TransformFn from lib.rs in crates/marketschema-adapters/src/lib.rs
- [ ] T026 [US3] Run cargo test and verify all US3 tests pass

**Checkpoint**: Transforms module complete. cargo test should pass for all transform functions.

---

## Phase 4: User Story 2 - ModelMapping struct による型安全なフィールドマッピング (Priority: P1)

**Goal**: Rust アダプター開発者が ModelMapping を使用してフィールドマッピングを定義・適用できる

**Independent Test**: ModelMapping インスタンスを作成し、apply() メソッドで値を取得・変換できる

**Dependencies**: Phase 3 (Transforms) must be complete for with_transform() to work

### Tests for User Story 2

- [ ] T027 [P] [US2] Write unit tests for ModelMapping::new() and builder methods in crates/marketschema-adapters/src/mapping.rs
- [ ] T028 [P] [US2] Write unit tests for dot notation path access in crates/marketschema-adapters/src/mapping.rs
- [ ] T029 [P] [US2] Write unit tests for apply() with transform in crates/marketschema-adapters/src/mapping.rs
- [ ] T030 [P] [US2] Write unit tests for required/optional and default value handling in crates/marketschema-adapters/src/mapping.rs

### Implementation for User Story 2

- [ ] T031 [US2] Implement ModelMapping struct with fields (target_field, source_field, transform, default, required) in crates/marketschema-adapters/src/mapping.rs
- [ ] T032 [US2] Implement ModelMapping::new() constructor with required=true default in crates/marketschema-adapters/src/mapping.rs
- [ ] T033 [US2] Implement builder methods with_transform(), with_default(), optional() in crates/marketschema-adapters/src/mapping.rs
- [ ] T034 [US2] Implement target_field() and source_field() getter methods in crates/marketschema-adapters/src/mapping.rs
- [ ] T035 [US2] Implement dot_to_pointer() helper function for JSON Pointer conversion in crates/marketschema-adapters/src/mapping.rs
- [ ] T036 [US2] Implement apply() method with dot notation, default, required, transform logic in crates/marketschema-adapters/src/mapping.rs
- [ ] T037 [US2] Derive Clone for ModelMapping in crates/marketschema-adapters/src/mapping.rs
- [ ] T038 [US2] Export ModelMapping from lib.rs in crates/marketschema-adapters/src/lib.rs
- [ ] T039 [US2] Run cargo test and verify all US2 tests pass

**Checkpoint**: ModelMapping module complete. apply() with transforms should work.

---

## Phase 5: User Story 1 - BaseAdapter trait による非同期データ取得と変換 (Priority: P1)

**Goal**: Rust アダプター開発者が BaseAdapter trait を実装して新しいデータソース用のアダプターを作成できる

**Independent Test**: BaseAdapter trait を実装したサンプルアダプターを作成し、マッピングメソッドでデータ変換ができる

**Dependencies**: Phase 4 (ModelMapping) must be complete for mapping methods to return Vec<ModelMapping>

### Tests for User Story 1

- [ ] T040 [P] [US1] Write unit tests for sample adapter source_name() in crates/marketschema-adapters/src/adapter.rs
- [ ] T041 [P] [US1] Write unit tests for sample adapter get_quote_mapping() in crates/marketschema-adapters/src/adapter.rs
- [ ] T042 [P] [US1] Write integration test with sample adapter applying mappings in crates/marketschema-adapters/tests/adapter_integration.rs

### Implementation for User Story 1

- [ ] T043 [US1] Define BaseAdapter trait with source_name() required method in crates/marketschema-adapters/src/adapter.rs
- [ ] T044 [US1] Add default implementations for get_quote_mapping(), get_ohlcv_mapping(), get_trade_mapping(), get_orderbook_mapping(), get_instrument_mapping() in crates/marketschema-adapters/src/adapter.rs
- [ ] T045 [US1] Add Send + Sync bounds to BaseAdapter trait in crates/marketschema-adapters/src/adapter.rs
- [ ] T046 [US1] Create sample adapter struct (SampleAdapter) implementing BaseAdapter for tests in crates/marketschema-adapters/src/adapter.rs
- [ ] T047 [US1] Export BaseAdapter trait from lib.rs in crates/marketschema-adapters/src/lib.rs
- [ ] T048 [US1] Run cargo test and verify all US1 tests pass

**Checkpoint**: BaseAdapter trait complete. Sample adapter should work with ModelMapping.

---

## Phase 6: User Story 4 - AdapterRegistry によるスレッドセーフな動的管理 (Priority: P2)

**Goal**: Rust アダプター開発者が複数のアダプターを AdapterRegistry に登録し、source_name で動的に取得できる

**Independent Test**: 複数のアダプターを登録し、source_name で正しいアダプターを取得できる。スレッドセーフ。

**Dependencies**: Phase 5 (BaseAdapter) must be complete for registry to store adapters

### Tests for User Story 4

- [ ] T049 [P] [US4] Write unit tests for register() and get() in crates/marketschema-adapters/src/registry.rs
- [ ] T050 [P] [US4] Write unit tests for list_adapters() and is_registered() in crates/marketschema-adapters/src/registry.rs
- [ ] T051 [P] [US4] Write unit tests for duplicate registration error in crates/marketschema-adapters/src/registry.rs
- [ ] T052 [P] [US4] Write unit tests for clear() in crates/marketschema-adapters/src/registry.rs
- [ ] T053 [US4] Write multi-threaded test for thread safety in crates/marketschema-adapters/tests/registry_thread_safety.rs

### Implementation for User Story 4

- [ ] T054 [US4] Define AdapterFactory type alias in crates/marketschema-adapters/src/registry.rs
- [ ] T055 [US4] Implement global REGISTRY with Lazy<RwLock<HashMap>> in crates/marketschema-adapters/src/registry.rs
- [ ] T056 [US4] Implement AdapterRegistry::register() with duplicate check in crates/marketschema-adapters/src/registry.rs
- [ ] T057 [US4] Implement AdapterRegistry::get() in crates/marketschema-adapters/src/registry.rs
- [ ] T058 [US4] Implement AdapterRegistry::list_adapters() in crates/marketschema-adapters/src/registry.rs
- [ ] T059 [US4] Implement AdapterRegistry::is_registered() in crates/marketschema-adapters/src/registry.rs
- [ ] T060 [US4] Implement AdapterRegistry::clear() for test isolation in crates/marketschema-adapters/src/registry.rs
- [ ] T061 [US4] Export AdapterRegistry and AdapterFactory from lib.rs in crates/marketschema-adapters/src/lib.rs
- [ ] T062 [US4] Run cargo test and verify all US4 tests pass including thread safety

**Checkpoint**: AdapterRegistry module complete. Thread-safe adapter management works.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Quality assurance and final validation

- [ ] T063 [P] Run cargo clippy --all-targets and fix all warnings in crates/marketschema-adapters/
- [ ] T064 [P] Run cargo fmt --check and fix formatting in crates/marketschema-adapters/
- [ ] T065 Run cargo test --all to verify all tests pass
- [ ] T066 Validate quickstart.md examples compile and run correctly
- [ ] T067 Update lib.rs documentation with crate-level doc comments in crates/marketschema-adapters/src/lib.rs

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup
    ↓
Phase 2: Foundational (Error Types)
    ↓
Phase 3: User Story 3 (Transforms) ← 最初に実装（他のUSが依存）
    ↓
Phase 4: User Story 2 (ModelMapping) ← Transforms に依存
    ↓
Phase 5: User Story 1 (BaseAdapter) ← ModelMapping に依存
    ↓
Phase 6: User Story 4 (AdapterRegistry) ← BaseAdapter に依存
    ↓
Phase 7: Polish
```

### User Story Dependencies

| User Story | Depends On | Reason |
|------------|------------|--------|
| US3 (Transforms) | Phase 2 | Error types only |
| US2 (ModelMapping) | US3 | with_transform() needs TransformFn |
| US1 (BaseAdapter) | US2 | Mapping methods return Vec<ModelMapping> |
| US4 (AdapterRegistry) | US1 | Registry stores Box<dyn BaseAdapter> |

### Within Each User Story

1. Tests MUST be written and FAIL before implementation (TDD)
2. Type definitions before methods
3. Core implementation before helpers
4. Export to lib.rs after implementation complete

### Parallel Opportunities

- T003, T004 can run in parallel
- T010-T015 (US3 tests) can run in parallel
- T017-T024 (US3 implementation) can run in parallel
- T027-T030 (US2 tests) can run in parallel
- T040-T042 (US1 tests) can run in parallel
- T049-T052 (US4 tests) can run in parallel
- T063, T064 can run in parallel

---

## Parallel Example: User Story 3 (Transforms)

```bash
# Launch all tests for User Story 3 together (Red phase):
Task: "Write unit tests for to_float() in crates/marketschema-adapters/src/transforms.rs"
Task: "Write unit tests for to_int() in crates/marketschema-adapters/src/transforms.rs"
Task: "Write unit tests for timestamp functions in crates/marketschema-adapters/src/transforms.rs"
Task: "Write unit tests for jst_to_utc() in crates/marketschema-adapters/src/transforms.rs"
Task: "Write unit tests for side_from_string() in crates/marketschema-adapters/src/transforms.rs"
Task: "Write unit tests for uppercase/lowercase in crates/marketschema-adapters/src/transforms.rs"

# Then implement in parallel (Green phase):
Task: "Implement to_float() and to_float_fn() in crates/marketschema-adapters/src/transforms.rs"
Task: "Implement to_int() and to_int_fn() in crates/marketschema-adapters/src/transforms.rs"
# ... etc
```

---

## Implementation Strategy

### MVP First (User Story 3 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (Error Types)
3. Complete Phase 3: User Story 3 (Transforms)
4. **STOP and VALIDATE**: Run `cargo test` - all transform tests pass
5. Deliverable: Transforms module usable independently

### Incremental Delivery

1. Setup + Foundational + US3 (Transforms) → MVP
2. Add US2 (ModelMapping) → Test independently → ModelMapping works with Transforms
3. Add US1 (BaseAdapter) → Test independently → Full adapter framework without registry
4. Add US4 (AdapterRegistry) → Test independently → Complete feature

### Sequential Execution (Single Developer)

```
T001 → T002 → T003,T004 → T005 → T006 → T007 → T008 → T009
→ T010-T015 (parallel) → T016 → T017-T024 (parallel) → T025 → T026
→ T027-T030 (parallel) → T031 → T032 → T033 → T034 → T035 → T036 → T037 → T038 → T039
→ T040-T042 (parallel) → T043 → T044 → T045 → T046 → T047 → T048
→ T049-T053 (parallel) → T054 → T055 → T056 → T057 → T058 → T059 → T060 → T061 → T062
→ T063,T064 (parallel) → T065 → T066 → T067
```

---

## Notes

- [P] tasks = different files or independent functions, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD Red phase)
- Run cargo test after each story completion to verify
- Run cargo clippy periodically to catch issues early
