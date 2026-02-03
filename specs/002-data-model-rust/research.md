# Research: Rust Data Model Implementation

**Feature Branch**: `002-data-model-rust`
**Date**: 2026-02-03

## Overview

JSON Schema から Rust struct を自動生成するための調査結果。cargo-typify の制限事項と回避策を中心に文書化する。

## Research Tasks

### 1. typify の unevaluatedProperties サポート状況

**Status**: NOT SUPPORTED

- typify は Draft 2020-12 の `unevaluatedProperties` キーワードをサポートしていない
- [typify issue #579](https://github.com/oxidecomputer/typify/issues/579) で追跡中
- `unevaluatedProperties` が存在しても無視され、生成された struct は未知のフィールドを許容してしまう

**決定**: バンドル時に `unevaluatedProperties` → `additionalProperties` 変換を実施（ADR-001 採用）

### 2. additionalProperties: false の扱い

- typify は `"additionalProperties": false` を検出すると `#[serde(deny_unknown_fields)]` を生成
- これにより、デシリアライズ時に未定義フィールドがあるとエラーになる
- `unevaluatedProperties` 変換により、同等の保護を実現可能

```rust
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // additionalProperties: false から自動生成
struct MyType {
    field1: String,
}
```

### 3. Draft 2020-12 機能の制限

| 機能 | サポート | 備考 |
|------|---------|------|
| unevaluatedProperties | ❌ | バンドル時変換で回避 |
| anyOf | △ | struct + optional fields として生成、精度に課題 |
| if/then/else | ❌ | issue #927 で明示的に未サポート |
| not | ❌ | issue #954 |
| allOf | ○ | スキーママージで対応 |
| oneOf | △ | enum として生成、複雑なケースで問題あり |
| $ref | ○ | バンドル前提で解決 |

**推奨**: 複雑な条件分岐（if/then/else）は避け、シンプルなスキーマ構成を維持する。

### 4. typify の型マッピング

| JSON Schema | Rust Type |
|-------------|-----------|
| string | String |
| number | f64 |
| integer | i64 |
| boolean | bool |
| array | Vec<T> |
| null + 他の型 | Option<T> |
| format: date-time | chrono::DateTime<Utc> |
| pattern | regress による検証 |
| minLength/maxLength | Newtype + FromStr で検証 |

### 5. regress クレートの必要性

- JSON Schema の `pattern` キーワードを使用する場合、regress クレートが必要
- regress は ES 2018 互換の正規表現エンジン（backreferences、lookaround 対応）
- 標準の `regex` クレートより遅いがより多くのパターンをサポート

**依存関係**:
```toml
regress = "0.10"
```

### 6. 生成コードの特徴

- `#[derive(Serialize, Deserialize, Clone, Debug)]` が自動付与
- Builder パターンでの構築をサポート
- minLength/pattern 制約のある string は Newtype として生成
- optional フィールドは `Option<T>` + `#[serde(default)]`

### 7. x-rust-type 拡張

- 既存の Rust 型を再利用するための拡張キーワード
- 本プロジェクトでは使用しない（スキーマから完全生成）

## Decisions

### Decision 1: バンドル時スキーマ変換

**Decision**: バンドルプロセスで `unevaluatedProperties` を `additionalProperties` に変換する

**Rationale**:
1. 元のスキーマは Draft 2020-12 準拠を維持
2. typify が対応した時点で変換を削除するだけで移行完了
3. バンドル後は $ref が解決済みなので意味的に等価

**Alternatives Considered**:
- syn AST 後処理 → 追加依存が必要
- sed/正規表現 → 脆弱性あり
- スキーマ変更 → Draft 2020-12 逸脱

### Decision 2: 生成コード配置先

**Decision**: `rust/src/types/` に配置

**Rationale**:
- モジュール構造として整理
- lib.rs から再エクスポートで使いやすさを確保

### Decision 3: テスト戦略

**Decision**: デシリアライズテストを `rust/tests/types_test.rs` に配置

**Rationale**:
- 正常系: 各 struct × 3 ケース以上
- 異常系: 必須フィールド欠落、型不一致 × 2 ケース以上
- ラウンドトリップ: serialize → deserialize の往復確認

## References

- [typify GitHub](https://github.com/oxidecomputer/typify)
- [typify docs.rs](https://docs.rs/typify/latest/typify/)
- [regress docs.rs](https://docs.rs/regress/latest/regress/)
- [Rust and JSON Schema: odd couple or perfect strangers](https://ahl.dtrace.org/2024/01/22/rust-and-json-schema/)
- [ADR-001: unevaluatedProperties 対応策](../../docs/adr/codegen/001-unevaluated-properties-workaround.md)
