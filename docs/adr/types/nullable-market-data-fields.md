# ADR: マーケットデータフィールドのNULL許容

## ステータス

Accepted

## 日付

2026-02-05

## コンテキスト

現在、`Quote` の `bid`/`ask` および `OHLCV` の価格・出来高フィールドは必須（non-nullable）として定義されている。しかし、実際の市場では**データが存在しない状態**が有効な市場状態として発生する。

### 現在の定義

| エンティティ | フィールド | 現在の定義 |
|-------------|-----------|-----------|
| Quote | bid, ask | required=true, nullable=false |
| OHLCV | open, high, low, close, volume | required=true, nullable=false |

### 問題

現在の定義では、以下の有効な市場状態を表現できない：

| シナリオ | 説明 | 影響フィールド |
|----------|------|---------------|
| 流動性枯渇 | 片側または両側に注文がない | bid, ask |
| サーキットブレーカー | 取引停止中 | bid, ask, OHLCV |
| プレマーケット/アフターアワーズ | 取引時間外 | bid, ask |
| 低流動性先物 | 取引が発生していない | OHLCV |
| IPO/新規上場 | 片側のみ注文がある | bid または ask |
| ストップ高/ストップ安 | 片側のみ注文がある | bid または ask |
| 取引一時停止 | 一時的な取引停止 | bid, ask, OHLCV |

### 具体例: JPX先物API

JPX（日本取引所グループ）の先物APIは、データがない場合に `"-"` を返す：

```json
{
  "TTCODE": "101.2609",
  "DELI": "26年9月限",
  "QBP": "-",
  "QAP": "-",
  "DOP": "-",
  "DHP": "-",
  "DLP": "-",
  "DPP": "-",
  "DV": "-"
}
```

これは低流動性銘柄（例：限月が遠い先物）の有効な市場状態であり、エラーではない。

## 決定

以下のフィールドをオプショナル（nullable）に変更する：

### Quote

| フィールド | 変更前 | 変更後 |
|-----------|--------|--------|
| bid | required=true, nullable=false | required=false, nullable=true |
| ask | required=true, nullable=false | required=false, nullable=true |

### OHLCV

| フィールド | 変更前 | 変更後 |
|-----------|--------|--------|
| open | required=true, nullable=false | required=false, nullable=true |
| high | required=true, nullable=false | required=false, nullable=true |
| low | required=true, nullable=false | required=false, nullable=true |
| close | required=true, nullable=false | required=false, nullable=true |
| volume | required=true, nullable=false | required=false, nullable=true |

### None のセマンティクス

`None` は「**データが存在しない**」ことを意味し、「不明」や「エラー」ではない：

| フィールド | None の意味 |
|-----------|------------|
| bid | 買い注文が存在しない |
| ask | 売り注文が存在しない |
| open | 始値時点で取引が発生していない |
| high | 期間中に取引が発生していない |
| low | 期間中に取引が発生していない |
| close | 終値時点で取引が発生していない |
| volume | 取引が発生していない |

これは取引所APIがこれらの状態を表現する方法と一致している。

## 根拠

### 1. 実際の市場状態の表現

取引所APIは「データなし」を有効な状態として返す。現在のスキーマではこれを表現できず、アダプター実装時に不適切なデフォルト値（例：`0.0`）を使用せざるを得ない。

### 2. 業界標準との整合性

| ソース | 表現方法 |
|--------|---------|
| JPX | `"-"` 文字列 |
| FIX Protocol | 任意フィールドとして定義 |
| Binance | `null` または フィールド省略 |
| CCXT | `None` または `undefined` |

### 3. 後方互換性

この変更は後方互換性がある：
- 既存のコードで常に値を提供している場合、変更なく動作する
- 新しいコードは、データがない状態を適切に表現できる

### 4. 禁止事項との整合性

プロジェクトの禁止事項「暗黙的フォールバック禁止」に従い、データがない状態をデフォルト値で隠蔽するのではなく、明示的に `None` として表現する。

## 結果

1. **スキーマ更新**: JSON Schema および生成コード（Python/Rust）を更新
2. **field-requirements.csv 更新**: 対象フィールドの `required` と `nullable` を変更
3. **マイグレーションガイド**: 既存利用者向けに変更内容を文書化
4. **バージョン**: セマンティックバージョニングに従い、マイナーバージョンをインクリメント（後方互換のため）

## 関連

- GitHub Issue: [#173](https://github.com/drillan/marketschema/issues/173)
- 関連プロジェクト: [jpx-client](https://github.com/drillan/jpx-client) - JPX先物/オプションAPIラッパー
- ADR: [field-names/common.md](../field-names/common.md) - bid/ask/OHLCV フィールド名の定義
