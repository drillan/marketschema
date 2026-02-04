# Data Model: Adapter Interface Rust Implementation

**Feature Branch**: `004-adapter-rust`
**Date**: 2026-02-04

## Entity Overview

```
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  AdapterRegistry │◀────│   BaseAdapter   │◀────│   ModelMapping  │
│  (singleton)     │      │   (trait)       │      │   (struct)      │
└─────────────────┘      └────────┬────────┘      └────────┬────────┘
                                  │                         │
                                  ▼                         ▼
                         ┌─────────────────┐      ┌─────────────────┐
                         │  AdapterFactory │      │  TransformFn    │
                         │  (type alias)   │      │  (type alias)   │
                         └─────────────────┘      └────────┬────────┘
                                                           │
                                                           ▼
                                                  ┌─────────────────┐
                                                  │   Transforms    │
                                                  │   (struct)      │
                                                  └─────────────────┘
```

## Entities

### 1. TransformError

変換処理中に発生するエラー。

| Field | Type | Description |
|-------|------|-------------|
| `message` | `String` | エラーの説明（入力値を含む） |

**Validation Rules**:
- `message` は空でない文字列

**Methods**:
- `new(message: impl Into<String>) -> Self`

### 2. MappingError

フィールドマッピング中に発生するエラー。

| Field | Type | Description |
|-------|------|-------------|
| `message` | `String` | エラーの説明（フィールド名を含む） |

**Validation Rules**:
- `message` は空でない文字列

**Methods**:
- `new(message: impl Into<String>) -> Self`

### 3. AdapterError

アダプター操作全般のエラー。

| Variant | Payload | Description |
|---------|---------|-------------|
| `General` | `String` | 一般的なエラー |
| `DuplicateRegistration` | `String` | 重複登録エラー |
| `Mapping` | `MappingError` | マッピングエラー |
| `Transform` | `TransformError` | 変換エラー |

**Conversions**:
- `From<MappingError>` for `AdapterError`
- `From<TransformError>` for `AdapterError`

### 4. TransformFn (Type Alias)

```rust
pub type TransformFn = Arc<dyn Fn(&Value) -> Result<Value, TransformError> + Send + Sync>;
```

**Constraints**:
- `Send + Sync`: スレッド間で安全に共有可能
- `Arc`: 参照カウントによるクローン可能

### 5. Transforms (Static Methods Struct)

共通変換関数群を提供する struct。

**Associated Functions** (変換関数):

| Function | Input | Output | Error Condition |
|----------|-------|--------|-----------------|
| `to_float(value: &Value)` | String/Number | `f64` | 変換失敗 |
| `to_int(value: &Value)` | String/Number | `i64` | 変換失敗 |
| `iso_timestamp(value: &Value)` | ISO 8601 String | `String` (UTC) | 無効形式 |
| `unix_timestamp_ms(value: &Value)` | Integer | `String` (UTC ISO 8601) | 負値/変換失敗 |
| `unix_timestamp_sec(value: &Value)` | Integer | `String` (UTC ISO 8601) | 負値/変換失敗 |
| `jst_to_utc(value: &Value)` | JST String | `String` (UTC ISO 8601) | 無効形式 |
| `side_from_string(value: &Value)` | String | `"buy"` or `"sell"` | 未知の値 |
| `uppercase(value: &Value)` | String | `String` | 非文字列 |
| `lowercase(value: &Value)` | String | `String` | 非文字列 |

**Factory Functions** (`TransformFn` を返す):

| Function | Returns |
|----------|---------|
| `to_float_fn()` | `TransformFn` |
| `to_int_fn()` | `TransformFn` |
| `iso_timestamp_fn()` | `TransformFn` |
| `unix_timestamp_ms_fn()` | `TransformFn` |
| `unix_timestamp_sec_fn()` | `TransformFn` |
| `jst_to_utc_fn()` | `TransformFn` |
| `side_from_string_fn()` | `TransformFn` |
| `uppercase_fn()` | `TransformFn` |
| `lowercase_fn()` | `TransformFn` |

**Constants**:

| Constant | Value | Description |
|----------|-------|-------------|
| `MS_PER_SECOND` | `1000` | 1秒あたりのミリ秒 |
| `JST_UTC_OFFSET_HOURS` | `9` | JST の UTC からのオフセット |

### 6. ModelMapping

フィールドマッピング定義 struct。

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `target_field` | `String` | - | ターゲットモデルのフィールド名 |
| `source_field` | `String` | - | ソースデータのフィールドパス（ドット記法対応） |
| `transform` | `Option<TransformFn>` | `None` | 値の変換関数 |
| `default` | `Option<Value>` | `None` | ソース値が存在しない場合のデフォルト値 |
| `required` | `bool` | `true` | 必須フィールドかどうか |

**Traits**:
- `Clone`: すべてのフィールドが Clone 可能

**Constructor**:
- `new(target: &str, source: &str) -> Self`: `required=true` で初期化

**Builder Methods** (consuming self):

| Method | Parameter | Description |
|--------|-----------|-------------|
| `with_transform(self, f: TransformFn)` | `TransformFn` | transform を設定 |
| `with_default(self, value: Value)` | `Value` | デフォルト値を設定 |
| `optional(self)` | - | `required=false` に設定 |

**Core Method**:

```rust
pub fn apply(&self, source_data: &Value) -> Result<Value, MappingError>
```

**apply() Logic**:
1. `source_field` パスでソースデータから値を取得（ドット記法対応）
2. 値が null/None の場合:
   - `default` が設定されていればデフォルト値を返す
   - `required=true` なら `Err(MappingError)` を返す
   - `required=false` なら `Value::Null` を返す
3. `transform` が設定されていれば変換を適用
4. 変換後の値を返す

### 7. AdapterFactory (Type Alias)

```rust
pub type AdapterFactory = Arc<dyn Fn() -> Box<dyn BaseAdapter> + Send + Sync>;
```

### 8. BaseAdapter (Trait)

アダプターの基底 trait。

**Required Methods**:

```rust
fn source_name(&self) -> &'static str;
```

**Provided Methods** (デフォルト実装):

```rust
fn get_quote_mapping(&self) -> Vec<ModelMapping> { vec![] }
fn get_ohlcv_mapping(&self) -> Vec<ModelMapping> { vec![] }
fn get_trade_mapping(&self) -> Vec<ModelMapping> { vec![] }
fn get_orderbook_mapping(&self) -> Vec<ModelMapping> { vec![] }
fn get_instrument_mapping(&self) -> Vec<ModelMapping> { vec![] }
```

**Trait Bounds**:
- `Send + Sync`: スレッドセーフ

**Object Safety**:
- `dyn BaseAdapter` として使用可能

### 9. AdapterRegistry (Singleton)

スレッドセーフなグローバルレジストリ。

**Internal State**:
```rust
static REGISTRY: Lazy<RwLock<HashMap<String, AdapterFactory>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
```

**Public Functions**:

| Function | Signature | Description |
|----------|-----------|-------------|
| `register` | `register<F>(source_name: &str, factory: F) -> Result<(), AdapterError>` | アダプターを登録 |
| `get` | `get(source_name: &str) -> Option<Box<dyn BaseAdapter>>` | アダプターを取得 |
| `list_adapters` | `list_adapters() -> Vec<String>` | 登録済み名を取得 |
| `is_registered` | `is_registered(source_name: &str) -> bool` | 登録済みか確認 |
| `clear` | `clear()` | 全登録を解除（テスト用） |

**Error Conditions**:
- `register()`: 重複登録で `Err(AdapterError::DuplicateRegistration)`
- `get()`: 未登録で `None`

## Relationships

```
AdapterRegistry 1:N AdapterFactory
                    (stores factories by source_name)

AdapterFactory 1:1 BaseAdapter
                   (factory creates adapter instance)

BaseAdapter 1:N ModelMapping
                (each mapping method returns Vec<ModelMapping>)

ModelMapping 0:1 TransformFn
                 (optional transform function)

Transforms --provides--> TransformFn
                         (factory methods return TransformFn)
```

## State Transitions

### AdapterRegistry States

```
           register()
     ┌────────────────┐
     ▼                │
┌─────────┐     ┌─────────────┐
│  Empty  │────▶│ Has Entries │──┐
└─────────┘     └─────────────┘  │
     ▲                │          │ register() more
     │   clear()      │          │
     └────────────────┴──────────┘
```

### ModelMapping.apply() Flow

```
    ┌───────────────────┐
    │  Get source value │
    │  by source_field  │
    └─────────┬─────────┘
              │
              ▼
    ┌───────────────────┐
    │   Value found?    │──No──┬──────────────────────┐
    └─────────┬─────────┘      │                      │
              │ Yes            ▼                      ▼
              │        ┌──────────────┐      ┌──────────────┐
              │        │ Has default? │──Yes─│Return default│
              │        └──────┬───────┘      └──────────────┘
              │               │ No
              │               ▼
              │        ┌──────────────┐      ┌──────────────┐
              │        │  required?   │──Yes─│ MappingError │
              │        └──────┬───────┘      └──────────────┘
              │               │ No
              │               ▼
              │        ┌──────────────┐
              │        │ Return Null  │
              │        └──────────────┘
              ▼
    ┌───────────────────┐
    │  Has transform?   │──No──┐
    └─────────┬─────────┘      │
              │ Yes            │
              ▼                │
    ┌───────────────────┐      │
    │  Apply transform  │      │
    └─────────┬─────────┘      │
              │                │
              ▼                ▼
    ┌───────────────────────────┐
    │      Return value         │
    └───────────────────────────┘
```
