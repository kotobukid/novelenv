# NovelEnv v2 - Novel Environment Management System

創作プロジェクトの環境管理システムです。Python venvのような環境切り替え機能と、プロジェクト初期化、既存CLIツールの統合インターフェースを提供します。

## 主要機能

### 1. プロジェクト初期化
- インタラクティブなプロジェクトセットアップ
- ディレクトリ構造の自動作成
- 文体スタイルファイルの選択とコピー
- サンプルキャラクターのインポート
- CLAUDE.md生成（LLM連携用）

### 2. 環境切り替え
- プロジェクト間の環境切り替え
- シェルプロンプトの変更（Python venvと同様）
- 環境変数の設定

### 3. 統合CLIインターフェース
- find-context, context-weaver, dump-episode-info の統一インターフェース

## ビルド方法

```bash
cd cli-tools/novelenv
cargo build --release
```

## 使用方法

### プロジェクト初期化

```bash
# インタラクティブ初期化
./target/release/novel init my-novel

# 非インタラクティブ初期化
./target/release/novel init my-novel --non-interactive
```

### 環境切り替え

```bash
# 環境アクティベート
./target/release/novel activate my-novel

# アクティベート後のシェルでは
# プロンプトが (my-novel) user@host:path$ のように変更される
# 環境変数 NOVELENV_ACTIVE, NOVELENV_PROJECT_ROOT が設定される
```

### 統合CLIツール

```bash
# キャラクタープロファイル検索
./target/release/novel find-context profile ハンナ

# コンテキストウィーバー起動
./target/release/novel weave serve --port 3000

# エピソード情報ダンプ
./target/release/novel dump episodes
```

## 依存関係

このツールは以下の既存ツールに依存しています：
- `cli-tools/novel-init/target/release/novel-init`
- `cli-tools/find-context/target/release/find-context`
- `cli-tools/context-weaver/target/release/weaver`
- `cli-tools/dump-episode-info/target/release/dump-episode-info`

各ツールが事前にビルドされている必要があります。

## インストール方法

プロジェクトルートから：

```bash
# 1. 全ツールを一括ビルド
(cd cli-tools/find-context && cargo build --release)
(cd cli-tools/context-weaver && cargo build --release)
(cd cli-tools/dump-episode-info && cargo build --release)
(cd cli-tools/novel-init && cargo build --release)
(cd cli-tools/novelenv && cargo build --release)

# 2. バイナリをローカルパスにコピー
cp cli-tools/novelenv/target/release/novel ~/.local/bin/
cp cli-tools/novel-init/target/release/novel-init ~/.local/bin/
cp cli-tools/find-context/target/release/find-context ~/.local/bin/
cp cli-tools/context-weaver/target/release/weaver ~/.local/bin/
cp cli-tools/dump-episode-info/target/release/dump-episode-info ~/.local/bin/

# 3. PATHを通す（~/.bashrc や ~/.zshrc に追加）
export PATH="$HOME/.local/bin:$PATH"
```

## プロジェクト構造

初期化されたプロジェクトの構造：

```
my-novel/
├── character_profile/     # キャラクタープロファイル
├── episode/              # エピソード・章
├── scene_sketch/         # シーンスケッチ・下書き
├── summary/              # 要約・あらすじ
├── official/             # 世界観・設定資料
├── writing_style/        # 文体・スタイルガイド
│   └── always.md         # 基本文体設定（自動生成）
├── .novelenv/           # NovelEnv設定
│   └── config.toml      # プロジェクト設定
├── .claude/             # Claude Code連携
│   └── commands/        # カスタムスラッシュコマンド
├── CLAUDE.md            # LLM連携用プロジェクト説明
├── README.md            # プロジェクト説明
└── .gitignore           # Git無視設定
```

## 技術仕様

- 実行時に動的パス解決で他のツールを検索・実行
- 開発環境と本番環境の両方に対応
- 引数はそのまま各ツールに転送
- 終了コードも適切に継承
- 環境変数とシェルプロンプト制御によるPython venv風の環境切り替え