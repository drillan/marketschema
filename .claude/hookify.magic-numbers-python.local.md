---
name: warn-magic-numbers-python
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.py$
  - field: content
    operator: regex_match
    pattern: (==|!=|>=|<=|>|<)\s*(4\d{2}|5\d{2})\b|status_code\s*(==|!=)\s*\d{3}|/\s*1000\b|\*\s*1000\b|timeout\s*=\s*\d{2,}|sleep\(\s*\d{2,}\s*\)
---

## CLAUDE.md 違反の可能性: マジックナンバー検出

このコードは **ハードコード禁止** ルール (CLAUDE.md) に違反している可能性があります。

### 検出されたパターン

エラー系ステータスコード (4xx/5xx) を重点的に検出します。2xx/3xx は正常系のため対象外です。

| パターン | 問題 |
|---------|------|
| `== 429`, `== 500` など | エラー系ステータスコード (4xx/5xx) のハードコード |
| `status_code == 200` | `status_code` との比較は全3桁数字を検出 |
| `/ 1000`, `* 1000` | 時間変換のマジックナンバー |
| `timeout=30` | タイムアウト値のハードコード（2桁以上） |
| `sleep(60)` | スリープ時間のハードコード（2桁以上） |

### CLAUDE.md の規定

```python
# NG
if response.status_code == 429:
    raise RateLimitError()
return ms / 1000

# OK
HTTP_STATUS_TOO_MANY_REQUESTS = 429
MS_PER_SECOND = 1000

if response.status_code == HTTP_STATUS_TOO_MANY_REQUESTS:
    raise RateLimitError()
return ms / MS_PER_SECOND
```

### 正しい対処法

```python
# NG: HTTPステータスコードのハードコード
if response.status_code == 429:
    raise RateLimitError()

# OK: 定数を使用
HTTP_STATUS_TOO_MANY_REQUESTS = 429
if response.status_code == HTTP_STATUS_TOO_MANY_REQUESTS:
    raise RateLimitError()

# OK: http モジュールを使用
from http import HTTPStatus
if response.status_code == HTTPStatus.TOO_MANY_REQUESTS:
    raise RateLimitError()
```

```python
# NG: 時間変換のマジックナンバー
seconds = milliseconds / 1000

# OK: 定数を使用
MS_PER_SECOND = 1000
seconds = milliseconds / MS_PER_SECOND
```

```python
# NG: タイムアウト値のハードコード
response = client.get(url, timeout=30)

# OK: 定数を使用
DEFAULT_TIMEOUT_SECS = 30
response = client.get(url, timeout=DEFAULT_TIMEOUT_SECS)
```

### 例外ケース

以下の場合は許容される可能性があります:
- テストコード内での一時的な値
- 定数定義自体（`HTTP_STATUS_OK = 200`）
- 明確なコメントがある場合

この警告が誤検知の場合は、定数化するか、コメントでその理由を説明してください。
