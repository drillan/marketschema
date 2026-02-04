# アーキテクチャ

marketschema のアーキテクチャを解説する。

## 設計原則

### Schema First

JSON Schema を単一の真実の源（Single Source of Truth）とする。スキーマから Python と Rust のコードを自動生成することで、言語間の一貫性を保証する。

```{mermaid}
flowchart TB
    subgraph schema["スキーマ層"]
        JSON["JSON Schema<br/>(schemas/)"]
    end

    subgraph codegen["コード生成"]
        PY["Python<br/>datamodel-codegen"]
        RS["Rust<br/>cargo-typify"]
    end

    subgraph models["データモデル層"]
        PYM["Python Models<br/>(pydantic v2)"]
        RSM["Rust Models<br/>(serde)"]
    end

    JSON --> PY
    JSON --> RS
    PY --> PYM
    RS --> RSM
```

## レイヤー構造

marketschema は5層のレイヤーで構成される。

```{mermaid}
flowchart TB
    subgraph app["アプリケーション層"]
        USER["ユーザーコード"]
    end

    subgraph adapter["アダプター層"]
        BA["BaseAdapter"]
        MM["ModelMapping"]
        TF["Transforms"]
    end

    subgraph http["HTTP クライアント層"]
        HC["AsyncHttpClient"]
        MW["Middleware<br/>(Retry, RateLimit, Cache)"]
    end

    subgraph model["データモデル層"]
        Q["Quote"]
        O["OHLCV"]
        T["Trade"]
        OB["OrderBook"]
        I["Instrument"]
    end

    subgraph schema["スキーマ層"]
        JS["JSON Schema"]
    end

    USER --> BA
    BA --> HC
    BA --> MM
    MM --> TF
    HC --> MW
    TF --> Q & O & T & OB & I
    Q & O & T & OB & I -.->|generated from| JS
```

### 各層の責務

| 層 | 責務 | 主なコンポーネント |
|-----|------|-------------------|
| アプリケーション | ユーザーのビジネスロジック | ユーザーコード |
| アダプター | 外部データの変換とマッピング | BaseAdapter, ModelMapping, Transforms |
| HTTP クライアント | HTTP 通信、リトライ、レートリミット | AsyncHttpClient, Middleware |
| データモデル | 型安全なデータ構造 | Quote, OHLCV, Trade, OrderBook, Instrument |
| スキーマ | データ定義（Single Source of Truth） | JSON Schema |

## データフロー

外部 API からアダプターを経由してモデルに変換されるまでの流れ。

```{mermaid}
flowchart LR
    subgraph external["外部"]
        API["外部 API"]
    end

    subgraph transport["Transport 層"]
        HTTP["HTTP Client<br/>get_json / get_text"]
    end

    subgraph extract["Extract 層"]
        PARSE["JSON Parse<br/>CSV Parse<br/>HTML Parse"]
    end

    subgraph transform["Transform 層"]
        MAP["ModelMapping<br/>フィールドマッピング"]
        CONV["Transforms<br/>型変換"]
    end

    subgraph output["Output"]
        MODEL["marketschema<br/>Models"]
    end

    API --> HTTP
    HTTP --> PARSE
    PARSE --> MAP
    MAP --> CONV
    CONV --> MODEL
```

### 変換の流れ

1. **Transport**: HTTP クライアントが外部 API を呼び出し
2. **Extract**: レスポンスをパース（JSON, CSV, HTML）
3. **Transform**: ModelMapping でフィールドをマッピングし、Transforms で型変換
4. **Output**: pydantic/serde による検証を経て型安全なモデルを生成

## ディレクトリ構成

```{mermaid}
flowchart TB
    subgraph root["marketschema/"]
        SCHEMAS["schemas/<br/>JSON Schema 定義"]
        DOCS["docs/<br/>ドキュメント"]
        SPECS["specs/<br/>仕様書"]

        subgraph python["python/"]
            PY_MODELS["models/<br/>自動生成モデル"]
            PY_HTTP["http/<br/>HTTP クライアント"]
            PY_ADAPTERS["adapters/<br/>アダプター基盤"]
            PY_EXAMPLES["examples/<br/>実装例"]
        end

        subgraph rust["rust/ + crates/"]
            RS_MODELS["types/<br/>自動生成モデル"]
            RS_HTTP["marketschema-http/<br/>HTTP クライアント"]
            RS_ADAPTERS["marketschema-adapters/<br/>アダプター基盤"]
        end
    end

    SCHEMAS -->|"datamodel-codegen"| PY_MODELS
    SCHEMAS -->|"cargo-typify"| RS_MODELS
```

## 言語実装

Python と Rust で同等の機能を提供する。

| レイヤー | Python | Rust |
|---------|--------|------|
| スキーマ | `schemas/` (共通) | `schemas/` (共通) |
| モデル | `python/src/marketschema/models/` | `rust/src/types/` |
| HTTP | `python/src/marketschema/http/` | `crates/marketschema-http/` |
| アダプター | `python/src/marketschema/adapters/` | `crates/marketschema-adapters/` |

### 言語間の対応

| 機能 | Python | Rust |
|-----|--------|------|
| モデル基盤 | pydantic v2 | serde |
| HTTP クライアント | httpx (async) | reqwest (async) |
| バリデーション | pydantic validator | serde deserialize |
| 不正フィールド拒否 | `extra="forbid"` | `deny_unknown_fields` |

## 関連ドキュメント

- [コード生成ガイド](code-generation.md) - スキーマからのコード生成方法
- [アダプター開発ガイド](guides/adapter-development.md) - アダプター実装パターン
- [HTTP クライアント使用ガイド](guides/http-client.md) - HTTP クライアントの使い方
- [モデル実装ガイド](guides/models.md) - モデルの使い方
