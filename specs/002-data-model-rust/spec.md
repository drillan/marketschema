# Feature Specification: Rust Data Model Implementation

**Feature Branch**: `002-data-model-rust`
**Parent Spec**: [002-data-model](../002-data-model/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "JSON Schema から Rust struct を生成するための言語固有仕様"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rust struct の自動生成 (Priority: P1)

Rust 開発者として、JSON Schema から serde 対応の Rust struct を自動生成し、型安全でシリアライズ/デシリアライズ可能なコードを書ける。

**Why this priority**: 型安全な struct 生成は Rust 実装の根幹であり、手動で struct を書く労力を削減し、スキーマとの整合性を保証する

**Independent Test**: コード生成コマンドを実行し、生成された Rust ソースファイルが存在することを確認

**Acceptance Scenarios**:

1. **Given** バンドル済み JSON Schema ファイルが存在する, **When** cargo-typify を実行する, **Then** serde derive を持つ struct が生成される
2. **Given** スキーマに description が定義されている, **When** struct を生成する, **Then** doc comment として反映される
3. **Given** スキーマに制約（min/max など）が定義されている, **When** struct を生成する, **Then** 適切な型制約として反映される

---

### User Story 2 - 生成 struct でのデシリアライズ (Priority: P1)

Rust 開発者として、生成された struct で JSON データのデシリアライズを実行し、型安全なデータ操作ができる。

**Why this priority**: デシリアライズはデータ取得の核心機能であり、P1 の生成と同等に重要

**Independent Test**: 生成された struct に正常/異常な JSON データを渡し、期待通りの結果を得られることを確認

**Acceptance Scenarios**:

1. **Given** 生成された Quote struct, **When** 正しい形式の JSON でデシリアライズする, **Then** struct インスタンスが正常に作成される
2. **Given** 生成された OHLCV struct, **When** 必須フィールドが欠けた JSON を渡す, **Then** デシリアライズエラーが発生する
3. **Given** 生成された struct, **When** シリアライズしてから再度デシリアライズする, **Then** ラウンドトリップが成功する

---

### User Story 3 - コンパイラによる型検証 (Priority: P2)

Rust 開発者として、生成された struct を使用するコードがコンパイルを通過し、型安全性が保証される。

**Why this priority**: 型安全性は Rust の強みであり、コンパイル時のエラー検出により実行時エラーを防ぐ

**Independent Test**: 生成された struct に対して cargo check を実行し、コンパイルエラーがないことを確認

**Acceptance Scenarios**:

1. **Given** 生成された全 struct ファイル, **When** cargo check を実行する, **Then** コンパイルエラー 0 件で通過する
2. **Given** 生成された struct を使用するコード, **When** 不正な型を代入しようとする, **Then** コンパイルエラーが発生する
3. **Given** Option 型のフィールド, **When** unwrap なしでアクセスする, **Then** コンパイラが警告または match 文を要求する

---

### Edge Cases

- typify が外部 $ref を解決できない場合 → json-refs でバンドル後に生成する
- フィールド名が Rust 予約語（type, struct など）の場合 → serde rename で対応
- JSON Schema Draft 2020-12 の unevaluatedProperties を使用する場合 → バンドル時に additionalProperties に変換することで typify が `#[serde(deny_unknown_fields)]` を生成する
- 複数スキーマで同名の型（Symbol など）が定義される場合 → バンドル時に型名の衝突を解決する

## Requirements *(mandatory)*

### Functional Requirements

#### スキーマバンドリング

- **FR-R001**: 生成前にすべての JSON Schema を json-refs でバンドルし、自己完結した単一スキーマを作成しなければならない
- **FR-R002**: バンドリング時に `unevaluatedProperties` を `additionalProperties` に変換しなければならない（typify が `unevaluatedProperties` 未対応のため、[issue #39](https://github.com/drillan/marketschema/issues/39) 参照）
- **FR-R003**: バンドリングは `scripts/bundle_schemas.sh` で実行し、json-refs による $ref 解決と jq によるプロパティ変換を含むものとする
- **FR-R004**: バンドル済みスキーマは `rust/bundled/` ディレクトリに保存しなければならない

#### コード生成

- **FR-R005**: システムは cargo-typify を使用して JSON Schema から Rust struct を生成しなければならない
- **FR-R006**: 生成された struct は `#[derive(Serialize, Deserialize, Debug, Clone)]` を含まなければならない
- **FR-R007**: 必須でないプロパティは `#[serde(default)]` 属性を持つものとする
- **FR-R008**: フィールド名は snake_case に変換しなければならない（typify のデフォルト動作）
- **FR-R009**: 生成されたコードは `src/types/` ディレクトリに配置しなければならない

#### 型マッピング

- **FR-R010**: JSON Schema の string は Rust の String 型にマッピングしなければならない
- **FR-R011**: JSON Schema の number は Rust の f64 型にマッピングしなければならない
- **FR-R012**: JSON Schema の integer は Rust の i64 型にマッピングしなければならない
- **FR-R013**: JSON Schema の boolean は Rust の bool 型にマッピングしなければならない
- **FR-R014**: JSON Schema の array は Rust の Vec<T> 型にマッピングしなければならない
- **FR-R015**: nullable フィールドは Rust の Option<T> 型として生成しなければならない

#### 既知の制限事項への対応

- **FR-R016**: typify は JSON Schema Draft 2020-12 の unevaluatedProperties をサポートしないため、FR-R002 に従いバンドル時に additionalProperties へ変換することで `#[serde(deny_unknown_fields)]` の自動生成を実現する
- **FR-R017**: anyOf および if/then/else のサポートが限定的であることを文書化しなければならない
- **FR-R018**: スキーマの Draft 2020-12 互換性のため、`$defs` と `definitions` の両方を定義することを推奨する

#### パターン検証

- **FR-R019**: スキーマでパターン検証が使用されている場合、regress クレートを依存関係に追加しなければならない

#### 品質保証

- **FR-R020**: 生成されたコードは cargo check でコンパイルエラー 0 件を達成しなければならない
- **FR-R021**: 生成後は cargo fmt でフォーマットすることを推奨する

### Key Entities

- **Quote**: 最良気配値 struct。親仕様の Quote スキーマから生成
- **OHLCV**: ローソク足 struct。親仕様の OHLCV スキーマから生成
- **Trade**: 約定 struct。親仕様の Trade スキーマから生成
- **OrderBook**: 板情報 struct。親仕様の OrderBook スキーマから生成
- **PriceLevel**: 板情報の各気配レベル struct。親仕様から生成
- **Instrument**: 銘柄情報 struct。親仕様の Instrument スキーマから生成
- **VolumeInfo**: 出来高情報 struct。親仕様の VolumeInfo スキーマから生成
- **ExpiryInfo**: 満期情報 struct。親仕様の ExpiryInfo スキーマから生成
- **OptionInfo**: オプション情報 struct。親仕様の OptionInfo スキーマから生成
- **DerivativeInfo**: デリバティブ情報 struct。親仕様の DerivativeInfo スキーマから生成

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-R001**: すべての JSON Schema ファイル（Quote, OHLCV, Trade, OrderBook, Instrument 等）から Rust struct が正常に生成される
- **SC-R002**: 生成された struct が cargo check でコンパイルエラー 0 件を達成する
- **SC-R003**: 各 struct に対して最低 3 つの正常系 JSON データでデシリアライズが成功する
- **SC-R004**: 各 struct に対して最低 2 つの異常系 JSON データでデシリアライズエラーが発生する
- **SC-R005**: 生成されたコードが cargo clippy で重大な警告 0 件を達成する
- **SC-R006**: 生成された struct のシリアライズ/デシリアライズのラウンドトリップが成功する

## Assumptions

- cargo-typify (typify) の最新版を使用する
- Rust 最新安定版を対象とする
- serde および serde_json クレートを使用する
- JSON Schema は親仕様で定義された Draft 2020-12 形式に準拠している
- Node.js および npm が json-refs のためにインストールされている

## Out of Scope

- typify 以外のコード生成ツール（schemars、json-schema-to-rust など）
- カスタムバリデーターの自動生成
- runtime での JSON Schema 検証（serde のデシリアライズで検証）
- 生成されたコードの手動編集ルール（CLAUDE.md の Quality Standards に従う）

## Related Documents

- [002-data-model](../002-data-model/spec.md) - 親仕様（JSON Schema 定義）
- [docs/code-generation.md](../../docs/code-generation.md) - コード生成の実行手順
- [typify GitHub](https://github.com/oxidecomputer/typify) - コード生成ツールのリポジトリ
- [ADR-001: unevaluatedProperties 対応策](../../docs/adr/codegen/001-unevaluated-properties-workaround.md) - typify の制限への対応方針
