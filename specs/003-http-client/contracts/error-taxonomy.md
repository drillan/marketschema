# HTTP Error Taxonomy

**Feature**: 003-http-client
**Date**: 2026-02-03
**Status**: Active

## Overview

本ドキュメントは HTTP クライアントのエラー分類を言語非依存で定義する。
各言語の実装者はこの分類に従ってエラー型を実装すること。

## Error Hierarchy

```
MarketSchemaError (base)
└── HttpError
    ├── HttpTimeoutError
    ├── HttpConnectionError
    └── HttpStatusError
        └── HttpRateLimitError
```

## Error Types

### HttpError (Base)

HTTP 操作に関連するすべてのエラーの基底クラス。

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | string | Yes | エラーの説明 |
| `url` | string | No | エラーが発生した URL |

**Trigger Conditions**:
- すべての HTTP 関連エラーで発生
- サブクラスが存在する場合は、より具体的なサブクラスを使用すること

---

### HttpTimeoutError

リクエストがタイムアウトした場合に発生。

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | string | Yes | エラーの説明 |
| `url` | string | No | エラーが発生した URL |

**Trigger Conditions**:
- 接続タイムアウト（サーバーへの接続が確立できない）
- 読み取りタイムアウト（レスポンスの読み取りが完了しない）
- 書き込みタイムアウト（リクエストの送信が完了しない）

**Retryable**: Yes（一時的な問題の可能性が高い）

---

### HttpConnectionError

サーバーへの接続が失敗した場合に発生。

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | string | Yes | エラーの説明 |
| `url` | string | No | エラーが発生した URL |

**Trigger Conditions**:
- DNS 解決失敗
- TCP 接続拒否
- TLS ハンドシェイク失敗
- ネットワーク到達不能

**Retryable**: Depends（ネットワークの一時的な問題の場合は Yes）

---

### HttpStatusError

HTTP レスポンスのステータスコードがエラーを示す場合に発生。

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | string | Yes | エラーの説明 |
| `url` | string | No | エラーが発生した URL |
| `status_code` | integer | Yes | HTTP ステータスコード |
| `response_body` | string | No | レスポンスボディ（利用可能な場合） |

**Trigger Conditions**:
- HTTP 4xx（クライアントエラー）
- HTTP 5xx（サーバーエラー）

**Note**: ステータスコード 429 の場合は `HttpRateLimitError` を使用すること。

---

### HttpRateLimitError

レート制限に達した場合に発生（HTTP 429 Too Many Requests）。

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | string | Yes | エラーの説明 |
| `url` | string | No | エラーが発生した URL |
| `status_code` | integer | Yes | 常に 429 |
| `response_body` | string | No | レスポンスボディ |
| `retry_after` | float | No | 再試行までの待機秒数（`Retry-After` ヘッダーから取得） |

**Trigger Conditions**:
- HTTP 429 Too Many Requests

**Retryable**: Yes（`retry_after` 秒後に再試行可能）

## Status Code Classification

### Retryable Status Codes

以下のステータスコードは自動リトライの対象とする:

| Code | Name | Reason |
|------|------|--------|
| 429 | Too Many Requests | レート制限。`Retry-After` に従って待機後リトライ |
| 500 | Internal Server Error | サーバー側の一時的なエラーの可能性 |
| 502 | Bad Gateway | プロキシ/ゲートウェイの一時的なエラー |
| 503 | Service Unavailable | サービスの一時的な過負荷/メンテナンス |
| 504 | Gateway Timeout | プロキシ/ゲートウェイのタイムアウト |

### Non-Retryable Status Codes

以下のステータスコードは自動リトライを行わない:

| Code | Name | Reason |
|------|------|--------|
| 400 | Bad Request | リクエストの構文エラー。修正が必要 |
| 401 | Unauthorized | 認証が必要。認証情報の更新が必要 |
| 403 | Forbidden | アクセス権限がない。設定変更が必要 |
| 404 | Not Found | リソースが存在しない。URL の確認が必要 |
| 405 | Method Not Allowed | HTTP メソッドが許可されていない |
| 409 | Conflict | リソースの競合。アプリケーション側での対応が必要 |
| 410 | Gone | リソースが永久に削除された |
| 422 | Unprocessable Entity | バリデーションエラー |

## Exception Chaining Requirements

すべての HTTP 例外は、元の例外（HTTP ライブラリが発生させた例外）への参照を保持しなければならない。

### Purpose

1. **デバッグ**: 元の例外のスタックトレースとエラーメッセージを保持
2. **診断**: HTTP ライブラリ固有の情報にアクセス可能
3. **ログ**: 完全なエラーコンテキストをログに記録可能

### Implementation Patterns

**Python**:
```python
try:
    response = await httpx_client.get(url)
except httpx.TimeoutException as e:
    raise HttpTimeoutError(str(e), url=url) from e  # __cause__ に設定
```

**Rust**:
```rust
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("HTTP timeout: {message}")]
    Timeout {
        message: String,
        #[source]  // std::error::Error::source() で取得可能
        source: Option<reqwest::Error>,
    },
}
```

## Language Implementation Mapping

### Python

| Taxonomy | Python Type | Module |
|----------|-------------|--------|
| HttpError | `HttpError` | `marketschema.http.exceptions` |
| HttpTimeoutError | `HttpTimeoutError` | `marketschema.http.exceptions` |
| HttpConnectionError | `HttpConnectionError` | `marketschema.http.exceptions` |
| HttpStatusError | `HttpStatusError` | `marketschema.http.exceptions` |
| HttpRateLimitError | `HttpRateLimitError` | `marketschema.http.exceptions` |

See: [lang/python.md](../lang/python.md)

### Rust

| Taxonomy | Rust Type | Crate |
|----------|-----------|-------|
| HttpError | `HttpError` (enum) | `marketschema-http` |
| HttpTimeoutError | `HttpError::Timeout` | `marketschema-http` |
| HttpConnectionError | `HttpError::Connection` | `marketschema-http` |
| HttpStatusError | `HttpError::Status` | `marketschema-http` |
| HttpRateLimitError | `HttpError::RateLimit` | `marketschema-http` |

See: [lang/rust.md](../lang/rust.md)

## Testing Requirements

各言語の実装は以下のテストケースを含むこと:

1. **タイムアウト検出**: タイムアウトが `HttpTimeoutError` として報告される
2. **接続エラー検出**: 接続失敗が `HttpConnectionError` として報告される
3. **ステータスエラー検出**: 4xx/5xx が `HttpStatusError` として報告される
4. **レート制限検出**: 429 が `HttpRateLimitError` として報告される
5. **例外チェイン**: 元の例外にアクセス可能である
6. **属性アクセス**: すべての属性（`status_code`, `url`, `retry_after` など）にアクセス可能

## Reference

- [spec.md](../spec.md) - Feature specification (FR-008 ~ FR-014)
- [Python API Contract](./python-api.md) - Python API 契約
- [Python Implementation](../lang/python.md) - Python 実装ガイド
- [Rust Implementation](../lang/rust.md) - Rust 実装ガイド
