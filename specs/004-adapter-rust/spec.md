# Feature Specification: Adapter Interface Rust Implementation

**Feature Branch**: `004-adapter-rust`
**Parent Spec**: [004-adapter](../004-adapter/spec.md)
**Dependencies**: [003-http-client-rust](../003-http-client-rust/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "Rust 言語でのアダプターインターフェース実装仕様を定義する"

## Clarifications

### Session 2026-02-03

- 親仕様 [004-adapter](../004-adapter/spec.md) に基づき、Rust 言語固有の実装仕様を定義。
- 既存の contracts/adapter.md および contracts/transforms.md を API 契約として継承。
- Python 実装の成熟後に着手予定だが、仕様は先行して定義。

## Overview

marketschema ライブラリの Rust 実装におけるアダプターフレームワーク（BaseAdapter trait, ModelMapping, Transforms, AdapterRegistry）を提供する。
親仕様で定義されたインターフェース契約を Rust 言語の慣用的な方法で実装し、
アダプター開発者が外部データソースを marketschema の標準モデルに変換できるようにする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - BaseAdapter trait による非同期データ取得と変換 (Priority: P1)

Rust アダプター開発者として、`BaseAdapter` trait を実装して新しいデータソース用のアダプターを作成し、
生データを marketschema の標準モデル（Quote, OHLCV, Trade, OrderBook, Instrument）に変換できる。

**Why this priority**: アダプター基盤は外部データソース統合の核心であり、すべてのアダプター実装がこれに依存する

**Independent Test**: `BaseAdapter` trait を実装したサンプルアダプターを作成し、`apply_mapping()` でデータ変換ができることを確認

**Acceptance Scenarios**:

1. **Given** BaseAdapter trait を実装した構造体, **When** `source_name()` メソッドを呼び出す, **Then** データソース識別子の `&'static str` が返される
2. **Given** `get_quote_mapping()` を実装したアダプター, **When** 生の JSON データを変換する, **Then** 正しい Quote モデルインスタンスが返される
3. **Given** HTTP クライアントを必要とするアダプター, **When** `new()` で初期化する, **Then** 依存性注入パターンで HTTP クライアントを受け取れる
4. **Given** `async_trait` を使用したアダプター, **When** 非同期メソッドを呼び出す, **Then** 正常に非同期処理が実行される

---

### User Story 2 - ModelMapping struct による型安全なフィールドマッピング (Priority: P1)

Rust アダプター開発者として、`ModelMapping` struct を使用してソースフィールドとターゲットフィールドの対応関係を定義し、
ビルダーパターンで transform 関数やデフォルト値を指定できる。

**Why this priority**: フィールドマッピングはアダプター実装の基本単位であり、すべての変換処理に必須

**Independent Test**: `ModelMapping` インスタンスを作成し、`apply()` メソッドで値を取得・変換できることを確認

**Acceptance Scenarios**:

1. **Given** ドット記法のパス `"price.bid"` を指定した ModelMapping, **When** ネストした JSON `{"price": {"bid": 100.0}}` から値を取得する, **Then** `100.0` が返される
2. **Given** `with_transform(Transforms::to_float_fn())` を指定した ModelMapping, **When** `apply(&json!({"value": "123.45"}))` を呼び出す, **Then** `json!(123.45)` が返される
3. **Given** `required=true` で値が存在しない場合, **When** `apply()` を呼び出す, **Then** `Err(MappingError)` が返される
4. **Given** `with_default(json!(0.0)).optional()` を指定した ModelMapping, **When** ソースに値がない, **Then** `json!(0.0)` が返される
5. **Given** `ModelMapping::new("target", "source")`, **When** ビルダーメソッドをチェインする, **Then** 流暢なインターフェースで設定できる

---

### User Story 3 - Transforms 静的メソッドによる型変換 (Priority: P1)

Rust アダプター開発者として、`Transforms` struct の関連関数を使用して、
文字列から数値への変換、タイムスタンプの UTC 正規化、売買方向の正規化などを行える。

**Why this priority**: 型変換は各データソースの形式差異を吸収するために不可欠

**Independent Test**: 各変換関数に対して正常系・異常系の入力でテストを実行

**Acceptance Scenarios**:

1. **Given** 文字列 `"123.45"`, **When** `Transforms::to_float(&json!("123.45"))` を呼び出す, **Then** `Ok(123.45)` が返される
2. **Given** Unix ミリ秒タイムスタンプ `1704067200000`, **When** `Transforms::unix_timestamp_ms(&json!(1704067200000))` を呼び出す, **Then** ISO 8601 形式 `"2024-01-01T00:00:00Z"` が返される
3. **Given** JST タイムスタンプ `"2024-01-01T09:00:00"`, **When** `Transforms::jst_to_utc(&json!("2024-01-01T09:00:00"))` を呼び出す, **Then** `Ok("2024-01-01T00:00:00Z")` が返される
4. **Given** 不正な値 `"invalid"`, **When** `Transforms::to_float(&json!("invalid"))` を呼び出す, **Then** `Err(TransformError)` が返され、エラーメッセージに元の値が含まれる
5. **Given** 売買方向 `"BUY"`, **When** `Transforms::side_from_string(&json!("BUY"))` を呼び出す, **Then** `Ok("buy")` が返される

---

### User Story 4 - AdapterRegistry によるスレッドセーフな動的管理 (Priority: P2)

Rust アダプター開発者として、複数のアダプターを `AdapterRegistry` に登録し、
`source_name` で動的に取得できる。レジストリはスレッドセーフである。

**Why this priority**: プラグイン的な拡張性を提供し、動的なアダプター選択を可能にする

**Independent Test**: 複数のアダプターを登録し、`source_name` で正しいアダプターを取得できることを確認

**Acceptance Scenarios**:

1. **Given** アダプターファクトリをクロージャで定義, **When** `AdapterRegistry::register("myapi", || Box::new(MyAdapter))` を呼び出す, **Then** `Ok(())` が返され登録される
2. **Given** 登録済みの `source_name="bitbank"`, **When** `AdapterRegistry::get("bitbank")` を呼び出す, **Then** `Some(Box<dyn BaseAdapter>)` が返される
3. **Given** 未登録の `source_name="unknown"`, **When** `AdapterRegistry::get("unknown")` を呼び出す, **Then** `None` が返される
4. **Given** 既に登録済みの `source_name`, **When** 同じ名前で登録を試みる, **Then** `Err(AdapterError::DuplicateRegistration)` が返される
5. **Given** `AdapterRegistry::list_adapters()` を呼び出す, **When** 複数のアダプターが登録されている, **Then** `Vec<String>` が返される
6. **Given** マルチスレッド環境, **When** 複数スレッドから同時に `get()` を呼び出す, **Then** データ競合なく安全にアクセスできる

---

### Edge Cases

- `source_name` が空文字列の場合 → コンパイル時または初期化時に検出（`&'static str` を使用）
- ネストパス `"a.b.c"` で中間キー `"b"` が存在しない場合 → `None` または `Err(MappingError)` を返す（`required` フラグに依存）
- `transform` 関数がエラーを返す場合 → `MappingError` としてラップし、`#[source]` で元のエラーを保持
- タイムスタンプが負値の場合 → `Err(TransformError)` を返す
- `side` 文字列が未知の値 `"exchange"` の場合 → `Err(TransformError("Cannot normalize side value: 'exchange'"))`
- `ModelMapping.apply()` で `transform` が `None` を返す場合 → そのまま `None` として扱う（`required=true` なら `Err(MappingError)`）
- `RwLock` がポイズンされた場合 → パニックまたは `unwrap()` で伝播（復旧不能なエラー）

## Requirements *(mandatory)*

### Functional Requirements

#### BaseAdapter Trait

- **FR-R001**: `BaseAdapter` は `async_trait` を使用した trait として定義され、`Send + Sync` を要求しなければならない
- **FR-R002**: `BaseAdapter::source_name()` メソッドは `&'static str` を返さなければならない
- **FR-R003**: `BaseAdapter` は以下のマッピングメソッドをデフォルト実装付きで提供しなければならない: `get_quote_mapping()`, `get_ohlcv_mapping()`, `get_trade_mapping()`, `get_orderbook_mapping()`, `get_instrument_mapping()`
- **FR-R004**: 各マッピングメソッドはデフォルトで空の `Vec<ModelMapping>` を返し、実装者がオーバーライド可能でなければならない

#### ModelMapping Struct

- **FR-R005**: `ModelMapping` は `Clone` を実装した struct として定義されなければならない
- **FR-R006**: `ModelMapping` は以下のフィールドを持たなければならない: `target_field: String`, `source_field: String`, `transform: Option<TransformFn>`, `default: Option<serde_json::Value>`, `required: bool`
- **FR-R007**: `ModelMapping::new(target, source)` コンストラクタは `required=true` で初期化しなければならない
- **FR-R008**: `ModelMapping` はビルダーパターンのメソッドを提供しなければならない: `with_transform()`, `with_default()`, `optional()`
- **FR-R009**: `ModelMapping::apply(&self, source_data: &serde_json::Value)` メソッドはソースデータから値を取得・変換し、`Result<serde_json::Value, MappingError>` を返さなければならない
- **FR-R010**: `ModelMapping` は `source_field` でドット記法によるネストアクセスをサポートしなければならない（例: `"price.bid"` → `data["price"]["bid"]`）
- **FR-R011**: `ModelMapping` は `required=true` かつ値が存在しない場合に `Err(MappingError)` を返さなければならない

#### TransformFn Type

- **FR-R012**: `TransformFn` は `Arc<dyn Fn(&serde_json::Value) -> Result<serde_json::Value, TransformError> + Send + Sync>` として定義されなければならない

#### Transforms Struct

- **FR-R013**: `Transforms` struct は以下の関連関数を提供しなければならない: `to_float()`, `to_int()`, `iso_timestamp()`, `unix_timestamp_ms()`, `unix_timestamp_sec()`, `jst_to_utc()`, `side_from_string()`, `uppercase()`, `lowercase()`
- **FR-R014**: 各変換関数は `Result<T, TransformError>` を返し、変換失敗時にエラーを返さなければならない（サイレント障害禁止）
- **FR-R015**: タイムスタンプ変換関数は常に UTC の ISO 8601 形式（末尾 `Z`）を返さなければならない
- **FR-R016**: `side_from_string()` は `"buy"/"sell"` を返し、未知の値には `Err(TransformError)` を返さなければならない
- **FR-R017**: `Transforms` は各変換関数に対応する `_fn()` サフィックス付きの `TransformFn` ファクトリメソッドを提供しなければならない（例: `to_float_fn() -> TransformFn`）

#### AdapterRegistry

- **FR-R018**: `AdapterRegistry` は `once_cell::sync::Lazy` と `RwLock<HashMap<String, AdapterFactory>>` を使用したグローバルシングルトンとして実装しなければならない
- **FR-R019**: `AdapterFactory` は `Arc<dyn Fn() -> Box<dyn BaseAdapter> + Send + Sync>` として定義されなければならない
- **FR-R020**: `AdapterRegistry::register<F>(source_name, factory)` 関数はアダプターを登録し、`Result<(), AdapterError>` を返さなければならない
- **FR-R021**: `AdapterRegistry::get(source_name)` 関数は登録されたアダプターの新規インスタンスを `Option<Box<dyn BaseAdapter>>` で返さなければならない
- **FR-R022**: `AdapterRegistry::list_adapters()` 関数は登録済み `source_name` の `Vec<String>` を返さなければならない
- **FR-R023**: `AdapterRegistry::is_registered(source_name)` 関数は登録済みかどうかを `bool` で返さなければならない
- **FR-R024**: `AdapterRegistry::clear()` 関数はすべての登録を解除しなければならない（テスト用）
- **FR-R025**: `AdapterRegistry` は重複登録を禁止し、既存の `source_name` で登録を試みた場合に `Err(AdapterError::DuplicateRegistration)` を返さなければならない

#### Error Types

- **FR-R026**: `AdapterError` は `thiserror::Error` を derive し、`General(String)`, `DuplicateRegistration(String)`, `Mapping(MappingError)`, `Transform(TransformError)` バリアントを持たなければならない
- **FR-R027**: `MappingError` は `thiserror::Error` を derive し、`message: String` フィールドと `new()` コンストラクタを持たなければならない
- **FR-R028**: `TransformError` は `thiserror::Error` を derive し、`message: String` フィールドと `new()` コンストラクタを持たなければならない
- **FR-R029**: `AdapterError` は `MappingError` と `TransformError` からの `#[from]` 変換を実装しなければならない

### Key Entities

- **BaseAdapter**: アダプターの基底 trait。`source_name()` メソッド、マッピングメソッドを提供。`async_trait` + `Send + Sync` を要求
- **ModelMapping**: フィールドマッピング定義 struct。`target_field`, `source_field`, `transform`, `default`, `required` フィールドとビルダーパターンを提供
- **TransformFn**: transform 関数の型エイリアス。`Arc<dyn Fn(...) + Send + Sync>` でスレッドセーフ
- **Transforms**: 共通変換関数群を関連関数として提供する struct
- **AdapterRegistry**: スレッドセーフなグローバルレジストリ。`RwLock` でアクセス保護
- **AdapterFactory**: アダプターインスタンスを生成するファクトリ関数の型エイリアス
- **AdapterError**: アダプター操作エラーの enum（General, DuplicateRegistration, Mapping, Transform）
- **MappingError**: フィールドマッピングエラー struct
- **TransformError**: 値変換エラー struct

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-R001**: BaseAdapter trait を実装したサンプルアダプターが最低 2 つ実装でき、すべてのユニットテストが通過する
- **SC-R002**: すべての Transforms 関数に対して正常系・異常系のユニットテストが存在し、通過する
- **SC-R003**: ModelMapping の `apply()` メソッドに対して、ドット記法・デフォルト値・transform・required フラグのユニットテストが通過する
- **SC-R004**: AdapterRegistry を使用した動的アダプター取得・重複登録エラーのテストが通過する
- **SC-R005**: `cargo clippy --all-targets` で警告がゼロである
- **SC-R006**: `cargo fmt --check` でフォーマットエラーがゼロである
- **SC-R007**: `cargo test` ですべてのテストが通過する
- **SC-R008**: マルチスレッドテストで AdapterRegistry のスレッドセーフ性が確認できる

## Assumptions

- Rust latest stable（2024 edition）を使用する
- `async-trait = "0.1"` を使用して非同期 trait メソッドを実装する
- `chrono = { version = "0.4", features = ["serde"] }` を使用してタイムスタンプ処理を行う
- `once_cell = "1.19"` を使用してグローバルシングルトンを実装する
- `serde = { version = "1.0", features = ["derive"] }` と `serde_json = "1.0"` を使用して JSON 処理を行う
- `thiserror = "1.0"` を使用してエラー型を derive する
- `reqwest = { version = "0.12", features = ["json"] }` を使用して HTTP クライアントを提供する（将来）
- `tokio = { version = "1", features = ["rt-multi-thread", "macros"] }` を使用して非同期ランタイムを提供する
- Python 実装の成熟後に本仕様の実装を開始する
- 変換関数の追加は本仕様の更新を必要とするが、カスタム変換は各アダプターで自由に定義可能
- AdapterRegistry はプロセス内のシングルトンとし、複数プロセス間の共有は考慮しない

## Language-Specific Considerations

- `async_trait` マクロによる非同期 trait メソッド
- `Arc<dyn Fn(...) + Send + Sync>` による transform 関数のスレッドセーフな共有
- `once_cell::sync::Lazy` によるグローバル状態の遅延初期化
- `RwLock` によるスレッドセーフなレジストリアクセス
- `thiserror` による宣言的なエラー型定義
- `#[from]` アトリビュートによるエラー変換
- `#[source]` アトリビュートによるエラーチェイン
- ビルダーパターンによる流暢なインターフェース（`self` を消費して `Self` を返す）
- `serde_json::Value` による動的 JSON 処理

## Module Structure

```
crates/marketschema-adapters/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public exports
│   ├── adapter.rs       # BaseAdapter trait
│   ├── mapping.rs       # ModelMapping struct
│   ├── registry.rs      # AdapterRegistry
│   ├── transforms.rs    # Transforms functions
│   └── error.rs         # Error types
```

## Dependencies

```toml
# Cargo.toml
[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1.19"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"

# For HTTP client (future)
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

[dev-dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Implementation Priorities

1. **Phase 1**: Core types and errors (error.rs)
2. **Phase 2**: Transform functions (transforms.rs)
3. **Phase 3**: ModelMapping with builder pattern (mapping.rs)
4. **Phase 4**: BaseAdapter trait with async_trait (adapter.rs)
5. **Phase 5**: AdapterRegistry with thread-safe global state (registry.rs)
6. **Phase 6**: Integration with marketschema-models crate

## Out of Scope

- 各データソースの具体的な API 仕様（各アダプター crate で実装）
- WebSocket や gRPC などの非 HTTP プロトコル対応
- アダプターの自動生成機能（Derive macro は将来検討）
- アダプターのバージョン管理・互換性チェック
- アダプターの非同期並列実行制御
- 複数プロセス間のレジストリ共有

## Contracts

- [Adapter Contract (Rust)](contracts/adapter.md) - BaseAdapter trait, ModelMapping, AdapterRegistry の Rust 型定義
- [Transform Functions (Rust)](contracts/transforms.md) - 変換関数の Rust 型定義

## References

- [004-adapter](../004-adapter/spec.md) - 親仕様（言語非依存）
- [004-adapter/contracts/adapter-interface.md](../004-adapter/contracts/adapter-interface.md) - 言語非依存インターフェース契約
- [004-adapter/contracts/transforms.md](../004-adapter/contracts/transforms.md) - 言語非依存変換関数仕様
- [003-http-client-rust](../003-http-client-rust/spec.md) - HTTP クライアント Rust 実装
- [Rust typify](https://github.com/oxidecomputer/typify) - JSON Schema to Rust code generation
