# CLAUDE.md

## Documentation

ドキュメントのルールは `.claude/docs.md` に従ってください。

## Constitution

プロジェクトの詳細な原則は `.specify/memory/constitution.md` を参照してください。

## Coding Rules

### 命名規則

- 同一概念には同一名称を使用（`qty` と `quantity` の混在禁止）
- 業界標準名を採用（下表参照）

| 概念 | 標準名 |
|------|--------|
| 買い/売り気配値 | `bid`, `ask` |
| OHLCV | `open`, `high`, `low`, `close`, `volume` |
| 約定 | `price`, `size`, `side` |
| 板情報 | `bids`, `asks`, `bid_size`, `ask_size` |
| 時刻 | `timestamp` |
| 銘柄 | `symbol` |
| 売買代金 | `quote_volume` |

新規フィールド追加時は [ADR](docs/adr/index.md) で決定プロセスを踏む。

### 禁止事項

1. **暗黙的フォールバック禁止**: エラーを握りつぶしてデフォルト値で処理しない
   ```python
   # NG
   except: return 0.0
   # OK
   except (ValueError, TypeError) as e: raise ConversionError(...) from e
   ```

2. **ハードコード禁止**: マジックナンバーには名前を付ける
   ```python
   # NG
   return ms / 1000
   # OK
   MS_PER_SECOND = 1000
   return ms / MS_PER_SECOND
   ```

### Quality Standards

- 型ヒント / 型注釈は必須
- JSON Schema のフィールドには必ず `description` を記述
- 自動生成コードは手動編集しない

### Quality Checks

コミット前に以下のチェックがすべて通ることを確認すること：

```bash
uv run ruff check .      # リンター
uv run ruff format --check .  # フォーマッター
uv run mypy src          # 型チェック
uv run pytest            # テスト
```

### 判断の優先順位

1. 正確性 → 2. シンプルさ → 3. 互換性 → 4. パフォーマンス → 5. 機能性

## Development Workflow

TDD サイクル（Red → Green → Refactor）に従う。コアライブラリと変換関数は TDD 必須。

## Active Technologies
- Python 3.13, Rust (latest stable) (002-data-model)
- N/A（スキーマ定義ファイルのみ） (002-data-model)
- Python 3.13 + httpx>=0.27.0, pydantic>=2.0.0 (003-http-client)
- N/A（インメモリキャッシュのみ） (003-http-client)
- Rust (latest stable, MSRV は別途決定) + reqwest (HTTP), tokio (async runtime), thiserror (errors), serde/serde_json (serialization) (003-http-client-rust)

## Recent Changes
- 002-data-model: Added Python 3.13, Rust (latest stable)
