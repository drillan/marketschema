<!--
SYNC IMPACT REPORT
==================
Version Change: 0.4.0 → 0.5.0

Modified Principles:
- II. 軽量コア: 共通HTTPクライアントをコアに含めることを明記、「通信処理」の除外を撤回

Added Sections:
- Scope Definition に「共通HTTPクライアント」を追加
- 「コアに含めるもの」に共通インフラストラクチャの説明を追加
- Out of Scope を「個別データソース固有の実装」として明確化

Removed Sections:
- Out of Scope から「データソース接続」を削除（コアの共通HTTPで対応するため）

Templates Requiring Updates:
- .specify/templates/plan-template.md ✅ (Constitution Check セクションは動的に評価)
- .specify/templates/spec-template.md ✅ (変更不要)
- .specify/templates/tasks-template.md ✅ (変更不要)
- specs/003-http-client/features/http-client-layer/plan.md ⚠️ (Constitution改訂案を削除すべき - 本改訂で対応済み)

Follow-up TODOs:
- HTTP クライアント実装後、TDD Application Scope に http モジュールを追加検討
==================
-->

# marketschema Constitution

プロジェクトの基本原則を定義する。すべての設計判断はこの原則に基づいて行う。

## Mission

> 金融マーケットデータの取得・分析を、データソースに依存しない統一的な方法で行えるようにする

## Core Principles

### I. Schema First

JSON Schema が単一の真実の源（Single Source of Truth）である。

- すべてのデータモデルは JSON Schema で定義する
- 各言語のコードはスキーマから自動生成する
- スキーマとコードの不整合を許容しない

**Rationale**: スキーマを唯一の定義元とすることで、言語間の一貫性を保証し、
ドキュメントと実装の乖離を防ぐ。

### II. 軽量コア

コアは最小限に留めつつ、アダプター実装に必要な共通インフラストラクチャを提供する。

- **コアに含めるもの**:
  - スキーマ定義
  - 基底アダプタークラス
  - 共通変換関数
  - 共通 HTTP クライアント（データソース非依存のインフラストラクチャ）

- **コアに含めないもの**:
  - 個別データソース向けアダプター（bitbank, binance, stooq 等）
  - 認証処理（API キー管理、OAuth 等）
  - データソース固有のビジネスロジック

**共通インフラストラクチャの定義**:
共通 HTTP クライアントは、アダプター実装の 80% 以上で必要とされる汎用機能であり、
認証やデータソース固有のロジックを含まない。オプショナル依存として提供し、
必要なユーザーのみがインストールできる形式とする。

**Rationale**: コアを小さく保ちながらも、実用的なアダプター開発を支援する。
共通インフラを提供することで、個別アダプターの重複実装を防ぎ、
プロジェクト全体の品質と一貫性を向上させる。

### III. シンプルさ優先

80% のユースケースに最適化する。

- 複雑な要件は、ユーザーによるオーバーライドで対応
- 過度な抽象化・汎用化を避ける
- 「動くコード」を「美しいコード」より優先する
- YAGNI: 必要になるまで作らない

**Rationale**: シンプルなAPIは学習コストを下げ、採用障壁を低くする。

### IV. 言語非依存

特定のプログラミング言語に依存しない。

- JSON Schema による言語中立な定義
- 各言語で同等の機能を提供
- 言語固有の最適化はその言語パッケージ内で行う

**Rationale**: ユーザーは自分の得意な言語でmarketschemaを利用できる。

### V. エコシステム拡張

業者対応はコア外で行う。

- 各データソース向けアダプターは独立したパッケージまたは examples として提供
- コアへの依存は最小限に
- サードパーティによる拡張を歓迎する

**Rationale**: コミュニティによる拡張を促進し、多様なデータソースへの対応を可能にする。

## Scope Definition

### In Scope

| カテゴリ | 内容 |
|---------|------|
| データモデル | Quote, OHLCV, Trade, OrderBook, Instrument 等 |
| アダプター基盤 | BaseAdapter, ModelMapping, 共通変換関数 |
| 共通 HTTP クライアント | AsyncHttpClient（リトライ、タイムアウト、レート制限対応） |
| 対象商品 | 株式、投信、債券、デリバティブ、FX、暗号資産、CFD |
| コード生成 | JSON Schema から各言語へのモデル生成 |

### Out of Scope

| カテゴリ | 理由 |
|---------|------|
| 発注・約定管理 | 別ドメイン。精度・信頼性要件が異なる |
| ポートフォリオ管理 | 別ドメイン |
| リスク計算 | 別ドメイン |
| 会計・税務処理 | 別ドメイン |
| 個別データソースの API 仕様 | 各アダプターパッケージで実装 |
| 認証・認可 | 各アダプターパッケージで実装 |

## Compatibility Policy

セマンティックバージョニング（SemVer）を採用する。

### Version Number Semantics

| バージョン | 変更内容 |
|-----------|---------|
| MAJOR (x.0.0) | 破壊的変更（後方互換性なし） |
| MINOR (0.x.0) | 機能追加（後方互換性あり） |
| PATCH (0.0.x) | バグ修正（後方互換性あり） |

### Breaking Change Rules

- 破壊的変更はメジャーバージョンでのみ行う
- 非推奨化（deprecation）から削除まで最低1メジャーバージョンの猶予を設ける
- 破壊的変更は CHANGELOG に明記する

### Backward Compatible Changes

以下はマイナーバージョンで行える：

- 新しいオプショナルフィールドの追加
- 新しいモデルの追加
- 新しい変換関数の追加
- enum への新しい値の追加
- 新しいオプショナル依存（extras）の追加

## Defaults and Extensibility

「合理的なデフォルト」を提供しつつ、ユーザーによるオーバーライドを可能にする。

| 項目 | デフォルト | オーバーライド方法 |
|------|-----------|-------------------|
| 数値型 | `float` | カスタム変換関数で `Decimal` へ |
| タイムスタンプ | ISO 8601 (UTC) | アダプターで任意形式から変換 |
| NULL 許容 | required フィールドは非 NULL | スキーマで oneOf を使用 |
| バリデーション | スキーマによる型検証 | カスタムバリデータを追加可能 |
| HTTP クライアント | httpx（オプショナル） | `pip install marketschema[http]` |

## Development Workflow

Kent Beck の TDD（テスト駆動開発）サイクルに従う。

```
    ┌─────────────────────────────────────┐
    │                                     │
    ▼                                     │
┌───────┐     ┌───────┐     ┌──────────┐  │
│  Red  │────▶│ Green │────▶│ Refactor │──┘
└───────┘     └───────┘     └──────────┘
```

### Red（レッド）

失敗するテストを先に書く。

- 実装前にテストを書くことで、仕様を明確にする
- テストが失敗することを確認してから次へ進む

### Green（グリーン）

テストを通す最小限のコードを書く。

- 「動くコード」を最優先
- 完璧を目指さない。まず通すことに集中

### Refactor（リファクタ）

テストが通る状態を維持しながら、コードを改善する。

- 重複を排除
- 可読性を向上
- テストが壊れたら即座に修正

### TDD Application Scope

| 対象 | TDD適用 |
|------|---------|
| コアライブラリ | 必須 |
| 変換関数 | 必須 |
| HTTP クライアント | 必須 |
| アダプター | 推奨 |
| コード生成スクリプト | 推奨 |

## Prohibited Practices

### 命名の揺れの禁止（Naming Consistency）

同一の概念に対して異なる名前を付けてはならない。

```python
# NG: 同じデータに異なる名前
class Order:
    qty: float       # ある場所では qty
    quantity: float  # 別の場所では quantity

class Trade:
    amount: float    # さらに別の場所では amount

# OK: 統一された命名
class Order:
    quantity: float

class Trade:
    quantity: float
```

**原則:**

- プロジェクト全体で同一概念には同一名称を使用する
- 略語を使う場合は、その略語のみを使用する（`qty` と `quantity` の混在は禁止）
- 命名規則は用語集（Glossary）で定義し、スキーマの `description` で参照する
- 新しいフィールドを追加する前に、既存の命名を確認する

**違反例:**

| 概念 | NG（混在） | OK（統一） |
|------|-----------|-----------|
| 数量 | `qty`, `quantity`, `amount` | `size` |
| 価格 | `price`, `px`, `value` | `price` |
| 時刻 | `time`, `timestamp`, `ts`, `datetime` | `timestamp` |
| 識別子 | `id`, `ID`, `identifier`, `code` | `id` |

**Rationale**: 命名の揺れはコードの可読性を下げ、バグの温床となる。
統一された命名は検索性を高め、ドメイン知識の共有を促進する。

### 業界標準名の採用（Industry Standard Naming）

フィールド名は業界標準または最も一般的に使用されている名称を採用しなければならない。

**原則:**

- 金融業界で広く認知されている用語を優先する
- 複数の一次ソース（FIX Protocol、主要取引所API、データベンダー）で
  一致している名称を採用する
- 独自命名は、対応する業界標準が存在しない場合にのみ許容する
- 新しいフィールドを追加する前に、業界標準名を調査する

**確立された業界標準名:**

以下は複数の一次ソースで一致が確認された標準名である。
決定経緯は [ADR](docs/adr/index.md) を参照。

| 概念 | 標準名 | 根拠 |
|------|--------|------|
| 買い気配値 | `bid` | FIX, IB, Binance, Coinbase で統一 |
| 売り気配値 | `ask` | 同上 |
| 始値 | `open` | 全ソースで統一 |
| 高値 | `high` | 同上 |
| 安値 | `low` | 同上 |
| 終値 | `close` | 同上 |
| 出来高 | `volume` | 同上 |
| 約定価格 | `price` | 広く統一 |
| 売買方向 | `side` | FIX, Binance, Coinbase, Kraken で統一 |
| 買い板情報 | `bids` | Binance, Coinbase, Kraken で統一 |
| 売り板情報 | `asks` | 同上 |
| タイムスタンプ | `timestamp` | 多数派 |
| 約定数量 | `size` | FIX (Size), IB, Coinbase, Polygon, Alpaca で多数派 |
| 買い気配数量 | `bid_size` | FIX, IB, Polygon, Alpaca, Yahoo で多数派 |
| 売り気配数量 | `ask_size` | 同上 |
| 売買代金 | `quote_volume` | CCXT 標準、Binance で採用 |
| 銘柄識別子 | `symbol` | FIX (Tag 55) 標準、多数派 |

**フィールド名決定プロセス:**

新しいフィールドを追加する場合、または業界内で複数の用語が併用されている場合、
以下のプロセスに従う。詳細は [ADR: 決定プロセス](docs/adr/index.md) を参照。

1. **情報ソースの調査**
   - 標準プロトコル（FIX Protocol）
   - 取引所（JPX, NYSE, CME, 暗号資産取引所）
   - ブローカー（Interactive Brokers, Schwab, E*TRADE）
   - FX業者（OANDA, IG, Saxo Bank）
   - データプロバイダー（Polygon, Alpaca）

2. **ソースの一元化**
   - [情報ソース一覧](docs/research/sources.md) に番号付きで登録
   - 各ADRでは同じ番号で footnote を定義

3. **集計と決定**
   - 各フィールドについてソースごとの名称を調査
   - 多数派を採用（明確な根拠がある場合はそれに従う）
   - ADR として決定理由を記録

4. **用語集への登録**
   - [用語集](docs/glossary.md) に追加
   - プロジェクト全体で使用

**参照すべき一次ソース:**

情報ソース一覧は [docs/research/sources.md](docs/research/sources.md) で管理する。
主要なソースは以下の通り：

- [FIX Protocol](https://www.fixtrading.org/) - MDEntryType, OfferPx 等
- [Interactive Brokers TWS API](https://interactivebrokers.github.io/tws-api/tick_types.html)
- [Binance API](https://developers.binance.com/docs/binance-spot-api-docs)
- [Coinbase API](https://docs.cdp.coinbase.com/)

**Rationale**: 業界標準名を使用することで、他システムとの統合が容易になり、
金融業界の専門家がコードを理解しやすくなる。

### 暗黙的フォールバックの禁止

エラーを握りつぶしてデフォルト値で処理してはならない。

```python
# NG: 暗黙的フォールバック
def to_float(value):
    try:
        return float(value)
    except:
        return 0.0  # エラーを隠蔽している

# OK: 明示的なエラー
def to_float(value):
    try:
        return float(value)
    except (ValueError, TypeError) as e:
        raise ConversionError(f"Cannot convert {value!r} to float") from e

# OK: 明示的なオプション
def to_float_or_none(value) -> float | None:
    """None を返す可能性があることが明示されている"""
    if value is None or value == "":
        return None
    return float(value)
```

**原則:**

- 失敗は明示的に報告する
- デフォルト値を返す設計の場合は、関数名・型で明示する（例: `_or_none`, `_or_default`）
- 例外をキャッチする場合は、具体的な例外型を指定する

### ハードコードの禁止

マジックナンバーや固定値をコードに埋め込んではならない。

```python
# NG: ハードコード
def convert_timestamp(ms):
    return ms / 1000  # 1000 が何を意味するか不明

def get_orderbook(data):
    for i in range(10):  # なぜ 10 なのか不明
        ...

# OK: 定数として定義
MS_PER_SECOND = 1000

def convert_timestamp(ms):
    return ms / MS_PER_SECOND

# OK: 設定可能にする
def get_orderbook(data, depth: int = 10):
    for i in range(depth):
        ...
```

**原則:**

- 数値・文字列リテラルには名前を付ける
- 設定値は引数または設定ファイルで外部化する
- 例外: `0`, `1`, `""`, `None` など自明なリテラルは許容

## Quality Standards

### Schema

- JSON Schema Draft 2020-12 準拠
- すべてのフィールドに description を記述
- 機械的に検証可能であること

### Code

- 各言語のイディオム・規約に従う
- 型ヒント / 型注釈を必須とする
- 自動生成コードは手動編集しない

### Adapter

- 入力検証を行い、不正データは明確なエラーで拒否
- 変換ロジックはテスト可能な単位で実装
- データソース固有の例外を共通例外にラップ

### HTTP Client

- 非同期（async/await）を基本とする
- タイムアウトを必須設定とし、無限待ちを防止
- リトライはべき等な操作（GET 等）に限定
- HTTP エラーは明示的な例外として伝播

## Priority of Principles

設計判断で迷った場合、以下の優先順位に従う：

1. **正確性** - データが正しく変換されること
2. **シンプルさ** - 理解しやすく、使いやすいこと
3. **互換性** - 既存コードを壊さないこと
4. **パフォーマンス** - 十分に高速であること
5. **機能性** - より多くのユースケースをカバーすること

## Governance

### Amendment Procedure

1. 変更提案は Issue または PR で提出する
2. 変更理由と影響範囲を明記する
3. レビューを経て承認後、憲法を更新する
4. 更新後は依存ドキュメント（テンプレート等）も同期する

### Versioning Policy

憲法のバージョニングは SemVer に従う：

- **MAJOR**: 原則の削除、非互換な再定義
- **MINOR**: 新しい原則・セクションの追加、大幅な拡張
- **PATCH**: 文言修正、タイポ修正、非意味的な改善

### Compliance Review

- すべての PR/レビューは憲法への準拠を確認する
- 複雑さを追加する場合は正当化が必要
- 原則に違反する場合は、明示的な例外として文書化する

**Version**: 0.5.0 | **Ratified**: 2026-02-02 | **Last Amended**: 2026-02-03
