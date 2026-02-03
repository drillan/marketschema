# Feature Specification: HTTP Client Layer

**Feature Branch**: `003-http-client`
**Parent Spec**: [001-core](../001-core/spec.md)
**Created**: 2026-02-03
**Status**: Draft
**Input**: User description: "specs/003-http-client/features/http-client-layer をもとにHTTPクライアントの仕様を定義してください"

## Clarifications

### Session 2026-02-03

- 既存の計画ファイル（`features/http-client-layer/plan.md`, `architecture.md`）に基づいて仕様を定義。
- Q: 生の HTTP レスポンスを返す `get()` メソッドを FR に追加すべきか？ → A: 追加する（FR-028）。80% のユースケースは `get_json()` / `get_text()` で満たせるが、将来的な拡張性のため。

## Overview

marketschema ライブラリで利用可能な共通 HTTP クライアントレイヤーを提供する。
リトライ、レート制限、タイムアウト、キャッシュなどの機能を標準化し、
アダプター開発者が簡単にデータソースと通信できるようにする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 非同期 HTTP リクエストの実行 (Priority: P1)

アダプター開発者として、データソースの API を呼び出し、
JSON/テキスト形式のレスポンスを簡単に取得したい。
コネクションプーリングや適切なタイムアウト設定が自動的に行われることを期待する。

**Why this priority**: HTTP リクエストの基本機能がなければ、他のすべての機能が実現できない。

**Independent Test**: モック API に対して `get_json()` を呼び出し、正しいレスポンスを取得できれば成功。

**Acceptance Scenarios**:

1. **Given** AsyncHttpClient インスタンスが生成されている, **When** `get_json(url)` を呼び出す, **Then** JSON レスポンスが dict として返される
2. **Given** AsyncHttpClient インスタンスが生成されている, **When** `get_text(url)` を呼び出す, **Then** テキストレスポンスが str として返される
3. **Given** コンテキストマネージャで AsyncHttpClient を使用する, **When** with ブロックを抜ける, **Then** コネクションが自動的にクローズされる

---

### User Story 2 - HTTP エラーの適切な処理 (Priority: P1)

アダプター開発者として、タイムアウト、接続エラー、HTTP ステータスエラーを
明確な例外として受け取り、適切なエラーハンドリングを実装したい。
エラーの原因が特定しやすく、元の例外情報も保持されていることを期待する。

**Why this priority**: エラーハンドリングは信頼性の基盤であり、P1 の HTTP リクエスト機能と同時に必要。

**Independent Test**: 各種エラー状況（タイムアウト、接続失敗、4xx/5xx）をシミュレートし、適切な例外が発生することを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** リクエストがタイムアウトする, **When** `get_json(url)` を呼び出す, **Then** `HttpTimeoutError` が発生する
2. **Given** 接続先に到達できない, **When** `get_json(url)` を呼び出す, **Then** `HttpConnectionError` が発生する
3. **Given** API が 404 を返す, **When** `get_json(url)` を呼び出す, **Then** `HttpStatusError(status_code=404)` が発生する
4. **Given** API が 429 (Rate Limit) を返す, **When** `get_json(url)` を呼び出す, **Then** `HttpRateLimitError` が発生する
5. **Given** 例外が発生する, **When** 例外を catch する, **Then** `__cause__` で元の例外にアクセスできる

---

### User Story 3 - 失敗時の自動リトライ (Priority: P2)

アダプター開発者として、一時的なエラー（タイムアウト、サーバーエラー）に対して
自動的にリトライが行われ、API の一時的な不安定さを吸収したい。
リトライ回数やバックオフ間隔を設定可能であることを期待する。

**Why this priority**: 安定した運用には必要だが、基本機能なしでも動作は可能。

**Independent Test**: 一時的にエラーを返した後に成功するモック API で、自動リトライが機能することを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** 最初の2回が 503 を返し、3回目が成功する API, **When** リトライ設定ありで `get_json(url)` を呼び出す, **Then** 3回目のレスポンスが返される
2. **Given** リトライ可能なステータス（429, 500, 502, 503, 504）, **When** エラーが発生する, **Then** 自動リトライが実行される
3. **Given** リトライ不可能なステータス（400, 401, 403, 404）, **When** エラーが発生する, **Then** 即座に例外が発生する
4. **Given** max_retries を超えてもエラーが続く, **When** リトライを繰り返す, **Then** 最終的に例外が発生する

---

### User Story 4 - レート制限の遵守 (Priority: P2)

アダプター開発者として、API のレート制限を遵守し、
429 エラーの発生を未然に防ぎたい。
トークンバケットアルゴリズムによる流量制御を期待する。

**Why this priority**: API プロバイダーとの良好な関係維持に重要だが、基本機能なしでも動作可能。

**Independent Test**: レート制限設定で高速リクエストを送信し、適切な間隔で送信されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** レート制限が 10 req/sec に設定されている, **When** 100ms 間隔でリクエストを送信する, **Then** 100ms 以上の間隔でリクエストが送信される
2. **Given** バーストサイズが 5 に設定されている, **When** 5 リクエストを同時に送信する, **Then** 5 リクエストは即座に処理される
3. **Given** レート制限が設定されていない, **When** リクエストを送信する, **Then** 流量制御なしで即座に送信される

---

### User Story 5 - レスポンスのキャッシュ (Priority: P3)

アダプター開発者として、同一 URL への繰り返しリクエストに対して
キャッシュされたレスポンスを返し、API 呼び出し回数を削減したい。
TTL とキャッシュサイズを設定可能であることを期待する。

**Why this priority**: パフォーマンス最適化であり、基本機能・安定性機能の後で実装可能。

**Independent Test**: 同一 URL に2回リクエストし、2回目がキャッシュから返されることを確認できれば成功。

**Acceptance Scenarios**:

1. **Given** キャッシュが有効で TTL 内, **When** 同一 URL に2回目のリクエスト, **Then** キャッシュされたレスポンスが返される
2. **Given** キャッシュの TTL が経過した, **When** 同一 URL にリクエスト, **Then** 新しいリクエストが送信される
3. **Given** キャッシュサイズが上限に達した, **When** 新しい URL にリクエスト, **Then** 最も古いエントリが削除される

---

### User Story 6 - BaseAdapter との統合 (Priority: P2)

アダプター開発者として、BaseAdapter を継承したクラスで
HTTP クライアントを簡単に利用したい。
遅延初期化とコンテキストマネージャによる適切なリソース管理を期待する。

**Why this priority**: ライブラリとしての使いやすさに直結するが、HTTP クライアント単体でも利用可能。

**Independent Test**: BaseAdapter を継承したクラスで `http_client` プロパティにアクセスし、HTTP リクエストを実行できれば成功。

**Acceptance Scenarios**:

1. **Given** BaseAdapter を継承したクラス, **When** `self.http_client` にアクセスする, **Then** AsyncHttpClient インスタンスが遅延初期化される
2. **Given** アダプターをコンテキストマネージャで使用する, **When** with ブロックを抜ける, **Then** HTTP クライアントが自動的にクローズされる
3. **Given** カスタム HTTP クライアントを渡す, **When** アダプターを生成する, **Then** 渡されたクライアントが使用される

---

### Edge Cases

- 非常に大きなレスポンス（メモリ制限）：httpx のデフォルト制限に依存
- 不正な JSON レスポンス：明確なパースエラーを発生させる
- 空のレスポンス：適切なエラーまたは空のデータとして処理
- 同時多数リクエスト：コネクションプーリングで処理
- ネットワーク切断中のリトライ：最大リトライ後にエラー
- キャッシュキーの衝突：URL をキーとして使用、クエリパラメータも含む

## Requirements *(mandatory)*

### Functional Requirements

**US1: 非同期 HTTP リクエストの実行**

- **FR-001**: AsyncHttpClient は非同期 HTTP GET リクエストを実行できなければならない
- **FR-002**: AsyncHttpClient は JSON レスポンスを dict として返す `get_json()` を提供しなければならない
- **FR-003**: AsyncHttpClient はテキストレスポンスを str として返す `get_text()` を提供しなければならない
- **FR-004**: AsyncHttpClient は生の HTTP レスポンスを返す `get()` を提供しなければならない
- **FR-005**: AsyncHttpClient はコンテキストマネージャ（async with）をサポートしなければならない
- **FR-006**: AsyncHttpClient はタイムアウトを設定可能でなければならない（デフォルト: 30秒）
- **FR-007**: AsyncHttpClient はコネクションプーリングを行わなければならない（デフォルト: 最大100接続）

**US2: HTTP エラーの適切な処理**

- **FR-008**: タイムアウト時は `HttpTimeoutError` を発生させなければならない
- **FR-009**: 接続失敗時は `HttpConnectionError` を発生させなければならない
- **FR-010**: HTTP 4xx/5xx ステータス時は `HttpStatusError` を発生させなければならない
- **FR-011**: HTTP 429 ステータス時は `HttpRateLimitError` を発生させなければならない
- **FR-012**: すべての HTTP 例外は `HttpError` を継承しなければならない
- **FR-013**: すべての HTTP 例外は `MarketSchemaError` を継承しなければならない
- **FR-014**: 例外は元の例外を `__cause__` として保持しなければならない

**US3: 失敗時の自動リトライ**

- **FR-015**: リトライ対象のステータスコード（429, 500, 502, 503, 504）に対して自動リトライを実行できなければならない
- **FR-016**: リトライ回数を設定可能でなければならない（デフォルト: 3回）
- **FR-017**: リトライ間隔は指数バックオフで増加しなければならない（デフォルト: 係数 0.5）
- **FR-018**: リトライ不可能なステータス（400, 401, 403, 404）は即座に例外を発生させなければならない

**US4: レート制限の遵守**

- **FR-019**: トークンバケットアルゴリズムによるレート制限を実装しなければならない
- **FR-020**: 1秒あたりのリクエスト数を設定可能でなければならない
- **FR-021**: バーストサイズを設定可能でなければならない

**US5: レスポンスのキャッシュ**

- **FR-022**: LRU キャッシュによるレスポンスキャッシュを実装しなければならない
- **FR-023**: キャッシュの TTL を設定可能でなければならない（デフォルト: 5分）
- **FR-024**: キャッシュの最大サイズを設定可能でなければならない（デフォルト: 1000エントリ）

**US6: BaseAdapter との統合**

- **FR-025**: BaseAdapter に `http_client` プロパティを追加しなければならない
- **FR-026**: `http_client` は遅延初期化されなければならない
- **FR-027**: BaseAdapter はコンテキストマネージャ（async with）をサポートしなければならない
- **FR-028**: コンテキストマネージャ終了時に HTTP クライアントをクローズしなければならない

### Key Entities

- **AsyncHttpClient**: 非同期 HTTP クライアント。タイムアウト、コネクションプーリング、ヘッダーを設定可能
- **HttpError**: HTTP 関連エラーの基底例外クラス
- **HttpTimeoutError**: タイムアウトエラー
- **HttpConnectionError**: 接続エラー
- **HttpStatusError**: HTTP ステータスエラー。`status_code` 属性を持つ
- **HttpRateLimitError**: レート制限エラー（429）
- **RetryMiddleware**: リトライロジック。リトライ回数、バックオフ係数、対象ステータスを設定可能
- **RateLimitMiddleware**: レート制限ロジック。リクエスト数/秒、バーストサイズを設定可能
- **ResponseCache**: レスポンスキャッシュ。TTL、最大サイズを設定可能

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 3つの example（bitbank, stooq, stockanalysis）を AsyncHttpClient を使用した非同期実装に移行できる
- **SC-002**: HTTP モジュールのテストカバレッジが 90% 以上である
- **SC-003**: 同一エンドポイントへの連続リクエストで、キャッシュにより2回目以降のレスポンス時間が 90% 以上短縮される
- **SC-004**: レート制限設定により、API プロバイダーからの 429 エラー発生率をゼロに抑制できる
- **SC-005**: 一時的なサーバーエラー（503）に対して、自動リトライにより最終的な成功率が向上する

## Assumptions

- httpx ライブラリを使用する（Constitution v0.5.0 で承認済み）
- Python 3.13 以上を対象とする
- すべての API は非同期（async/await）で実装する
- TLS 証明書検証は httpx のデフォルト動作に従う
- 認証処理は HTTP クライアントの責務外とし、各アダプターで実装する

## References

- [Constitution v0.5.0](/.specify/memory/constitution.md) - HTTP クライアントをコアスコープに追加
- [001-core](../001-core/spec.md) - コアアーキテクチャ仕様
- [002-data-model](../002-data-model/spec.md) - データモデル仕様
- [Implementation Plan](./features/http-client-layer/plan.md) - 実装計画
- [Architecture Design](./features/http-client-layer/architecture.md) - アーキテクチャ設計
