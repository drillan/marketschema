# Research: Async HTTP Client in Rust

**Feature**: 003-http-client-rust
**Date**: 2026-02-03

## 1. Reqwest Best Practices

**Decision**: `reqwest` + Tokio を使用し、クライアントの再利用とコネクションプーリングを基本パターンとする。

**Rationale**:
- reqwest は Rust の事実上標準 HTTP クライアント
- `Client` インスタンスの再利用によるコネクションプーリングで大幅な性能向上
- Tokio との統合により効率的な並行リクエスト処理が可能
- 組み込み機能: TLS（デフォルト rustls）、クッキー管理、リダイレクトポリシー、プロキシサポート

**Implementation Patterns**:
- 単一の `Client` インスタンスを作成し、複数リクエストで再利用
- タイムアウト設定: `Client::builder().timeout(Duration::from_secs(10))`
- ビルダーパターンでカスタマイズ（TLS、プロキシ、ヘッダー、リダイレクト）
- Tokio の Semaphore で並行リクエスト数を制限可能

**Alternatives Considered**:
- **ureq**: 軽量だが blocking のみ、非同期シナリオに不適
- **hyper**: 低レベル、手動実装が多く必要
- **surf**: async-std 向け、Tokio との統合が限定的

---

## 2. Token Bucket Rate Limiting in Rust

**Decision**: 手動実装を採用。外部依存を減らし、`Send + Sync` 境界を確実に満たすため。

**Rationale**:
- **leaky_bucket**: Tokio/async_std と完全互換の非同期対応
- **rater**: アトミック操作によるロックフリー実装で高性能
- トークンバケットアルゴリズムはバーストに対応した予測可能なレート制限を提供

**Implementation Patterns**:
- 非同期: `leaky_bucket` でワーカースレッドをブロックしない
- 高スループット: `rater` のアトミック操作とキャッシュアライン構造
- トークン補充は連続（タイマー）またはバッチ方式

**Alternatives Considered**:
- **tokenbucket**: シンプルだが同期的
- **ratelimit_meter**: 多機能だが重い
- **手動実装**: mutex/atomic の慎重な使用が必要

**決定**: 本実装では手動実装を採用。外部依存を減らし、`Send + Sync` 境界を確実に満たすため。`Mutex` + `Instant` による基本的なトークンバケットを実装。

---

## 3. LRU Cache in Rust

**Decision**: `moka` を使用。並行性とパフォーマンスのバランスが最良。

**Rationale**:
- **moka**: TinyLFU エビクション（LFU admission + LRU eviction）で最適に近いヒット率
- sync/async 両方のキャッシュバリアントをサポート
- 本番環境で実績あり（crate registries、高トラフィックサービス）
- ロックフリー並行ハッシュテーブルで中央ストレージ

**Moka Features**:
- 有効期限ポリシー: time-to-live, time-to-idle, エントリごとの可変期限
- エビクションリスナーで削除通知
- Eventually consistent 設計で最適性能

**Implementation Patterns**:
- 並行非同期: `Cache<K, V>` を async API で使用
- キャパシティ境界とエビクションポリシーを作成時に設定
- エビクションリスナーでキャッシュ無効化の副作用を処理

**Alternatives Considered**:
- **lru**: シンプルだが並行安全でない
- **cached**: 属性マクロ中心、低レベル制御が限定的
- **手動実装**: `RwLock<HashMap>` + LRU リスト

**決定**: `moka` を採用。`features = ["future"]` で非同期サポートを有効化。

---

## 4. WireMock Testing Patterns

**Decision**: `wiremock` を使用し、テストごとに独立した `MockServer` インスタンスを作成。

**Rationale**:
- Rust の非同期テスト用に設計（tokio, async_std サポート）
- 完全なテスト分離でクロステスト干渉を防止、並列実行が可能
- リクエストマッチングとレスポンステンプレート
- 期待値ベースの検証でスパイ的な動作

**Best Practices**:
1. **分離**: テストごとに1つの `MockServer` を作成、共有しない
2. **1対1マッピング**: モックする API ごとに1つの `MockServer`
3. **ランダムポート**: 自動ランダムポート割り当てで並列テスト安全
4. **期待値**: 呼び出し回数の期待値で副作用を検証
5. **スコープ付きモック**: ヘルパー関数では `register_as_scoped()` を使用

**Implementation Pattern**:
```rust
#[tokio::test]
async fn test_api_call() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // テストコード
}
```

**Alternatives Considered**:
- **mockito**: 逐次実行のみ、並列不可
- **httpmock**: 並列テスト対応だが非同期ネイティブでない

---

## 5. Cargo Workspace Structure

**Decision**: モノレポ構成で `crates/` サブディレクトリ構造とワークスペースレベルの依存管理を採用。

**Rationale**:
- 単一 `Cargo.lock` で全メンバークレートのバージョン一貫性
- 共有 `target/` ディレクトリで冗長な再ビルドを排除
- `workspace.dependencies` による依存管理の簡素化
- 並列テスト実行と協調リリースをサポート

**Recommended Structure**:
```
project-root/
├── Cargo.toml                    # Workspace root
├── Cargo.lock
├── target/                       # Shared build output
├── crates/
│   └── marketschema-http/
│       ├── Cargo.toml
│       └── src/
└── rust/                         # Existing marketschema crate
    ├── Cargo.toml
    └── src/
```

**Workspace Configuration Pattern**:
```toml
[workspace]
resolver = "2"
members = ["rust", "crates/marketschema-http"]

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
moka = { version = "0.12", features = ["future"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Best Practices**:
1. **モジュラーアーキテクチャ**: 各クレートは明確な責務
2. **明示的な依存関係**: ワークスペースにあっても各クレートで宣言
3. **内部パス依存**: `marketschema = { path = "../rust" }`
4. **共有開発依存**: dev-dependencies をワークスペースで集約
5. **選択的公開**: 公開すべきクレートのみ publish

**Common Commands**:
```bash
cargo build                           # ワークスペース全体をビルド
cargo test -p marketschema-http      # 特定クレートをテスト
cargo publish -p marketschema-http   # 特定クレートを公開
```

---

## Source References

- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Moka GitHub](https://github.com/moka-rs/moka)
- [WireMock Rust GitHub](https://github.com/LukeMathWalker/wiremock-rs)
- [Cargo Workspaces - The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Leaky Bucket Documentation](https://docs.rs/leaky-bucket)
