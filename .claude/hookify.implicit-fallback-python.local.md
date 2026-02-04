---
name: warn-implicit-fallback-python
enabled: true
event: file
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.py$
  - field: new_text
    operator: regex_match
    pattern: except.*:\s*(return|pass|continue|\.{3})
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

### 例外ケース

以下の場合は許容される可能性があります:
- テストコード内での意図的なエラー無視
- クリーンアップ処理での例外無視（要コメント）
- 明示的な型指定がある `except SpecificError` の場合

この警告が誤検知の場合は、コメントでその理由を説明してください。
