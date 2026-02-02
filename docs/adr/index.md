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
