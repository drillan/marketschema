# Tasks: Rust Data Model Implementation

**Input**: Design documents from `/specs/002-data-model-rust/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, quickstart.md ✅

**Tests**: Required (spec.md の Success Criteria SC-R003〜SC-R006 でテスト要件が明示されている)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust crate**: `rust/src/`, `rust/tests/`
- **Scripts**: `scripts/`
- **Bundled schemas**: `rust/bundled/`
- **Source schemas**: `schemas/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 前提条件の確認と開発環境準備

- [ ] T001 Verify cargo-typify is installed (`cargo typify --version`)
- [ ] T002 Verify Node.js and npm are installed for json-refs
- [ ] T003 Verify jq is installed for schema transformation
- [ ] T004 [P] Verify Cargo.toml has required dependencies (serde, serde_json, chrono, regress) in rust/Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: スキーマバンドリングとコード生成インフラの整備

**⚠️ CRITICAL**: User Story の実装はこのフェーズ完了後に開始可能

- [ ] T005 Modify scripts/bundle_schemas.sh to add jq transformation for unevaluatedProperties → additionalProperties conversion (FR-R002, FR-R003)
- [ ] T006 Run scripts/bundle_schemas.sh to regenerate all bundled schemas in rust/bundled/
- [ ] T007 Verify bundled schemas have additionalProperties: false instead of unevaluatedProperties
- [ ] T008 Run scripts/generate_rust.sh to regenerate all Rust types in rust/src/types/
- [ ] T009 Run cargo fmt in rust/ to format generated code (FR-R021)

**Checkpoint**: Foundation ready - バンドル済みスキーマと生成コードが最新状態

---

## Phase 3: User Story 1 - Rust struct の自動生成 (Priority: P1) 🎯 MVP

**Goal**: JSON Schema から serde 対応の Rust struct を自動生成し、型安全でシリアライズ/デシリアライズ可能なコードを書ける

**Independent Test**: 生成された Rust ソースファイルが存在し、derive マクロが正しく付与されていることを確認

### Implementation for User Story 1

- [ ] T010 [US1] Verify generated Quote struct in rust/src/types/quote.rs has #[derive(Serialize, Deserialize, Debug, Clone)] (FR-R006)
- [ ] T011 [P] [US1] Verify generated Ohlcv struct in rust/src/types/ohlcv.rs has correct derive macros
- [ ] T012 [P] [US1] Verify generated Trade struct in rust/src/types/trade.rs has correct derive macros
- [ ] T013 [P] [US1] Verify generated OrderBook struct in rust/src/types/orderbook.rs has correct derive macros
- [ ] T014 [P] [US1] Verify generated Instrument struct in rust/src/types/instrument.rs has correct derive macros
- [ ] T055 [P] [US1] Verify generated VolumeInfo struct in rust/src/types/volume_info.rs has correct derive macros
- [ ] T056 [P] [US1] Verify generated ExpiryInfo struct in rust/src/types/expiry_info.rs has correct derive macros
- [ ] T057 [P] [US1] Verify generated OptionInfo struct in rust/src/types/option_info.rs has correct derive macros
- [ ] T058 [P] [US1] Verify generated DerivativeInfo struct in rust/src/types/derivative_info.rs has correct derive macros
- [ ] T015 [US1] Verify rust/src/lib.rs re-exports all generated types (Quote, Ohlcv, Trade, OrderBook, Instrument, VolumeInfo, ExpiryInfo, OptionInfo, DerivativeInfo)
- [ ] T016 [US1] Verify doc comments are generated from schema descriptions in all rust/src/types/*.rs files
- [ ] T017 [US1] Verify optional fields have #[serde(default)] attribute in generated structs (FR-R007)

**Checkpoint**: US1 完了 - すべてのスキーマから struct が正常に生成されている (SC-R001)

---

## Phase 4: User Story 2 - 生成 struct でのデシリアライズ (Priority: P1)

**Goal**: 生成された struct で JSON データのデシリアライズを実行し、型安全なデータ操作ができる

**Independent Test**: 生成された struct に正常/異常な JSON データを渡し、期待通りの結果を得られることを確認

### Tests for User Story 2

- [ ] T018 [P] [US2] Add test_quote_deserialize_valid_with_all_fields in rust/tests/types_test.rs (SC-R003)
- [ ] T019 [P] [US2] Add test_quote_deserialize_valid_without_optional_fields in rust/tests/types_test.rs
- [ ] T020 [P] [US2] Add test_quote_deserialize_invalid_missing_required in rust/tests/types_test.rs (SC-R004)
- [ ] T021 [P] [US2] Add test_ohlcv_deserialize_valid in rust/tests/types_test.rs (3+ cases for SC-R003)
- [ ] T022 [P] [US2] Add test_ohlcv_deserialize_invalid_missing_required in rust/tests/types_test.rs
- [ ] T023 [P] [US2] Add test_trade_deserialize_valid in rust/tests/types_test.rs
- [ ] T024 [P] [US2] Add test_trade_deserialize_invalid_missing_side in rust/tests/types_test.rs (SC-R004)
- [ ] T025 [P] [US2] Add test_trade_deserialize_invalid_wrong_type in rust/tests/types_test.rs (SC-R004)
- [ ] T026 [P] [US2] Add test_orderbook_deserialize_valid in rust/tests/types_test.rs
- [ ] T027 [P] [US2] Add test_orderbook_deserialize_empty_arrays in rust/tests/types_test.rs
- [ ] T028 [P] [US2] Add test_instrument_deserialize_valid in rust/tests/types_test.rs
- [ ] T029 [P] [US2] Add test_instrument_deserialize_invalid_currency_pattern in rust/tests/types_test.rs

### Roundtrip Tests for User Story 2

- [ ] T030 [P] [US2] Add test_quote_roundtrip in rust/tests/types_test.rs (SC-R006)
- [ ] T031 [P] [US2] Add test_ohlcv_roundtrip in rust/tests/types_test.rs
- [ ] T032 [P] [US2] Add test_trade_roundtrip in rust/tests/types_test.rs
- [ ] T033 [P] [US2] Add test_orderbook_roundtrip in rust/tests/types_test.rs
- [ ] T034 [P] [US2] Add test_instrument_roundtrip in rust/tests/types_test.rs
- [ ] T047 [P] [US2] Add test_volume_info_deserialize_valid in rust/tests/types_test.rs
- [ ] T048 [P] [US2] Add test_expiry_info_deserialize_valid in rust/tests/types_test.rs
- [ ] T049 [P] [US2] Add test_option_info_deserialize_valid in rust/tests/types_test.rs
- [ ] T050 [P] [US2] Add test_derivative_info_deserialize_valid in rust/tests/types_test.rs
- [ ] T051 [P] [US2] Add test_volume_info_roundtrip in rust/tests/types_test.rs
- [ ] T052 [P] [US2] Add test_expiry_info_roundtrip in rust/tests/types_test.rs
- [ ] T053 [P] [US2] Add test_option_info_roundtrip in rust/tests/types_test.rs
- [ ] T054 [P] [US2] Add test_derivative_info_roundtrip in rust/tests/types_test.rs
- [ ] T059 [P] [US2] Add test_volume_info_deserialize_invalid in rust/tests/types_test.rs (SC-R004)
- [ ] T060 [P] [US2] Add test_expiry_info_deserialize_invalid in rust/tests/types_test.rs (SC-R004)
- [ ] T061 [P] [US2] Add test_option_info_deserialize_invalid in rust/tests/types_test.rs (SC-R004)
- [ ] T062 [P] [US2] Add test_derivative_info_deserialize_invalid in rust/tests/types_test.rs (SC-R004)

### Unknown Fields Rejection Tests (deny_unknown_fields)

- [ ] T035 [P] [US2] Add test_quote_reject_unknown_fields in rust/tests/types_test.rs (FR-R016)
- [ ] T036 [P] [US2] Add test_ohlcv_reject_unknown_fields in rust/tests/types_test.rs
- [ ] T037 [P] [US2] Add test_trade_reject_unknown_fields in rust/tests/types_test.rs

### Implementation Verification

- [ ] T038 [US2] Run cargo test in rust/ and verify all tests pass (SC-R003, SC-R004, SC-R006)

**Checkpoint**: US2 完了 - デシリアライズ正常系・異常系・ラウンドトリップがすべてテスト済み

---

## Phase 5: User Story 3 - コンパイラによる型検証 (Priority: P2)

**Goal**: 生成された struct を使用するコードがコンパイルを通過し、型安全性が保証される

**Independent Test**: cargo check / cargo clippy でコンパイルエラー・警告 0 件

### Implementation for User Story 3

- [ ] T039 [US3] Run cargo check in rust/ and verify 0 compile errors (SC-R002, FR-R020)
- [ ] T040 [US3] Run cargo clippy in rust/ and verify 0 critical warnings (SC-R005)
- [ ] T041 [P] [US3] Add compile-fail test for type mismatch in rust/tests/compile_tests.rs (optional: trybuild crate)

**Checkpoint**: US3 完了 - 型安全性がコンパイラレベルで保証されている

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: ドキュメント更新とクリーンアップ

- [ ] T042 [P] Update docs/code-generation.md Rust section with jq transformation details
- [ ] T043 [P] Document typify limitations (anyOf, if/then/else) in docs/code-generation.md (FR-R017)
- [ ] T044 Run quickstart.md validation: execute code examples from specs/002-data-model-rust/quickstart.md
- [ ] T045 [P] Update rust/README.md with usage examples (optional)
- [ ] T046 Final verification: run full test suite with cargo test in rust/

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational - struct 生成の検証
- **User Story 2 (Phase 4)**: Depends on US1 - 生成された struct のテスト
- **User Story 3 (Phase 5)**: Depends on US2 - コンパイラ検証（テスト含む）
- **Polish (Phase 6)**: Depends on US3 - ドキュメント更新

### User Story Dependencies

- **User Story 1 (P1)**: Foundation 完了後に開始可能 - 他のストーリーへの依存なし
- **User Story 2 (P1)**: US1 完了後 - 生成 struct が必要
- **User Story 3 (P2)**: US2 完了後 - テストコード含めてコンパイル検証

### Within Each User Story

- 検証タスク（verify）を先に実行
- テスト追加タスク（add test）は並列実行可能
- 最終確認タスク（run cargo test）で完了

### Parallel Opportunities

- **Phase 1**: T001-T004 は並列実行可能
- **Phase 2**: T005 → T006 → T007 → T008 → T009 は順次（依存関係あり）
- **Phase 3**: T010-T014 は並列、T015-T017 は順次
- **Phase 4**: T018-T037 はすべて並列（異なるテスト関数）、T038 で確認
- **Phase 5**: T039-T040 は順次、T041 は任意
- **Phase 6**: T042-T045 は並列、T046 で最終確認

---

## Parallel Example: User Story 2 Tests

```bash
# Launch all deserialize tests together (can run in parallel):
Task: "Add test_quote_deserialize_valid_with_all_fields in rust/tests/types_test.rs"
Task: "Add test_ohlcv_deserialize_valid in rust/tests/types_test.rs"
Task: "Add test_trade_deserialize_valid_with_side in rust/tests/types_test.rs"
Task: "Add test_orderbook_deserialize_valid in rust/tests/types_test.rs"
Task: "Add test_instrument_deserialize_valid in rust/tests/types_test.rs"

# Launch all roundtrip tests together:
Task: "Add test_quote_roundtrip in rust/tests/types_test.rs"
Task: "Add test_ohlcv_roundtrip in rust/tests/types_test.rs"
Task: "Add test_trade_roundtrip in rust/tests/types_test.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup ✅
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 - struct 生成検証
4. **STOP and VALIDATE**: 生成されたファイルの存在と derive マクロを確認

### Incremental Delivery

1. Complete Setup + Foundational → バンドル・生成インフラ完成
2. Add User Story 1 → struct 生成の検証 → 生成確認
3. Add User Story 2 → デシリアライズテスト追加 → テスト通過確認
4. Add User Story 3 → コンパイラ検証 → clippy 警告 0 件
5. Polish → ドキュメント更新

### Key Success Criteria Mapping

| Success Criteria | Task ID | Verification |
|------------------|---------|--------------|
| SC-R001 | T010-T014 | 全スキーマから struct 生成 |
| SC-R002 | T039 | cargo check エラー 0 件 |
| SC-R003 | T018-T028 | 各 struct × 3+ 正常系テスト |
| SC-R004 | T020, T022, T024, T025, T029, T059-T062 | 各 struct × 2+ 異常系テスト |
| SC-R005 | T040 | cargo clippy 警告 0 件 |
| SC-R006 | T030-T034 | ラウンドトリップテスト |

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- 既存の rust/tests/types_test.rs にテストを追加する形式
- FR-R002 の unevaluatedProperties 変換が最重要タスク（T005）
- 生成コードは手動編集禁止（CLAUDE.md Quality Standards）
- Commit after each task or logical group
