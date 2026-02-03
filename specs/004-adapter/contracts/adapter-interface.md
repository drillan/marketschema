# Adapter Interface Contract

**Feature**: 004-adapter
**Date**: 2026-02-03

## Overview

本ドキュメントはアダプターインターフェースの言語非依存な概念定義を提供する。
各言語での型シグネチャは言語別 spec を参照:
- [004-adapter-python](../../004-adapter-python/contracts/adapter-interface.md)
- [004-adapter-rust](../../004-adapter-rust/contracts/adapter.md)

## BaseAdapter

### Concept

データソースから取得したデータを marketschema の標準モデルに変換するための基底クラス/トレイト。

### Required Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `source_name` | string | データソース識別子（空文字列禁止） |
| `transforms` | Transforms | 変換関数群への参照 |

### Required Methods

#### Mapping Methods

各メソッドは対応するモデルへのフィールドマッピング定義を返す。

| Method | Returns | Description |
|--------|---------|-------------|
| `get_quote_mapping()` | List of ModelMapping | Quote モデルへのマッピング |
| `get_ohlcv_mapping()` | List of ModelMapping | OHLCV モデルへのマッピング |
| `get_trade_mapping()` | List of ModelMapping | Trade モデルへのマッピング |
| `get_orderbook_mapping()` | List of ModelMapping | OrderBook モデルへのマッピング |
| `get_instrument_mapping()` | List of ModelMapping | Instrument モデルへのマッピング |

#### Core Methods

| Method | Description |
|--------|-------------|
| `_apply_mapping(raw_data, mappings, model_class)` | マッピングを適用してモデルインスタンスを生成 |
| `close()` | HTTP クライアントなどのリソースを解放 |

### HTTP Client Integration

- `http_client` プロパティ/フィールド: 遅延初期化された HTTP クライアントを取得
- アダプター内部で作成した場合は `close()` で解放
- 外部から注入された場合は解放しない

### Resource Management Pattern

各言語で適切なリソース管理パターンを実装:
- Python: `async with` (async context manager)
- Rust: `Drop` trait / RAII

---

## ModelMapping

### Concept

ソースフィールドからターゲットフィールドへの変換ルールを定義する不変データ構造。

### Attributes

| Attribute | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `target_field` | string | Yes | - | ターゲットモデルのフィールド名 |
| `source_field` | string | Yes | - | ソースデータのフィールドパス（ドット記法対応） |
| `transform` | function | No | null | 値の変換関数 |
| `default` | any | No | null | ソース値が存在しない場合のデフォルト値 |
| `required` | boolean | No | true | 必須フィールドかどうか |

### Dot Notation

`source_field` はドット記法でネストしたデータにアクセス可能:
- `"price"` → `data["price"]`
- `"ticker.bid"` → `data["ticker"]["bid"]`
- `"level1.level2.value"` → `data["level1"]["level2"]["value"]`

### apply() Method Behavior

1. `source_field` パスでソースデータから値を取得
2. 値が null/None の場合:
   - `default` が設定されていればデフォルト値を返す
   - `required=true` なら MappingError を発生
   - `required=false` なら null/None を返す
3. `transform` が設定されていれば変換を適用
4. 変換後の値を返す

---

## AdapterRegistry

### Concept

アダプタークラスの登録と取得を管理するシングルトン。

### Methods

| Method | Description |
|--------|-------------|
| `register(adapter_class)` | アダプターを登録（デコレータとしても使用可） |
| `get(source_name)` | source_name でアダプターの新規インスタンスを取得 |
| `list_adapters()` | 登録済み source_name のリストを取得 |
| `is_registered(source_name)` | 登録済みかどうかを確認 |
| `clear()` | すべての登録を解除（テスト用） |

### Error Conditions

| Condition | Error |
|-----------|-------|
| 重複登録 | AdapterError |
| 未登録の source_name で get | KeyError |
| source_name が空文字列 | AdapterError |

---

## Exception Types

### Hierarchy

```
MarketSchemaError (base)
├── AdapterError
├── MappingError
└── TransformError
```

### AdapterError

アダプターの初期化や操作に関するエラー。

| Attribute | Type | Description |
|-----------|------|-------------|
| `message` | string | エラーの説明 |

**Trigger Conditions**:
- `source_name` が空文字列
- 重複登録

### MappingError

フィールドマッピング中のエラー。

| Attribute | Type | Description |
|-----------|------|-------------|
| `message` | string | エラーの説明（フィールド名を含む） |

**Trigger Conditions**:
- `required=true` で値が存在しない

### TransformError

値変換中のエラー。

| Attribute | Type | Description |
|-----------|------|-------------|
| `message` | string | エラーの説明（入力値を含む） |

**Trigger Conditions**:
- 変換関数が値を処理できない
- 入力値の型が不正

---

## Type Exports

各言語で以下のエクスポートを提供:

- BaseAdapter
- ModelMapping
- AdapterRegistry
- register (decorator/macro)
- Transforms
