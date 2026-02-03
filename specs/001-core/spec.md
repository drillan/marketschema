# Core Architecture Specification

**Spec ID**: 001-core
**Created**: 2026-02-03
**Status**: Active

## Overview

marketschema プロジェクト全体のアーキテクチャと設計原則を定義する。
各機能仕様（002-data-model, 003-http-client 等）はこの仕様に基づいて設計・実装される。

## Project Mission

> 金融マーケットデータの取得・分析を、データソースに依存しない統一的な方法で行えるようにする

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       marketschema                           │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    Core Layer                          │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │  │
│  │  │   Models    │  │  Adapters   │  │    HTTP     │    │  │
│  │  │             │  │   (Base)    │  │  (Client)   │    │  │
│  │  │ - Quote     │  │             │  │             │    │  │
│  │  │ - OHLCV     │  │ - Mapping   │  │ - Async     │    │  │
│  │  │ - Trade     │  │ - Transform │  │ - Retry     │    │  │
│  │  │ - OrderBook │  │             │  │ - RateLimit │    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘    │  │
│  │        ▲                ▲                ▲            │  │
│  │        │                │                │            │  │
│  │        └────────────────┴────────────────┘            │  │
│  │                    Spec 002, 003                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Examples / Extensions                     │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐         │  │
│  │  │ bitbank  │  │  stooq   │  │stockanalysis │  ...    │  │
│  │  └──────────┘  └──────────┘  └──────────────┘         │  │
│  │                                                       │  │
│  │  Out of Scope: 個別データソース固有の実装              │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | 責務 | 実装場所 |
|-------|------|----------|
| Models | 統一データモデルの定義 | `src/marketschema/models/` |
| Adapters | データ変換・マッピング基盤 | `src/marketschema/adapters/` |
| HTTP | 共通 HTTP クライアント | `src/marketschema/http/` |
| Examples | 個別データソース対応例 | `examples/` |

## Spec Registry

| ID | Name | Description | Status | Dependencies |
|----|------|-------------|--------|--------------|
| 001 | core | 全体アーキテクチャ | Active | - |
| 002 | data-model | データモデル定義 | Implemented | 001 |
| 003 | http-client | HTTP クライアント | Planned | 001, 002 |

## Design Principles

本プロジェクトは以下の原則に従う。詳細は [Constitution](.specify/memory/constitution.md) を参照。

1. **Schema First** - JSON Schema が単一の真実の源
2. **軽量コア** - コアは最小限 + 共通インフラ
3. **シンプルさ優先** - 80% のユースケースに最適化
4. **言語非依存** - JSON Schema による言語中立な定義
5. **エコシステム拡張** - 業者対応はコア外で

## Technology Stack

| Category | Technology | Notes |
|----------|------------|-------|
| Schema | JSON Schema Draft 2020-12 | Single Source of Truth |
| Python | 3.13+ | Primary implementation |
| Python Models | Pydantic v2 | Auto-generated from schema |
| HTTP Client | httpx | Optional dependency |
| Rust | stable | Secondary implementation |

## Quality Gates

すべての spec 実装は以下を満たすこと：

- [ ] 型チェック合格 (`mypy src`)
- [ ] リンター合格 (`ruff check .`)
- [ ] テスト合格 (`pytest`)
- [ ] Constitution 準拠

## References

- [Constitution](/.specify/memory/constitution.md) - プロジェクト原則
- [002-data-model](../002-data-model/spec.md) - データモデル仕様
- [003-http-client](/.specify/features/http-client-layer/plan.md) - HTTP クライアント計画
