# Feature Specification: Adapter Interface Python Implementation

**Feature Branch**: `004-adapter-python`
**Parent Spec**: [004-adapter](../004-adapter/spec.md)
**Dependencies**: [003-http-client-python](../003-http-client-python/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "Python 言語でのアダプターインターフェース実装仕様を定義する"

## Clarifications

### Session 2026-02-03

- 親仕様 [004-adapter](../004-adapter/spec.md) に基づき、Python 言語固有の実装仕様を定義。
- 既存の contracts/adapter-interface.md および contracts/transforms.md を API 契約として継承。
- 既存実装が `python/src/marketschema/adapters/` に存在するため、実装仕様はこれを正式化する。

## Overview

marketschema ライブラリの Python 実装におけるアダプターフレームワーク（BaseAdapter, ModelMapping, Transforms, AdapterRegistry）を提供する。
親仕様で定義されたインターフェース契約を Python 言語の慣用的な方法で実装し、
アダプター開発者が外部データソースを marketschema の標準モデルに変換できるようにする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - BaseAdapter による非同期データ取得と変換 (Priority: P1)

Python アダプター開発者として、`BaseAdapter` を継承して新しいデータソース用のアダプターを実装し、
`async with` 構文でリソースを管理しながら、生データを marketschema の標準モデル（Quote, OHLCV, Trade, OrderBook, Instrument）に変換できる。

**Why this priority**: アダプター基盤は外部データソース統合の核心であり、すべてのアダプター実装がこれに依存する

**Independent Test**: `BaseAdapter` を継承したサンプルアダプターを実装し、`_apply_mapping()` でデータ変換ができることを確認

**Acceptance Scenarios**:

1. **Given** BaseAdapter を継承したアダプタークラス, **When** `source_name` クラス属性を定義して初期化する, **Then** インスタンスが正常に作成される
2. **Given** `get_quote_mapping()` を実装したアダプター, **When** 生データを `_apply_mapping(raw_data, mappings, Quote)` で変換する, **Then** 正しい Quote モデルインスタンスが返される
3. **Given** http_client 引数なしで初期化されたアダプター, **When** `http_client` プロパティにアクセスする, **Then** 遅延初期化された `AsyncHttpClient` インスタンスが返される
4. **Given** `async with MyAdapter() as adapter:` で使用する, **When** スコープを抜ける, **Then** HTTP クライアントが自動的にクローズされる
5. **Given** コンストラクタでカスタム `http_client` を渡す, **When** `http_client` プロパティにアクセスする, **Then** 渡されたクライアントが使用され、アダプターはクローズしない

---

### User Story 2 - ModelMapping による型安全なフィールドマッピング (Priority: P1)

Python アダプター開発者として、`ModelMapping` データクラスを使用してソースフィールドとターゲットフィールドの対応関係を定義し、
型ヒント付きの transform 関数やデフォルト値を指定できる。

**Why this priority**: フィールドマッピングはアダプター実装の基本単位であり、すべての変換処理に必須

**Independent Test**: `ModelMapping` インスタンスを作成し、`apply()` メソッドで値を取得・変換できることを確認

**Acceptance Scenarios**:

1. **Given** ドット記法のパス `"price.bid"` を指定した ModelMapping, **When** ネストした辞書 `{"price": {"bid": 100.0}}` から値を取得する, **Then** `100.0` が返される
2. **Given** `transform=Transforms.to_float` を指定した ModelMapping, **When** `apply({"value": "123.45"})` を呼び出す, **Then** `123.45` (float) が返される
3. **Given** `required=True` で値が存在しない場合, **When** `apply({"other": "value"})` を呼び出す, **Then** `MappingError` が発生する
4. **Given** `default=0.0, required=False` を指定した ModelMapping, **When** ソースに値がない, **Then** `0.0` が返される
5. **Given** ModelMapping インスタンス, **When** `frozen=True` であるため属性を変更しようとする, **Then** `FrozenInstanceError` が発生する

---

### User Story 3 - Transforms 静的メソッドによる型変換 (Priority: P1)

Python アダプター開発者として、`Transforms` クラスの静的メソッドを使用して、
文字列から数値への変換、タイムスタンプの UTC 正規化、売買方向の正規化などを行える。

**Why this priority**: 型変換は各データソースの形式差異を吸収するために不可欠

**Independent Test**: 各変換関数に対して正常系・異常系の入力でテストを実行

**Acceptance Scenarios**:

1. **Given** 文字列 `"123.45"`, **When** `Transforms.to_float("123.45")` を呼び出す, **Then** `123.45` (float) が返される
2. **Given** Unix ミリ秒タイムスタンプ `1704067200000`, **When** `Transforms.unix_timestamp_ms(1704067200000)` を呼び出す, **Then** ISO 8601 形式 `"2024-01-01T00:00:00Z"` が返される
3. **Given** JST タイムスタンプ `"2024-01-01T09:00:00"`, **When** `Transforms.jst_to_utc("2024-01-01T09:00:00")` を呼び出す, **Then** `"2024-01-01T00:00:00Z"` が返される
4. **Given** 不正な値 `"invalid"`, **When** `Transforms.to_float("invalid")` を呼び出す, **Then** `TransformError` が発生し、`__cause__` で元の例外にアクセスできる
5. **Given** 売買方向 `"BUY"`, **When** `Transforms.side_from_string("BUY")` を呼び出す, **Then** `"buy"` が返される

---

### User Story 4 - AdapterRegistry による動的管理と @register デコレータ (Priority: P2)

Python アダプター開発者として、`@register` デコレータで複数のアダプターを `AdapterRegistry` に登録し、
`source_name` で動的に取得できる。

**Why this priority**: プラグイン的な拡張性を提供し、動的なアダプター選択を可能にする

**Independent Test**: 複数のアダプターを登録し、`source_name` で正しいアダプターを取得できることを確認

**Acceptance Scenarios**:

1. **Given** `@register` デコレータを付けたアダプタークラス, **When** クラス定義が評価される, **Then** `AdapterRegistry` にクラスが登録される
2. **Given** 登録済みの `source_name="bitbank"`, **When** `AdapterRegistry.get("bitbank")` を呼び出す, **Then** 新しい `BitbankAdapter` インスタンスが返される
3. **Given** 未登録の `source_name="unknown"`, **When** `AdapterRegistry.get("unknown")` を呼び出す, **Then** `KeyError` が発生し、利用可能なアダプター名がエラーメッセージに含まれる
4. **Given** 既に登録済みの `source_name`, **When** 同じ名前で登録を試みる, **Then** `AdapterError` が発生する
5. **Given** `AdapterRegistry.list_adapters()` を呼び出す, **When** 複数のアダプターが登録されている, **Then** `["bitbank", "stooq", ...]` のようなリストが返される

---

### Edge Cases

- `source_name` が空文字列の場合 → `AdapterError("source_name must be defined")`
- ネストパス `"a.b.c"` で中間キー `"b"` が存在しない場合 → `None` を返す（`required=True` なら `MappingError`）
- `transform` 関数が例外を投げる場合 → `TransformError` としてラップし、`__cause__` で元の例外を保持
- タイムスタンプが負値の場合 → `TransformError`（`OSError` をラップ）
- `side` 文字列が未知の値 `"exchange"` の場合 → `TransformError("Cannot normalize side value: 'exchange'")`
- `ModelMapping.apply()` で `transform` が `None` を返す場合 → `None` がそのまま返される（`required=True` なら `MappingError`）

## Requirements *(mandatory)*

### Functional Requirements

#### BaseAdapter

- **FR-P001**: `BaseAdapter` クラスは `source_name: str` クラス属性を持ち、空文字列の場合は初期化時に `AdapterError` を発生させなければならない
- **FR-P002**: `BaseAdapter` クラスは `transforms: type[Transforms]` クラス属性を持ち、デフォルトで `Transforms` クラスを参照しなければならない
- **FR-P003**: `BaseAdapter.__init__()` は `http_client: AsyncHttpClient | None` 引数を受け取り、遅延初期化をサポートしなければならない
- **FR-P004**: `BaseAdapter.http_client` プロパティは初回アクセス時に `AsyncHttpClient` を遅延初期化しなければならない
- **FR-P005**: `BaseAdapter` は `async with` 構文（`__aenter__`/`__aexit__`）をサポートし、所有する HTTP クライアントを自動クローズしなければならない
- **FR-P006**: `BaseAdapter.close()` は非同期メソッドとして、所有する HTTP クライアントのみをクローズしなければならない
- **FR-P007**: `BaseAdapter` は以下のマッピングメソッドを提供しなければならない: `get_quote_mapping()`, `get_ohlcv_mapping()`, `get_trade_mapping()`, `get_orderbook_mapping()`, `get_instrument_mapping()`
- **FR-P008**: 各マッピングメソッドはデフォルトで空リスト `[]` を返し、サブクラスでオーバーライド可能でなければならない
- **FR-P009**: `BaseAdapter._apply_mapping(raw_data, mappings, model_class)` メソッドはマッピング適用とモデルインスタンス化を行い、失敗時は `AdapterError` を発生させなければならない

#### ModelMapping

- **FR-P010**: `ModelMapping` は `@dataclass(frozen=True, slots=True)` で定義された不変データクラスでなければならない
- **FR-P011**: `ModelMapping` は以下の属性を持たなければならない: `target_field: str`, `source_field: str`, `transform: Callable[[Any], Any] | None`, `default: Any | None`, `required: bool`
- **FR-P012**: `ModelMapping.apply(source_data)` メソッドはソースデータから値を取得・変換し、結果を返さなければならない
- **FR-P013**: `ModelMapping` は `source_field` でドット記法によるネストアクセスをサポートしなければならない（例: `"price.bid"` → `data["price"]["bid"]`）
- **FR-P014**: `ModelMapping` は `required=True` かつ値が存在しない場合に `MappingError` を発生させなければならない

#### Transforms

- **FR-P015**: `Transforms` クラスは以下の静的メソッドを提供しなければならない: `to_float()`, `to_int()`, `iso_timestamp()`, `unix_timestamp_ms()`, `unix_timestamp_sec()`, `jst_to_utc()`, `side_from_string()`, `uppercase()`, `lowercase()`
- **FR-P016**: すべての変換関数は変換失敗時に `TransformError` を発生させ、元の例外を `from e` で保持しなければならない（サイレント障害禁止）
- **FR-P017**: タイムスタンプ変換関数は常に UTC の ISO 8601 形式（末尾 `Z`）を返さなければならない
- **FR-P018**: `side_from_string()` は `"buy"/"sell"` を返し、未知の値には `TransformError` を発生させなければならない

#### AdapterRegistry

- **FR-P019**: `AdapterRegistry` はシングルトンパターン（クラスレベル状態）で実装しなければならない
- **FR-P020**: `AdapterRegistry.register(adapter_class)` クラスメソッドはアダプターを登録し、デコレータとしても使用可能でなければならない
- **FR-P021**: `AdapterRegistry.get(source_name)` クラスメソッドは登録されたアダプターの新規インスタンスを返さなければならない
- **FR-P022**: `AdapterRegistry.list_adapters()` クラスメソッドは登録済み `source_name` のリストを返さなければならない
- **FR-P023**: `AdapterRegistry.is_registered(source_name)` クラスメソッドは登録済みかどうかを返さなければならない
- **FR-P024**: `AdapterRegistry.clear()` クラスメソッドはすべての登録を解除しなければならない（テスト用）
- **FR-P025**: `AdapterRegistry` は重複登録を禁止し、既存の `source_name` で登録を試みた場合に `AdapterError` を発生させなければならない
- **FR-P026**: `AdapterRegistry.get()` は未登録の `source_name` に対して `KeyError` を発生させ、利用可能なアダプター名をエラーメッセージに含めなければならない

#### 例外クラス

- **FR-P027**: `AdapterError`, `MappingError`, `TransformError` は `MarketSchemaError` を継承しなければならない
- **FR-P028**: 各例外クラスは `message: str` を引数として受け取らなければならない

#### 便利関数

- **FR-P029**: `register` 関数は `AdapterRegistry.register()` に委譲するデコレータとして使用可能でなければならない

### Key Entities

- **BaseAdapter**: アダプターの基底クラス。`source_name` 属性、マッピングメソッド、`_apply_mapping()` 変換メソッド、HTTP クライアント統合を提供
- **ModelMapping**: `@dataclass(frozen=True, slots=True)` で定義された不変フィールドマッピング。`target_field`, `source_field`, `transform`, `default`, `required` 属性を持つ
- **Transforms**: 共通変換関数群を静的メソッドとして提供するクラス
- **AdapterRegistry**: シングルトンレジストリ。クラスメソッドでアダプターの登録・取得・リスト化を提供
- **register**: `@register` デコレータ関数。`AdapterRegistry.register()` のショートカット
- **AdapterError**: アダプター初期化・操作エラー
- **MappingError**: フィールドマッピングエラー（必須フィールド欠落）
- **TransformError**: 値変換エラー

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-P001**: BaseAdapter を継承したサンプルアダプターが最低 3 つ（bitbank, stooq, stockanalysis 形式）実装でき、すべてのユニットテストが通過する
- **SC-P002**: すべての Transforms 関数に対して正常系・異常系のユニットテストが存在し、通過する
- **SC-P003**: ModelMapping の `apply()` メソッドに対して、ドット記法・デフォルト値・transform・required フラグのユニットテストが通過する
- **SC-P004**: AdapterRegistry を使用した動的アダプター取得・重複登録エラー・未登録エラーのテストが通過する
- **SC-P005**: `mypy python/src/marketschema/adapters/` で型チェックエラーがゼロである
- **SC-P006**: `ruff check python/src/marketschema/adapters/` で lint エラーがゼロである
- **SC-P007**: `ruff format --check python/src/marketschema/adapters/` でフォーマットエラーがゼロである

## Assumptions

- Python 3.13 以上を使用する
- pydantic v2.0 以上を使用する（モデル定義用）
- httpx>=0.27.0 を使用する（AsyncHttpClient 経由）
- テスト用に respx モッキングライブラリを使用する
- BaseAdapter は HTTP 通信を行うアダプターを主な対象とするが、HTTP を使用しないアダプター（ファイル読み込み等）も実装可能
- 変換関数の追加は本仕様の更新を必要とするが、カスタム変換は各アダプターで自由に定義可能
- AdapterRegistry はプロセス内のシングルトンとし、マルチプロセス間の共有は考慮しない

## Language-Specific Considerations

- `async/await` 構文による非同期処理
- `async with` 構文（コンテキストマネージャ）によるリソース管理
- `raise ... from e` による例外チェイン（`__cause__` でアクセス）
- `@dataclass(frozen=True, slots=True)` による不変データクラス
- 型ジェネリクス `[T: BaseAdapter]` 構文（Python 3.12+）
- `Self` 型（Python 3.11+ `typing.Self`）によるメソッドチェイン
- クラス属性とインスタンス属性の区別

## Module Structure

```
python/src/marketschema/adapters/
├── __init__.py          # Public exports
├── base.py              # BaseAdapter implementation
├── mapping.py           # ModelMapping implementation
├── registry.py          # AdapterRegistry and @register decorator
└── transforms.py        # Transforms static methods
```

## Out of Scope

- 各データソースの具体的な API 仕様（各アダプターパッケージ/モジュールで実装）
- WebSocket や gRPC などの非 HTTP プロトコル対応
- アダプターの自動生成機能
- アダプターのバージョン管理・互換性チェック
- アダプターの非同期並列実行制御
- マルチプロセス間のレジストリ共有

## Contracts

- [Adapter Interface (Python)](contracts/adapter-interface.md) - BaseAdapter, ModelMapping, AdapterRegistry の Python 型シグネチャ
- [Transform Functions (Python)](contracts/transforms.md) - 変換関数の Python 型シグネチャ

## References

- [004-adapter](../004-adapter/spec.md) - 親仕様（言語非依存）
- [004-adapter/contracts/adapter-interface.md](../004-adapter/contracts/adapter-interface.md) - 言語非依存インターフェース契約
- [004-adapter/contracts/transforms.md](../004-adapter/contracts/transforms.md) - 言語非依存変換関数仕様
- [003-http-client-python](../003-http-client-python/spec.md) - HTTP クライアント Python 実装
- [Adapter Development Guide](../../docs/guides/adapter-development.md) - 実践的なアダプター開発チュートリアル
