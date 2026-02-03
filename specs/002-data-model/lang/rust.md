# Rust 実装仕様

**Parent Spec**: [002-data-model](../spec.md)
**Status**: Active

## 概要

JSON Schema から Rust struct を生成するための仕様。

## コード生成ツール

### cargo-typify

- **ツール**: [typify](https://github.com/oxidecomputer/typify)
- **インストール**: `cargo install cargo-typify`

## 制限事項

### 外部 $ref の解決

typify は外部 `$ref` の解決に制限がある。事前にスキーマをバンドルする必要がある。

### Draft 2020-12 サポート

Draft 2020-12 の明示的サポートがないため、互換性のために `$defs` と `definitions` の両方を定義することを推奨。

### unevaluatedProperties のサポート

typify は JSON Schema Draft 2020-12 の `unevaluatedProperties` をサポートしていません。
スキーマで `unevaluatedProperties: false` を指定しても、生成される Rust コードには
`#[serde(deny_unknown_fields)]` が付与されません。

この問題は issue #39 で追跡しています。

現在の挙動:
- デシリアライズ時に未知のフィールドは無視される
- FR-010 の厳密な準拠は Rust 実装では保証されない

### サポート制限

以下の JSON Schema 機能はサポートが限定的:
- `anyOf`
- `if/then/else`

## スキーマバンドリング

### 必要性

typify は自己完結型のスキーマを要求する。`json-refs` ツールで全ての `$ref` を解決してインライン化する。

### バンドリングツール

```bash
# json-refs のインストール
npm install json-refs

# バンドル実行
npx json-refs resolve schema.json > bundled-schema.json
```

## 自動生成される serde 属性

| 属性 | 条件 |
|-----|------|
| `#[derive(Serialize, Deserialize, Debug, Clone)]` | すべての型 |
| `#[serde(default)]` | `required` に含まれないプロパティ |
| `#[serde(deny_unknown_fields)]` | 手動追加が必要（issue #39 参照） |

## 型チェック

### cargo check

生成されたコードは cargo check でコンパイルエラーがないことを確認する。

```bash
cargo check
```

### 期待される結果

- コンパイルエラー: 0件
- 警告: 許容（ただし削減を推奨）

## 注意事項

### regress クレート

スキーマのパターンバリデーションに `regress` クレートが必要な場合がある。

```toml
# Cargo.toml
[dependencies]
regress = "0.10"
```

### Symbol 型の重複

複数のスキーマで `Symbol` 型が定義されている場合、バンドル後に型名が重複する可能性がある。必要に応じてリネームや型エイリアスで対応する。

## 推奨ワークフロー

1. スキーマをバンドル
2. Rust コード生成
3. cargo check でコンパイル確認
4. 必要に応じて手動調整

## 実行手順

実際のコマンド実行手順は [docs/code-generation.md](../../../docs/code-generation.md) を参照。
