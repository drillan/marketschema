---
name: warn-magic-numbers-rust
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.rs$
  - field: content
    operator: regex_match
    pattern: (==|!=|>=|<=|>|<)\s*(4\d{2}|5\d{2})\b|matches!\s*\([^,]+,\s*\d{3}|/\s*1000\b|\*\s*1000\b|Duration::from_secs\(\d{2,}\)|Duration::from_millis\(\d{4,}\)
---

## CLAUDE.md 違反の可能性: マジックナンバー検出

このコードは **ハードコード禁止** ルール (CLAUDE.md) に違反している可能性があります。

### 検出されたパターン

エラー系ステータスコード (4xx/5xx) を重点的に検出します。2xx/3xx は正常系のため対象外です。

| パターン | 問題 |
|---------|------|
| `== 429`, `== 500` など | エラー系ステータスコード (4xx/5xx) のハードコード |
| `matches!(x, 500)` | matches! マクロでの3桁数字 |
| `/ 1000`, `* 1000` | 時間変換のマジックナンバー |
| `Duration::from_secs(10)` | 10秒以上のタイムアウト値（2桁以上） |
| `Duration::from_millis(1000)` | 1000ミリ秒以上の値（4桁以上） |

### CLAUDE.md の規定

```rust
// NG
if status_code == 429 { ... }
matches!(status_code, 500 | 502 | 503 | 504)
return ms / 1000;

// OK
const HTTP_STATUS_TOO_MANY_REQUESTS: u16 = 429;
const HTTP_STATUS_INTERNAL_SERVER_ERROR: u16 = 500;
const MS_PER_SECOND: u64 = 1000;

if status_code == HTTP_STATUS_TOO_MANY_REQUESTS { ... }
return ms / MS_PER_SECOND;
```

### 正しい対処法

```rust
// NG: HTTPステータスコードのハードコード
if status_code == 429 {
    return Err(RateLimitError);
}

// OK: 定数を使用
const HTTP_STATUS_TOO_MANY_REQUESTS: u16 = 429;
if status_code == HTTP_STATUS_TOO_MANY_REQUESTS {
    return Err(RateLimitError);
}
```

```rust
// NG: 時間変換のマジックナンバー
let seconds = milliseconds / 1000;

// OK: 定数を使用
const MS_PER_SECOND: u64 = 1000;
let seconds = milliseconds / MS_PER_SECOND;
```

```rust
// NG: タイムアウト値のハードコード
let timeout = Duration::from_secs(30);

// OK: 定数を使用
const DEFAULT_TIMEOUT_SECS: u64 = 30;
let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
```

### 例外ケース

以下の場合は許容される可能性があります:
- テストコード内での一時的な値
- 定数定義自体（`const X: u16 = 429;`）
- 明確なコメントがある場合

この警告が誤検知の場合は、定数化するか、コメントでその理由を説明してください。
