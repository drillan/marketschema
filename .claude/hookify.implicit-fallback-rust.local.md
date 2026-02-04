---
name: warn-implicit-fallback-rust
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.rs$
  - field: content
    operator: regex_match
    pattern: \.ok\(\)\??|Err\(_\)|unwrap_or_default\(\)|unwrap_or\(|\.unwrap_or_else\(\|_\||let\s+_\s*=\s*\w+.*\.(set|send|await|try_|insert|push)
---

## CLAUDE.md 違反: 暗黙的フォールバック検出

このコードは **暗黙的フォールバック禁止** ルール (CLAUDE.md) に違反している可能性があります。

### 検出されたパターン

検出されたパターンが以下のいずれかに該当しないか確認してください:

| パターン | 問題 |
|---------|------|
| `.ok()` / `.ok()?` | `Result` のエラー情報を破棄して `Option` に変換 |
| `Err(_)` | match文でエラー情報を無視 |
| `unwrap_or_default()` | エラー時にデフォルト値で処理（エラー情報消失）|
| `unwrap_or(...)` | エラー時に固定値で処理（エラー情報消失）|
| `unwrap_or_else(\|_\| ...)` | エラー情報を使用せずにフォールバック |
| `let _ = x.set(...)` | `OnceLock::set()` 等の `Result` を無視 |
| `let _ = x.send(...)` | チャンネル送信の `Result` を無視 |
| `let _ = x.await` | async操作の結果を完全に無視 |

### 正しい対処法

```rust
// NG: エラー情報が消失
let body = response.text().await.ok();

// OK: エラーを明示的に処理
let body = match response.text().await {
    Ok(text) => Some(text),
    Err(e) => {
        tracing::warn!(error = %e, "Failed to read response body");
        None
    }
};
```

```rust
// NG: エラー情報が消失
match result {
    Ok(v) => v,
    Err(_) => return default,
}

// OK: エラー情報を保持
match result {
    Ok(v) => v,
    Err(e) => {
        tracing::warn!(error = %e, "Operation failed");
        return default;
    }
}
```

```rust
// NG: OnceLock::set() の結果を無視
let _ = once_lock.set(value);

// OK: 失敗時にパニック（テストコード）
once_lock.set(value).expect("OnceLock should be empty");

// OK: 失敗時に適切に処理
if once_lock.set(value).is_err() {
    tracing::warn!("OnceLock was already initialized");
}
```

### 例外ケース

以下の場合は許容される可能性があります:
- テストコード内でのエラー無視
- エラーが発生し得ないことが明らかな場合（要コメント）
- 明示的なログ出力が直前にある場合

この警告が誤検知の場合は、コメントでその理由を説明してください。
