# NovelEnv v2 - 小説創作環境管理システム

Python venvのような環境切り替え機能と統合CLIツールを提供する、創作プロジェクトの包括的な環境管理システムです。

## プロジェクト概要

このリポジトリは創作小説プロジェクトの管理システムです。キャラクター設定、エピソード管理、世界設定の整理と、それらを効率的に扱うための専用ツール群を提供します。複雑な創作プロジェクトでのコンテンツ管理とナラティブ構築を支援することを目的としています。

## リポジトリ構造

```
project/
├── character_profile/     # キャラクタープロファイルと開発
├── episode/              # ストーリーエピソードとチャプター
├── scene_sketch/         # 生成されたシーンドラフトとスニペット
├── summary/              # エピソード要約
├── official/             # 世界設定、ルール、コンセプト文書
├── writing_style/        # 文体ガイドライン
├── cli-tools/           # 専用コマンドラインユーティリティ
└── episode_index.json   # 生成されたエピソードメタデータインデックス
```

## 主要機能

### NovelEnv v2の新機能
- **プロジェクト初期化**: インタラクティブなセットアップウィザード
- **環境切り替え**: Python venv風の環境アクティベーション
- **統合CLI**: すべてのツールを`novel`コマンドで統一操作
- **文体管理**: 複数の文体スタイルファイルを選択・管理
- **サンプルキャラクター**: テンプレートキャラクターのインポート

## コンテンツ管理機能

### コンテンツ管理
- **キャラクタープロファイル**: 性格、能力、関係性を含む詳細なキャラクター開発テンプレート
- **世界設定**: 創作世界の包括的なドキュメント管理
- **エピソード管理**: 自動メタデータ抽出によるエピソード追跡
- **文体ガイドライン**: 一貫した文体のためのスタイルガイド管理

### 統合コマンドラインツール

NovelEnv v2では、すべてのツールが`novel`コマンドから統一的にアクセスできます。

#### 1. プロジェクト初期化
```bash
# インタラクティブモード
novel init my-novel

# 非インタラクティブモード
novel init my-novel --non-interactive
```

#### 2. 環境アクティベーション
```bash
# 環境をアクティベート
novel activate my-novel

# 現在のディレクトリをアクティベート
novel activate .
```

#### 3. Context Weaver（コンテキストウィーバー）
```bash
# Webサーバー起動
novel weave serve --port 3000

# ナラティブコンテキストを解決
novel weave resolve <NARRATIVE_ID>
```

#### 4. Find Context（ファインドコンテキスト）
```bash
# キャラクタープロファイル取得
novel find-context profile <character_name>

# キャラクターが登場するエピソードを検索
novel find-context episode --character <character_name>
```

#### 5. Episode Info Dumper（エピソード情報ダンパー）
```bash
# エピソード情報をダンプ
novel dump episodes
```

## はじめに

### 前提条件
- Rust 1.70+（CLIツール用）
- モダンWebブラウザ（Context Weaver WebUI用）
- LLM CLIツール（Episode Info DumperにはClaude CLI推奨）

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/novelenv.git
cd novelenv

# インストールスクリプトを実行
./install.sh

# PATHを通す（~/.bashrc や ~/.zshrc に追加）
export PATH="$HOME/.local/bin:$PATH"
```

インストールスクリプトは以下を実行します：
- すべてのCLIツールをビルド
- `~/.local/bin`にシンボリックリンクを作成
- リビルド時は自動的に最新版が反映される

### クイックスタート

```bash
# 1. 新規プロジェクトを作成
novel init my-novel

# 2. プロジェクトに移動
cd my-novel

# 3. 環境をアクティベート（オプション）
novel activate .

# 4. キャラクタープロファイルを検索
novel find-context profile アベル

# 5. Context Weaver WebUIを起動
novel weave serve --port 3000

# 6. エピソード情報をダンプ
novel dump episodes
```

## 設定

プロジェクトはツール設定に`find_context.toml`設定ファイルを使用します:

```toml
# キャラクタープロファイルエイリアス
[profile.aliases]
"アベル" = "character_profile/アベル.md"
"ハンナ" = "character_profile/ハンナ.md"

# LLM CLI設定
[tools.llm_cli]
command = "claude"
prompt_flag = "--prompt"

# ダンプ設定
[dump_settings]
input_dir = "episode"
output_file = "episode_index.json"
```

## 貢献

お気軽に

## ライセンス

MITライセンス

## お問い合わせ

ツールや方法論についての質問は、`cli-tools/`サブディレクトリ内の個別のREADMEファイルを参照してください。