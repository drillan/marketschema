# ADR: デリバティブフィールド名

## ステータス

Accepted

## 日付

2026-02-02

## コンテキスト

Instrument（銘柄情報）およびデリバティブ関連スキーマで使用するフィールド名について、業界標準を調査している。

- **決定済み**: DerivativeInfo共通、ExpiryInfo、OptionInfo、取引制限、原資産、契約タイプ、決済関連、精度

ソース番号は [情報ソース一覧](../../research/sources.md) を参照。

---

## 決定済みフィールド

### 1. DerivativeInfo フィールド名（デリバティブ共通）

先物・オプション両方に適用される共通フィールド。

#### 1.1 契約乗数: `multiplier`

**採用名:** `multiplier`

| ソース | 名称 | 型 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `ContractMultiplier` (Tag 231) | Float | [^STD-2] |
| OKX | `ctMult` (contract multiplier) | String | [^CRYPTO-4] |
| Binance | `multiplier` | String | [^CRYPTO-1] |
| CME | "multiplier" (文書内表現) | - | [^CME-MICRO] |
| CCXT | `contractSize` | number | [^CRYPTO-6] |

**用語選択: `multiplier` vs `contract_size` vs `contract_multiplier`**

| 用語 | 使用状況 | 長所/短所 |
|------|---------|----------|
| `multiplier` | Binance, CME文書 | シンプル、乗数の概念に忠実 |
| `contract_multiplier` | FIX Protocol | 明示的だが冗長 |
| `contract_size` | CCXT | 「サイズ」は曖昧（数量と混同可能） |

**決定理由:**
- FIX Protocol の `ContractMultiplier` (Tag 231) に基づく概念
- Binance、CME で `multiplier` として参照
- `contract_size` は CCXT で使用されるが、「契約数量」と混同しやすい
- 先物・オプション両方に適用（オプションでは100株=1契約など）

#### 1.2 契約価値: `contract_value`

**採用名:** `contract_value`

| ソース | 名称 | 説明 | 参照 |
|--------|------|------|------|
| OKX | `ctVal` (contract value) | 契約面値 | [^CRYPTO-4] |
| OKX | `ctValCcy` | 契約価値の通貨 | [^CRYPTO-4] |

**概念の整理:**

OKX のドキュメントによると、デリバティブ契約の想定元本 (notional value) は以下で計算される：
```
notional_value = ctVal × ctMult
```

**決定理由:**
- OKX の `ctVal` に基づく概念
- 「契約1枚あたりの基本価値」を表現
- 多くのAPIでは独立したフィールドとして持たないため任意フィールド

#### 1.3 呼値単位: `tick_size`

**採用名:** `tick_size`

| ソース | 名称 | 型 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `MinPriceIncrement` (Tag 969) | Float | [^STD-2] |
| OKX | `tickSz` | String | [^CRYPTO-4] |
| CCXT | `precision.price` | number | [^CRYPTO-6] |
| JPX | 呼値単位 | - | [^JPX-FUTURES] |

**各市場の呼値単位例:**

| 市場 | 商品 | 呼値単位 | 最小変動金額 |
|------|------|---------|-------------|
| JPX | 日経225先物（ラージ） | 10円 | 10,000円 (10円×1000) |
| JPX | 日経225mini | 5円 | 500円 (5円×100) |
| CME | E-mini S&P 500 | 0.25ポイント | $12.50 (0.25×$50) |
| CBOE | SPX オプション | 0.05/0.10 | - |

**用語選択: `tick_size` vs `min_price_increment`**

| 用語 | 使用状況 |
|------|---------|
| `tick_size` | OKX (`tickSz`), 一般的な取引用語 |
| `min_price_increment` | FIX Protocol (Tag 969) |

**決定理由:**
- `tick_size` は取引業界で広く使用される用語
- OKX の `tickSz` と対応
- 先物・オプション両方に適用

#### 1.4 ティック価値: `tick_value`

**採用名:** `tick_value`

| ソース | 名称 | 計算方法 |
|--------|------|---------|
| CME | "tick value" | tick_size × multiplier |
| 一般 | 最小変動金額 | tick_size × multiplier |

**概念:**
1ティック（最小価格変動）あたりの金額変動。

例:
- E-mini S&P 500: tick_size=0.25, multiplier=$50 → tick_value=$12.50
- 日経225mini: tick_size=5円, multiplier=100 → tick_value=500円

**決定理由:**
- `tick_size × multiplier` で計算可能だが、便利なフィールドとして提供
- 任意フィールドとする

#### 1.5 契約バリアント（Mini/Micro等）

**決定:** 独立したフィールドとして定義しない

| ソース | バリアント識別方法 |
|--------|-------------------|
| CME | シンボルで区別 (SP, ES, MES) |
| JPX | 商品コードで区別 (NK225F, NK225MF, NK225MIF) |
| CCXT | シンボル/market IDで区別 |
| OKX | instId で区別 |
| Binance | symbol で区別 |

**理由:**
- 調査した全ソースで契約バリアント（Standard/Mini/Micro）は独立したフィールドではなく、シンボル/商品コードで識別
- バリアントは `multiplier` の違いとして数値的に表現される
- 分類が必要な場合は、シンボルパターンまたは `multiplier` 値で判定可能

**バリアントと乗数の対応例:**

| 取引所 | 商品 | バリアント | 乗数 |
|--------|------|-----------|------|
| CME | S&P 500 先物 | Standard (SP) | $250 |
| CME | S&P 500 先物 | E-mini (ES) | $50 |
| CME | S&P 500 先物 | Micro E-mini (MES) | $5 |
| JPX | 日経225先物 | ラージ | ×1000 |
| JPX | 日経225先物 | mini | ×100 |
| JPX | 日経225先物 | マイクロ | ×10 |

#### 推奨フィールド定義（デリバティブ共通）

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `multiplier` | number | 必須 | 契約乗数（1契約あたりの乗数） |
| `tick_size` | number | 必須 | 呼値単位（最小価格変動） |
| `tick_value` | number | 任意 | ティック価値（1ティックあたりの金額変動） |
| `contract_value` | number | 任意 | 契約基本価値（OKX ctVal相当） |
| `contract_value_currency` | string | 任意 | 契約価値の通貨（ISO 4217） |

---

### 2. ExpiryInfo フィールド名（満期関連・デリバティブ共通）

#### 背景: 日付フィールドの概念整理

デリバティブ商品には複数の関連日付が存在し、それぞれ異なる概念を表す。

| 概念 | 説明 | 例 (J-Quants TOPIX先物) |
|------|------|------------------------|
| 満期系列 | 契約の満期を識別するコード | 月次: `2024-09`, 週次: `2024-W37` |
| 最終取引日 | 取引可能な最終日 | 2024-09-12 |
| 満期日/SQ日 | 権利が消滅する日、清算値が決定される日 | 2024-09-13 |
| 決済日 | 実際に決済が行われる日 | 2024-09-13 (現金決済の場合) |

#### 背景: 満期サイクルの多様性

現在のデリバティブ市場では多様な満期サイクルが存在する。

| サイクル | 説明 | 例 |
|---------|------|-----|
| 月次 (Monthly) | 毎月特定日 | 第2金曜日 (JPX)、第3金曜日 (CME) |
| 四半期 (Quarterly) | 3,6,9,12月 | 先物の標準限月 |
| 週次 (Weekly) | 毎週特定曜日 | 月・水・金曜日 (CME, JPX) |
| 0DTE (Daily) | 毎取引日 | CBOE SPX 0DTEオプション |
| 月末 (EOM) | 各月最終営業日 | CME EOMオプション |

JPX日経225ミニオプションでは金曜限月・水曜限月があり、CMEでは月曜・水曜・金曜の週次オプションが存在する [^EX-1][^CME-WEEKLY]。

#### 2.1 満期系列: `expiry`

**採用名:** `expiry`

| ソース | 名称 | フォーマット | 参照 |
|--------|------|-------------|------|
| FIX Protocol | `MaturityMonthYear` (Tag 200) | YYYYMM / YYYYMMDD / YYYYMMwN | [^STD-2] |
| J-Quants | `ContractMonth` | YYYY-MM / YYYY-WW (週次) | [^EX-1] |
| Interactive Brokers | `lastTradeDateOrContractMonth` | YYYYMM / YYYYMMDD | [^BRK-1] |
| CCXT | `expiry` | Unix timestamp (ms) | [^CRYPTO-6] |
| OKX | `expTime` | Unix timestamp (ms) | [^CRYPTO-4] |

**用語選択: `maturity` vs `expiry`**

| 用語 | 主な用途 | データソースでの採用 |
|------|---------|-------------------|
| `maturity` | 債券・固定収入商品 | FIX Protocol |
| `expiry`/`expiration` | オプション・先物 | CCXT, OKX, Polygon, Alpaca, CME, CBOE |

デリバティブの文脈では `expiry`/`expiration` がより一般的 [^CFI-EXPIRY]。FIX Protocol は `maturity` を使用するが、これは債券を含む全金融商品を対象とした汎用規格であるため。

**フィールド名選択: `contract_month` vs `expiry`**

`contract_month` は月次限月を前提とした名前であり、週次・日次オプションには意味的に不適合。J-Quants では `ContractMonth` フィールドに週番号（例: `2024-51`）を格納しているが、名前と内容に不整合がある。`expiry` はより汎用的で、月次・週次・日次すべてに対応可能。

**決定理由:**
- CCXT, OKX 等のモダンなデリバティブAPIで `expiry` が主流
- CME, CBOE 等の主要取引所でも "expiration" を使用
- 月次・週次・日次すべての満期サイクルに対応可能
- デリバティブの文脈で意味が明確

**推奨フォーマット (ISO 8601ベース):**

| サイクル | フォーマット | 例 |
|---------|-------------|-----|
| 月次 | YYYY-MM | `2024-09` |
| 週次 | YYYY-Www | `2024-W37` |
| 日次 | YYYY-MM-DD | `2024-09-13` |

#### 2.2 最終取引日: `last_trading_day`

**採用名:** `last_trading_day`

| ソース | 名称 | フォーマット | 参照 |
|--------|------|-------------|------|
| J-Quants | `LastTradingDay` | YYYY-MM-DD | [^EX-1] |
| Interactive Brokers | `lastTradeDate` | YYYYMMDD | [^BRK-1] |
| CME | Last Trading Day | カレンダーで管理 | [^CME-EXPIRY] |

**決定理由:**
- J-Quants の命名に準拠
- 「取引可能な最終日」を明確に表現
- 満期日 (`expiration_date`) とは異なる概念であることを明示

#### 2.3 満期日: `expiration_date`

**採用名:** `expiration_date`

| ソース | 名称 | フォーマット | 参照 |
|--------|------|-------------|------|
| J-Quants | `SpecialQuotationDay` | YYYY-MM-DD | [^EX-1] |
| FIX Protocol | `MaturityDate` (Tag 541) | LocalMktDate | [^STD-2] |
| Polygon.io | `expiration_date` | YYYY-MM-DD | [^DATA-1] |
| Alpaca | `expiration_date` | YYYY-MM-DD | [^DATA-2] |
| CCXT | `expiryDatetime` | ISO8601 | [^CRYPTO-6] |
| OKX | `expTime` | Unix timestamp (ms) | [^CRYPTO-4] |

**集計:**

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `expiration_date` | 3+ | Polygon.io, Alpaca, Schwab |
| `expiry` / `expiryDatetime` | 2 | CCXT, OKX |
| `maturity_date` | 1 | FIX Protocol |

**決定理由:**
- Polygon.io、Alpaca 等のモダンなAPI設計で標準
- snake_case で統一
- 「権利が消滅する日」を明確に表現

#### 2.4 決済日: `settlement_date`

**採用名:** `settlement_date`

| ソース | 名称 | フォーマット | 参照 |
|--------|------|-------------|------|
| FIX Protocol | `SettlDate` (Tag 64) | LocalMktDate | [^STD-2] |
| FIX Protocol | `DeliveryDate` (Tag 743) | LocalMktDate | [^STD-2] |
| OKX | `settDate` | - | [^CRYPTO-4] |
| Binance | `deliveryDate` | Unix timestamp | [^CRYPTO-1] |

**用語選択: `settlement_date` vs `delivery_date`**

| 用語 | 用途 |
|------|------|
| `settlement_date` | 現金決済を含む一般的な決済日 |
| `delivery_date` | 物理的受渡（コモディティ先物等） |

**決定理由:**
- FIX Protocol の `SettlDate` と対応
- 現金決済・物理受渡の両方をカバーする汎用的な用語
- 物理受渡が必要な場合は別途 `delivery_date` を定義可能

#### 推奨フィールド定義

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `expiry` | string | 任意 | 満期系列識別子 (YYYY-MM, YYYY-Www, YYYY-MM-DD) |
| `last_trading_day` | date | 任意 | 取引可能な最終日 |
| `expiration_date` | date | 必須 | 満期日/SQ日 |
| `settlement_date` | date | 任意 | 決済日 |

### 3. OptionInfo フィールド名（オプション固有）

#### 3.1 権利行使価格: `strike_price`

**採用名:** `strike_price`

| ソース | 名称 | 型 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `StrikePrice` (Tag 202) | Price | [^STD-2] |
| Polygon.io | `strike_price` | number | [^DATA-1] |
| Alpaca | `strike_price` | string | [^DATA-2] |
| J-Quants | `StrikePrice` | Number | [^EX-1-OPT] |
| CCXT | `strike` | number | [^CRYPTO-6] |
| OKX | `stk` | String | [^CRYPTO-4] |

**集計:**

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `strike_price` / `StrikePrice` | 4 | FIX, Polygon, Alpaca, J-Quants |
| `strike` | 1 | CCXT |
| `stk` | 1 | OKX (短縮形) |

**用語選択: `strike` vs `strike_price` vs `exercise_price`**

| 用語 | 使用状況 |
|------|---------|
| `strike_price` | FIX Protocol、Polygon、Alpaca、J-Quants で使用。最も明示的 |
| `strike` | CCXT で使用。簡潔だが曖昧な可能性 |
| `exercise_price` | 同義語として存在するが、APIでの採用例なし [^CFI-OPTIONS] |

**決定理由:**
- FIX Protocol の `StrikePrice` (Tag 202) と互換
- Polygon.io、Alpaca、J-Quants 等の主要ソースで採用
- `strike` よりも明示的で、金融用語として明確
- snake_case で統一

#### 3.2 コール/プット: `option_type`

**採用名:** `option_type`

| ソース | 名称 | 値 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `PutOrCall` (Tag 201) | 0: Put, 1: Call | [^STD-2] (deprecated, CFICodeに移行) |
| Polygon.io | `contract_type` | "put", "call", "other" | [^DATA-1] |
| Alpaca | `type` | "call", "put" | [^DATA-2] |
| J-Quants | `PutCallDivision` | "1": Put, "2": Call | [^EX-1-OPT] |
| CCXT | `optionType` | "call", "put" | [^CRYPTO-6] |
| OKX | `optType` | "C": Call, "P": Put | [^CRYPTO-4] |
| Interactive Brokers | `right` | "P", "PUT", "C", "CALL" | [^BRK-1] |

**集計:**

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `type` / `contract_type` | 2 | Alpaca, Polygon |
| `optionType` / `optType` | 2 | CCXT, OKX |
| `PutCallDivision` | 1 | J-Quants |
| `right` | 1 | Interactive Brokers |
| `PutOrCall` | 1 | FIX Protocol (deprecated) |

**フィールド名選択の考慮事項:**

| 候補 | 長所 | 短所 |
|------|------|------|
| `type` | Alpacaで使用、シンプル | 汎用的すぎる（資産タイプ等と混同） |
| `contract_type` | Polygonで使用 | 先物契約タイプとも混同可能 |
| `option_type` | 明示的、オプション固有 | 直接使用しているソースなし |
| `put_call` | FIX由来 | 値の順序が曖昧 |

**決定理由:**
- `type` は汎用的すぎて文脈依存（資産タイプ、注文タイプ等と混同しやすい）
- `contract_type` は先物の契約タイプ（PERPETUAL等）とも混同可能
- `option_type` は CCXT の `optionType` を snake_case 化した形式で、オプション固有であることが明確
- 値は小文字の `"call"`, `"put"` を推奨（Alpaca、CCXT と同様）

**推奨 Enum 値:**

| 値 | 説明 | 根拠 |
|-----|------|------|
| `call` | コールオプション（買う権利） | Alpaca, CCXT, Polygon で使用 |
| `put` | プットオプション（売る権利） | Alpaca, CCXT, Polygon で使用 |

#### 3.3 行使スタイル: `exercise_style`

**採用名:** `exercise_style`

| ソース | 名称 | 値 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `CFICode` (Tag 461) | コード内に埋め込み | [^STD-2] |
| Polygon.io | `exercise_style` | "american", "bermudan", "european" | [^DATA-1] |
| Alpaca | `style` | "american", "european" | [^DATA-2] |
| Interactive Brokers | `OptAttribute` (Tag 206) | "L": American, "S": European | [^STD-2] (deprecated) |

**集計:**

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `exercise_style` | 1 | Polygon.io |
| `style` | 1 | Alpaca |

**フィールド名選択:**

| 候補 | 長所 | 短所 |
|------|------|------|
| `style` | Alpacaで使用、シンプル | 汎用的すぎる |
| `exercise_style` | Polygonで使用、明示的 | やや長い |
| `option_style` | オプション固有 | 直接使用しているソースなし |

**決定理由:**
- `style` は汎用的すぎる（UIスタイル等と混同可能）
- `exercise_style` は Polygon.io で使用されており、「行使のスタイル」を明確に表現
- `option_style` より `exercise_style` の方が金融用語として正確（「行使」に焦点）

**推奨 Enum 値:**

| 値 | 説明 | 根拠 |
|-----|------|------|
| `american` | 満期前いつでも行使可能 | Polygon, Alpaca で使用 [^SCHWAB] |
| `european` | 満期日のみ行使可能 | Polygon, Alpaca で使用 [^SCHWAB] |
| `bermudan` | 特定の日付のみ行使可能 | Polygon で使用（稀） |

#### 推奨フィールド定義

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `strike_price` | number | 必須 | 権利行使価格 |
| `option_type` | enum | 必須 | オプションタイプ: `call`, `put` |
| `exercise_style` | enum | 任意 | 行使スタイル: `american`, `european`, `bermudan` |

### 4. 取引制限フィールド（Trading Limits）

#### 4.1 取引単位: `lot_size`

**採用名:** `lot_size`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `RoundLot` (Tag 561) | [^STD-2] |
| OKX | `lotSz` | [^CRYPTO-4] |
| Binance | `stepSize` | [^CRYPTO-1] |
| CCXT | `precision.amount` / `limits.amount` | [^CRYPTO-6] |

**用語選択: `lot_size` vs `round_lot` vs `step_size`**

| 用語 | 使用状況 | 長所/短所 |
|------|---------|----------|
| `lot_size` | OKX (`lotSz`) | ADR決定の `size` と整合、直感的 |
| `round_lot` | FIX Protocol | 伝統的金融用語だが、`lot` の意味が曖昧 |
| `step_size` | Binance | 「ステップ」は増分を示すが、取引単位としては不明確 |

**決定理由:**
- [共通フィールド名ADR](common.md) で `size` が `qty` より採用された（7ソース vs 3ソース）
- OKX の `lotSz` と整合
- 「取引単位サイズ」として意味が明確

#### 4.2 最小注文数量: `min_order_size`

**採用名:** `min_order_size`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `MinTradeVol` (Tag 562) | [^STD-2] |
| OKX | `minSz` | [^CRYPTO-4] |
| Binance | `minQty` | [^CRYPTO-1] |
| Alpaca | `min_order_size` | [^DATA-2] |
| CCXT | `limits.amount.min` | [^CRYPTO-6] |

**用語選択: `min_order_size` vs `min_trade_vol` vs `min_order_qty`**

| 用語 | 使用状況 | 長所/短所 |
|------|---------|----------|
| `min_order_size` | Alpaca, OKX (`minSz`) | ADR決定の `size` と整合 |
| `min_trade_vol` | FIX Protocol | `vol` は `volume` と混同しやすい |
| `min_order_qty` | Binance | ADR決定で `qty` より `size` が採用 |

**決定理由:**
- [共通フィールド名ADR](common.md) の決定に準拠（`size` 採用）
- Alpaca で `min_order_size` として採用
- OKX の `minSz` と整合
- `order` を含めることで「注文」の文脈が明確

#### 4.3 最大注文数量: `max_order_size`

**採用名:** `max_order_size`

| ソース | 名称 | 参照 |
|--------|------|------|
| FIX Protocol | `MaxTradeVol` (Tag 1140) | [^STD-2] |
| OKX | `maxSz` | [^CRYPTO-4] |
| Binance | `maxQty` | [^CRYPTO-1] |
| Alpaca | `max_order_size` | [^DATA-2] |
| CCXT | `limits.amount.max` | [^CRYPTO-6] |

**決定理由:**
- `min_order_size` と対称性を保つ
- [共通フィールド名ADR](common.md) の決定に準拠（`size` 採用）
- Alpaca、OKX (`maxSz`) と整合

#### 背景: `lot_size` vs `multiplier` の違い

| 概念 | 質問 | 例 |
|------|------|-----|
| `multiplier` | 1契約でいくら動く？ | E-mini: $50/ポイント |
| `lot_size` | 何契約単位で注文できる？ | 通常: 1契約単位 |

#### 推奨フィールド定義（取引制限）

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `lot_size` | number | 任意 | 取引単位（注文可能な最小数量単位） |
| `min_order_size` | number | 任意 | 最小注文数量 |
| `max_order_size` | number | 任意 | 最大注文数量 |

### 5. 原資産フィールド（Underlying）

#### 5.1 原資産シンボル: `underlying_symbol`

**採用名:** `underlying_symbol`

| ソース | 名称 | 説明 | 参照 |
|--------|------|------|------|
| FIX Protocol | `UnderlyingSymbol` (Tag 311) | 原資産の Symbol | [^STD-2] |
| Alpaca | `underlying_symbol` | 原資産シンボル（例: "AAPL"） | [^DATA-2] |
| Polygon.io | `underlying_ticker` | クエリパラメータ | [^DATA-1] |
| OKX | `uly` | 原資産インデックス（例: "BTC-USD"） | [^CRYPTO-4] |
| Interactive Brokers | `symbol` | 先物・オプションでは原資産シンボル | [^BRK-1] |
| J-Quants | `Code` + 商品分類 | コード体系で原資産を識別 | [^EX-1] |
| CCXT | `base` | ペア商品の基軸通貨/資産 | [^CRYPTO-6] |

**用語選択: `underlying` vs `underlying_symbol` vs `underlying_asset`**

| 用語 | 使用状況 | 長所/短所 |
|------|---------|----------|
| `underlying_symbol` | Alpaca, FIX Protocol | 明示的、`symbol` ADR決定と整合 |
| `underlying` | OKX (`uly`) | 簡潔だが曖昧 |
| `underlying_asset` | - | 明示的だが `symbol` との整合性なし |
| `underlying_ticker` | Polygon.io | `ticker` より `symbol` が ADR で採用済み |

**決定理由:**
- FIX Protocol の `UnderlyingSymbol` (Tag 311) と直接対応
- Alpaca で `underlying_symbol` として採用
- [銘柄情報フィールド名ADR](instrument.md) で `symbol` が `ticker` より採用された決定と整合
- 原資産が「シンボル」であることが明確

#### 5.2 原資産タイプ: `underlying_type`

**採用名:** `underlying_type`

| ソース | 名称 | 値 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `UnderlyingSecurityType` (Tag 310) | FUT, OPT, CS (Common Stock), INDEX 等 | [^STD-2] |
| Interactive Brokers | `secType` / `underlyingSecType` | STK, IND, FUT, OPT 等 | [^BRK-1] |
| CCXT | `type` | spot, future, swap, option | [^CRYPTO-6] |

**決定理由:**
- `underlying_symbol` だけでは原資産がインデックスか株式かを判別できない
- 例: `SPX`（S&P 500 インデックス）と `SPY`（S&P 500 ETF）は両方シンボル文字列
- FIX Protocol の `UnderlyingSecurityType` (Tag 310) と対応
- Interactive Brokers の `secType` と同様の概念
- スキーマ内で自己完結し、外部の銘柄マスタ参照が不要

**推奨 Enum 値:**

| 値 | 説明 | 例 |
|-----|------|-----|
| `stock` | 個別株式 | AAPL, TSLA, 7203 |
| `index` | 株価指数 | SPX, NDX, NK225, TPX |
| `etf` | 上場投資信託 | SPY, QQQ, 1321 |
| `commodity` | コモディティ | GC (金), CL (原油) |
| `currency` | 通貨 | EUR, JPY, USD |
| `crypto` | 暗号資産 | BTC, ETH |

#### 背景

オプション・先物の原資産を識別するために必要：

| デリバティブ | underlying_symbol | underlying_type |
|-------------|-------------------|-----------------|
| AAPL コールオプション | `AAPL` | `stock` |
| SPX インデックスオプション | `SPX` | `index` |
| 日経225先物 | `NK225` | `index` |
| 金先物 | `GC` | `commodity` |
| BTC-USDT-SWAP | `BTC` | `crypto` |

#### 推奨フィールド定義（原資産）

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `underlying_symbol` | string | 必須 | 原資産のシンボル |
| `underlying_type` | enum | 必須 | 原資産タイプ: `stock`, `index`, `etf`, `commodity`, `currency`, `crypto` |

### 6. 契約タイプフィールド（Contract Type）

暗号資産デリバティブで使用される契約タイプの分類。

#### 6.1 無期限契約フラグ: `is_perpetual`

**採用名:** `is_perpetual`

| ソース | 名称 | 値/判定方法 | 参照 |
|--------|------|------------|------|
| CCXT | `swap` | boolean | [^CRYPTO-6] |
| OKX | `instType` | `SWAP` = perpetual, `FUTURES` = expiring | [^CRYPTO-4] |
| Binance | `contractType` | `PERPETUAL` vs `CURRENT_QUARTER` | [^CRYPTO-1] |
| Bybit | `contractType` | `*Perpetual` を含む | [^BYBIT] |

**集計:**

| 表現方法 | 使用ソース数 | 主なソース |
|---------|-------------|-----------|
| boolean フラグ | 1 | CCXT (`swap`) |
| enum 値 | 3 | OKX (`SWAP`), Binance (`PERPETUAL`), Bybit (`LinearPerpetual`) |

**決定理由:**
- 無期限契約（perpetual）は暗号資産デリバティブの標準的な契約形態
- `expiration_date` が null で表現可能だが、明示的なフラグがあると便利
- boolean 型の `is_perpetual` は意味が明確で、検索・フィルタリングに有用
- CCXT の `swap` boolean フィールドと同等の概念

**注記:**
- 伝統的金融（FIX Protocol, ISO 10962 CFICode）には perpetual の標準定義なし
- perpetual は暗号資産固有の概念として扱う

#### 6.2 インバース契約フラグ: `is_inverse`

**採用名:** `is_inverse`

| ソース | 名称 | 値/判定方法 | 参照 |
|--------|------|------------|------|
| CCXT | `inverse` / `linear` | boolean | [^CRYPTO-6] |
| Bybit | `category` | `linear` / `inverse` | [^BYBIT] |
| OKX | ペア名 | BTC-USD = inverse, BTC-USDT = linear | [^CRYPTO-4] |
| Binance | Product line | COIN-M = inverse, USDⓈ-M = linear | [^CRYPTO-1] |

**集計:**

| 表現方法 | 使用ソース数 | 主なソース |
|---------|-------------|-----------|
| boolean フラグ | 2 | CCXT (`inverse`, `linear`), Bybit (`category`) |
| 暗黙的（ペア名/商品ライン） | 2 | OKX, Binance |

**Linear vs Inverse の違い:**

| 契約タイプ | 証拠金通貨 | 損益計算 | 例 |
|-----------|-----------|---------|-----|
| Linear | USDT/USDC | ステーブルコイン建て | BTC-USDT-SWAP |
| Inverse | BTC/ETH等 | ベース通貨建て | BTC-USD-SWAP |

**決定理由:**
- CCXT が `inverse` / `linear` の両方を boolean で提供
- `is_inverse` を採用（false = linear）
- `is_linear` も検討したが、inverse がデフォルトではないため `is_inverse` が適切
- 伝統的金融では常に linear（法定通貨建て）のため、暗号資産固有のフィールド

**注記:**
- `is_inverse = true` の場合、`settlement_currency` は通常ベース通貨（BTC等）
- `is_inverse = false` の場合、`settlement_currency` は通常ステーブルコイン（USDT等）

#### 背景: 概念の整理

契約タイプには2つの独立した軸が存在する：

| 軸 | 概念 | 値 | 関連フィールド |
|----|------|-----|---------------|
| **満期** | 契約の有効期限 | perpetual / expiring | `is_perpetual`, `expiration_date` |
| **証拠金/決済** | 損益計算・証拠金通貨 | linear / inverse | `is_inverse`, `settlement_currency` |

これらは直交する概念であり、組み合わせが可能：

| 組み合わせ | 例 |
|-----------|-----|
| Linear + Perpetual | BTC-USDT-SWAP（Binance USDⓈ-M Perpetual） |
| Inverse + Perpetual | BTC-USD-SWAP（Binance COIN-M Perpetual） |
| Linear + Expiring | BTC-USDT-231229（Binance USDⓈ-M Quarterly） |
| Inverse + Expiring | BTC-USD-231229（Binance COIN-M Quarterly） |

#### 推奨フィールド定義（契約タイプ）

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `is_perpetual` | boolean | 任意 | 無期限契約か否か（暗号資産デリバティブ向け） |
| `is_inverse` | boolean | 任意 | インバース契約か否か（false = linear、暗号資産デリバティブ向け） |

### 7. 決済関連フィールド（Settlement）

#### 7.1 決済方法: `settlement_method`

**採用名:** `settlement_method`

| ソース | 名称 | 値 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `SettlMethod` (Tag 1193) | C: Cash, P: Physical | [^FIX-SETTLMETHOD] |
| CME | Settlement Type | Cash / Physical Delivery | [^CME-SETTLEMENT] |
| JPX | 決済方法 | 現金決済 / 受渡決済 | [^JPX-FUTURES] |

**決定理由:**
- FIX Protocol の `SettlMethod` (Tag 1193) に基づく
- CME、JPX 等の主要取引所で使用される概念
- `settlement_method` は snake_case で意味が明確

**推奨 Enum 値:**

| 値 | 説明 | 例 |
|-----|------|-----|
| `cash` | 現金決済（差金決済） | 株価指数先物、暗号資産デリバティブ |
| `physical` | 現物受渡 | コモディティ先物、FX先物（一部） |

**商品タイプ別の決済方法:**

| 商品タイプ | 決済方法 | 備考 |
|-----------|---------|------|
| 株価指数先物・オプション | `cash` | SQ値で現金決済 |
| 個別株オプション | `physical` | 株式の受渡 |
| コモディティ先物 | `cash` or `physical` | 商品により異なる |
| FX先物 | `physical` or `cash` | 通貨ペアにより異なる |
| 暗号資産デリバティブ | `cash` | 常に現金決済 |

#### 7.2 決済通貨: `settlement_currency`

**採用名:** `settlement_currency`

| ソース | 名称 | 例 | 参照 |
|--------|------|-----|------|
| FIX Protocol | `SettlCurrency` (Tag 120) | USD, EUR | [^STD-2] |
| CCXT | `settle` (シンボル形式) | `ETH/USDT:USDT` → settle=USDT | [^CRYPTO-6] |
| OKX | `settleCcy` | USDT, BTC, USD | [^CRYPTO-4] |
| Binance | `marginAsset` | USDT, BTC | [^CRYPTO-1] |

**集計:**

| フィールド名 | 使用ソース数 | 主なソース |
|-------------|-------------|-----------|
| `settle` / `settleCcy` | 2 | CCXT, OKX |
| `marginAsset` | 1 | Binance |
| `SettlCurrency` | 1 | FIX Protocol |

**決定理由:**
- FIX Protocol の `SettlCurrency` (Tag 120) を snake_case 化
- OKX の `settleCcy` と同等の概念
- `margin_asset` より `settlement_currency` の方が決済の文脈で明確
- ADR の `currency` フィールド命名規則と整合

**`is_inverse` との関係:**

| `is_inverse` | `settlement_currency` の典型値 |
|--------------|-------------------------------|
| `false` (linear) | USDT, USDC, USD |
| `true` (inverse) | BTC, ETH 等のベース通貨 |

**値の形式:**
- 法定通貨: ISO 4217 コード（USD, EUR, JPY）
- 暗号資産: 標準的なティッカー（BTC, ETH, USDT）

#### 推奨フィールド定義（決済関連）

| フィールド名 | 型 | 必須 | 説明 |
|-------------|-----|------|------|
| `settlement_method` | enum | 任意 | 決済方法: `cash`, `physical` |
| `settlement_currency` | string | 任意 | 決済通貨（ISO 4217 または暗号資産コード） |

**注記:**
- 暗号資産デリバティブでは `settlement_method` は常に `cash` のため省略可能
- `settlement_currency` は `is_inverse` から推測可能な場合もあるが、明示的に指定することを推奨

### 8. 精度フィールド（Precision）

**決定:** 独立したフィールドとして定義しない

#### 調査結果

| ソース | 価格精度 | 数量精度 | 参照 |
|--------|---------|---------|------|
| CCXT | `precision.price` | `precision.amount` | [^CRYPTO-6] |
| Binance | `pricePrecision` / `tickSize` | `quantityPrecision` / `stepSize` | [^CRYPTO-1] |
| OKX | `tickSz` | `lotSz` | [^CRYPTO-4] |

#### 精度の表現方法

精度には2つの表現方法がある：

| 表現 | 例 | 説明 |
|------|-----|------|
| 小数桁数（整数） | `2` | 小数点以下2桁 |
| ティックサイズ（小数） | `0.01` | 最小変動単位 |

**変換関係:** `precision = -log10(tick_size)`

| tick_size | precision |
|-----------|-----------|
| 0.1 | 1 |
| 0.01 | 2 |
| 0.001 | 3 |

#### 決定理由

- **価格精度**: `tick_size`（呼値）から導出可能
- **数量精度**: `lot_size`（売買単位）から導出可能（暗号資産の場合）
- 伝統的金融では数量は通常整数（1契約、100株単位）
- 独立したフィールドを追加する必要性が低い
- 必要な場合はアプリケーション側で計算可能

#### 関連フィールド

| 概念 | 既存フィールド | 精度の導出 |
|------|--------------|-----------|
| 価格精度 | `tick_size` | `price_precision = -log10(tick_size)` |
| 数量精度 | `lot_size` | `quantity_precision = -log10(lot_size)` |

---

## 参考資料

[^STD-2]: [FIX Protocol Field Tags](https://www.onixs.biz/fix-dictionary/4.4/fields_by_tag.html)
[^BRK-1]: [Interactive Brokers TWS API](https://interactivebrokers.github.io/tws-api/classIBApi_1_1Contract.html)
[^DATA-1]: [Polygon.io Options API](https://massive.com/docs/rest/options/contracts/all-contracts)
[^DATA-2]: [Alpaca Options Trading](https://docs.alpaca.markets/docs/options-trading)
[^EX-1]: [J-Quants API 先物四本値](https://jpx.gitbook.io/j-quants-ja/api-reference/futures)
[^EX-1-OPT]: [J-Quants API オプション四本値](https://jpx.gitbook.io/j-quants-ja/api-reference/options)
[^CRYPTO-1]: [Binance Futures API](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information)
[^CRYPTO-4]: [OKX API](https://www.okx.com/docs-v5/en/#public-data-rest-api-get-instruments)
[^CRYPTO-6]: [CCXT Manual](https://github.com/ccxt/ccxt/wiki/Manual#market-structure)
[^BYBIT]: [Bybit API - Get Instruments Info](https://bybit-exchange.github.io/docs/v5/market/instrument)
[^FIX-SETTLMETHOD]: [FIX Protocol SettlMethod (Tag 1193)](https://www.onixs.biz/fix-dictionary/5.0.sp2/tagnum_1193.html)
[^CME-SETTLEMENT]: [CME Cash Settlement vs Physical Delivery](https://www.cmegroup.com/articles/2025/cash-settlement-vs-physical-delivery.html)
[^CME-EXPIRY]: [CME Futures Expiration and Settlement](https://www.cmegroup.com/education/courses/introduction-to-futures/get-to-know-futures-expiration-and-settlement)
[^CME-WEEKLY]: [CME Weekly Options FAQ](https://www.cmegroup.com/trading/equity-index/weekly-eom-options-faq.html)
[^CFI-EXPIRY]: [Corporate Finance Institute - Expiration Date](https://corporatefinanceinstitute.com/resources/derivatives/expiration-date-derivatives/)
[^CFI-OPTIONS]: [Corporate Finance Institute - Options: Calls and Puts](https://corporatefinanceinstitute.com/resources/derivatives/options-calls-and-puts/)
[^SCHWAB]: [Schwab Options Expiration](https://www.schwab.com/learn/story/options-expiration-definitions-checklist-more)
[^CME-MICRO]: [CME Micro E-mini Equity Index Futures FAQ](https://www.cmegroup.com/articles/faqs/micro-e-mini-equity-index-futures-frequently-asked-questions.html)
[^JPX-FUTURES]: [JPX 日経225マイクロ先物](https://www.jpx.co.jp/derivatives/products/domestic/225micro-futures/01.html)
