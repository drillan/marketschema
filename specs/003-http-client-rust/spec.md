# Feature Specification: HTTP Client Rust Implementation

**Feature Branch**: `003-http-client-rust`
**Parent Spec**: [003-http-client](../003-http-client/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "Rust 言語での HTTP クライアント実装仕様を定義する"

## Clarifications

### Session 2026-02-03

- 親仕様 [003-http-client](../003-http-client/spec.md) に基づき、Rust 言語固有の実装仕様を定義。
- Rust の所有権システムとライフタイム、`Send + Sync` 境界を考慮した設計。
- エラー処理は `thiserror` による `#[source]` 属性を使用。

## Overview

marketschema ライブラリの Rust 実装における HTTP クライアントレイヤーを提供する。
親仕様で定義された機能要件を Rust 言語の慣用的な方法で実装し、
アダプター開発者が簡単にデータソースと通信できるようにする。

> **Scope**: 本仕様は HTTP GET リクエストのみを対象とする。POST/PUT/DELETE は将来の拡張として検討可能だが、現時点ではスコープ外とする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 非同期 HTTP リクエストの実行 (Priority: P1)

Rust アダプター開発者として、reqwest ベースの非同期 HTTP クライアントを使用して
データソースの API を呼び出し、JSON/テキスト形式のレスポンスを簡単に取得したい。
`Arc<AsyncHttpClient>` による共有と、RAII パターンによるリソース管理を期待する。

**Why this priority**: HTTP リクエストの基本機能がなければ、他のすべての機能が実現できない。

**Independent Test**: モック API に対して `get_json()` を呼び出し、正しいレスポンスを取得できれば成功。

**Acceptance Scenarios**:

1. **Given** `AsyncHttpClient` インスタンスが生成されている, **When** `client.get_json(url).await` を呼び出す, **Then** JSON レスポンスが `serde_json::Value` 型として返される
2. **Given** `AsyncHttpClient` インスタンスが生成されている, **When** `client.get_text(url).await` を呼び出す, **Then** テキストレスポンスが `String` 型として返される
3. **Given** `AsyncHttpClient` インスタンスが生成されている, **When** `client.get(url).await` を呼び出す, **Then** 生の `reqwest::Response` オブジェクトが返される
4. **Given** `Arc<AsyncHttpClient>` でクライアントを共有する, **When** 複数タスクで同時使用する, **Then** スレッドセーフに動作する
5. **Given** カスタムヘッダーを設定する, **When** リクエストを送信する, **Then** 設定したヘッダーがリクエストに含まれる

---

### User Story 2 - Result 型による HTTP エラーの処理 (Priority: P1)

Rust アダプター開発者として、タイムアウト、接続エラー、HTTP ステータスエラーを
`Result<T, HttpError>` として受け取り、`match` や `?` 演算子で適切なエラーハンドリングを実装したい。
`#[source]` 属性による元の reqwest エラーへのアクセスを期待する。

**Why this priority**: エラーハンドリングは信頼性の基盤であり、P1 の HTTP リクエスト機能と同時に必要。

**Independent Test**: 各種エラー状況（タイムアウト、接続失敗、4xx/5xx）をシミュレートし、適切なエラーが返されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** リクエストがタイムアウトする, **When** `client.get_json(url).await` を呼び出す, **Then** `Err(HttpError::Timeout { .. })` が返される
2. **Given** 接続先に到達できない, **When** `client.get_json(url).await` を呼び出す, **Then** `Err(HttpError::Connection { .. })` が返される
3. **Given** API が 404 を返す, **When** `client.get_json(url).await` を呼び出す, **Then** `Err(HttpError::Status { status_code: 404, .. })` が返される
4. **Given** API が 429 (Rate Limit) を返す, **When** `client.get_json(url).await` を呼び出す, **Then** `Err(HttpError::RateLimit { retry_after, .. })` が返される
5. **Given** エラーが発生する, **When** `error.source()` を呼び出す, **Then** 元の reqwest エラーにアクセスできる
6. **Given** JSON パースエラーが発生する, **When** `client.get_json(url).await` を呼び出す, **Then** `Err(HttpError::Parse { .. })` が返される

---

### User Story 3 - 指数バックオフによる自動リトライ (Priority: P2)

Rust アダプター開発者として、一時的なエラーに対して
自動的にリトライが行われ、API の一時的な不安定さを吸収したい。
`RetryConfig` 構造体でリトライポリシーを設定可能であることを期待する。

**Why this priority**: 安定した運用には必要だが、基本機能なしでも動作は可能。

**Independent Test**: wiremock でモックした API が一時的にエラーを返した後に成功する場合、自動リトライが機能することを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** 最初の2回が 503 を返し、3回目が成功する API, **When** `RetryConfig { max_retries: 3, .. }` ありで `client.get_json(url).await` を呼び出す, **Then** 3回目のレスポンスが返される
2. **Given** リトライ可能なステータス（429, 500, 502, 503, 504）, **When** エラーが発生する, **Then** 自動リトライが実行される
3. **Given** リトライ不可能なステータス（400, 401, 403, 404）, **When** エラーが発生する, **Then** 即座にエラーが返されリトライされない
4. **Given** `max_retries` を超えてもエラーが続く, **When** リトライを繰り返す, **Then** 最終的にエラーが返される
5. **Given** `backoff_factor: 0.5` を設定する, **When** リトライが発生する, **Then** 指数バックオフでリトライ間隔が増加する

---

### User Story 4 - トークンバケットによるレート制限 (Priority: P2)

Rust アダプター開発者として、API のレート制限を遵守し、
429 エラーの発生を未然に防ぎたい。
`RateLimiter` 構造体でトークンバケットの設定が可能であることを期待する。

**Why this priority**: API プロバイダーとの良好な関係維持に重要だが、基本機能なしでも動作可能。

**Independent Test**: レート制限設定で高速リクエストを送信し、適切な間隔で送信されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** `RateLimiter::new(10.0, 10)` を設定する, **When** 100ms 間隔でリクエストを送信する, **Then** 100ms 以上の間隔でリクエストが送信される
2. **Given** `burst_size: 5` を設定する, **When** 5 リクエストを同時に送信する, **Then** 5 リクエストは即座に処理される
3. **Given** レート制限が設定されていない, **When** リクエストを送信する, **Then** 流量制御なしで即座に送信される
4. **Given** `rate_limiter.acquire().await` を呼び出す, **When** トークンが利用可能になる, **Then** 処理が続行される

---

### User Story 5 - LRU キャッシュによるレスポンスキャッシュ (Priority: P3)

Rust アダプター開発者として、同一 URL への繰り返しリクエストに対して
キャッシュされたレスポンスを返し、API 呼び出し回数を削減したい。
`ResponseCache` 構造体で TTL とキャッシュサイズを設定可能であることを期待する。

**Why this priority**: パフォーマンス最適化であり、基本機能・安定性機能の後で実装可能。

**Independent Test**: 同一 URL に2回リクエストし、2回目がキャッシュから返されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** キャッシュが有効で TTL 内, **When** 同一 URL に2回目のリクエスト, **Then** キャッシュされたレスポンスが返され API は呼ばれない
2. **Given** キャッシュの TTL が経過した, **When** 同一 URL にリクエスト, **Then** 新しいリクエストが送信される
3. **Given** `max_size: 1000` でキャッシュサイズが上限に達した, **When** 新しい URL にリクエスト, **Then** LRU アルゴリズムで最も古いエントリが削除される
4. **Given** `cache.clear()` を呼び出す, **When** キャッシュを確認する, **Then** すべてのエントリが削除されている

---

### User Story 6 - BaseAdapter トレイトとの統合 (Priority: P2)

Rust アダプター開発者として、`BaseAdapter` トレイトを実装した構造体で
HTTP クライアントを簡単に利用したい。
`Arc<AsyncHttpClient>` による共有と `Drop` トレイトによるリソース管理を期待する。

**Why this priority**: ライブラリとしての使いやすさに直結するが、HTTP クライアント単体でも利用可能。

**Independent Test**: `BaseAdapter` トレイトを実装した構造体で `http_client()` メソッドにアクセスし、HTTP リクエストを実行できれば成功。

**Acceptance Scenarios**:

1. **Given** `BaseAdapter` トレイトを実装した構造体, **When** `self.http_client()` にアクセスする, **Then** `Arc<AsyncHttpClient>` が返される
2. **Given** アダプターが `Drop` される, **When** 所有する HTTP クライアントへの参照がなくなる, **Then** HTTP クライアントが自動的にドロップされる
3. **Given** コンストラクタでカスタム `Arc<AsyncHttpClient>` を渡す, **When** `self.http_client()` にアクセスする, **Then** 渡されたクライアントが使用される
4. **Given** `OnceCell` で遅延初期化する, **When** 初回アクセス時, **Then** クライアントが生成される

---

### Edge Cases

- **非常に大きなレスポンス**: reqwest のデフォルトメモリ制限に依存
- **不正な JSON レスポンス**: `HttpError::Parse` を返す（`serde_json::Error` をラップ）
- **空のレスポンス**: 空文字列または空の JSON として処理
- **同時多数リクエスト**: reqwest のコネクションプーリング（デフォルト100接続）で処理
- **ネットワーク切断中のリトライ**: 最大リトライ後に `HttpError::Connection` を返す
- **キャッシュキーの衝突**: URL 全体（クエリパラメータ含む）をキーとして使用
- **tokio ランタイムの管理**: クライアントは呼び出し元のランタイムで動作

## Requirements *(mandatory)*

### Functional Requirements

**US1: 非同期 HTTP リクエストの実行**

- **FR-R001**: `AsyncHttpClient` 構造体は `reqwest::Client` をラップして非同期 HTTP GET リクエストを実行できなければならない
- **FR-R002**: `AsyncHttpClient::get_json()` メソッドは JSON レスポンスを `serde_json::Value` 型として返さなければならない
- **FR-R003**: `AsyncHttpClient::get_text()` メソッドはテキストレスポンスを `String` 型として返さなければならない
- **FR-R004**: `AsyncHttpClient::get()` メソッドは生の `reqwest::Response` オブジェクトを返さなければならない
- **FR-R005**: `AsyncHttpClient` は `Send + Sync` を実装し、`Arc<AsyncHttpClient>` として共有可能でなければならない
- **FR-R006**: `AsyncHttpClient` はタイムアウトを `Duration` 型で設定可能でなければならない（デフォルト: 30秒）
- **FR-R007**: `AsyncHttpClient` は `max_connections: usize` でコネクションプールサイズを設定可能でなければならない（デフォルト: 100）
- **FR-R008**: `AsyncHttpClient` は `HeaderMap` でデフォルトヘッダーを設定可能でなければならない
- **FR-R009**: 各メソッドは `&[(&str, &str)]` でクエリパラメータを設定可能でなければならない
- **FR-R010**: `AsyncHttpClientBuilder` パターンで設定を行えなければならない

**US2: Result 型による HTTP エラーの処理**

- **FR-R011**: タイムアウト時は `HttpError::Timeout { message, url, source }` を返さなければならない
- **FR-R012**: 接続失敗時は `HttpError::Connection { message, url, source }` を返さなければならない
- **FR-R013**: HTTP 4xx/5xx ステータス時は `HttpError::Status { message, url, status_code, response_body, source }` を返さなければならない
- **FR-R014**: HTTP 429 ステータス時は `HttpError::RateLimit { message, url, status_code, response_body, retry_after, source }` を返さなければならない
- **FR-R015**: `HttpError` は `thiserror::Error` を derive し、`std::error::Error` を実装しなければならない
- **FR-R016**: `HttpError` は `MarketSchemaError` 列挙型のバリアントとしてラップ可能でなければならない
- **FR-R017**: エラーは `#[source]` 属性により `std::error::Error::source()` で元の reqwest エラーにアクセス可能でなければならない
- **FR-R018**: JSON パースエラー時は `HttpError::Parse { message, url, source }` を返さなければならない

**US3: 指数バックオフによる自動リトライ**

- **FR-R019**: `RetryConfig` 構造体はリトライ対象のステータスコード（デフォルト: 429, 500, 502, 503, 504）に対して自動リトライを実行できなければならない
- **FR-R020**: `RetryConfig` は `max_retries: u32` でリトライ回数を設定可能でなければならない（デフォルト: 3）
- **FR-R021**: `RetryConfig` は `backoff_factor: f64` で指数バックオフ係数を設定可能でなければならない（デフォルト: 0.5）
- **FR-R022**: `RetryConfig` は `jitter: f64` でランダムジッタを設定可能でなければならない（デフォルト: 0.1）
- **FR-R023**: リトライ不可能なステータス（400, 401, 403, 404）は即座にエラーを返さなければならない

**US4: トークンバケットによるレート制限**

- **FR-R024**: `RateLimiter` 構造体はトークンバケットアルゴリズムによるレート制限を実装しなければならない
- **FR-R025**: `RateLimiter` は `requests_per_second: f64` で1秒あたりのリクエスト数を設定可能でなければならない
- **FR-R026**: `RateLimiter` は `burst_size: usize` でバーストサイズを設定可能でなければならない（デフォルト: `requests_per_second` と同値）
- **FR-R027**: `RateLimiter::acquire()` は非同期メソッドとしてトークン取得を待機できなければならない
- **FR-R028**: `RateLimiter::try_acquire()` は同期メソッドとして即座にトークン取得を試行し `bool` を返さなければならない
- **FR-R029**: `RateLimiter` は `Send + Sync` を実装し、複数タスク間で共有可能でなければならない

**US5: LRU キャッシュによるレスポンスキャッシュ**

- **FR-R030**: `ResponseCache` 構造体は LRU キャッシュによるレスポンスキャッシュを実装しなければならない
- **FR-R031**: `ResponseCache` は `default_ttl: Duration` でキャッシュの TTL を設定可能でなければならない（デフォルト: 5分）
- **FR-R032**: `ResponseCache` は `max_size: usize` でキャッシュの最大サイズを設定可能でなければならない（デフォルト: 1000）
- **FR-R033**: `ResponseCache::get(&self, key: &str)` はキャッシュされた値または `None` を返さなければならない
- **FR-R034**: `ResponseCache::set(&self, key: &str, value: T, ttl: Option<Duration>)` はキャッシュにエントリを追加できなければならない
- **FR-R035**: `ResponseCache::delete(&self, key: &str)` はキャッシュからエントリを削除できなければならない
- **FR-R036**: `ResponseCache::clear(&self)` はすべてのキャッシュエントリを削除できなければならない
- **FR-R037**: `ResponseCache` は `Send + Sync` を実装し、複数タスク間で共有可能でなければならない

**US6: BaseAdapter トレイトとの統合**

- **FR-R038**: `BaseAdapter` トレイトに `fn http_client(&self) -> Arc<AsyncHttpClient>` メソッドを追加しなければならない
- **FR-R039**: `http_client()` メソッドはデフォルト実装で `std::sync::OnceLock` による遅延初期化を提供しなければならない
- **FR-R040**: `BaseAdapter` 実装者はコンストラクタで `Arc<AsyncHttpClient>` を注入可能でなければならない
- **FR-R041**: `Drop` トレイトにより、アダプターが破棄されたときにリソースが適切に解放されなければならない

### Key Entities

- **AsyncHttpClient**: 非同期 HTTP クライアント。`reqwest::Client` をラップし、型安全な API を提供
- **AsyncHttpClientBuilder**: ビルダーパターンでクライアントを構築
- **HttpError**: HTTP 関連エラーの列挙型。`thiserror::Error` を derive
- **HttpError::Timeout**: タイムアウトエラー
- **HttpError::Connection**: 接続エラー
- **HttpError::Status**: HTTP ステータスエラー。`status_code: u16`, `response_body: Option<String>` フィールドを持つ
- **HttpError::RateLimit**: レート制限エラー。`HttpError::Status` の特殊ケースで、`retry_after: Option<Duration>` フィールドを持つ
- **HttpError::Parse**: JSON パースエラー
- **RetryConfig**: リトライ設定。`max_retries`, `backoff_factor`, `retry_statuses`, `jitter` を設定可能
- **RateLimiter**: レート制限。`requests_per_second`, `burst_size` を設定可能
- **ResponseCache**: レスポンスキャッシュ。`max_size`, `default_ttl` を設定可能

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-R001**: Rust example（存在する場合）を `AsyncHttpClient` を使用した非同期実装に移行できる
- **SC-R002**: `marketschema-http` クレートのテストカバレッジが 90% 以上である
- **SC-R003**: 同一エンドポイントへの連続リクエストで、キャッシュにより2回目以降のレスポンス時間が 90% 以上短縮される
- **SC-R004**: レート制限設定により、テスト環境での 429 エラー発生率をゼロに抑制できる
- **SC-R005**: 一時的なサーバーエラー（503）に対して、自動リトライにより最終的な成功率が向上する
- **SC-R006**: `cargo clippy` によるリントチェックが警告なしで通過する
- **SC-R007**: `cargo fmt --check` によるフォーマットチェックが通過する
- **SC-R008**: `cargo test` によるすべてのテストが通過する

## Assumptions

- Rust 1.70.0 以上を使用する（MSRV: 1.70.0 - `std::sync::OnceLock` 安定化）
- HTTP ライブラリとして reqwest を使用する
- 非同期ランタイムとして tokio を使用する
- エラー型の derive には thiserror を使用する
- テストモックライブラリとして wiremock を使用する
- TLS 証明書検証は reqwest のデフォルト動作（rustls または native-tls）に従う
- 認証処理は HTTP クライアントの責務外とし、各アダプターで実装する
- tokio ランタイムの管理は呼び出し元の責務とする

## Language-Specific Considerations

- `async/await` 構文による非同期処理（tokio ランタイム）
- `Arc<T>` によるスレッドセーフな共有
- `OnceCell` または `OnceLock` による遅延初期化
- `#[derive(Error)]` による `thiserror` でのエラー型定義
- `#[source]` 属性による例外チェイン（`std::error::Error::source()` でアクセス）
- `Duration` による時間表現
- `Send + Sync` 境界による並行安全性の保証
- RAII パターンと `Drop` トレイトによるリソース管理
- ビルダーパターンによる構造体の構築

## Crate Structure

```
crates/marketschema-http/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public exports
│   ├── client.rs        # AsyncHttpClient, AsyncHttpClientBuilder
│   ├── error.rs         # HttpError enum
│   ├── retry.rs         # RetryConfig
│   ├── rate_limit.rs    # RateLimiter
│   └── cache.rs         # ResponseCache
└── tests/
    ├── client_tests.rs
    ├── error_tests.rs
    └── integration_tests.rs
```

## Contracts

- [Rust API Contract](contracts/rust-api.md) - API contract with Rust type signatures

## References

- [003-http-client](../003-http-client/spec.md) - 親仕様（言語非依存）
- [Error Taxonomy](../003-http-client/contracts/error-taxonomy.md) - 言語非依存エラー分類
- [reqwest Documentation](https://docs.rs/reqwest/) - Rust HTTP ライブラリ
- [tokio Documentation](https://tokio.rs/) - Rust 非同期ランタイム
- [thiserror Documentation](https://docs.rs/thiserror/) - エラー型 derive マクロ
- [wiremock Documentation](https://docs.rs/wiremock/) - HTTP モッキングライブラリ
