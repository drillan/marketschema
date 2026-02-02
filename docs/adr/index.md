# Architecture Decision Records

プロジェクトの設計決定を記録する。

## 決定プロセス

### フィールド名の決定

1. **情報ソースの調査**
   - 標準プロトコル（FIX Protocol）
   - 取引所（JPX, NYSE, CME, 暗号資産取引所）
   - ブローカー（Interactive Brokers, Schwab, E*TRADE）
   - FX業者（OANDA, IG, Saxo Bank）
   - データプロバイダー（Polygon, Alpaca）

2. **ソースの一元化**
   - [情報ソース一覧](../research/sources.md) に番号付きで登録

3. **集計と決定**
   - 各フィールドについてソースごとの名称を調査
   - 多数派を採用（明確な根拠がある場合はそれに従う）
   - ADRとして決定理由を記録

4. **用語集への登録**
   - [用語集](../glossary.md) に追加
   - プロジェクト全体で使用

## フィールド名

```{toctree}
:maxdepth: 1

field-names/common
field-names/instrument
field-names/pairs
field-names/derivatives
```

## 型・フォーマット

```{toctree}
:maxdepth: 1

types/enum-values
types/format-conventions
```

## フィールド要件表

ADRで定義されたフィールド情報をまとめた要件表:

- [field-requirements.csv](../../specs/001-market-data-schema/field-requirements.csv) - エンティティ別フィールド定義
- [enum-values.csv](../../specs/001-market-data-schema/enum-values.csv) - Enum値の定義と代替案
- [format-conventions.csv](../../specs/001-market-data-schema/format-conventions.csv) - フォーマット規約と標準

### field-requirements.csv 列構造

| 列名 | 説明 |
|------|------|
| entity | 所属エンティティ（Quote, OHLCV, Trade, OrderBook, Instrument, DerivativeInfo, ExpiryInfo, OptionInfo） |
| field_name | フィールド名（標準名） |
| data_type | データ型（number, string, boolean, enum, array） |
| required | 必須フラグ（true/false） |
| nullable | NULL許容（true/false） |
| enum_values | Enum型の場合の許可値（パイプ区切り） |
| format | フォーマット制約（ISO 8601, ISO 4217等） |
| description | 日本語説明 |
| adr_reference | 参照ADRファイル |

### enum-values.csv 列構造

| 列名 | 説明 |
|------|------|
| enum_name | Enum名（Side, AssetClass, OptionType等） |
| value | 許可される値 |
| description | 日本語説明 |
| alternatives_considered | 検討した代替値（パイプ区切り） |
| usage_example | 使用例（取引所等） |
| adr_reference | 参照ADRファイル |

### format-conventions.csv 列構造

| 列名 | 説明 |
|------|------|
| format_name | フォーマット名 |
| standard | 採用標準 |
| pattern | 正規表現パターン |
| example | 例 |
| description | 日本語説明 |
| usage_example | 使用例（取引所等） |
| adr_reference | 参照ADRファイル |
