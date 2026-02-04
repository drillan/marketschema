# Feature Specification: Core Architecture

**Feature Branch**: `001-core`
**Created**: 2026-02-03
**Status**: Active
**Input**: User description: "コア仕様 001-core を定義してください"

## Clarifications

### Session 2026-02-03

- Q: コア機能の優先順位はどうするか？ → A: 優先順位はこの仕様では定めない。未定義・未実装のものは将来対応と記述する。
- Q: Quality Gates に具体的な数値指標を含めるべきか？ → A: 指標の種類のみ定義し、具体的数値は各 spec / CI 設定に委ねる。
- Q: エラーハンドリングの共通方針を定義するか？ → A: 各 spec で個別に定義。001-core では触れない。
- Q: Spec 追加・変更プロセスを定義するか？ → A: 最小限のルール（Registry 更新必須、依存関係明記）のみ定義。
- Q: User Story は仕様書の理解について書くべきか？ → A: プロジェクトの実際の機能・価値を説明するものに変更。統一データ取得、アダプター開発、型安全性を中心に記述。
- Q: FR を User Stories に沿ったものにするか？ → A: FR-001〜FR-009 を US1〜US3 に対応させて再定義。Key Entities もデータモデル・アダプター中心に変更。

## Overview

marketschema プロジェクト全体のアーキテクチャと設計原則を定義する。
この仕様は他のすべての spec（002-data-model, 003-http-client 等）の設計基盤となる。
本仕様は実装の優先順位を定めず、各コンポーネントの位置づけと責務のみを定義する。

### Mission

> 金融マーケットデータの取得・分析を、データソースに依存しない統一的な方法で行えるようにする

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 複数データソースからの統一的なデータ取得 (Priority: P1)

アプリケーション開発者として、複数の金融データソース（bitbank、stooq、stockanalysis 等）から
統一されたフォーマットでマーケットデータを取得し、データソース固有の実装詳細を意識せずに処理したい。

**Why this priority**: データソースの抽象化はライブラリの核心的価値であり、最も多くの開発者に恩恵をもたらす。

**Independent Test**: 異なるデータソースから同一の Quote/OHLCV モデルでデータを取得できれば成功。

**Acceptance Scenarios**:

1. **Given** 開発者が bitbank から BTC/JPY の価格を取得したい, **When** BitbankAdapter を使用する, **Then** 標準の Quote モデル（bid, ask, timestamp, symbol）でデータが返される
2. **Given** 開発者が stooq から AAPL の OHLCV を取得したい, **When** StooqAdapter を使用する, **Then** 標準の OHLCV モデル（open, high, low, close, volume）でデータが返される
3. **Given** 開発者がデータソースを切り替えたい, **When** アダプターを差し替える, **Then** アプリケーションコードの変更なしにデータソースを変更できる

---

### User Story 2 - カスタムアダプターの開発 (Priority: P2)

ライブラリ拡張者として、新しいデータソース（例：Yahoo Finance、Bloomberg）用の
アダプターを開発し、marketschema エコシステムに統合したい。

**Why this priority**: エコシステムの拡張性がライブラリの長期的な価値を決定する。

**Independent Test**: BaseAdapter を継承して新規アダプターを実装し、標準モデルに変換できれば成功。

**Acceptance Scenarios**:

1. **Given** 開発者が新しい取引所 API を統合したい, **When** BaseAdapter を継承してアダプターを作成する, **Then** ModelMapping を使ってソースデータを標準モデルに変換できる
2. **Given** アダプターがエラーハンドリングを実装したい, **When** ConversionError 等の標準例外を使用する, **Then** 一貫したエラー処理がアプリケーションに提供される
3. **Given** 開発者がカスタムフィールドを追加したい, **When** 標準モデルを拡張する, **Then** 互換性を維持しつつ追加データを扱える

---

### User Story 3 - 型安全なマーケットデータ処理 (Priority: P3)

データ分析者として、型安全で検証済みのマーケットデータを受け取り、
ランタイムエラーを防ぎながら分析処理を行いたい。

**Why this priority**: データの正確性と型安全性は金融データ処理の信頼性を担保する。

**Independent Test**: 不正なデータが適切にバリデーションエラーとして検出されれば成功。

**Acceptance Scenarios**:

1. **Given** OHLCV データを受け取る, **When** データに欠損や不正値がある, **Then** Pydantic バリデーションで明確なエラーが発生する
2. **Given** Quote データを処理する, **When** bid, ask, timestamp にアクセスする, **Then** IDE の型補完が機能し、型安全にコードを書ける
3. **Given** JSON Schema からモデルを生成したい, **When** コード生成ツールを使用する, **Then** Python / Rust で型安全なモデルが自動生成される

---

### Edge Cases

- データソースが一時的に利用不可の場合、適切なエラー（HttpConnectionError 等）で通知される
- フィールドの欠損や null 値は、モデル定義に従って Optional として処理される
- タイムスタンプの形式が異なる場合（Unix epoch、ISO 8601 等）、アダプターが正規化する

## Requirements *(mandatory)*

### Functional Requirements

**US1: 複数データソースからの統一的なデータ取得**

- **FR-001**: 標準データモデル（Quote, OHLCV, Trade, OrderBook）を提供しなければならない
- **FR-002**: アダプターを通じて、データソース固有のフォーマットを標準モデルに変換できなければならない
- **FR-003**: アダプターの差し替えにより、アプリケーションコードを変更せずにデータソースを切り替えられなければならない

**US2: カスタムアダプターの開発**

- **FR-004**: BaseAdapter クラスを提供し、新規アダプターの実装基盤としなければならない
- **FR-005**: ModelMapping を通じて、ソースデータから標準モデルへの変換ルールを定義できなければならない
- **FR-006**: 標準例外（ConversionError 等）を提供し、一貫したエラーハンドリングを可能にしなければならない

**US3: 型安全なマーケットデータ処理**

- **FR-007**: すべてのモデルは Pydantic ベースとし、実行時バリデーションを提供しなければならない
- **FR-008**: JSON Schema を単一の真実の源とし、各言語のモデルコードを生成できなければならない
- **FR-009**: 不正なデータに対して、明確なバリデーションエラーを発生させ、サイレント障害を防がなければならない

### Key Entities

- **Quote**: 気配値データ。bid, ask, timestamp, symbol を持つ
- **OHLCV**: ローソク足データ。open, high, low, close, volume, timestamp を持つ
- **Trade**: 約定データ。price, size, side, timestamp を持つ
- **OrderBook**: 板情報。bids, asks のリストを持つ
- **BaseAdapter**: アダプターの基底クラス。データソースと標準モデル間の変換を担う
- **ModelMapping**: フィールドマッピング定義。ソースフィールドと標準フィールドの対応を記述

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

| Layer    | 責務                     | 実装場所                       | 状態        |
|----------|--------------------------|-------------------------------|-------------|
| Models   | 統一データモデルの定義   | `python/src/marketschema/models/`    | 実装済み    |
| Adapters | データ変換・マッピング基盤 | `python/src/marketschema/adapters/` | 実装済み    |
| HTTP     | 共通 HTTP クライアント   | `python/src/marketschema/http/`      | 将来対応    |
| Examples | 個別データソース対応例   | `examples/`                   | 実装済み    |

## Spec Registry

### Core Specs (Language-Independent)

| ID  | Name        | Description              | Status      | Dependencies |
|-----|-------------|--------------------------|-------------|--------------|
| 001 | core        | 全体アーキテクチャ       | Active      | -            |
| 002 | data-model  | データモデル定義         | Implemented | 001          |
| 003 | http-client | HTTP クライアント        | Planned     | 001, 002     |
| 004 | adapter     | アダプターインターフェース | Draft       | 001, 002, 003 |

### Language-Specific Specs

| ID  | Name                 | Description                    | Status      | Dependencies |
|-----|----------------------|--------------------------------|-------------|--------------|
| 002-python | data-model-python  | Python データモデル実装    | Active      | 002          |
| 002-rust   | data-model-rust    | Rust データモデル実装      | Active      | 002          |
| 003-python | http-client-python | Python HTTP クライアント実装 | Implemented | 003          |
| 003-rust   | http-client-rust   | Rust HTTP クライアント実装 | Planned     | 003          |
| 004-python | adapter-python     | Python アダプター実装      | Implemented | 004          |
| 004-rust   | adapter-rust       | Rust アダプター実装        | Planned     | 004          |

**Status の定義**:
- **Active**: 有効な仕様として機能中
- **Implemented**: 実装完了
- **Planned**: 将来対応予定（設計済み、未実装）
- **Draft**: 検討中

### Spec 追加・変更ルール

新しい spec を追加または変更する際は、以下の最小限のルールに従うこと：

1. **Registry 更新必須**: 新規 spec は必ず Spec Registry に登録する
2. **依存関係明記**: 他の spec への依存がある場合は Dependencies に記載する
3. **ID の採番**: 作成順に連番を付与（001, 002, 003...）

## Design Principles

本プロジェクトは以下の原則に従う。詳細は [Constitution](../../.specify/memory/constitution.md) を参照。

1. **Schema First** - JSON Schema が単一の真実の源
2. **軽量コア** - コアは最小限 + 共通インフラ
3. **シンプルさ優先** - 80% のユースケースに最適化
4. **言語非依存** - JSON Schema による言語中立な定義
5. **エコシステム拡張** - 業者対応はコア外で

### Priority of Principles

設計判断で迷った場合の優先順位：

1. **正確性** - データが正しく変換されること
2. **シンプルさ** - 理解しやすく、使いやすいこと
3. **互換性** - 既存コードを壊さないこと
4. **パフォーマンス** - 十分に高速であること
5. **機能性** - より多くのユースケースをカバーすること

## Scope Definition

### In Scope

| カテゴリ               | 内容                                                       |
|------------------------|-----------------------------------------------------------|
| データモデル           | Quote, OHLCV, Trade, OrderBook, Instrument 等              |
| アダプター基盤         | BaseAdapter, ModelMapping, 共通変換関数                    |
| 共通 HTTP クライアント | AsyncHttpClient（リトライ、タイムアウト、レート制限対応） |
| 対象商品               | 株式、投信、債券、デリバティブ、FX、暗号資産、CFD          |
| コード生成             | JSON Schema から各言語へのモデル生成                       |

### Out of Scope

| カテゴリ                   | 理由                                   |
|---------------------------|----------------------------------------|
| 発注・約定管理            | 別ドメイン。精度・信頼性要件が異なる   |
| ポートフォリオ管理        | 別ドメイン                             |
| リスク計算                | 別ドメイン                             |
| 会計・税務処理            | 別ドメイン                             |
| 個別データソースの API 仕様 | 各アダプターパッケージで実装           |
| 認証・認可                | 各アダプターパッケージで実装           |

## Quality Gates

すべての spec 実装は以下の種類のチェックを通過すること。
具体的な数値指標（カバレッジ率、パフォーマンス閾値等）は各 spec または CI 設定で定義する。

| 指標の種類         | 説明                                       |
|--------------------|--------------------------------------------|
| 型チェック         | 静的型検査（Python: mypy, Rust: cargo check） |
| リンター           | コードスタイル・品質検査                   |
| テスト             | 単体・統合テストの実行                     |
| Constitution 準拠  | 設計原則との整合性確認                     |

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 3 つ以上のデータソース（bitbank, stooq, stockanalysis）から統一された Quote/OHLCV モデルでデータを取得できる
- **SC-002**: BaseAdapter を継承した新規アダプターが、サンプルコードを参考に実装可能である
- **SC-003**: 不正なデータ入力に対して、明確なバリデーションエラーが発生し、サイレント障害が起きない
- **SC-004**: JSON Schema から Python / Rust の型安全なモデルコードを自動生成できる

## References

- [Constitution](../../.specify/memory/constitution.md) - プロジェクト原則

### Core Specs

- [002-data-model](../002-data-model/spec.md) - データモデル仕様
- [003-http-client](../003-http-client/spec.md) - HTTP クライアント仕様
- [004-adapter](../004-adapter/spec.md) - アダプターインターフェース仕様

### Language-Specific Specs

- [002-data-model-python](../002-data-model-python/spec.md) - Python データモデル実装
- [002-data-model-rust](../002-data-model-rust/spec.md) - Rust データモデル実装
- [003-http-client-python](../003-http-client-python/spec.md) - Python HTTP クライアント実装
- [003-http-client-rust](../003-http-client-rust/spec.md) - Rust HTTP クライアント実装
- [004-adapter-python](../004-adapter-python/spec.md) - Python アダプター実装
- [004-adapter-rust](../004-adapter-rust/spec.md) - Rust アダプター実装
