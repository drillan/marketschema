# Quickstart: Adapter Interface Rust Implementation

**Feature Branch**: `004-adapter-rust`
**Date**: 2026-02-04

## Prerequisites

- Rust 1.71.1 以上（MSRV: moka 0.12 要求）
- Cargo

## Installation

`Cargo.toml` に依存関係を追加:

```toml
[dependencies]
marketschema-adapters = { path = "crates/marketschema-adapters" }
```

## Basic Usage

### 1. カスタムアダプターの作成

```rust
use marketschema_adapters::{BaseAdapter, ModelMapping, Transforms};

/// 架空の API 用アダプター
struct MyApiAdapter;

impl BaseAdapter for MyApiAdapter {
    fn source_name(&self) -> &'static str {
        "myapi"
    }

    fn get_quote_mapping(&self) -> Vec<ModelMapping> {
        vec![
            ModelMapping::new("bid", "ticker.bid")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("ask", "ticker.ask")
                .with_transform(Transforms::to_float_fn()),
            ModelMapping::new("timestamp", "ticker.time")
                .with_transform(Transforms::unix_timestamp_ms_fn()),
        ]
    }
}
```

### 2. アダプターの登録と使用

```rust
use marketschema_adapters::AdapterRegistry;
use serde_json::json;

fn main() {
    // アダプターを登録
    AdapterRegistry::register("myapi", || Box::new(MyApiAdapter))
        .expect("Failed to register adapter");

    // 登録済みアダプター一覧を確認
    println!("Registered adapters: {:?}", AdapterRegistry::list_adapters());

    // アダプターを取得してデータを変換
    if let Some(adapter) = AdapterRegistry::get("myapi") {
        let raw_data = json!({
            "ticker": {
                "bid": "100.50",
                "ask": "100.75",
                "time": 1704067200000_i64
            }
        });

        for mapping in adapter.get_quote_mapping() {
            match mapping.apply(&raw_data) {
                Ok(value) => println!("{}: {}", mapping.target_field, value),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
```

### 3. 出力例

```
Registered adapters: ["myapi"]
bid: 100.5
ask: 100.75
timestamp: "2024-01-01T00:00:00Z"
```

## ModelMapping の高度な使用法

### オプショナルフィールドとデフォルト値

```rust
use serde_json::json;

let mappings = vec![
    // 必須フィールド（デフォルト）
    ModelMapping::new("symbol", "ticker.symbol"),

    // オプショナルフィールド
    ModelMapping::new("volume", "ticker.vol")
        .optional(),

    // デフォルト値付きフィールド
    ModelMapping::new("exchange", "ticker.exchange")
        .with_default(json!("unknown"))
        .optional(),
];
```

### ネストされたデータへのアクセス

```rust
let data = json!({
    "level1": {
        "level2": {
            "value": 42
        }
    }
});

let mapping = ModelMapping::new("result", "level1.level2.value");
let value = mapping.apply(&data).unwrap();
assert_eq!(value, json!(42));
```

## Transform 関数一覧

| 関数 | 入力 | 出力 | 説明 |
|-----|------|------|------|
| `to_float_fn()` | String/Number | `f64` | 浮動小数点に変換 |
| `to_int_fn()` | String/Number | `i64` | 整数に変換 |
| `iso_timestamp_fn()` | ISO 8601 String | ISO 8601 UTC String | タイムスタンプを検証・正規化 |
| `unix_timestamp_ms_fn()` | Integer (ms) | ISO 8601 UTC String | Unix ミリ秒から変換 |
| `unix_timestamp_sec_fn()` | Integer (sec) | ISO 8601 UTC String | Unix 秒から変換 |
| `jst_to_utc_fn()` | JST String | ISO 8601 UTC String | JST を UTC に変換 |
| `side_from_string_fn()` | String | `"buy"` or `"sell"` | 売買方向を正規化 |
| `uppercase_fn()` | String | String | 大文字に変換 |
| `lowercase_fn()` | String | String | 小文字に変換 |

## エラーハンドリング

```rust
use marketschema_adapters::{AdapterError, MappingError};

match mapping.apply(&data) {
    Ok(value) => println!("Success: {}", value),
    Err(MappingError { message }) => {
        eprintln!("Mapping failed: {}", message);
    }
}

match AdapterRegistry::register("duplicate", || Box::new(MyApiAdapter)) {
    Ok(()) => println!("Registered"),
    Err(AdapterError::DuplicateRegistration(name)) => {
        eprintln!("Adapter '{}' already exists", name);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## テストでのベストプラクティス

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        // テスト間でレジストリをクリア
        AdapterRegistry::clear();
    }

    #[test]
    fn test_adapter_registration() {
        setup();

        AdapterRegistry::register("test", || Box::new(MyApiAdapter)).unwrap();
        assert!(AdapterRegistry::is_registered("test"));
    }

    #[test]
    fn test_mapping_apply() {
        let mapping = ModelMapping::new("price", "bid")
            .with_transform(Transforms::to_float_fn());

        let data = json!({"bid": "123.45"});
        let result = mapping.apply(&data).unwrap();

        assert_eq!(result, json!(123.45));
    }
}
```

## 次のステップ

- [spec.md](./spec.md) - 詳細な仕様
- [contracts/adapter.md](./contracts/adapter.md) - API コントラクト
- [contracts/transforms.md](./contracts/transforms.md) - 変換関数コントラクト
