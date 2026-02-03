# Python 実装仕様

**Parent Spec**: [002-data-model](../spec.md)
**Status**: Active

## 概要

JSON Schema から Python pydantic v2 モデルを生成するための仕様。

## コード生成ツール

### datamodel-code-generator

- **ツール**: [datamodel-code-generator](https://github.com/koxudaxi/datamodel-code-generator)
- **バージョン**: 最新版を推奨

### 必須オプション

| オプション | 値 | 説明 |
|-----------|-----|------|
| `--output-model-type` | `pydantic_v2.BaseModel` | pydantic v2 モデル出力 |
| `--use-annotated` | - | `typing.Annotated` を使用 |
| `--field-constraints` | - | min/max 制約を含める |
| `--use-standard-collections` | - | `list`, `dict` を使用（`List`, `Dict` ではなく）|
| `--use-union-operator` | - | `X | Y` 構文を使用 |
| `--snake-case-field` | - | フィールド名を snake_case に変換 |
| `--use-schema-description` | - | スキーマの description を docstring に |
| `--use-field-description` | - | フィールドの description を Field に |
| `--reuse-model` | - | 同一内容のモデルを再利用 |
| `--disable-timestamp` | - | 生成日時コメントを無効化 |

### 推奨オプション

| オプション | 値 | 説明 |
|-----------|-----|------|
| `--target-python-version` | `3.13` | ターゲット Python バージョン |
| `--reuse-scope` | `tree` | ファイル横断的な重複排除 |
| `--strict-types` | `str int float bool` | 厳格な型チェック |

## pydantic v2 設定

### unevaluatedProperties マッピング

JSON Schema の `unevaluatedProperties` は pydantic の `model_config` にマッピングされる:

| JSON Schema | pydantic v2 |
|------------|-------------|
| `unevaluatedProperties: false` | `model_config = ConfigDict(extra='forbid')` |
| `unevaluatedProperties: true` | `model_config = ConfigDict(extra='allow')` |

### Nullable フィールド

```python
# JSON Schema: {"type": ["string", "null"]}
field: str | None = None
```

## 型チェック

### mypy

生成されたコードは mypy で型チェックを通過しなければならない。

```bash
uv run mypy src/marketschema/models/
```

### 期待される結果

- エラー: 0件
- 警告: 許容（ただし `--strict` モードでの警告削減を推奨）

## 注意事項

### $ref の解決

`$ref` は相対パスで指定すること。絶対 URL を使用すると httpx モジュールが必要になる。

```json
// OK
{"$ref": "definitions.json#/$defs/Symbol"}

// NG（httpx が必要）
{"$ref": "https://example.com/schemas/definitions.json#/$defs/Symbol"}
```

### 自動フォーマット

生成後に ruff でフォーマットすることを推奨:

```bash
uv run ruff format src/marketschema/models/
uv run ruff check --fix src/marketschema/models/
```

## 実行手順

実際のコマンド実行手順は [docs/code-generation.md](../../../docs/code-generation.md) を参照。
