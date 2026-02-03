# Implementation Plan: 統一マーケットデータスキーマ

**Branch**: `002-data-model` | **Date**: 2026-02-02 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-data-model/spec.md`

## Summary

金融マーケットデータの統一スキーマを JSON Schema Draft 2020-12 で定義し、Python (pydantic v2) および Rust (serde) への自動コード生成を可能にする。Quote、OHLCV、Trade、OrderBook、Instrument 等のデータモデルとアダプター基盤を提供する。

## Technical Context

**Language/Version**: Python 3.13, Rust (latest stable)
**Primary Dependencies**:
- JSON Schema バリデーション: ajv-cli
- Python コード生成: datamodel-code-generator
- Rust コード生成: typify
- Python モデル: pydantic v2
- 型チェック: mypy
**Storage**: N/A（スキーマ定義ファイルのみ）
**Testing**: pytest (Python), cargo test (Rust)
**Target Platform**: Linux, macOS (CLI/ライブラリ)
**Project Type**: Single project（ライブラリ＋CLI）
**Performance Goals**: N/A（スキーマ定義とコード生成が主目的、実行時パフォーマンスは対象外）
**Constraints**: JSON Schema Draft 2020-12 準拠必須

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Schema First | ✅ PASS | JSON Schema を唯一の定義元とし、pydantic/Rust コードは自動生成 |
| II. 軽量コア | ✅ PASS | スキーマ定義、BaseAdapter、共通変換関数のみをコアに含める。個別データソース対応は対象外 |
| III. シンプルさ優先 | ✅ PASS | 80%ユースケース対応。Quote, OHLCV, Trade, OrderBook 等の基本モデルに集中 |
| IV. 言語非依存 | ✅ PASS | JSON Schema による言語中立定義、Python/Rust で同等機能提供 |
| V. エコシステム拡張 | ✅ PASS | 個別業者アダプターは独立パッケージとしてスコープ外 |
| 命名の揺れの禁止 | ✅ PASS | ADR で標準名を定義済み（bid, ask, open, high, low, close, volume, price, size, side, timestamp, symbol） |
| 業界標準名の採用 | ✅ PASS | FIX Protocol, 主要取引所 API から標準名を調査済み。ADR に決定理由を記録 |
| 暗黙的フォールバック禁止 | ✅ PASS | アダプター変換関数で明示的エラーを返す設計 |
| ハードコード禁止 | ✅ PASS | 変換定数（MS_PER_SECOND 等）は名前付き定数として定義 |

**Gate Result**: PASS - 全原則に準拠

## Project Structure

### Documentation (this feature)

```text
specs/002-data-model/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (JSON Schema files)
├── field-requirements.csv  # Entity-field definitions
├── enum-values.csv      # Enum value definitions
└── format-conventions.csv  # Format standards
```

### Source Code (repository root)

```text
src/
├── marketschema/
│   ├── __init__.py
│   ├── py.typed                    # PEP 561 marker
│   ├── schemas/                    # JSON Schema files
│   │   ├── definitions.json        # 共通型定義（Timestamp, Side, AssetClass, Currency）
│   │   ├── quote.json              # Quote スキーマ
│   │   ├── ohlcv.json              # OHLCV スキーマ
│   │   ├── trade.json              # Trade スキーマ
│   │   ├── orderbook.json          # OrderBook スキーマ
│   │   ├── volume_info.json        # VolumeInfo スキーマ
│   │   ├── expiry_info.json        # ExpiryInfo スキーマ
│   │   ├── option_info.json        # OptionInfo スキーマ
│   │   ├── derivative_info.json    # DerivativeInfo スキーマ
│   │   └── instrument.json         # Instrument スキーマ
│   ├── models/                     # Generated pydantic models
│   │   └── __init__.py             # Re-exports all models
│   ├── adapters/                   # Adapter infrastructure
│   │   ├── __init__.py
│   │   ├── base.py                 # BaseAdapter class
│   │   ├── mapping.py              # ModelMapping class
│   │   ├── registry.py             # AdapterRegistry
│   │   └── transforms.py           # Common transform functions
│   └── exceptions.py               # Custom exceptions

tests/
├── unit/
│   ├── test_schemas.py             # Schema validation tests
│   ├── test_models.py              # Generated model tests
│   └── test_transforms.py          # Transform function tests
├── integration/
│   └── test_adapter_base.py        # Adapter infrastructure tests
└── contract/
    └── test_schema_compliance.py   # JSON Schema Draft 2020-12 compliance

docs/
├── adr/                            # Architecture Decision Records
│   ├── index.md
│   ├── field-names/                # Field naming decisions
│   └── types/                      # Type and format decisions
├── research/                       # Research sources
└── glossary.md                     # Term definitions
```

**Structure Decision**: Single project 構造を採用。JSON Schema ファイルは `src/marketschema/schemas/` に配置し、パッケージデータとして配布可能にする。

## Constitution Check (Post-Design)

*Re-evaluation after Phase 1 design completion.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Schema First | ✅ PASS | contracts/ に JSON Schema を定義。pydantic/Rust コードは datamodel-codegen/typify で自動生成 |
| II. 軽量コア | ✅ PASS | 9 エンティティ + アダプター基盤のみ。個別業者対応なし |
| III. シンプルさ優先 | ✅ PASS | 共通型を definitions.json に集約。各スキーマは単一ファイルでシンプルな構造 |
| IV. 言語非依存 | ✅ PASS | JSON Schema を定義元として Python/Rust 両対応 |
| V. エコシステム拡張 | ✅ PASS | unevaluatedProperties: false で厳密化しつつ、allOf による派生スキーマで拡張可能 |
| 命名の揺れの禁止 | ✅ PASS | 全フィールド名が ADR に準拠（bid, ask, open, high, low, close, volume, price, size, side, timestamp, symbol） |
| 業界標準名の採用 | ✅ PASS | field-requirements.csv で ADR 参照を明記 |
| 暗黙的フォールバック禁止 | ✅ PASS | quickstart.md でアダプターのエラーハンドリングパターンを記載 |
| ハードコード禁止 | ✅ PASS | 該当なし（スキーマ定義のみ） |

**Gate Result**: PASS - Phase 1 設計完了、全原則に準拠

## Phase 1 Artifacts

以下の成果物を生成:

| Artifact | Path | Description |
|----------|------|-------------|
| research.md | specs/002-data-model/research.md | 技術調査結果（JSON Schema, datamodel-codegen, typify） |
| data-model.md | specs/002-data-model/data-model.md | エンティティ定義とリレーション |
| quickstart.md | specs/002-data-model/quickstart.md | 使用方法ガイド |
| contracts/ | specs/002-data-model/contracts/ | JSON Schema ファイル（9 ファイル） |

## Complexity Tracking

> No violations requiring justification. All requirements are aligned with constitution principles.
