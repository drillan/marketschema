# Feature Specification: 統一マーケットデータスキーマ

**Feature Branch**: `002-data-model`
**Parent Spec**: [001-core](../001-core/spec.md)
**Created**: 2026-02-02
**Status**: Draft
**Input**: User description: "drafts/00-03*.md を基に金融データソースの統一スキーマとアダプター仕様を定義"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - スキーマを使ったデータ検証 (Priority: P1)

開発者として、各種金融データソースから取得したデータが統一スキーマに準拠しているか検証できる。JSON Schema を使用して、Quote、OHLCV、Trade、OrderBook などのデータ構造が正しい形式であることを確認する。

**Why this priority**: スキーマ定義はプロジェクトの根幹であり、他のすべての機能（コード生成、アダプター）がこれに依存する

**Independent Test**: JSON Schema ファイルを使用して、サンプルデータをバリデーションできる

**Acceptance Scenarios**:

1. **Given** Quote スキーマ定義が存在する, **When** 正しい形式の気配値データを検証する, **Then** バリデーションが成功する
2. **Given** OHLCV スキーマ定義が存在する, **When** 必須フィールドが欠けたデータを検証する, **Then** バリデーションエラーが返される
3. **Given** 共通型定義（Timestamp, Side, AssetClass など）が存在する, **When** 各スキーマから参照する, **Then** 正しく型が解決される

---

### User Story 2 - Python pydantic モデルの生成 (Priority: P2)

Python 開発者として、JSON Schema から pydantic v2 モデルを自動生成し、型安全かつバリデーション付きのコードを書ける。

**Why this priority**: Python は金融データ分析で広く利用されている言語であり、最優先の言語サポート対象

**Independent Test**: datamodel-code-generator で Python コードを生成し、型チェッカー（mypy）でエラーがないことを確認

**Acceptance Scenarios**:

1. **Given** JSON Schema ファイル一式が存在する, **When** datamodel-codegen を実行する, **Then** pydantic v2 モデルが生成される
2. **Given** 生成された pydantic モデル, **When** mypy で型チェックする, **Then** エラーなく通過する
3. **Given** 生成された Quote クラス, **When** 正しいデータでインスタンス化する, **Then** 期待通りのオブジェクトが作成され、バリデーションが実行される

---

### User Story 3 - Rust 構造体の生成 (Priority: P2)

Rust 開発者として、JSON Schema から struct を自動生成し、高パフォーマンスなアプリケーションで使用できる。

**Why this priority**: Rust は高頻度取引など、パフォーマンス重視の用途で必要

**Independent Test**: typify で Rust コードを生成し、cargo check でコンパイルエラーがないことを確認

**Acceptance Scenarios**:

1. **Given** JSON Schema ファイル一式が存在する, **When** typify を実行する, **Then** Rust struct が生成される
2. **Given** 生成された struct, **When** cargo build する, **Then** コンパイルが成功する

---

### User Story 4 - アダプター基盤の提供 (Priority: P3)

アダプター開発者として、BaseAdapter クラスを継承して、新しいデータソース用のアダプターを簡単に実装できる。

> **Note**: インターフェース契約の詳細は [004-adapter](../004-adapter/spec.md) を参照。

**Why this priority**: アダプターはコアライブラリの拡張ポイントであり、個別業者対応は外部パッケージで行う想定

**Independent Test**: BaseAdapter を継承したサンプルアダプターを実装し、マッピング定義でデータ変換できることを確認

**Acceptance Scenarios**:

1. **Given** BaseAdapter クラスが存在する, **When** 継承してマッピング定義を実装する, **Then** 生データを統一モデルに変換できる
2. **Given** ModelMapping 定義, **When** ネストしたフィールドパス（例: `price.bid`）を指定する, **Then** 正しく値を取得できる
3. **Given** 変換関数（transforms）, **When** タイムスタンプ変換を行う, **Then** JST から UTC への変換が正しく行われる

---

### User Story 5 - アダプターレジストリでの管理 (Priority: P3)

システム設計者として、複数のアダプターをレジストリに登録し、データソース名で取得できる。

> **Note**: インターフェース契約の詳細は [004-adapter](../004-adapter/spec.md) を参照。

**Why this priority**: 動的なアダプター選択を可能にし、プラグイン的な拡張性を提供

**Independent Test**: 複数のアダプターを登録し、source_name で正しいアダプターを取得できることを確認

**Acceptance Scenarios**:

1. **Given** AdapterRegistry が存在する, **When** @register デコレータでアダプターを登録する, **Then** 登録が成功する
2. **Given** 登録済みアダプター, **When** source_name で取得する, **Then** 正しいアダプターインスタンスが返される
3. **Given** 未登録の source_name, **When** 取得を試みる, **Then** KeyError が発生する

---

### Edge Cases

- 数値フィールドに文字列が渡された場合 → 変換関数で適切に変換またはエラー
- タイムスタンプが異なるフォーマット（ミリ秒、秒、ISO 8601）の場合 → 各変換関数で対応
- 板情報の bids/asks が空配列の場合 → 空の OrderBook として有効
- オプショナルフィールド（side, quote_volume など）が null の場合 → null として許容
- 取引所固有のデータフォーマット（Binance の配列形式 Klines など）→ カスタム変換メソッドで対応

## Requirements *(mandatory)*

### Functional Requirements

#### スキーマ定義

- **FR-001**: システムは JSON Schema Draft 2020-12 に準拠した共通型定義（definitions.json）を提供しなければならない
- **FR-002**: システムは Quote（気配値）スキーマを提供しなければならない
- **FR-003**: システムは OHLCV（4本値＋出来高）スキーマを提供しなければならない
- **FR-004**: システムは Trade（約定）スキーマを提供しなければならない
- **FR-005**: システムは OrderBook（板情報）スキーマを提供しなければならない
- **FR-006**: システムは VolumeInfo（出来高・売買代金）スキーマを提供しなければならない
- **FR-007**: システムは ExpiryInfo（限月・満期情報）スキーマを提供しなければならない
- **FR-008**: システムは OptionInfo（オプション情報）スキーマを提供しなければならない
- **FR-009**: システムは Instrument（銘柄情報）スキーマを提供しなければならない
- **FR-010**: すべてのスキーマは unevaluatedProperties: false を指定し、未定義フィールドを許容しないものとする。拡張が必要な場合は allOf で基本スキーマを継承した派生スキーマを定義する
- **FR-011**: 各フィールドの必須/任意は本仕様のスコープ外とし、[フィールド要件表](field-requirements.csv) または金融商品カテゴリごとのサブ仕様で定義するものとする

#### 共通型定義

- **FR-012**: Timestamp 型は ISO 8601 形式の date-time フォーマットで定義しなければならない
- **FR-013**: Side 型は enum として定義し、具体的な値は [ADR: Enum値](../../docs/adr/types/enum-values.md) に従うものとする
- **FR-014**: AssetClass 型は enum として定義し、具体的な値は [ADR: Enum値](../../docs/adr/types/enum-values.md) に従うものとする
- **FR-015**: Currency 型は ISO 4217 の3文字コードパターンで定義しなければならない

#### コード生成

- **FR-016**: JSON Schema から pydantic v2 モデルを生成するためのコマンドとオプションを文書化しなければならない
- **FR-017**: JSON Schema から Rust struct を生成するためのコマンドとオプションを文書化しなければならない

#### アダプター基盤

> **Note**: 以下の FR のインターフェース契約詳細は [004-adapter](../004-adapter/spec.md) を参照。

- **FR-018**: BaseAdapter 抽象クラスを提供し、各データモデルへのマッピング定義メソッドを要求しなければならない
- **FR-019**: ModelMapping クラスを提供し、target_field, source_field, transform, default の属性を持つものとする
- **FR-020**: 共通変換関数（transforms モジュール）を提供し、数値変換、タイムスタンプ変換、サイド変換を含むものとする
- **FR-021**: AdapterRegistry を提供し、アダプターの登録と取得を管理できるものとする

#### データ型規約

- **FR-022**: 価格・金額・数量は float 型（JSON の number）で表現するものとする
- **FR-023**: すべてのタイムスタンプは UTC で保存し、ISO 8601 形式で表現するものとする
- **FR-024**: オプショナルフィールドは null を許容するものとする

#### 命名規約

- **FR-025**: すべてのフィールド名は [ADR: フィールド名](../../docs/adr/index.md) 配下のADRで定義された標準名に従わなければならない
- **FR-026**: ADR で定義されていない新しいフィールドを追加する場合は、[ADR決定プロセス](../../docs/adr/index.md) に従って標準名を決定してから追加しなければならない
- **FR-027**: スキーマの `description` フィールドには [用語集](../../docs/glossary.md) の定義を参照するものとする

### Key Entities

各エンティティのフィールド名は [ADR: フィールド名](../../docs/adr/index.md) に従う。

- **Quote**: 最良気配値（BBO）を表現。買い/売り気配の価格と数量、取得時刻を含む
- **OHLCV**: ローソク足データを表現。始値、高値、安値、終値、出来高、足の開始時刻を含む
- **Trade**: 個別約定を表現。約定価格、数量、売買方向、約定時刻を含む
- **OrderBook**: 複数レベルの板情報を表現。買い板と売り板の配列、取得時刻を含む
- **PriceLevel**: 板情報の各気配レベルを表現。価格と数量を含む
- **VolumeInfo**: 出来高と売買代金を表現
- **ExpiryInfo**: 先物・オプションの満期情報を表現。限月コード、満期日、最終取引日、決済日を含む
- **OptionInfo**: オプション固有情報を表現。権利行使価格、コール/プット区分、行使スタイルを含む
- **Instrument**: 銘柄識別情報を表現。銘柄コード、資産クラス、通貨コード、取引所を含む
- **BaseAdapter**: アダプターの基底クラス。各データモデルへのマッピング定義と変換メソッドを提供
- **ModelMapping**: フィールドマッピング定義。ソースフィールドからターゲットフィールドへの変換ルールを定義
- **AdapterRegistry**: アダプターの登録・取得を管理するシングルトン

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: すべての JSON Schema ファイルが Draft 2020-12 に準拠し、ajv-cli でバリデーションが成功する
- **SC-002**: pydantic v2 モデルの自動生成が成功し、mypy で型エラーがゼロである
- **SC-003**: Rust struct の自動生成が成功し、cargo check でコンパイルエラーがゼロである
- **SC-004**: 各スキーマに対して最低5つのサンプルデータで正常系バリデーションが成功する
- **SC-005**: 各スキーマに対して最低3つの異常系データでバリデーションエラーが正しく検出される
- **SC-006**: BaseAdapter を使用したサンプルアダプター（SBI、Binance 形式）が実装でき、テストが通過する
- **SC-007**: 対象金融商品（株式、投信、債券、先物、オプション、FX、暗号資産、CFD）のデータが表現できることを確認

## Assumptions

- 数値精度: マーケットデータの表示・分析用途では float 型で十分。発注・会計処理で厳密な精度が必要な場合は、アプリケーション側で Decimal 等に変換する想定
- タイムゾーン: すべてのタイムスタンプは UTC で正規化して保存。ローカルタイムゾーンへの変換はアプリケーション側の責務
- スキーマバージョニング: セマンティックバージョニングを採用し、破壊的変更はメジャーバージョンで管理
- 言語サポート: Python と Rust を第一優先とし、TypeScript/Go は将来対応
- 各業者アダプターは本プロジェクトのスコープ外（外部パッケージとして提供）

## Out of Scope

- 発注・約定管理（Order, Execution）
- ポートフォリオ管理
- リスク計算
- 会計・税務処理
- コーポレートアクション（配当、株式分割、合併等）- 将来の拡張として別仕様で対応予定
- 各データソースとの実際の接続ロジック（アダプターパッケージで実装）
- WebSocket や REST API クライアント実装
- 各フィールドの必須/任意定義（別表またはサブ仕様で管理）
- 具体的なenum値の定義（ADRで管理）

## Clarifications

### Session 2026-02-02

- Q: 配当（dividend）や投資信託の分配金（distribution）をスキーマに含めるべきか？ → A: スコープ外とする。コーポレートアクション（配当、株式分割、合併等）は将来の拡張として別仕様で対応予定
- Q: Python を「金融データ分析のデファクトスタンダード」と記載しているが、裏付けがない場合は表現を修正すべきか？ → A: 「広く利用されている言語」に修正（主観的な断定を避け、客観的な表現に変更）
- Q: フィールド名はADRにしたがった名前を遵守することを明記すべきか？ → A: FR-025〜FR-027として命名規約セクションを追加（ADR準拠を必須要件として明文化）
- Q: Edge Cases で `turnover` が使用されているが、ADR標準名は `quote_volume` である。どちらに統一すべきか？ → A: `quote_volume` に修正（ADR準拠）
- Q: `asset_class`, `currency` が用語集に未登録。どう対応すべきか？ → A: `docs/adr/field-names/instrument.md` を作成し、調査が必要なフィールドとして登録
- Q: Python のデータモデルは dataclass と pydantic v2 のどちらを使用するか？ → A: pydantic v2 に変更（バリデーション統合、高機能、依存関係に既に含まれている）
- Q: Trade（約定）スキーマは必要か？（ポートフォリオ管理は非スコープ） → A: 維持。Trade は「市場の約定データ（歩み値 / Time & Sales）」であり、ポートフォリオ管理（自分の約定）とは別概念
- Q: FR-010 の `additionalProperties: false` は金融商品特有の情報（貸借銘柄など）を追加する際に柔軟性に欠けるのではないか？ → A: `unevaluatedProperties: false` に変更し、`allOf` によるスキーマ継承で拡張を可能にする。派生スキーマは本プロジェクトのスコープ外とし、外部パッケージで定義する想定
- Q: ADRで多種多様なフィールドが定義されているが、個々のフィールドの必須/任意をこの仕様で定義すべきか？ → A: フィールドの必須/任意は本仕様のスコープ外とする。本仕様ではフィールド名の標準化（ADR準拠）とスキーマ構造の定義に集中し、必須/任意の定義は [フィールド要件表](field-requirements.csv) または金融商品カテゴリごとのサブ仕様で管理する
- Q: FR-013（Side型）、FR-014（AssetClass型）の具体的なenum値も本仕様でハードコードすべきか？ → A: enum値も本仕様のスコープ外とし、ADR（types/enum-values.md）を参照する形にする
- Q: Key Entities のフィールド例とADRに不整合がある（Instrumentの「銘柄名」がADR未定義、ExpiryInfoの「最終取引日」がspec未記載）。どう対応すべきか？ → A: spec.mdは最小限に保ち「詳細はADR参照」と明記。フィールド例は日本語名で残しつつADRに合致させる。必須/任意の記述は削除

## Related Documents

- [004-adapter](../004-adapter/spec.md) - アダプターインターフェース契約（User Story 4-5, FR-018〜021 の詳細）
- [フィールド要件表](field-requirements.csv) - エンティティ別フィールド定義（データ型、必須/任意、Enum値、フォーマット）
- [ADR: フィールド名](../../docs/adr/index.md) - フィールド名の標準化決定
- [ADR: Enum値](../../docs/adr/types/enum-values.md) - Enum型の許可値
- [ADR: フォーマット規約](../../docs/adr/types/format-conventions.md) - タイムスタンプ・通貨コードのフォーマット
- [用語集](../../docs/glossary.md) - フィールドの日本語説明
