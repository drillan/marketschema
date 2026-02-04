# Tasks: HTTP Client Layer

**Input**: Design documents from `/specs/003-http-client/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/python-api.md, quickstart.md

**Tests**: TDD 必須（CLAUDE.md で TDD サイクルが指定されているため、テストタスクを含む）

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `python/src/marketschema/`, `python/tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and HTTP module structure

- [x] T001 Create HTTP module directory structure per plan.md (`python/src/marketschema/http/`)
- [x] T002 Add httpx>=0.27.0 to project dependencies (pyproject.toml)
- [x] T003 [P] Add pytest-asyncio>=0.24.0, respx>=0.21.0 to dev dependencies (pyproject.toml)
- [x] T004 [P] Create `python/src/marketschema/http/__init__.py` with public API exports

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Create base exception `HttpError` in `python/src/marketschema/http/exceptions.py`
- [x] T006 [P] Create `python/tests/unit/http/__init__.py` test directory structure
- [x] T007 Verify httpx and respx are installed and importable

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - 非同期 HTTP リクエストの実行 (Priority: P1) 🎯 MVP

**Goal**: AsyncHttpClient で JSON/テキストレスポンスを取得できる

**Independent Test**: モック API に対して `get_json()` を呼び出し、正しいレスポンスを取得できれば成功

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T008 [P] [US1] Unit test for AsyncHttpClient constructor in `python/tests/unit/http/test_client.py`
- [x] T009 [P] [US1] Unit test for `get_json()` method in `python/tests/unit/http/test_client.py`
- [x] T010 [P] [US1] Unit test for `get_text()` method in `python/tests/unit/http/test_client.py`
- [x] T011 [P] [US1] Unit test for `get()` method in `python/tests/unit/http/test_client.py`
- [x] T012 [P] [US1] Unit test for context manager (`async with`) in `python/tests/unit/http/test_client.py`

### Implementation for User Story 1

- [x] T013 [US1] Define constants (DEFAULT_TIMEOUT_SECONDS, DEFAULT_MAX_CONNECTIONS) in `python/src/marketschema/http/client.py`
- [x] T014 [US1] Implement AsyncHttpClient constructor with timeout, max_connections, headers in `python/src/marketschema/http/client.py`
- [x] T015 [US1] Implement `get()` method returning raw httpx.Response in `python/src/marketschema/http/client.py`
- [x] T016 [US1] Implement `get_json()` method returning dict in `python/src/marketschema/http/client.py`
- [x] T017 [US1] Implement `get_text()` method returning str in `python/src/marketschema/http/client.py`
- [x] T018 [US1] Implement `close()` method in `python/src/marketschema/http/client.py`
- [x] T019 [US1] Implement `__aenter__` and `__aexit__` for context manager in `python/src/marketschema/http/client.py`
- [x] T020 [US1] Update `python/src/marketschema/http/__init__.py` to export AsyncHttpClient

**Checkpoint**: User Story 1 should be fully functional - basic HTTP requests work with context manager

---

## Phase 4: User Story 2 - HTTP エラーの適切な処理 (Priority: P1)

**Goal**: タイムアウト、接続エラー、HTTP ステータスエラーを明確な例外として受け取れる

**Independent Test**: 各種エラー状況（タイムアウト、接続失敗、4xx/5xx）をシミュレートし、適切な例外が発生することを確認

### Tests for User Story 2 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T021 [P] [US2] Unit test for HttpError base exception in `python/tests/unit/http/test_exceptions.py`
- [x] T022 [P] [US2] Unit test for HttpTimeoutError in `python/tests/unit/http/test_exceptions.py`
- [x] T023 [P] [US2] Unit test for HttpConnectionError in `python/tests/unit/http/test_exceptions.py`
- [x] T024 [P] [US2] Unit test for HttpStatusError in `python/tests/unit/http/test_exceptions.py`
- [x] T025 [P] [US2] Unit test for HttpRateLimitError in `python/tests/unit/http/test_exceptions.py`
- [x] T026 [P] [US2] Unit test for `__cause__` exception chaining in `python/tests/unit/http/test_exceptions.py`
- [x] T027 [P] [US2] Unit test for client raising correct exceptions in `python/tests/unit/http/test_client.py`

### Implementation for User Story 2

- [x] T028 [P] [US2] Implement HttpTimeoutError in `python/src/marketschema/http/exceptions.py`
- [x] T029 [P] [US2] Implement HttpConnectionError in `python/src/marketschema/http/exceptions.py`
- [x] T030 [P] [US2] Implement HttpStatusError with status_code, response_body in `python/src/marketschema/http/exceptions.py`
- [x] T031 [US2] Implement HttpRateLimitError with retry_after in `python/src/marketschema/http/exceptions.py`
- [x] T032 [US2] Update AsyncHttpClient to catch httpx exceptions and raise custom exceptions in `python/src/marketschema/http/client.py`
- [x] T033 [US2] Ensure all exceptions preserve original exception via `__cause__` in `python/src/marketschema/http/exceptions.py`
- [x] T034 [US2] Update `python/src/marketschema/http/__init__.py` to export all exception classes

**Checkpoint**: User Story 2 complete - all HTTP errors are properly wrapped in custom exceptions

---

## Phase 5: User Story 3 - 失敗時の自動リトライ (Priority: P2)

**Goal**: 一時的なエラーに対して自動リトライが行われる

**Independent Test**: 一時的にエラーを返した後に成功するモック API で、自動リトライが機能することを確認

### Tests for User Story 3 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T035 [P] [US3] Unit test for RetryMiddleware constructor in `python/tests/unit/http/test_middleware.py`
- [x] T036 [P] [US3] Unit test for `should_retry()` method in `python/tests/unit/http/test_middleware.py`
- [x] T037 [P] [US3] Unit test for `get_delay()` exponential backoff in `python/tests/unit/http/test_middleware.py`
- [x] T038 [P] [US3] Unit test for jitter randomization in `python/tests/unit/http/test_middleware.py`
- [x] T039 [US3] Unit test for AsyncHttpClient with retry middleware integration in `python/tests/unit/http/test_client.py`

### Implementation for User Story 3

- [x] T040 [US3] Define retry constants (DEFAULT_MAX_RETRIES, DEFAULT_BACKOFF_FACTOR, DEFAULT_JITTER, RETRYABLE_STATUS_CODES, NON_RETRYABLE_STATUS_CODES) in `python/src/marketschema/http/middleware.py`
- [x] T041 [US3] Implement RetryMiddleware constructor in `python/src/marketschema/http/middleware.py`
- [x] T042 [US3] Implement `should_retry()` method in `python/src/marketschema/http/middleware.py`
- [x] T043 [US3] Implement `get_delay()` method with exponential backoff and jitter in `python/src/marketschema/http/middleware.py`
- [x] T044 [US3] Integrate RetryMiddleware into AsyncHttpClient request flow in `python/src/marketschema/http/client.py`
- [x] T045 [US3] Update `python/src/marketschema/http/__init__.py` to export RetryMiddleware

**Checkpoint**: User Story 3 complete - automatic retries work with exponential backoff

---

## Phase 6: User Story 4 - レート制限の遵守 (Priority: P2)

**Goal**: トークンバケットアルゴリズムによる流量制御が機能する

**Independent Test**: レート制限設定で高速リクエストを送信し、適切な間隔で送信されることを確認

### Tests for User Story 4 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T046 [P] [US4] Unit test for RateLimitMiddleware constructor in `python/tests/unit/http/test_middleware.py`
- [x] T047 [P] [US4] Unit test for `acquire()` blocking behavior in `python/tests/unit/http/test_middleware.py`
- [x] T048 [P] [US4] Unit test for `try_acquire()` non-blocking in `python/tests/unit/http/test_middleware.py`
- [x] T049 [P] [US4] Unit test for burst size handling in `python/tests/unit/http/test_middleware.py`
- [x] T050 [US4] Unit test for AsyncHttpClient with rate limit middleware integration in `python/tests/unit/http/test_client.py`

### Implementation for User Story 4

- [x] T051 [US4] Implement RateLimitMiddleware constructor with token bucket state in `python/src/marketschema/http/middleware.py`
- [x] T052 [US4] Implement token refill logic in `python/src/marketschema/http/middleware.py`
- [x] T053 [US4] Implement `acquire()` async method (blocking) in `python/src/marketschema/http/middleware.py`
- [x] T054 [US4] Implement `try_acquire()` method (non-blocking) in `python/src/marketschema/http/middleware.py`
- [x] T055 [US4] Integrate RateLimitMiddleware into AsyncHttpClient request flow in `python/src/marketschema/http/client.py`
- [x] T056 [US4] Update `python/src/marketschema/http/__init__.py` to export RateLimitMiddleware

**Checkpoint**: User Story 4 complete - rate limiting prevents excessive requests

---

## Phase 7: User Story 5 - レスポンスのキャッシュ (Priority: P3)

**Goal**: 同一 URL への繰り返しリクエストでキャッシュが機能する

**Independent Test**: 同一 URL に2回リクエストし、2回目がキャッシュから返されることを確認

### Tests for User Story 5 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T057 [P] [US5] Unit test for ResponseCache constructor in `python/tests/unit/http/test_cache.py`
- [x] T058 [P] [US5] Unit test for `get()` and `set()` methods in `python/tests/unit/http/test_cache.py`
- [x] T059 [P] [US5] Unit test for TTL expiration in `python/tests/unit/http/test_cache.py`
- [x] T060 [P] [US5] Unit test for LRU eviction in `python/tests/unit/http/test_cache.py`
- [x] T061 [P] [US5] Unit test for `delete()` and `clear()` methods in `python/tests/unit/http/test_cache.py`
- [x] T062 [US5] Unit test for AsyncHttpClient with cache integration in `python/tests/unit/http/test_client.py`

### Implementation for User Story 5

- [x] T063 [US5] Define cache constants (DEFAULT_CACHE_MAX_SIZE, DEFAULT_CACHE_TTL_SECONDS) in `python/src/marketschema/http/cache.py`
- [x] T064 [US5] Implement CacheEntry dataclass in `python/src/marketschema/http/cache.py`
- [x] T065 [US5] Implement ResponseCache constructor with OrderedDict in `python/src/marketschema/http/cache.py`
- [x] T066 [US5] Implement `get()` method with TTL check in `python/src/marketschema/http/cache.py`
- [x] T067 [US5] Implement `set()` method with LRU eviction in `python/src/marketschema/http/cache.py`
- [x] T068 [US5] Implement `delete()` and `clear()` methods in `python/src/marketschema/http/cache.py`
- [x] T069 [US5] Integrate ResponseCache into AsyncHttpClient request flow in `python/src/marketschema/http/client.py`
- [x] T070 [US5] Update `python/src/marketschema/http/__init__.py` to export ResponseCache

**Checkpoint**: User Story 5 complete - response caching reduces redundant requests

---

## Phase 8: User Story 6 - BaseAdapter との統合 (Priority: P2)

**Goal**: BaseAdapter で HTTP クライアントを簡単に利用できる

**Independent Test**: BaseAdapter を継承したクラスで `http_client` プロパティにアクセスし、HTTP リクエストを実行できれば成功

### Tests for User Story 6 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T071 [P] [US6] Unit test for `http_client` property lazy initialization in `python/tests/unit/adapters/test_base.py`
- [x] T072 [P] [US6] Unit test for BaseAdapter context manager in `python/tests/unit/adapters/test_base.py`
- [x] T073 [P] [US6] Unit test for custom http_client injection in `python/tests/unit/adapters/test_base.py`
- [x] T074 [US6] Integration test for BaseAdapter with HTTP client in `python/tests/integration/test_http_adapter.py`

### Implementation for User Story 6

- [x] T075 [US6] Add `_http_client` attribute to BaseAdapter in `python/src/marketschema/adapters/base.py`
- [x] T076 [US6] Add `http_client` property with lazy initialization in `python/src/marketschema/adapters/base.py`
- [x] T077 [US6] Update BaseAdapter constructor to accept optional http_client in `python/src/marketschema/adapters/base.py`
- [x] T078 [US6] Implement `close()` method in BaseAdapter to close HTTP client in `python/src/marketschema/adapters/base.py`
- [x] T079 [US6] Implement `__aenter__` and `__aexit__` for BaseAdapter in `python/src/marketschema/adapters/base.py`
- [x] T080 [US6] Update `python/src/marketschema/exceptions.py` to reference HttpError (optional, for documentation)

**Checkpoint**: User Story 6 complete - BaseAdapter integrates seamlessly with HTTP client

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T081 [P] Run all tests and verify 90%+ coverage on HTTP module
- [x] T082 [P] Run ruff check and format on all new files
- [x] T083 [P] Run mypy on python/src/marketschema/http/ and verify no type errors
- [x] T084 Validate quickstart.md examples work correctly
- [x] T085 Update `python/src/marketschema/__init__.py` to optionally expose http module

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - US1 (P1) and US2 (P1) can proceed in parallel
  - US3 (P2), US4 (P2), US6 (P2) can proceed after US1+US2 or in parallel if resources allow
  - US5 (P3) can proceed after US1+US2
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Foundation only - No dependencies on other stories
- **User Story 2 (P1)**: Foundation only - Can run in parallel with US1
- **User Story 3 (P2)**: Depends on US1 (client must exist to add retry)
- **User Story 4 (P2)**: Depends on US1 (client must exist to add rate limiting)
- **User Story 5 (P3)**: Depends on US1 (client must exist to add caching)
- **User Story 6 (P2)**: Depends on US1 + US2 (needs working client with exceptions)

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Middleware/cache components before client integration
- Core implementation before exports
- Story complete before moving to next priority

### Parallel Opportunities

- T002 + T003: Dependencies can be added in parallel
- T008-T012: All US1 tests can run in parallel
- T021-T027: All US2 tests can run in parallel
- T028-T030: Exception implementations can run in parallel
- T035-T038: All US3 RetryMiddleware tests can run in parallel
- T046-T049: All US4 RateLimitMiddleware tests can run in parallel
- T057-T061: All US5 ResponseCache tests can run in parallel
- T071-T073: All US6 BaseAdapter tests can run in parallel
- T081-T083: Polish verification tasks can run in parallel

---

## Parallel Example: User Story 1 Tests

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for AsyncHttpClient constructor in tests/unit/http/test_client.py"
Task: "Unit test for get_json() method in tests/unit/http/test_client.py"
Task: "Unit test for get_text() method in tests/unit/http/test_client.py"
Task: "Unit test for get() method in tests/unit/http/test_client.py"
Task: "Unit test for context manager in tests/unit/http/test_client.py"
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (HTTP リクエスト基本機能)
4. Complete Phase 4: User Story 2 (エラーハンドリング)
5. **STOP and VALIDATE**: Test US1 + US2 independently
6. Deploy/demo if ready - 基本的な HTTP クライアントが利用可能

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 + US2 → MVP: 基本 HTTP クライアントとエラー処理
3. US3 (リトライ) → 信頼性向上
4. US4 (レート制限) → API プロバイダーとの良好な関係
5. US5 (キャッシュ) → パフォーマンス最適化
6. US6 (BaseAdapter 統合) → ライブラリとしての使いやすさ

### Suggested MVP Scope

**MVP = US1 + US2**: 基本的な HTTP リクエストとエラーハンドリングだけで、アダプター開発者は作業を開始できる。

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD: Red → Green → Refactor)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- httpx mocking is done with respx library
