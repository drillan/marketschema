# ADR-001: typify の unevaluatedProperties 未対応への対応策

**Status**: Accepted
**Date**: 2026-02-03
**Issue**: [#39](https://github.com/drillan/marketschema/issues/39)

## Context

JSON Schema Draft 2020-12 では `unevaluatedProperties: false` を使用して未定義フィールドを禁止できる。これは `additionalProperties` と異なり、`$ref` や `allOf` を横断してプロパティを認識できる。

本プロジェクトでは FR-010（002-data-model/spec.md）により、すべてのスキーマで `unevaluatedProperties: false` を指定している。

Rust コード生成ツール typify は `additionalProperties: false` を検出すると `#[serde(deny_unknown_fields)]` を生成するが、`unevaluatedProperties` は**サポートしていない**（[typify issue #579](https://github.com/oxidecomputer/typify/issues/579)）。

結果として、生成された Rust struct はデシリアライズ時に未知のフィールドを無視してしまい、FR-010 に違反する。

## Decision Drivers

- FR-010 準拠: 未定義フィールドを許容しない
- Draft 2020-12 準拠: 元のスキーマは標準に従う
- 保守性: typify が将来対応した場合の移行容易性
- 堅牢性: 変換処理の信頼性

## Considered Options

### Option A: バンドル時のスキーマ変換（採用）

バンドルプロセスで `$ref` を解決した後、`unevaluatedProperties` を `additionalProperties` に変換する。

```bash
npx json-refs resolve "$schema" | \
    jq 'walk(if type == "object" and has("unevaluatedProperties")
        then .additionalProperties = .unevaluatedProperties | del(.unevaluatedProperties)
        else . end)' > "$OUTPUT"
```

| 評価軸 | 評価 |
|--------|------|
| 堅牢性 | ◎ 変換ロジックがシンプル（キー名の置換のみ） |
| 柔軟性 | ◎ 元のスキーマは Draft 2020-12 準拠を維持 |
| 正確性 | ◎ バンドル後は $ref 解決済みなので意味的に等価 |
| 保守性 | ◎ typify 対応後は変換を削除するだけ |

### Option B: syn AST 後処理

typify 生成後に Rust の `syn` クレートで AST を解析し、struct に `#[serde(deny_unknown_fields)]` を追加する。

| 評価軸 | 評価 |
|--------|------|
| 堅牢性 | ◎ AST 操作なので正規表現より安全 |
| 柔軟性 | ○ Rust コードにのみ影響 |
| 正確性 | ◎ 正確に属性を追加できる |
| 保守性 | △ Rust 依存ツールの追加が必要 |

### Option C: sed/正規表現による後処理

生成された Rust コードを正規表現で書き換える。

| 評価軸 | 評価 |
|--------|------|
| 堅牢性 | △ 複雑なパターンで誤変換の可能性 |
| 柔軟性 | △ パターン変更時に修正が必要 |
| 正確性 | △ エッジケースで失敗する可能性 |
| 保守性 | ○ 追加依存なし |

### Option D: スキーマを additionalProperties に変更

元のスキーマで `unevaluatedProperties` の代わりに `additionalProperties` を使用する。

| 評価軸 | 評価 |
|--------|------|
| 堅牢性 | ○ typify が直接対応 |
| 柔軟性 | × Draft 2020-12 のベストプラクティスから逸脱 |
| 正確性 | △ $ref を含むスキーマで挙動が異なる |
| 保守性 | × 元のスキーマを変更する必要がある |

### Option E: typify の対応を待つ

typify issue #579 の対応を待つ。

| 評価軸 | 評価 |
|--------|------|
| 堅牢性 | - 対応時期不明 |
| 柔軟性 | - 対応時期不明 |
| 正確性 | - 対応時期不明 |
| 保守性 | - 対応時期不明 |

## Decision

**Option A: バンドル時のスキーマ変換**を採用する。

### 理由

1. **意味的等価性**: バンドル後のスキーマでは `$ref` が解決されているため、`additionalProperties` と `unevaluatedProperties` の挙動は実質的に同じになる

2. **関心の分離**:
   - 元のスキーマ（`schemas/`）: Draft 2020-12 準拠
   - バンドル済みスキーマ（`rust/bundled/`）: typify 互換

3. **変更の局所化**: `scripts/bundle_schemas.sh` のみ修正すればよい

4. **移行容易性**: typify が `unevaluatedProperties` に対応した場合、jq 変換を削除するだけで移行完了

## Consequences

### Positive

- FR-010 準拠を達成
- 元のスキーマは変更不要
- 他の言語（Python）への影響なし
- JSON Schema 検証は元のスキーマで実行可能

### Negative

- バンドルスクリプトに jq 依存が追加される
- バンドル済みスキーマと元のスキーマで差異が生じる（ドキュメント化が必要）

### Neutral

- typify が対応するまでの暫定措置

## References

- [typify issue #579: The Big Plan for 2020-12 support](https://github.com/oxidecomputer/typify/issues/579)
- [Rust and JSON Schema: odd couple or perfect strangers](https://ahl.dtrace.org/2024/01/22/rust-and-json-schema/) - typify 作者による設計思想
- [Learn JSON Schema - unevaluatedProperties](https://www.learnjsonschema.com/2020-12/unevaluated/unevaluatedproperties/)
- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12)

## Related

- [002-data-model/spec.md](../../../specs/002-data-model/spec.md) - FR-010
- [002-data-model-rust/spec.md](../../../specs/002-data-model-rust/spec.md) - FR-R002
