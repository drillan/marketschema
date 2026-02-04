# Implementation Plan: HTTP Client Layer

**Branch**: `003-http-client` | **Date**: 2026-02-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-http-client/spec.md`

## Summary

marketschema ライブラリに共通 HTTP クライアントレイヤーを追加する。httpx を使用した非同期 HTTP クライアント `AsyncHttpClient` を実装し、リトライ、レート制限、キャッシュ機能を段階的に提供する。BaseAdapter との統合により、アダプター開発者が簡単にデータソースと通信できるようにする。

## Technical Context

**Language/Version**: Python 3.13
**Primary Dependencies**: httpx>=0.27.0, pydantic>=2.0.0
**Storage**: N/A（インメモリキャッシュのみ）
**Testing**: pytest, pytest-asyncio, respx (httpx mocking)
**Target Platform**: Linux, macOS, Windows（Python サポートプラットフォーム）
**Project Type**: Single project (Python library)
**Performance Goals**: コネクションプーリングによる効率的な HTTP 通信
**Constraints**: タイムアウト必須（デフォルト 30 秒）、無限待ち防止
**Scale/Scope**: アダプター開発者向けの共通インフラストラクチャ

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Schema First | ✅ PASS | HTTP クライアントはスキーマ定義外（インフラレイヤー） |
| II. 軽量コア | ✅ PASS | Constitution v0.5.0 で共通 HTTP クライアントをコアに含めることを承認済み |
| III. シンプルさ優先 | ✅ PASS | 80% のユースケースに最適化、段階的実装（YAGNI） |
| IV. 言語非依存 | ⚠️ N/A | Python 専用インフラ。他言語は別途実装 |
| V. エコシステム拡張 | ✅ PASS | httpx はオプショナル依存として提供 |
| 命名規則 | ✅ PASS | 標準名（timeout, retry, rate_limit）を使用 |
| 暗黙的フォールバック禁止 | ✅ PASS | エラーは明示的な例外として伝播 |
| ハードコード禁止 | ✅ PASS | 定数は名前付き定数として定義 |
| TDD | ✅ PASS | HTTP クライアントは TDD 必須対象 |

**Gate Result**: ✅ PASS - 実装を進めて良い

## Project Structure

### Documentation (this feature)

```text
specs/003-http-client/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── contracts/           # Phase 1 output (N/A - internal API)
```

### Source Code (repository root)

```text
python/src/marketschema/
├── __init__.py
├── exceptions.py              # EXTEND: HTTP exceptions reference
├── adapters/
│   ├── __init__.py
│   ├── base.py               # MODIFY: Add http_client property
│   ├── mapping.py
│   ├── transforms.py
│   └── registry.py
├── http/                      # NEW MODULE
│   ├── __init__.py           # Public API exports
│   ├── client.py             # AsyncHttpClient
│   ├── exceptions.py         # HTTP-specific exceptions
│   ├── middleware.py         # Retry/RateLimit middleware
│   └── cache.py              # Response caching
└── models/
    └── ... (unchanged)

tests/
├── unit/
│   ├── http/                  # NEW
│   │   ├── __init__.py
│   │   ├── test_client.py
│   │   ├── test_exceptions.py
│   │   ├── test_middleware.py
│   │   └── test_cache.py
│   └── adapters/
│       └── test_base.py      # EXTEND: http_client tests
└── integration/
    └── test_http_adapter.py  # NEW: E2E tests
```

**Structure Decision**: 既存の単一プロジェクト構造を維持し、`python/src/marketschema/http/` モジュールを新規追加

## Complexity Tracking

> **No violations requiring justification**

本実装は Constitution の原則に完全に適合しており、複雑さの正当化は不要。

