# Research: HTTP Client Layer

**Feature**: 003-http-client
**Date**: 2026-02-03
**Status**: Complete

## Research Topics

### 1. HTTP Client Library Selection

**Decision**: httpx

**Rationale**:
- モダンな非同期 HTTP クライアント（async/await ネイティブサポート）
- requests API と互換性のある直感的なインターフェース
- HTTP/2 サポート
- 自動コネクションプーリング
- タイムアウト設定の細かい制御
- テスト用の respx ライブラリが利用可能

**Alternatives Considered**:

| ライブラリ | 評価 | 却下理由 |
|-----------|------|----------|
| aiohttp | 広く使用されている | API が httpx より複雑、より低レベル |
| urllib3 | 標準的 | 同期のみ、非同期サポートなし |
| requests | 最も人気 | 同期のみ、非同期サポートなし |
| niquests | requests 後継 | まだ成熟度が低い |

### 2. Error Handling Strategy

**Decision**: カスタム例外階層で httpx 例外をラップ

**Rationale**:
- ライブラリ利用者が httpx に依存しなくて良い
- 一貫したエラーメッセージとエラー情報
- `__cause__` で元の例外を保持し、デバッグ可能性を維持
- Constitution「暗黙的フォールバック禁止」に適合

**Exception Hierarchy**:
```
MarketSchemaError
└── HttpError
    ├── HttpTimeoutError
    ├── HttpConnectionError
    └── HttpStatusError
        └── HttpRateLimitError
```

**Alternatives Considered**:
- httpx 例外をそのまま伝播 → ライブラリの抽象化が破れる（却下）
- 単一の HttpError のみ → エラー種別による分岐が困難（却下）

### 3. Retry Strategy

**Decision**: 指数バックオフ（Exponential Backoff）

**Rationale**:
- 一時的なエラーに対する標準的なアプローチ
- サーバー負荷を軽減
- ジッター（jitter）追加で同時リトライを分散

**Implementation Details**:
- デフォルト最大リトライ: 3 回
- バックオフ係数: 0.5
- 計算式: `delay = backoff_factor * (2 ** attempt)`
- ジッター: ±10% のランダム化

**Retry Target Status Codes**:
- 429 (Too Many Requests) - レート制限
- 500 (Internal Server Error)
- 502 (Bad Gateway)
- 503 (Service Unavailable)
- 504 (Gateway Timeout)

**Non-Retryable Status Codes**:
- 400 (Bad Request)
- 401 (Unauthorized)
- 403 (Forbidden)
- 404 (Not Found)

**Alternatives Considered**:
- 固定間隔リトライ → サーバー負荷軽減効果が低い（却下）
- リトライなし → 一時的エラーでの失敗率が高い（却下）

### 4. Rate Limiting Strategy

**Decision**: トークンバケットアルゴリズム

**Rationale**:
- バースト許容と平均レート制限のバランス
- 実装がシンプル
- 多くの API プロバイダーの制限モデルと親和性が高い

**Implementation Details**:
- `requests_per_second`: 1 秒あたりの最大リクエスト数
- `burst_size`: 瞬時に処理可能な最大リクエスト数（デフォルト: requests_per_second と同じ）
- トークン補充: 1/requests_per_second 秒ごとに 1 トークン

**Alternatives Considered**:
- リーキーバケット → バーストを許容しない（却下）
- 固定ウィンドウ → ウィンドウ境界での不公平（却下）
- スライディングウィンドウ → 実装が複雑（却下）

### 5. Caching Strategy

**Decision**: インメモリ LRU キャッシュ

**Rationale**:
- シンプルで高速
- 追加の依存関係なし
- 大半のユースケースで十分
- 複雑な分散キャッシュは YAGNI

**Implementation Details**:
- キャッシュキー: URL + クエリパラメータ
- TTL: デフォルト 5 分
- 最大エントリ数: デフォルト 1000
- LRU（Least Recently Used）による古いエントリの削除

**Alternatives Considered**:
- Redis → 追加の依存関係、セットアップが必要（却下）
- ディスクキャッシュ → I/O オーバーヘッド、複雑さ（却下）
- キャッシュなし → 同一リクエストの重複（却下）

### 6. Connection Pooling

**Decision**: httpx デフォルトのコネクションプーリングを使用

**Rationale**:
- httpx.AsyncClient は自動でコネクションプーリングを実装
- 設定可能な最大接続数（デフォルト: 100）
- キープアライブ対応

**Configuration**:
- `max_connections`: 最大コネクション数（デフォルト: 100）
- `max_keepalive_connections`: キープアライブ接続数（httpx デフォルト）

### 7. Timeout Configuration

**Decision**: 単一のタイムアウト設定（httpx Timeout オブジェクトへの拡張可能性を維持）

**Rationale**:
- シンプルなユースケースでは単一のタイムアウト値で十分
- httpx は内部で connect/read/write/pool タイムアウトをサポート
- 必要に応じて Timeout オブジェクトを直接渡せるようにする

**Default Values**:
- timeout: 30 秒

### 8. BaseAdapter Integration

**Decision**: 遅延初期化 + コンテキストマネージャ

**Rationale**:
- 遅延初期化により、HTTP 不要なケースでのオーバーヘッドを回避
- コンテキストマネージャでリソース管理を自動化
- カスタムクライアントの注入を許可し、テスト容易性を向上

**Implementation Pattern**:
```python
class BaseAdapter:
    _http_client: AsyncHttpClient | None = None

    @property
    def http_client(self) -> AsyncHttpClient:
        if self._http_client is None:
            self._http_client = AsyncHttpClient()
        return self._http_client

    async def __aenter__(self) -> Self:
        return self

    async def __aexit__(self, *args) -> None:
        await self.close()
```

### 9. Testing Strategy

**Decision**: respx を使用した httpx モッキング

**Rationale**:
- httpx 専用のモッキングライブラリ
- シンプルな API
- リクエストアサーションのサポート
- 非同期テストとの完全な互換性

**Test Categories**:
1. **Unit Tests**: 個々のコンポーネント（client, middleware, cache）
2. **Integration Tests**: BaseAdapter との統合
3. **Contract Tests**: API コントラクトの検証

### 10. Phase Implementation Order

**Decision**: 3 フェーズに分割

**Phase 1 (P0-P1)**: Core HTTP Client
- AsyncHttpClient
- HTTP exceptions
- BaseAdapter integration
- 基本テスト

**Phase 2 (P2)**: Middleware
- RetryMiddleware
- RateLimitMiddleware
- Middleware テスト

**Phase 3 (P3)**: Cache
- ResponseCache
- Cache テスト

**Rationale**:
- YAGNI: 必要最小限から開始
- リスク低減: 小さな変更で問題を早期発見
- 価値の早期提供: Phase 1 完了でアダプター開発が可能

## Dependencies

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| httpx | >=0.27.0 | Async HTTP client |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| pytest-asyncio | >=0.24.0 | Async test support |
| respx | >=0.21.0 | httpx mocking |

## References

- [httpx Documentation](https://www.python-httpx.org/)
- [respx Documentation](https://lundberg.github.io/respx/)
- [Token Bucket Algorithm](https://en.wikipedia.org/wiki/Token_bucket)
- [Exponential Backoff](https://en.wikipedia.org/wiki/Exponential_backoff)
