---
name: warn-implicit-fallback-python
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.py$
  - field: content
    operator: regex_match
    pattern: except.*:\s*(return|pass|continue|\.{3})|else:\s*(return\s+["'\d\[\{]|return\s+None|\w+\s*=\s*["'\d\[\{]|\w+\s*=\s*None)
---

## CLAUDE.md 違反: 暗黙的フォールバック検出

このコードは **暗黙的フォールバック禁止** ルール (CLAUDE.md) に違反している可能性があります。

### 検出されたパターン

以下のパターンが検出されました:

| パターン | 問題 |
|---------|------|
| `except: return ...` | 例外を握りつぶしてデフォルト値を返す |
| `except: pass` | 例外を完全に無視 |
| `except: continue` | ループ内で例外を無視して続行 |
| `except: ...` | 例外を握りつぶす省略記法 |
| `else: return "..."` | ⚠️ 要確認: else でハードコードされた値を返していないか |
| `else: x = "..."` | ⚠️ 要確認: else でハードコードされた値を代入していないか |

### CLAUDE.md の規定

```python
# NG
except: return 0.0

# OK
except (ValueError, TypeError) as e:
    raise ConversionError(...) from e
```

### 正しい対処法

```python
# NG: エラー情報が消失
try:
    value = parse(data)
except:
    return default_value

# OK: 明示的なエラー処理
try:
    value = parse(data)
except (ValueError, TypeError) as e:
    logger.warning(f"Failed to parse data: {e}")
    raise ParseError("Invalid data format") from e

# OK: 適切なログ出力とリレイズ
try:
    value = parse(data)
except ValueError as e:
    logger.error(f"Parse failed: {e}")
    raise
```

```python
# ⚠️ 要確認: else でハードコードされた値を返す/代入するパターン

# NG: return でハードコード
if day:
    return day
else:
    return "Sunday"  # マジックナンバー

# NG: 代入でハードコード
if day:
    current_day = day
else:
    current_day = "Sunday"  # マジックナンバー

# OK: 定数を使用
DEFAULT_DAY = "Sunday"
if day:
    current_day = day
else:
    current_day = DEFAULT_DAY

# OK: エラーを明示的に処理
if day:
    return day
else:
    raise ValueError("day is required")
```

### 例外ケース

以下の場合は許容される可能性があります:
- テストコード内での意図的なエラー無視
- クリーンアップ処理での例外無視（要コメント）
- 明示的な型指定がある `except SpecificError` の場合

この警告が誤検知の場合は、コメントでその理由を説明してください。
