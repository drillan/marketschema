# ADR: Enum値の標準化

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

金融データスキーマで使用するEnum値を標準化する必要がある。業界調査の結果、一部の値は統一されておらず、選択が必要である。

## 決定

### 1. Side（売買方向）

採用値: `buy`, `sell`

| 検討した選択肢 | 使用例 |
|---------------|--------|
| `buy` / `sell`（小文字） | Coinbase, Kraken, OKX |
| `BUY` / `SELL`（大文字） | Binance, Interactive Brokers |
| `1` / `2`（数値） | FIX Protocol |

採用理由:
- JSON APIでは小文字が慣例的
- 可読性が高い
- 大文字への変換は容易

### 2. AssetClass（資産クラス）

採用値: `equity`, `fund`, `bond`, `future`, `option`, `fx`, `crypto`, `cfd`

| 値 | 説明 | 検討した代替 |
|----|------|-------------|
| `equity` | 株式 | `stock`, `CS`(FIX), `STK`(IB) |
| `fund` | 投資信託 | `mutual_fund`, `MF`(FIX) |
| `bond` | 債券 | `CORP`(FIX), `BOND`(IB) |
| `future` | 先物 | `futures`, `FUT` |
| `option` | オプション | `OPT` |
| `fx` | 外国為替 | `forex`, `currency`, `CASH`(IB) |
| `crypto` | 暗号資産 | `cryptocurrency`, `DIGITAL`(FIX) |
| `cfd` | CFD | `CFD`(IB) |

採用理由:
- 可読性と簡潔さのバランス
- ISO 10962 CFIコードの概念に準拠（E=Equity等）
- 主要APIとの互換性

## 根拠

### Side

FIX Protocolでは数値コード（Tag 54: 1=Buy, 2=Sell）を使用するが、現代のREST APIでは文字列が主流。大文字/小文字の違いはあるが、フィールド名としては`side`が業界標準。

### AssetClass

ISO 10962（CFI Code）が国際標準として存在するが、単一文字コード（E, D, F等）は可読性に欠ける。FIXやInteractive Brokersの略語（CS, STK, FUT等）も同様。可読性を重視し、フルワードを採用する。

## 参考資料

- [FIX Protocol - Side Field (Tag 54)](https://www.onixs.biz/fix-dictionary/4.4/tagnum_54.html)
- [FIX Protocol - SecurityType Field (Tag 167)](https://www.onixs.biz/fix-dictionary/5.0.sp2.ep266/tagnum_167.html)
- [ISO 10962 - CFI Code](https://www.iso.org/standard/81140.html)
- [Interactive Brokers - Contract Types](https://interactivebrokers.github.io/tws-api/basic_contracts.html)
