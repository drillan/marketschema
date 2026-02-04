# Tasks: 統一マーケットデータスキーマ

**Input**: Design documents from `/specs/002-data-model/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Project structure**: `python/src/marketschema/` for source code, `python/tests/` for tests at repository root
- **Schemas**: `schemas/` for JSON Schema files
- **Models**: `python/src/marketschema/models/` for generated pydantic models
- **Adapters**: `python/src/marketschema/adapters/` for adapter infrastructure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project structure per implementation plan (python/src/marketschema/, tests/, docs/)
- [x] T002 Initialize Python project with uv and pyproject.toml (pydantic v2, mypy dependencies)
- [x] T003 [P] Configure ruff for linting and formatting in pyproject.toml
- [x] T004 [P] Configure mypy for type checking in pyproject.toml
- [x] T005 [P] Install ajv-cli for JSON Schema validation via npm
- [x] T005b [P] Install json-refs for schema bundling via npm (required for Rust code generation)
- [x] T006 Create python/src/marketschema/__init__.py with package exports
- [x] T007 Create python/src/marketschema/py.typed marker file for PEP 561

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 Create schemas/ directory structure
- [x] T009 Verify JSON Schema contracts in specs/002-data-model/contracts/ are complete (10 files expected)
- [x] T010 Create python/src/marketschema/exceptions.py with base exception classes (MarketSchemaError, ValidationError, TransformError, AdapterError)
- [x] T011 Create tests/ directory structure (tests/unit/, tests/integration/, tests/contract/)
- [x] T012 Create tests/conftest.py with shared pytest fixtures

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - スキーマを使ったデータ検証 (Priority: P1) 🎯 MVP

**Goal**: JSON Schema を使用して Quote、OHLCV、Trade、OrderBook などのデータ構造を検証できる

**Independent Test**: JSON Schema ファイルを ajv-cli でバリデーションし、サンプルデータの検証が成功する

### Implementation for User Story 1

- [x] T013 [P] [US1] Copy and verify definitions.json from contracts/ to schemas/definitions.json
- [x] T014 [P] [US1] Copy and verify quote.json from contracts/ to schemas/quote.json
- [x] T015 [P] [US1] Copy and verify ohlcv.json from contracts/ to schemas/ohlcv.json
- [x] T016 [P] [US1] Copy and verify trade.json from contracts/ to schemas/trade.json
- [x] T017 [P] [US1] Copy and verify orderbook.json from contracts/ to schemas/orderbook.json
- [x] T018 [P] [US1] Copy and verify instrument.json from contracts/ to schemas/instrument.json
- [x] T019 [P] [US1] Copy and verify derivative_info.json from contracts/ to schemas/derivative_info.json
- [x] T020 [P] [US1] Copy and verify expiry_info.json from contracts/ to schemas/expiry_info.json
- [x] T021 [P] [US1] Copy and verify option_info.json from contracts/ to schemas/option_info.json
- [x] T022 [P] [US1] Copy and verify volume_info.json from contracts/ to schemas/volume_info.json
- [x] T023 [US1] Create tests/fixtures/ directory with sample JSON data files (valid and invalid cases)
- [x] T024 [US1] Create tests/contract/test_schema_compliance.py to verify all schemas are Draft 2020-12 compliant
- [x] T025 [US1] Create tests/unit/test_schemas.py with ajv-cli validation tests for each schema

**Checkpoint**: User Story 1 complete - all schemas validated with ajv-cli, sample data passes/fails as expected

---

## Phase 4: User Story 2 - Python pydantic モデルの生成 (Priority: P2)

**Goal**: JSON Schema から pydantic v2 モデルを自動生成し、型安全かつバリデーション付きのコードを書ける

**Independent Test**: datamodel-codegen で Python コードを生成し、mypy で型チェックが成功する

### Implementation for User Story 2

- [x] T026 [US2] Configure datamodel-codegen options in pyproject.toml [tool.datamodel-codegen]
- [x] T027 [US2] Create scripts/generate_models.sh to run datamodel-codegen with correct options
- [x] T028 [US2] Run datamodel-codegen to generate pydantic models in python/src/marketschema/models/
- [x] T029 [US2] Create python/src/marketschema/models/__init__.py with re-exports of all models (Quote, OHLCV, Trade, OrderBook, Instrument, etc.)
- [x] T030 [US2] Verify generated models have ConfigDict(extra='forbid') for unevaluatedProperties: false
- [x] T031 [US2] Run mypy on python/src/marketschema/models/ to verify type correctness
- [x] T032 [US2] Create tests/unit/test_models.py with pydantic model instantiation and validation tests

**Checkpoint**: User Story 2 complete - pydantic models generated, mypy passes, model instantiation works

---

## Phase 5: User Story 3 - Rust 構造体の生成 (Priority: P2)

**Goal**: JSON Schema から Rust struct を自動生成し、高パフォーマンスなアプリケーションで使用できる

**Independent Test**: typify で Rust コードを生成し、cargo check でコンパイルが成功する

### Implementation for User Story 3

- [x] T033 [US3] Create rust/ directory at repository root for Rust crate
- [x] T034 [US3] Initialize Rust crate with Cargo.toml (serde, serde_json dependencies) in rust/
- [x] T035 [US3] Create scripts/bundle_schemas.sh to resolve $ref with json-refs
- [x] T036 [US3] Create scripts/generate_rust.sh to run typify on bundled schemas
- [x] T037 [US3] Run schema bundling for all schemas to rust/bundled/
- [x] T038 [US3] Run typify to generate Rust structs in rust/src/types/
- [x] T039 [US3] Create rust/src/lib.rs with module declarations and re-exports
- [x] T040 [US3] Run cargo check to verify Rust code compiles
- [x] T041 [US3] Create rust/tests/types_test.rs with basic deserialization tests

**Checkpoint**: User Story 3 complete - Rust structs generated, cargo check passes

---

## Phase 6: User Story 4 - アダプター基盤の提供 (Priority: P3)

**Goal**: BaseAdapter クラスを継承して、新しいデータソース用のアダプターを簡単に実装できる

**Independent Test**: BaseAdapter を継承したサンプルアダプターを実装し、マッピング定義でデータ変換できることを確認

### Implementation for User Story 4

- [x] T042 [US4] Create python/src/marketschema/adapters/__init__.py with module exports
- [x] T043 [US4] Create python/src/marketschema/adapters/mapping.py with ModelMapping dataclass (target_field, source_field, transform, default)
- [x] T044 [US4] Create python/src/marketschema/adapters/transforms.py with common transform functions (to_float, to_int, iso_timestamp, unix_timestamp_ms, unix_timestamp_sec, side_from_string, jst_to_utc)
- [x] T045 [US4] Create python/src/marketschema/adapters/base.py with BaseAdapter abstract class (source_name, get_*_mapping methods, _apply_mapping, _get_nested_value)
- [x] T046 [US4] Update python/src/marketschema/__init__.py to export adapter classes
- [x] T047 [US4] Create tests/unit/test_transforms.py with tests for all transform functions
- [x] T048 [US4] Create tests/integration/test_adapter_base.py with sample adapter implementation test

**Checkpoint**: User Story 4 complete - BaseAdapter usable, sample adapter transforms data correctly

---

## Phase 7: User Story 5 - アダプターレジストリでの管理 (Priority: P3)

**Goal**: 複数のアダプターをレジストリに登録し、データソース名で取得できる

**Independent Test**: 複数のアダプターを登録し、source_name で正しいアダプターを取得できることを確認

### Implementation for User Story 5

- [x] T049 [US5] Create python/src/marketschema/adapters/registry.py with AdapterRegistry class (singleton pattern, register decorator, get method, list_adapters method)
- [x] T050 [US5] Update python/src/marketschema/adapters/__init__.py to export registry and register decorator
- [x] T051 [US5] Update python/src/marketschema/__init__.py to export AdapterRegistry
- [x] T052 [US5] Create tests/unit/test_registry.py with registration and retrieval tests (including KeyError for unknown source)

**Checkpoint**: User Story 5 complete - AdapterRegistry functional, @register decorator works

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T053 [P] Create Makefile with common commands (generate-models, generate-rust, validate-schemas, test, lint)
- [x] T054 [P] Update README.md with installation and usage instructions
- [x] T054b [P] Add code generation commands section to README.md (Python: datamodel-codegen, Rust: typify) for FR-016/FR-017
- [x] T054c [P] Create docs/code-generation.md with detailed options and examples
- [ ] T055 [P] Create docs/adr/field-names/instrument.md for undocumented fields (asset_class, currency) [Deferred - no undocumented fields]
- [x] T056 Run quickstart.md validation (verify all code examples work)
- [x] T057 Run full test suite (pytest, mypy, ruff, ajv)
- [x] T058 Verify SC-001 through SC-007 success criteria are met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - US1 (Schemas): No dependencies on other stories
  - US2 (Python): Depends on US1 (needs schemas)
  - US3 (Rust): Depends on US1 (needs schemas)
  - US4 (Adapter Base): Depends on US2 (needs pydantic models)
  - US5 (Registry): Depends on US4 (needs BaseAdapter)
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories - **MVP**
- **User Story 2 (P2)**: Depends on US1 completion (needs schemas to generate from)
- **User Story 3 (P2)**: Depends on US1 completion (needs schemas to generate from) - Can run parallel with US2
- **User Story 4 (P3)**: Depends on US2 completion (needs pydantic models for type hints)
- **User Story 5 (P3)**: Depends on US4 completion (needs BaseAdapter class)

### Within Each User Story

- Schema tasks marked [P] can run in parallel (different files)
- Models/services depend on schemas being complete
- Tests depend on implementation being complete
- Story complete before moving to next priority

### Parallel Opportunities

- **Phase 1 Setup**: T003, T004, T005 can run in parallel
- **Phase 3 US1**: All schema files (T013-T022) can be created in parallel
- **Phase 4+5**: US2 and US3 can run in parallel after US1 completes
- **Phase 8 Polish**: T053, T054, T055 can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all schema creation tasks together:
Task: "Create definitions.json in schemas/definitions.json"
Task: "Create quote.json in schemas/quote.json"
Task: "Create ohlcv.json in schemas/ohlcv.json"
Task: "Create trade.json in schemas/trade.json"
Task: "Create orderbook.json in schemas/orderbook.json"
Task: "Create instrument.json in schemas/instrument.json"
Task: "Create derivative_info.json in schemas/derivative_info.json"
Task: "Create expiry_info.json in schemas/expiry_info.json"
Task: "Create option_info.json in schemas/option_info.json"
Task: "Create volume_info.json in schemas/volume_info.json"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (Schema Validation)
4. **STOP and VALIDATE**: Test all schemas with ajv-cli
5. Deploy/demo if ready - JSON Schemas can be used standalone

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test schemas → Deploy/Demo (MVP - schemas only!)
3. Add User Story 2 → Test Python models → Deploy/Demo (Python support!)
4. Add User Story 3 → Test Rust structs → Deploy/Demo (Rust support!)
5. Add User Story 4 → Test adapters → Deploy/Demo (Adapter infrastructure!)
6. Add User Story 5 → Test registry → Deploy/Demo (Full feature!)

### Recommended Execution Order (Single Developer)

1. Phase 1: Setup (T001-T007)
2. Phase 2: Foundational (T008-T012)
3. Phase 3: US1 Schemas (T013-T025) - **MVP milestone**
4. Phase 4: US2 Python (T026-T032) - Python users can start using
5. Phase 5: US3 Rust (T033-T041) - Rust users can start using
6. Phase 6: US4 Adapter Base (T042-T048)
7. Phase 7: US5 Registry (T049-T052)
8. Phase 8: Polish (T053-T058)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- JSON Schema contracts already exist in specs/002-data-model/contracts/ - copy to src/
- datamodel-codegen generates pydantic v2 models from JSON Schema
- typify generates Rust structs but requires bundled schemas (no external $ref)
- All timestamps in UTC, ISO 8601 format
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
