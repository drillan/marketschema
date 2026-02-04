# CLAUDE.md

## Documentation

ドキュメントのルールは `.claude/docs.md` に従ってください。

## Constitution

プロジェクトの詳細な原則は `.specify/memory/constitution.md` を参照してください。

## Python使用ルール

- システムの`python3`コマンドを直接使用しないこと
- このプロジェクトでは`--directory`オプションを付けてuvを使用する:
  ```bash
  # プロジェクトルートにいる場合（推奨）
  uv --directory ./python run python
  uv --directory ./python run pytest

  # カレントディレクトリが異なる場合
  uv --directory $PROJECT_ROOT/python run python
  uv --directory $PROJECT_ROOT/python run pytest
  ```
- 可能な限りプロジェクトルートで作業し、相対パス（`./python`）を使用すること

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

3. **一時ファイルの配置**: 一時的なスクリプト、デバッグ出力、手動テスト用の一時ファイルはプロジェクトルートや新規ディレクトリを作成せず、`ai_working/` に配置する（`ai_working/` は `.gitignore` に含まれているためコミットされない）
   ```
   # NG
   <project-root>/test_script.py
   <project-root>/debug/output.txt

   # OK
   <project-root>/ai_working/test_script.py
   <project-root>/ai_working/output.txt
   ```

### Quality Standards

- 型ヒント / 型注釈は必須
- JSON Schema のフィールドには必ず `description` を記述
- 自動生成コードは手動編集しない

### Quality Checks

コミット前に以下のチェックがすべて通ることを確認すること：

```bash
uv --directory ./python run ruff check .      # リンター
uv --directory ./python run ruff format --check .  # フォーマッター
uv --directory ./python run mypy src          # 型チェック
uv --directory ./python run pytest            # テスト
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
- Rust (latest stable) + serde, serde_json, chrono, regress (002-data-model-rust)
- Rust 2024 edition (latest stable, MSRV 1.71.1 for moka compatibility) + async-trait 0.1, chrono 0.4, once_cell 1.19, serde 1.0, serde_json 1.0, thiserror 2.0, tokio 1.0 (004-adapter-rust)
- N/A（インメモリ処理のみ） (004-adapter-rust)

## Recent Changes
- 002-data-model: Added Python 3.13, Rust (latest stable)
