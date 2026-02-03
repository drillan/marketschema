# Feature Specification: Adapter Interface

**Feature Branch**: `004-adapter`
**Parent Spec**: [001-core](../001-core/spec.md)
**Dependencies**: [002-data-model](../002-data-model/spec.md), [003-http-client](../003-http-client/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: Issue #33 - 002-data-model からアダプター関連仕様を独立させる

## Overview

本仕様はアダプターフレームワーク（BaseAdapter, ModelMapping, Transforms, AdapterRegistry）のインターフェース契約を定義する。

### 責務分離

- **002-data-model**: 「何を」定義（User Stories, Functional Requirements のスコープ）
- **004-adapter**: 「どのように」定義（インターフェース契約の詳細）

002-data-model の User Story 4-5 および FR-018〜021 は維持し、本仕様で詳細契約を定義する。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - BaseAdapter によるデータソース変換 (Priority: P1)

アダプター開発者として、BaseAdapter を継承して新しいデータソース用のアダプターを実装し、生データを marketschema の標準モデル（Quote, OHLCV, Trade, OrderBook）に変換できる。

**Why this priority**: アダプター基盤は外部データソース統合の核心であり、すべてのアダプター実装がこれに依存する

**Independent Test**: BaseAdapter を継承したサンプルアダプターを実装し、`_apply_mapping()` でデータ変換ができることを確認

**Acceptance Scenarios**:

1. **Given** BaseAdapter を継承したアダプタークラス, **When** `source_name` を定義して初期化する, **Then** インスタンスが正常に作成される
2. **Given** `get_quote_mapping()` を実装したアダプター, **When** 生データを `_apply_mapping()` で変換する, **Then** 正しい Quote モデルが返される
3. **Given** http_client が渡されない場合, **When** `http_client` プロパティにアクセスする, **Then** 遅延初期化された AsyncHttpClient が返される

---

### User Story 2 - ModelMapping によるフィールドマッピング (Priority: P1)

アダプター開発者として、ModelMapping を使用してソースフィールドとターゲットフィールドの対応関係を定義し、変換関数やデフォルト値を指定できる。

**Why this priority**: フィールドマッピングはアダプター実装の基本単位であり、すべての変換処理に必須

**Independent Test**: ModelMapping を作成し、`apply()` メソッドで値を取得・変換できることを確認

**Acceptance Scenarios**:

1. **Given** ドット記法のパス `price.bid` を指定した ModelMapping, **When** ネストした辞書から値を取得する, **Then** 正しい値が返される
2. **Given** transform 関数を指定した ModelMapping, **When** `apply()` を呼び出す, **Then** 変換後の値が返される
3. **Given** `required=True` で値が存在しない場合, **When** `apply()` を呼び出す, **Then** MappingError が発生する
4. **Given** `default` を指定した ModelMapping, **When** ソースに値がない, **Then** デフォルト値が返される

---

### User Story 3 - Transforms による型変換 (Priority: P1)

アダプター開発者として、Transforms クラスの静的メソッドを使用して、文字列から数値への変換、タイムスタンプの正規化、売買方向の正規化などを行える。

**Why this priority**: 型変換は各データソースの形式差異を吸収するために不可欠

**Independent Test**: 各変換関数に対して正常系・異常系の入力でテストを実行

**Acceptance Scenarios**:

1. **Given** 文字列 `"123.45"`, **When** `to_float()` を呼び出す, **Then** `123.45` (float) が返される
2. **Given** Unix ミリ秒タイムスタンプ `1704067200000`, **When** `unix_timestamp_ms()` を呼び出す, **Then** ISO 8601 形式 `"2024-01-01T00:00:00Z"` が返される
3. **Given** 不正な値, **When** 変換関数を呼び出す, **Then** TransformError が発生する（サイレント障害禁止）

---

### User Story 4 - AdapterRegistry による動的管理 (Priority: P2)

システム設計者として、複数のアダプターを AdapterRegistry に登録し、source_name で動的に取得できる。

**Why this priority**: プラグイン的な拡張性を提供し、動的なアダプター選択を可能にする

**Independent Test**: 複数のアダプターを登録し、source_name で正しいアダプターを取得できることを確認

**Acceptance Scenarios**:

1. **Given** `@register` デコレータを付けたアダプタークラス, **When** クラス定義が評価される, **Then** AdapterRegistry に登録される
2. **Given** 登録済みの source_name, **When** `AdapterRegistry.get()` を呼び出す, **Then** 正しいアダプターインスタンスが返される
3. **Given** 未登録の source_name, **When** `AdapterRegistry.get()` を呼び出す, **Then** KeyError が発生する
4. **Given** 既に登録済みの source_name, **When** 同じ名前で登録を試みる, **Then** AdapterError が発生する

---

### Edge Cases

- source_name が空文字列の場合 → AdapterError
- ネストパスで中間キーが存在しない場合 → None を返す（required=True なら MappingError）
- transform 関数が例外を投げる場合 → TransformError としてラップ
- タイムスタンプが負値の場合 → TransformError（OSError を変換）
- side 文字列が "buy"/"sell" 以外の未知の値の場合 → TransformError

## Requirements *(mandatory)*

### Functional Requirements

#### BaseAdapter

- **FR-001**: BaseAdapter は `source_name` クラス属性を要求し、空文字列の場合は初期化時に AdapterError を発生させなければならない
- **FR-002**: BaseAdapter は `transforms` クラス属性を持ち、デフォルトで Transforms クラスを参照しなければならない
- **FR-003**: BaseAdapter は以下のマッピングメソッドを提供しなければならない: `get_quote_mapping()`, `get_ohlcv_mapping()`, `get_trade_mapping()`, `get_orderbook_mapping()`, `get_instrument_mapping()`
- **FR-004**: BaseAdapter は `_apply_mapping(raw_data, mappings, model_class)` メソッドを提供し、マッピング適用とモデルインスタンス化を行わなければならない
- **FR-005**: BaseAdapter は async context manager プロトコル (`__aenter__`, `__aexit__`) を実装しなければならない
- **FR-006**: BaseAdapter は http_client の遅延初期化をサポートし、所有する場合は `close()` で解放しなければならない

#### ModelMapping

- **FR-007**: ModelMapping は以下の属性を持たなければならない: `target_field`, `source_field`, `transform`, `default`, `required`
- **FR-008**: ModelMapping は `apply(source_data)` メソッドを提供し、ソースデータから値を取得・変換しなければならない
- **FR-009**: ModelMapping は `source_field` でドット記法によるネストアクセスをサポートしなければならない（例: `price.bid`）
- **FR-010**: ModelMapping は `required=True` かつ値が存在しない場合に MappingError を発生させなければならない

#### Transforms

- **FR-011**: Transforms は以下の静的メソッドを提供しなければならない: `to_float()`, `to_int()`, `iso_timestamp()`, `unix_timestamp_ms()`, `unix_timestamp_sec()`, `jst_to_utc()`, `side_from_string()`, `uppercase()`, `lowercase()`
- **FR-012**: すべての変換関数は変換失敗時に TransformError を発生させ、サイレント障害を禁止しなければならない
- **FR-013**: タイムスタンプ変換関数は常に UTC の ISO 8601 形式（末尾 Z）を返さなければならない

#### AdapterRegistry

- **FR-014**: AdapterRegistry はシングルトンパターンで実装しなければならない
- **FR-015**: AdapterRegistry は `register(adapter_class)` メソッドを提供し、デコレータとしても使用可能でなければならない
- **FR-016**: AdapterRegistry は `get(source_name)` メソッドを提供し、登録されたアダプターの新規インスタンスを返さなければならない
- **FR-017**: AdapterRegistry は `list_adapters()` メソッドを提供し、登録済み source_name のリストを返さなければならない
- **FR-018**: AdapterRegistry は `is_registered(source_name)` メソッドを提供しなければならない
- **FR-019**: AdapterRegistry は重複登録を禁止し、既存の source_name で登録を試みた場合に AdapterError を発生させなければならない

### Key Entities

各エンティティの詳細契約は [contracts/](contracts/) を参照。

- **BaseAdapter**: アダプターの基底クラス。データソース識別子（source_name）と各モデルへのマッピング定義メソッドを提供。HTTP クライアント統合をサポート
- **ModelMapping**: フィールドマッピング定義。ソースフィールドからターゲットフィールドへの変換ルールを定義。immutable な dataclass として実装
- **Transforms**: 共通変換関数群。静的メソッドとして数値変換、タイムスタンプ変換、文字列操作を提供
- **AdapterRegistry**: アダプター登録・取得を管理するシングルトン。デコレータによる宣言的登録をサポート

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: BaseAdapter を継承したサンプルアダプターが最低 3 つ（bitbank, stooq, stockanalysis 形式）実装でき、テストが通過する
- **SC-002**: すべての変換関数に対して正常系・異常系のユニットテストが存在し、通過する
- **SC-003**: AdapterRegistry を使用した動的アダプター取得のテストが通過する
- **SC-004**: 型チェッカー（mypy）でエラーがゼロである
- **SC-005**: Python 実装が本仕様の契約（contracts/）に準拠している

## Assumptions

- BaseAdapter は HTTP 通信を行うアダプターを主な対象とするが、他のプロトコル（WebSocket 等）も拡張可能な設計とする
- 変換関数の追加は本仕様の更新を必要とするが、カスタム変換は各アダプターで自由に定義可能
- AdapterRegistry はプロセス内のシングルトンとし、マルチプロセス間の共有は考慮しない
- Rust 実装は将来対応とし、Python 実装を先行する

## Out of Scope

- 各データソースの具体的な API 仕様（各アダプターパッケージで実装）
- WebSocket や gRPC などの非 HTTP プロトコル対応
- アダプターの自動生成機能
- アダプターのバージョン管理・互換性チェック
- アダプターの非同期並列実行制御

## Related Documents

- [contracts/adapter-interface.md](contracts/adapter-interface.md) - BaseAdapter, ModelMapping, AdapterRegistry の契約
- [contracts/transforms.md](contracts/transforms.md) - 変換関数の入出力仕様
- [lang/python.md](lang/python.md) - Python 実装ガイド
- [lang/rust.md](lang/rust.md) - Rust 実装ガイド（Planned）
- [checklists/requirements.md](checklists/requirements.md) - 要件チェックリスト
- [002-data-model](../002-data-model/spec.md) - User Story 4-5, FR-018〜021 のスコープ定義
- [docs/guides/adapter-development.md](../../docs/guides/adapter-development.md) - 実践的なアダプター開発チュートリアル
