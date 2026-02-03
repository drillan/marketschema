# Feature Specification: HTTP Client Python Implementation

**Feature Branch**: `003-http-client-python`
**Parent Spec**: [003-http-client](../003-http-client/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "Python 言語での HTTP クライアント実装仕様を定義する"

## Clarifications

### Session 2026-02-03

- 親仕様 [003-http-client](../003-http-client/spec.md) に基づき、Python 言語固有の実装仕様を定義。
- 既存の contracts/python-api.md を API 契約として継承。

## Overview

marketschema ライブラリの Python 実装における HTTP クライアントレイヤーを提供する。
親仕様で定義された機能要件を Python 言語の慣用的な方法で実装し、
アダプター開発者が簡単にデータソースと通信できるようにする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 非同期 HTTP リクエストの実行 (Priority: P1)

Python アダプター開発者として、httpx ベースの非同期 HTTP クライアントを使用して
データソースの API を呼び出し、JSON/テキスト形式のレスポンスを簡単に取得したい。
`async with` 構文によるリソース管理と、完全な型ヒントによる IDE サポートを期待する。

**Why this priority**: HTTP リクエストの基本機能がなければ、他のすべての機能が実現できない。

**Independent Test**: モック API に対して `get_json()` を呼び出し、正しいレスポンスを取得できれば成功。

**Acceptance Scenarios**:

1. **Given** AsyncHttpClient インスタンスが生成されている, **When** `await client.get_json(url)` を呼び出す, **Then** JSON レスポンスが `dict[str, Any]` 型として返される
2. **Given** AsyncHttpClient インスタンスが生成されている, **When** `await client.get_text(url)` を呼び出す, **Then** テキストレスポンスが `str` 型として返される
3. **Given** AsyncHttpClient インスタンスが生成されている, **When** `await client.get(url)` を呼び出す, **Then** 生の `httpx.Response` オブジェクトが返される
4. **Given** `async with AsyncHttpClient() as client:` でクライアントを使用する, **When** スコープを抜ける, **Then** コネクションが自動的にクローズされる
5. **Given** カスタムヘッダーを設定する, **When** リクエストを送信する, **Then** 設定したヘッダーがリクエストに含まれる

---

### User Story 2 - Python 例外による HTTP エラーの処理 (Priority: P1)

Python アダプター開発者として、タイムアウト、接続エラー、HTTP ステータスエラーを
Python の例外階層として受け取り、`try/except` で適切なエラーハンドリングを実装したい。
`__cause__` による例外チェインで元の httpx 例外にもアクセスできることを期待する。

**Why this priority**: エラーハンドリングは信頼性の基盤であり、P1 の HTTP リクエスト機能と同時に必要。

**Independent Test**: 各種エラー状況（タイムアウト、接続失敗、4xx/5xx）をシミュレートし、適切な例外が発生することを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** リクエストがタイムアウトする, **When** `await client.get_json(url)` を呼び出す, **Then** `HttpTimeoutError` が発生する
2. **Given** 接続先に到達できない, **When** `await client.get_json(url)` を呼び出す, **Then** `HttpConnectionError` が発生する
3. **Given** API が 404 を返す, **When** `await client.get_json(url)` を呼び出す, **Then** `HttpStatusError` が発生し `e.status_code == 404` である
4. **Given** API が 429 (Rate Limit) を返す, **When** `await client.get_json(url)` を呼び出す, **Then** `HttpRateLimitError` が発生し `e.retry_after` にアクセスできる
5. **Given** 例外が発生する, **When** `except HttpError as e:` で catch する, **Then** `e.__cause__` で元の httpx 例外にアクセスできる
6. **Given** JSON パースエラーが発生する, **When** `await client.get_json(url)` を呼び出す, **Then** `ValueError` が発生する

---

### User Story 3 - 指数バックオフによる自動リトライ (Priority: P2)

Python アダプター開発者として、一時的なエラーに対して
自動的にリトライが行われ、API の一時的な不安定さを吸収したい。
`RetryMiddleware` クラスでリトライポリシーを設定可能であることを期待する。

**Why this priority**: 安定した運用には必要だが、基本機能なしでも動作は可能。

**Independent Test**: respx でモックした API が一時的にエラーを返した後に成功する場合、自動リトライが機能することを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** 最初の2回が 503 を返し、3回目が成功する API, **When** `RetryMiddleware(max_retries=3)` ありで `await client.get_json(url)` を呼び出す, **Then** 3回目のレスポンスが返される
2. **Given** リトライ可能なステータス（429, 500, 502, 503, 504）, **When** エラーが発生する, **Then** 自動リトライが実行される
3. **Given** リトライ不可能なステータス（400, 401, 403, 404）, **When** エラーが発生する, **Then** 即座に例外が発生しリトライされない
4. **Given** `max_retries` を超えてもエラーが続く, **When** リトライを繰り返す, **Then** 最終的に例外が発生する
5. **Given** `backoff_factor=0.5` を設定する, **When** リトライが発生する, **Then** 指数バックオフでリトライ間隔が増加する

---

### User Story 4 - トークンバケットによるレート制限 (Priority: P2)

Python アダプター開発者として、API のレート制限を遵守し、
429 エラーの発生を未然に防ぎたい。
`RateLimitMiddleware` クラスでトークンバケットの設定が可能であることを期待する。

**Why this priority**: API プロバイダーとの良好な関係維持に重要だが、基本機能なしでも動作可能。

**Independent Test**: レート制限設定で高速リクエストを送信し、適切な間隔で送信されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** `RateLimitMiddleware(requests_per_second=10.0)` を設定する, **When** 100ms 間隔でリクエストを送信する, **Then** 100ms 以上の間隔でリクエストが送信される
2. **Given** `burst_size=5` を設定する, **When** 5 リクエストを同時に送信する, **Then** 5 リクエストは即座に処理される
3. **Given** レート制限が設定されていない, **When** リクエストを送信する, **Then** 流量制御なしで即座に送信される
4. **Given** `await rate_limit.acquire()` を呼び出す, **When** トークンが利用可能になる, **Then** 処理が続行される

---

### User Story 5 - LRU キャッシュによるレスポンスキャッシュ (Priority: P3)

Python アダプター開発者として、同一 URL への繰り返しリクエストに対して
キャッシュされたレスポンスを返し、API 呼び出し回数を削減したい。
`ResponseCache` クラスで TTL とキャッシュサイズを設定可能であることを期待する。

**Why this priority**: パフォーマンス最適化であり、基本機能・安定性機能の後で実装可能。

**Independent Test**: 同一 URL に2回リクエストし、2回目がキャッシュから返されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** キャッシュが有効で TTL 内, **When** 同一 URL に2回目のリクエスト, **Then** キャッシュされたレスポンスが返され API は呼ばれない
2. **Given** キャッシュの TTL が経過した, **When** 同一 URL にリクエスト, **Then** 新しいリクエストが送信される
3. **Given** `max_size=1000` でキャッシュサイズが上限に達した, **When** 新しい URL にリクエスト, **Then** LRU アルゴリズムで最も古いエントリが削除される
4. **Given** `cache.clear()` を呼び出す, **When** キャッシュを確認する, **Then** すべてのエントリが削除されている

---

### User Story 6 - BaseAdapter との統合 (Priority: P2)

Python アダプター開発者として、`BaseAdapter` を継承したクラスで
HTTP クライアントを簡単に利用したい。
遅延初期化と `async with` によるリソース管理を期待する。

**Why this priority**: ライブラリとしての使いやすさに直結するが、HTTP クライアント単体でも利用可能。

**Independent Test**: `BaseAdapter` を継承したクラスで `self.http_client` プロパティにアクセスし、HTTP リクエストを実行できれば成功。

**Acceptance Scenarios**:

1. **Given** `BaseAdapter` を継承したクラス, **When** `self.http_client` にアクセスする, **Then** `AsyncHttpClient` インスタンスが遅延初期化される
2. **Given** `async with MyAdapter() as adapter:` でアダプターを使用する, **When** スコープを抜ける, **Then** HTTP クライアントが自動的にクローズされる
3. **Given** コンストラクタでカスタム `http_client` を渡す, **When** `self.http_client` にアクセスする, **Then** 渡されたクライアントが使用される
4. **Given** `await adapter.close()` を呼び出す, **When** アダプターが所有する HTTP クライアントがある, **Then** HTTP クライアントがクローズされる

---

### Edge Cases

- **非常に大きなレスポンス**: httpx のデフォルトメモリ制限に依存
- **不正な JSON レスポンス**: `ValueError` を発生させる（`json.JSONDecodeError` をラップ）
- **空のレスポンス**: 空文字列または空の JSON として処理
- **同時多数リクエスト**: httpx のコネクションプーリング（デフォルト100接続）で処理
- **ネットワーク切断中のリトライ**: 最大リトライ後に `HttpConnectionError` を発生
- **キャッシュキーの衝突**: URL 全体（クエリパラメータ含む）をキーとして使用
- **asyncio イベントループの管理**: クライアントは呼び出し元のイベントループで動作

## Requirements *(mandatory)*

### Functional Requirements

**US1: 非同期 HTTP リクエストの実行**

- **FR-P001**: `AsyncHttpClient` クラスは `httpx.AsyncClient` をラップして非同期 HTTP GET リクエストを実行できなければならない
- **FR-P002**: `AsyncHttpClient.get_json()` メソッドは JSON レスポンスを `dict[str, Any]` 型として返さなければならない
- **FR-P003**: `AsyncHttpClient.get_text()` メソッドはテキストレスポンスを `str` 型として返さなければならない
- **FR-P004**: `AsyncHttpClient.get()` メソッドは生の `httpx.Response` オブジェクトを返さなければならない
- **FR-P005**: `AsyncHttpClient` は `async with` 構文（`__aenter__`/`__aexit__`）をサポートしなければならない
- **FR-P006**: `AsyncHttpClient` はタイムアウトを `float` 型（秒）で設定可能でなければならない（デフォルト: 30.0秒）
- **FR-P007**: `AsyncHttpClient` は `max_connections: int` でコネクションプールサイズを設定可能でなければならない（デフォルト: 100）
- **FR-P008**: `AsyncHttpClient` は `headers: dict[str, str]` でデフォルトヘッダーを設定可能でなければならない
- **FR-P009**: 各メソッドは `params: dict[str, str | int | float | bool]` でクエリパラメータを設定可能でなければならない

**US2: Python 例外による HTTP エラーの処理**

- **FR-P010**: タイムアウト時（`httpx.TimeoutException`）は `HttpTimeoutError` を発生させなければならない
- **FR-P011**: 接続失敗時（`httpx.ConnectError`）は `HttpConnectionError` を発生させなければならない
- **FR-P012**: HTTP 4xx/5xx ステータス時は `HttpStatusError` を発生させ、`status_code` 属性でステータスコードにアクセス可能でなければならない
- **FR-P013**: HTTP 429 ステータス時は `HttpRateLimitError` を発生させ、`retry_after` 属性で待機時間にアクセス可能でなければならない
- **FR-P014**: すべての HTTP 例外は `HttpError` を継承しなければならない
- **FR-P015**: すべての HTTP 例外は `MarketSchemaError` を継承しなければならない
- **FR-P016**: 例外は `raise ... from e` により `__cause__` で元の httpx 例外にアクセス可能でなければならない
- **FR-P017**: JSON パースエラー時は `ValueError` を発生させなければならない

**US3: 指数バックオフによる自動リトライ**

- **FR-P018**: `RetryMiddleware` クラスはリトライ対象のステータスコード（デフォルト: 429, 500, 502, 503, 504）に対して自動リトライを実行できなければならない
- **FR-P019**: `RetryMiddleware` は `max_retries: int` でリトライ回数を設定可能でなければならない（デフォルト: 3）
- **FR-P020**: `RetryMiddleware` は `backoff_factor: float` で指数バックオフ係数を設定可能でなければならない（デフォルト: 0.5）
- **FR-P021**: `RetryMiddleware` は `jitter: float` でランダムジッタを設定可能でなければならない（デフォルト: 0.1）
- **FR-P022**: リトライ不可能なステータス（400, 401, 403, 404）は即座に例外を発生させなければならない

**US4: トークンバケットによるレート制限**

- **FR-P023**: `RateLimitMiddleware` クラスはトークンバケットアルゴリズムによるレート制限を実装しなければならない
- **FR-P024**: `RateLimitMiddleware` は `requests_per_second: float` で1秒あたりのリクエスト数を設定可能でなければならない
- **FR-P025**: `RateLimitMiddleware` は `burst_size: int` でバーストサイズを設定可能でなければならない（デフォルト: `requests_per_second` と同値）
- **FR-P026**: `RateLimitMiddleware.acquire()` は非同期メソッドとしてトークン取得を待機できなければならない
- **FR-P027**: `RateLimitMiddleware.try_acquire()` は同期メソッドとして即座にトークン取得を試行できなければならない

**US5: LRU キャッシュによるレスポンスキャッシュ**

- **FR-P028**: `ResponseCache` クラスは LRU キャッシュによるレスポンスキャッシュを実装しなければならない
- **FR-P029**: `ResponseCache` は `default_ttl: timedelta` でキャッシュの TTL を設定可能でなければならない（デフォルト: 5分）
- **FR-P030**: `ResponseCache` は `max_size: int` でキャッシュの最大サイズを設定可能でなければならない（デフォルト: 1000）
- **FR-P031**: `ResponseCache.get(key)` はキャッシュされた値または `None` を返さなければならない
- **FR-P032**: `ResponseCache.set(key, value, ttl)` はキャッシュにエントリを追加できなければならない
- **FR-P033**: `ResponseCache.delete(key)` はキャッシュからエントリを削除できなければならない
- **FR-P034**: `ResponseCache.clear()` はすべてのキャッシュエントリを削除できなければならない

**US6: BaseAdapter との統合**

- **FR-P035**: `BaseAdapter` クラスに `http_client` プロパティを追加しなければならない
- **FR-P036**: `http_client` プロパティは遅延初期化されなければならない（初回アクセス時に `AsyncHttpClient` を生成）
- **FR-P037**: `BaseAdapter` は `async with` 構文をサポートしなければならない
- **FR-P038**: `BaseAdapter.__aexit__` は所有する HTTP クライアントを自動クローズしなければならない
- **FR-P039**: `BaseAdapter` コンストラクタは `http_client: AsyncHttpClient | None` パラメータを受け取れなければならない

### Key Entities

- **AsyncHttpClient**: 非同期 HTTP クライアント。`httpx.AsyncClient` をラップし、型安全な API を提供
- **HttpError**: HTTP 関連エラーの基底例外クラス。`MarketSchemaError` を継承
- **HttpTimeoutError**: タイムアウトエラー。`HttpError` を継承
- **HttpConnectionError**: 接続エラー。`HttpError` を継承
- **HttpStatusError**: HTTP ステータスエラー。`status_code: int`, `response_body: str | None` 属性を持つ
- **HttpRateLimitError**: レート制限エラー。`HttpStatusError` を継承し、`retry_after: float | None` 属性を持つ
- **RetryMiddleware**: リトライロジック。`max_retries`, `backoff_factor`, `retry_statuses`, `jitter` を設定可能
- **RateLimitMiddleware**: レート制限ロジック。`requests_per_second`, `burst_size` を設定可能
- **ResponseCache**: レスポンスキャッシュ。`max_size`, `default_ttl` を設定可能

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-P001**: すべての Python example（bitbank, stooq, stockanalysis）を `AsyncHttpClient` を使用した非同期実装に移行できる
- **SC-P002**: `marketschema.http` モジュールのテストカバレッジが 90% 以上である
- **SC-P003**: 同一エンドポイントへの連続リクエストで、キャッシュにより2回目以降のレスポンス時間が 90% 以上短縮される
- **SC-P004**: レート制限設定により、テスト環境での 429 エラー発生率をゼロに抑制できる
- **SC-P005**: 一時的なサーバーエラー（503）に対して、自動リトライにより最終的な成功率が向上する
- **SC-P006**: mypy による型チェックがエラーなしで通過する
- **SC-P007**: ruff による lint および format チェックがエラーなしで通過する

## Assumptions

- Python 3.13 以上を使用する
- HTTP ライブラリとして httpx>=0.27.0 を使用する
- テストモックライブラリとして respx を使用する
- TLS 証明書検証は httpx のデフォルト動作に従う
- 認証処理は HTTP クライアントの責務外とし、各アダプターで実装する
- asyncio イベントループの管理は呼び出し元の責務とする

## Language-Specific Considerations

- `async/await` 構文による非同期処理
- `async with` 構文（コンテキストマネージャ）によるリソース管理
- `raise ... from e` による例外チェイン（`__cause__` でアクセス）
- `dict[str, str]` 形式の型ヒント（Python 3.9+ ビルトインジェネリクス）
- `Self` 型（Python 3.11+ `typing.Self`）によるメソッドチェイン
- `timedelta` による時間表現

## Module Structure

```
src/marketschema/http/
├── __init__.py       # Public exports
├── client.py         # AsyncHttpClient
├── exceptions.py     # Exception classes
├── middleware.py     # RetryMiddleware, RateLimitMiddleware
└── cache.py          # ResponseCache
```

## Contracts

- [Python API Contract](contracts/python-api.md) - API contract with Python type signatures

## References

- [003-http-client](../003-http-client/spec.md) - 親仕様（言語非依存）
- [Error Taxonomy](../003-http-client/contracts/error-taxonomy.md) - 言語非依存エラー分類
- [HTTP Client Guide](../../docs/guides/http-client.md) - 実装ガイドとコード例
- [httpx Documentation](https://www.python-httpx.org/) - Python HTTP ライブラリ
- [respx Documentation](https://github.com/lundberg/respx) - httpx モッキングライブラリ
