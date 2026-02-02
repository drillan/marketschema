# ADR: フォーマット規約

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

金融データスキーマで使用するデータフォーマットを標準化する必要がある。タイムスタンプと通貨コードについて、業界標準を調査し採用フォーマットを決定する。

## 決定

### 1. Timestamp（タイムスタンプ）

採用フォーマット: ISO 8601 (UTC)

形式: `YYYY-MM-DDTHH:MM:SS.sssZ`

例: `2026-02-02T12:00:00.000Z`

| 検討した選択肢 | 使用例 |
|---------------|--------|
| ISO 8601 | Coinbase, Alpaca, Kraken (RFC 3339) |
| Unix timestamp（ミリ秒） | Binance, Bybit, OKX |
| Unix timestamp（秒） | 一部API |
| FIX形式 (`YYYYMMDD-HH:MM:SS`) | FIX Protocol |

採用理由:
- 国際標準（ISO）に準拠
- 人間可読性が高い
- タイムゾーンが明示的（末尾の`Z`はUTC）
- RFC 3339はISO 8601のインターネット向けプロファイル

### 2. Currency（通貨コード）

採用フォーマット: ISO 4217（3文字アルファコード）

パターン: `^[A-Z]{3}$`

例: `USD`, `JPY`, `EUR`

| 対象 | 標準 | 備考 |
|------|------|------|
| 法定通貨 | ISO 4217 | 国際標準、全金融機関で使用 |
| 暗号資産 | 業界慣行 | ISO 24165 (DTI) が存在するが普及途上 |

採用理由:
- ISO 4217は法定通貨の国際標準
- FIX Protocol (Tag 15) でも採用
- 暗号資産（BTC, ETH等）は3文字コードに準じた形式が業界慣行

## 根拠

### Timestamp

業界では2つの形式が主流：
- 伝統的金融: ISO 8601 / FIX形式
- 暗号資産取引所: Unixミリ秒

ISO 8601を採用する理由：
1. 国際標準機構（ISO）が定めた正式な標準
2. デバッグ時の可読性が高い
3. タイムゾーン情報が明示的
4. JSON Schemaの`date-time`フォーマットがISO 8601/RFC 3339に準拠

Unixタイムスタンプとの相互変換は容易であり、アダプター層で対応可能。

### Currency

ISO 4217は1978年から使用されている確立された国際標準。暗号資産についてはISO 24165（Digital Token Identifier）が2021年に標準化されたが、9文字の識別子形式であり、業界での普及は途上にある。

実用上は、暗号資産も3文字コード（BTC, ETH等）で表記されることが多く、ISO 4217のパターン（3文字大文字）に準じた形式を許容する。

## 参考資料

- [ISO 8601 - Date and Time Format](https://www.iso.org/iso-8601-date-and-time-format.html)
- [RFC 3339 - Date and Time on the Internet](https://datatracker.ietf.org/doc/html/rfc3339)
- [ISO 4217 - Currency Codes](https://www.iso.org/iso-4217-currency-codes.html)
- [ISO 24165 - Digital Token Identifier](https://www.iso.org/standard/80601.html)
- [FIX Protocol - Currency Tag 15](https://www.onixs.biz/fix-dictionary/4.4/tagnum_15.html)
