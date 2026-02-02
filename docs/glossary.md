# 用語集（Glossary）

プロジェクト全体で使用する標準用語を定義する。
すべてのスキーマ・コードはこの用語集に従う。

## 全商品共通フィールド名

以下のフィールド名は業界標準として採用が確定している。
決定経緯は [ADR: 全商品共通フィールド名](adr/field-names/common.md) を参照。

```{glossary}
ask
    売り気配値。

asks
    売り板情報の配列。

ask_size
    売り気配の数量。

bid
    買い気配値。

bids
    買い板情報の配列。

bid_size
    買い気配の数量。

close
    終値。

high
    高値。

low
    安値。

open
    始値。

price
    価格（約定価格、現在値など）。

quote_volume
    決済通貨建ての出来高（売買代金）。

side
    売買方向。

size
    約定数量。

symbol
    銘柄識別子。

timestamp
    タイムスタンプ。

volume
    出来高（基軸通貨建て）。
```

## Instrument フィールド名

銘柄情報に関するフィールド名。
決定経緯は [ADR: 銘柄情報フィールド名](adr/field-names/instrument.md) および [ADR: ペア商品フィールド名](adr/field-names/pairs.md) を参照。

```{glossary}
asset_class
    資産クラス。商品の資産分類（equity, fund, bond, future, option, fx, crypto, cfd）。

base_currency
    基軸通貨。ペア商品（FX・暗号資産）の基軸通貨コード。

currency
    通貨コード。単一通貨商品（株式・債券等）の通貨。ISO 4217 の3文字コード。

exchange
    取引所。銘柄が上場している取引所。ISO 10383 MIC コード推奨。

quote_currency
    決済通貨。ペア商品（FX・暗号資産）の決済通貨コード。
```

## デリバティブ共通フィールド名

先物・オプション両方に適用されるフィールド名。
決定経緯は [ADR: デリバティブフィールド名](adr/field-names/derivatives.md) を参照。

```{glossary}
contract_value
    契約基本価値。契約1枚あたりの基本価値（OKX ctVal 相当）。

contract_value_currency
    契約価値の通貨。ISO 4217 コード。

multiplier
    契約乗数。1契約あたりの乗数。

tick_size
    呼値単位。最小価格変動単位。

tick_value
    ティック価値。1ティックあたりの金額変動（tick_size × multiplier）。
```

## 満期関連フィールド名

デリバティブの満期・決済に関するフィールド名。

```{glossary}
expiration_date
    満期日。権利が消滅する日（SQ日）。

expiry
    満期系列。契約の満期を識別するコード（YYYY-MM, YYYY-Www, YYYY-MM-DD）。

last_trading_day
    最終取引日。取引可能な最終日。

settlement_date
    決済日。実際に決済が行われる日。
```

## オプションフィールド名

オプション固有のフィールド名。

```{glossary}
exercise_style
    行使スタイル。american（満期前いつでも行使可能）、european（満期日のみ）、bermudan（特定日のみ）。

option_type
    オプションタイプ。call（コール）または put（プット）。

strike_price
    権利行使価格。オプションの行使価格。
```

## 取引制限フィールド名

注文数量に関する制限フィールド名。

```{glossary}
lot_size
    取引単位。注文可能な最小数量単位。

max_order_size
    最大注文数量。

min_order_size
    最小注文数量。
```

## 原資産フィールド名

デリバティブの原資産に関するフィールド名。

```{glossary}
underlying_symbol
    原資産シンボル。デリバティブの原資産を識別するシンボル。

underlying_type
    原資産タイプ。stock（株式）、index（指数）、etf、commodity、currency、crypto。
```

## 契約タイプフィールド名

暗号資産デリバティブの契約タイプに関するフィールド名。

```{glossary}
is_inverse
    インバース契約フラグ。true の場合、ベース通貨（BTC等）で決済。false の場合、linear（USDT等で決済）。

is_perpetual
    無期限契約フラグ。true の場合、満期日のない永続契約。
```

## 決済関連フィールド名

決済方法・通貨に関するフィールド名。

```{glossary}
settlement_currency
    決済通貨。決済に使用する通貨コード（ISO 4217 または暗号資産コード）。

settlement_method
    決済方法。cash（現金決済）または physical（現物受渡）。
```
