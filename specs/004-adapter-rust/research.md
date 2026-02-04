# Research: Adapter Interface Rust Implementation

**Feature Branch**: `004-adapter-rust`
**Date**: 2026-02-04

## 1. async_trait のベストプラクティス（Rust 2024）

### Decision
**動的ディスパッチ（`dyn BaseAdapter`）が必要なため `async_trait` クレートを使用する**

### Rationale
- Rust 1.75 で `async fn` in traits が安定化されたが、現時点で **dyn-safe ではない**
- AdapterRegistry は `Box<dyn BaseAdapter>` を返す必要があり、動的ディスパッチが必須
- `async_trait` は `Box<dyn Future>` によるヒープアロケーションを導入するが、HTTP リクエストの IO バウンドな処理では許容範囲

### Alternatives Considered

| オプション | 利点 | 欠点 | 採用 |
|----------|------|------|-----|
| `async_trait` クレート | dyn-safe、安定 | ヒープアロケーション | ✅ |
| ネイティブ async traits | ゼロコスト | dyn-safe ではない | ❌ |
| `trait-variant` クレート | Send バウンド生成 | dyn-safe ではない | ❌ |

---

## 2. グローバルシングルトンパターン

### Decision
**MSRV 1.71.1 を維持し `once_cell::sync::Lazy` を使用する**

### Rationale
- `std::sync::LazyLock` は Rust 1.80 以降で利用可能だが、現 MSRV は 1.71.1（moka 0.12 要求）
- `once_cell::sync::Lazy` は既に広く使用されており、将来の std への移行も容易
- テスト干渉を防ぐため `clear()` メソッドを提供

### Alternatives Considered

| オプション | MSRV | 採用 |
|----------|------|-----|
| `std::sync::LazyLock` | 1.80 | ❌（MSRV 上げ必要） |
| `once_cell::sync::Lazy` | 1.56 | ✅ |
| `lazy_static!` | 低い | ❌（マクロベース、非推奨） |

### テスト分離パターン

```rust
// テスト間でグローバル状態をクリア
#[cfg(test)]
fn setup() {
    AdapterRegistry::clear();
}
```

---

## 3. Transform 関数の型パターン

### Decision
**`Arc<dyn Fn(&Value) -> Result<Value, TransformError> + Send + Sync>` を使用する**

### Rationale
- `ModelMapping` は `Clone` を実装する必要があり、transform 関数も共有可能でなければならない
- `Arc<dyn Fn>` は参照カウントによりクローン可能
- `Send + Sync` はマルチスレッド環境で AdapterRegistry を安全に使用するために必須

### 型定義

```rust
/// Transform 関数の型エイリアス
pub type TransformFn = Arc<dyn Fn(&Value) -> Result<Value, TransformError> + Send + Sync>;
```

### Alternatives Considered

| 型 | Clone 可能 | スレッドセーフ | 採用 |
|---|-----------|--------------|-----|
| `Arc<dyn Fn + Send + Sync>` | ✅ | ✅ | ✅ |
| `Box<dyn Fn + Send>` | ❌ | ⚠️ | ❌ |
| `fn(...)` | ✅ | ✅ | ❌（キャプチャ不可） |

---

## 4. ビルダーパターン

### Decision
**Consuming self（`mut self`）パターンを使用する**

### Rationale
- 既存の `AsyncHttpClientBuilder` と一貫したスタイル
- メソッドチェーンが自然
- `#[must_use]` 属性で未使用の builder を警告

### 実装パターン

```rust
pub struct ModelMappingBuilder {
    target_field: String,
    source_field: String,
    transform: Option<TransformFn>,
    default: Option<Value>,
    required: bool,
}

impl ModelMappingBuilder {
    pub fn new(target: &str, source: &str) -> Self { ... }

    #[must_use]
    pub fn with_transform(mut self, f: TransformFn) -> Self {
        self.transform = Some(f);
        self
    }

    pub fn build(self) -> ModelMapping { ... }
}
```

---

## 5. JSON パスナビゲーション

### Decision
**`serde_json::Value::pointer` メソッドを使用し、ドット記法から JSON Pointer 形式に変換する**

### Rationale
- 外部依存なしで実装可能
- RFC 6901 JSON Pointer 準拠
- `Option` を返すため、CLAUDE.md の「暗黙的フォールバック禁止」に準拠

### 実装パターン

```rust
/// ドット記法を JSON Pointer 形式に変換
/// "price.bid" -> "/price/bid"
fn dot_to_pointer(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    format!("/{}", path.replace('.', "/"))
}

/// ネストされた値を取得
fn get_nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let pointer = dot_to_pointer(path);
    value.pointer(&pointer)
}
```

### Alternatives Considered

| 方法 | 依存 | 採用 |
|-----|------|-----|
| `Value::pointer` + 変換 | なし | ✅ |
| `json_dotpath` クレート | あり | ❌ |
| インデックスアクセス | なし | ❌（Null を返す、エラー不明確） |

---

## 6. エラー型設計

### Decision
**`thiserror` 2.0 を使用し、`#[from]` と `#[source]` でエラーチェインを構成する**

### Rationale
- 既存の `marketschema-http` クレートと同じパターン
- `std::error::Error::source()` でエラーの原因にアクセス可能
- エラーメッセージに入力値を含める（デバッグ支援）

### エラー階層

```rust
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("General adapter error: {0}")]
    General(String),

    #[error("Duplicate registration: adapter '{0}' already registered")]
    DuplicateRegistration(String),

    #[error(transparent)]
    Mapping(#[from] MappingError),

    #[error(transparent)]
    Transform(#[from] TransformError),
}

#[derive(Debug, Error)]
#[error("Mapping error: {message}")]
pub struct MappingError {
    pub message: String,
}

#[derive(Debug, Error)]
#[error("Transform error: {message}")]
pub struct TransformError {
    pub message: String,
}
```

---

## まとめ

| トピック | 決定 | 理由 |
|---------|------|------|
| async_trait | `async_trait` クレート使用 | dyn-safe 必須 |
| グローバル状態 | `once_cell::sync::Lazy` | MSRV 1.71.1 維持 |
| Transform 関数 | `Arc<dyn Fn + Send + Sync>` | Clone + スレッドセーフ |
| ビルダーパターン | Consuming self | 既存コードとの一貫性 |
| JSON パス | `Value::pointer` + 変換 | 依存なし、明示的エラー |
| エラー型 | `thiserror` 2.0 | 既存パターン継承 |
