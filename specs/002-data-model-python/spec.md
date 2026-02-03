# Feature Specification: Python Data Model Implementation

**Feature Branch**: `002-data-model-python`
**Parent Spec**: [002-data-model](../002-data-model/spec.md)
**Created**: 2026-02-02
**Status**: Draft
**Input**: User description: "JSON Schema から Python pydantic v2 モデルを生成するための言語固有仕様"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - pydantic v2 モデルの自動生成 (Priority: P1)

Python 開発者として、JSON Schema から pydantic v2 モデルを自動生成し、型安全でバリデーション付きのコードを書ける。

**Why this priority**: 型安全なモデル生成は Python 実装の根幹であり、手動でモデルを書く労力を削減し、スキーマとの整合性を保証する

**Independent Test**: コード生成コマンドを実行し、生成されたモデルファイルが存在することを確認

**Acceptance Scenarios**:

1. **Given** JSON Schema ファイル一式が存在する, **When** コード生成ツールを実行する, **Then** pydantic v2 の BaseModel を継承したクラスが生成される
2. **Given** スキーマに description が定義されている, **When** モデルを生成する, **Then** docstring またはフィールド説明として反映される
3. **Given** スキーマに制約（min/max など）が定義されている, **When** モデルを生成する, **Then** Field constraints として反映される

---

### User Story 2 - 生成モデルでのデータバリデーション (Priority: P1)

Python 開発者として、生成された pydantic モデルでデータのバリデーションを実行し、不正なデータを検出できる。

**Why this priority**: バリデーションはデータ品質を保証する核心機能であり、P1 の生成と同等に重要

**Independent Test**: 生成されたモデルに正常/異常データを渡し、期待通りの結果を得られることを確認

**Acceptance Scenarios**:

1. **Given** 生成された Quote モデル, **When** 正しい形式のデータでインスタンス化する, **Then** オブジェクトが正常に作成される
2. **Given** 生成された OHLCV モデル, **When** 必須フィールドが欠けたデータを渡す, **Then** ValidationError が発生する
3. **Given** スキーマで unevaluatedProperties: false が指定されている, **When** 未定義のフィールドを含むデータを渡す, **Then** ValidationError が発生する

---

### User Story 3 - 型チェッカーとの互換性 (Priority: P2)

Python 開発者として、生成されたモデルを使用するコードが mypy 型チェックを通過し、IDE での補完が正しく機能する。

**Why this priority**: 型安全性は開発効率と品質向上に寄与するが、まずモデル生成とバリデーションが動作することが前提

**Independent Test**: 生成されたモデルに対して mypy を実行し、エラーがないことを確認

**Acceptance Scenarios**:

1. **Given** 生成された全モデルファイル, **When** mypy で型チェックを実行する, **Then** エラー 0 件で通過する
2. **Given** 生成されたモデルを使用するコード, **When** IDE でフィールドアクセスする, **Then** 正しい型補完が表示される
3. **Given** nullable フィールド（str | None）, **When** None チェックなしでアクセスする, **Then** mypy が警告を出す

---

### Edge Cases

- $ref が絶対 URL の場合 → httpx モジュールが必要になるため、相対パスを使用する
- フィールド名が Python 予約語（class, from など）の場合 → エイリアスで対応
- 非常に深くネストしたスキーマの場合 → 生成ツールの制限に注意

## Requirements *(mandatory)*

### Functional Requirements

#### コード生成

- **FR-P001**: システムは datamodel-code-generator を使用して JSON Schema から pydantic v2 モデルを生成しなければならない
- **FR-P002**: 生成オプションは pyproject.toml の `[tool.datamodel-codegen]` セクションで管理しなければならない
- **FR-P003**: 生成されたモデルは `pydantic_v2.BaseModel` を使用しなければならない
- **FR-P004**: 生成されたモデルは `typing.Annotated` を使用して制約を表現しなければならない
- **FR-P005**: 生成されたモデルは Python 3.10+ の型ヒント構文（`list`, `dict`, `X | Y`）を使用しなければならない
- **FR-P006**: フィールド名は snake_case に変換しなければならない

#### スキーママッピング

- **FR-P007**: JSON Schema の unevaluatedProperties: false は pydantic の `extra='forbid'` にマッピングしなければならない
- **FR-P008**: JSON Schema の unevaluatedProperties: true は pydantic の `extra='allow'` にマッピングしなければならない
- **FR-P009**: nullable フィールド（`type: ["string", "null"]`）は `str | None = None` として生成しなければならない
- **FR-P010**: スキーマの description は docstring またはフィールド説明として反映しなければならない

#### $ref 解決

- **FR-P011**: $ref は相対パスで指定しなければならない（絶対 URL は httpx 依存を避けるため非推奨）

#### 品質保証

- **FR-P012**: 生成されたコードは mypy 型チェックでエラー 0 件を達成しなければならない
- **FR-P013**: 生成後は ruff でフォーマットすることを推奨する

### Key Entities

- **Quote**: 最良気配値モデル。親仕様の Quote スキーマから生成
- **OHLCV**: ローソク足モデル。親仕様の OHLCV スキーマから生成
- **Trade**: 約定モデル。親仕様の Trade スキーマから生成
- **OrderBook**: 板情報モデル。親仕様の OrderBook スキーマから生成
- **Instrument**: 銘柄情報モデル。親仕様の Instrument スキーマから生成

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-P001**: すべての JSON Schema ファイル（Quote, OHLCV, Trade, OrderBook, Instrument 等）から pydantic v2 モデルが正常に生成される
- **SC-P002**: 生成されたモデルが mypy 型チェックでエラー 0 件を達成する
- **SC-P003**: 各モデルに対して最低 3 つの正常系データでインスタンス化が成功する
- **SC-P004**: 各モデルに対して最低 2 つの異常系データで ValidationError が発生する
- **SC-P005**: 生成されたコードが ruff check でエラー 0 件を達成する

## Assumptions

- datamodel-code-generator の最新版を使用する
- Python 3.13 以上を対象とする（3.10+ 導入の型ヒント構文を使用）
- pydantic v2.0 以上を使用する
- JSON Schema は親仕様で定義された Draft 2020-12 形式に準拠している

## Out of Scope

- datamodel-code-generator 以外のコード生成ツール
- Python 3.9 以下のサポート
- pydantic v1 のサポート
- カスタムバリデーターの自動生成（手動で追加する場合は別仕様）
- 生成されたコードの手動編集ルール（CLAUDE.md の Quality Standards に従う）

## Related Documents

- [002-data-model](../002-data-model/spec.md) - 親仕様（JSON Schema 定義）
- [docs/code-generation.md](../../docs/code-generation.md) - コード生成の実行手順
- [pyproject.toml](../../pyproject.toml) - datamodel-codegen 設定
