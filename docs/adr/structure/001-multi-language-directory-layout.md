# ADR-001: 多言語プロジェクトのディレクトリ構造

**Status**: Accepted
**Date**: 2026-02-04
**Issue**: #113

## Context

本プロジェクトは Python と Rust の両方でライブラリを提供するモノレポである。現在のディレクトリ構造は以下のようになっている：

```
marketschema/
├── src/
│   └── marketschema/
│       └── schemas/        # Python実装用スキーマ（相対$ref）
├── rust/
│   ├── src/
│   ├── bundled/            # Rust用バンドル済みスキーマ
│   └── Cargo.toml
├── specs/
│   └── 002-data-model/
│       └── contracts/      # 仕様としてのスキーマ（絶対URI $ref）
├── tests/
└── pyproject.toml
```

この構造には以下の問題がある：

1. **ディレクトリ構造の非対称性**: Python は `src/`、Rust は `rust/` と命名規則が異なる
2. **スキーマの重複**: 同じスキーマが3箇所（contracts、src/schemas、rust/bundled）に存在し、同期の問題がある
3. **単一の真実のソースの不在**: どのスキーマが正式な定義か不明確

### 現在のスキーマの関係

| 場所 | 用途 | `$ref` 形式 |
|------|------|-------------|
| `specs/.../contracts/` | 仕様定義 | 絶対URI |
| `schemas/` | Python実装 | 相対パス |
| `rust/bundled/` | Rust実装 | インライン化（$ref解決済み） |

## Decision Drivers

- **Single Source of Truth**: スキーマは1箇所で管理し、派生は自動生成
- **一貫性**: 言語間で対称的な構造
- **明確性**: 各ディレクトリの目的が名前から推測可能
- **ツールチェーン互換性**: 各言語の標準的なツール（uv/pip, cargo）との互換性維持
- **移行コスト**: 既存の設定ファイル・CI への影響を最小化

## Considered Options

### Option A: 共有スキーマをルートに、言語別ディレクトリに分離（推奨）

```
marketschema/
├── schemas/                # 単一の真実のソース（SSoT）
│   ├── definitions.json
│   ├── quote.json
│   └── ...
├── python/
│   ├── src/
│   │   └── marketschema/
│   │       └── schemas/    # ビルド時に schemas/ からコピー/変換
│   ├── tests/
│   └── pyproject.toml
├── rust/
│   ├── src/
│   ├── bundled/            # ビルド時に schemas/ からバンドル
│   ├── tests/
│   └── Cargo.toml
├── specs/                  # 設計仕様（contracts/ は schemas/ を参照）
└── docs/
```

| 評価軸 | 評価 |
|--------|------|
| SSoT | ◎ スキーマは `schemas/` に一元化 |
| 一貫性 | ◎ 言語ごとに対称的な構造 |
| 明確性 | ◎ ディレクトリ名から内容が明確 |
| ツールチェーン互換性 | ◎ 各言語内は標準構造を維持 |
| 移行コスト | △ 複数の設定変更が必要 |

### Option B: 言語別ディレクトリのみ分離（スキーマは現状維持）

```
marketschema/
├── python/
│   ├── src/
│   │   └── marketschema/
│   │       └── schemas/
│   └── pyproject.toml
├── rust/
│   ├── src/
│   ├── bundled/
│   └── Cargo.toml
├── specs/
│   └── 002-data-model/
│       └── contracts/      # SSoTとして機能
└── docs/
```

| 評価軸 | 評価 |
|--------|------|
| SSoT | ○ specs/contracts がSSoT |
| 一貫性 | ◎ 言語ごとに対称的な構造 |
| 明確性 | ○ スキーマの役割が分散 |
| ツールチェーン互換性 | ◎ 各言語内は標準構造を維持 |
| 移行コスト | ○ Option Aより少ない |

### Option C: 現状維持

```
marketschema/
├── schemas/
├── rust/bundled/
├── specs/.../contracts/
└── ...
```

| 評価軸 | 評価 |
|--------|------|
| SSoT | × 3箇所に分散 |
| 一貫性 | △ 非対称な構造 |
| 明確性 | △ 役割が不明確 |
| ツールチェーン互換性 | ◎ 変更不要 |
| 移行コスト | ◎ 変更不要 |

## Decision

**Option A: 共有スキーマをルートに、言語別ディレクトリに分離**を採用する。

### 理由

1. **Single Source of Truth**: `schemas/` ディレクトリがスキーマの正式な定義となり、各言語はここから派生
   - `specs/contracts/` は `schemas/` へのシンボリックリンクまたは参照に変更

2. **対称性**: 言語名によるディレクトリ分離は直感的で理解しやすい

3. **ビルドパイプラインの明確化**:
   ```
   schemas/ (SSoT)
      ├──[コピー/変換]──> python/schemas/
      └──[バンドル]─────> rust/bundled/
   ```

4. **拡張性**: 将来 TypeScript などを追加する場合、同じパターンで対応可能

## Implementation Plan

### Phase 1: スキーマの一元化

```bash
# 1. ルートに schemas/ を作成
mkdir schemas

# 2. schemas/ の内容を schemas/ に移動
#    （相対$refを使用しているため、これをSSoTとする）
mv schemas/*.json schemas/

# 3. specs/002-data-model/contracts/ は schemas/ を参照するよう更新
#    または、contracts/ の絶対URI版を schemas/ から自動生成
```

### Phase 2: 言語ディレクトリの分離

```bash
# Python
mkdir -p python
mv src python/
mv tests python/
mv pyproject.toml python/

# Rust は既に rust/ にあるため移動不要
```

### Phase 3: ビルドスクリプトの更新

1. `scripts/bundle_schemas.sh`:
   - 入力パスを `schemas/` に変更
   - 出力パスを `rust/bundled/` に維持

2. Python ビルド時に `schemas/` から `python/schemas/` にコピーするステップを追加
   - または、`schemas/` をPythonパッケージのデータとして直接参照

### Phase 4: 設定ファイルの更新

1. `python/pyproject.toml`: パス参照の確認
2. `Makefile`: パス更新
3. `.github/workflows/`: CI パスの更新
4. `CLAUDE.md`: パス参照の更新

## Consequences

### Positive

- スキーマの Single Source of Truth が確立される
- ディレクトリ構造の一貫性と明確性が向上
- 言語ごとの独立性が高まり、各言語チームが独立して作業しやすくなる
- スキーマの同期問題が解消される

### Negative

- 既存のパス参照を更新する必要がある
- ビルドプロセスが若干複雑化する（スキーマのコピー/変換ステップ追加）
- Git 履歴が `mv` コマンドにより追跡しにくくなる可能性

### Neutral

- `specs/contracts/` の役割を再定義する必要がある（参照のみ、または削除）

## References

- [PEP 517 - A build-system independent format for source trees](https://peps.python.org/pep-0517/)
- [Cargo Book - Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Monorepo Explained](https://monorepo.tools/)
- [Single Source of Truth (Wikipedia)](https://en.wikipedia.org/wiki/Single_source_of_truth)

## Related

- [pyproject.toml](../../../pyproject.toml) - Python プロジェクト設定
- [rust/Cargo.toml](../../../rust/Cargo.toml) - Rust プロジェクト設定
- [scripts/bundle_schemas.sh](../../../scripts/bundle_schemas.sh) - スキーマバンドルスクリプト
- [ADR-001: typify の unevaluatedProperties 未対応への対応策](../codegen/001-unevaluated-properties-workaround.md)
