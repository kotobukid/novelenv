# 小説創作プロジェクト

構造化された原稿管理、キャラクター開発、およびナラティブコンテキスト管理のための専用コマンドラインツールを備えた包括的な創作プロジェクトリポジトリです。

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

## 機能

### コンテンツ管理
- **キャラクタープロファイル**: 性格、能力、関係性を含む詳細なキャラクター開発テンプレート
- **世界設定**: 創作世界の包括的なドキュメント管理
- **エピソード管理**: 自動メタデータ抽出によるエピソード追跡
- **文体ガイドライン**: 一貫した文体のためのスタイルガイド管理

### コマンドラインツール

#### 1. Context Weaver（コンテキストウィーバー）
複雑な創作プロジェクトのためのWebベースのナラティブコンテキスト管理ツール。

**機能:**
- プロジェクトファイルを閲覧するインタラクティブWebUI
- ドラッグ&ドロップによるコンテキスト構築
- ユニークIDでナラティブコンテキストを保存・管理
- 保存されたコンテキストのCLI解決

**使用方法:**
```bash
# Webサーバー起動
./cli-tools/context-weaver/target/release/weaver serve --port 3000

# 保存されたコンテキストを解決
./cli-tools/context-weaver/target/release/weaver resolve <NARRATIVE_ID>
```

#### 2. Find Context（ファインドコンテキスト）
エイリアスサポート付きプロジェクト固有コンテンツ検索の統合インターフェース。

**機能:**
- 名前またはエイリアスによるキャラクタープロファイル検索
- キャラクター別エピソード検索
- プロジェクト対応コンテキスト検索

**使用方法:**
```bash
# キャラクタープロファイル取得
./cli-tools/find-context/target/release/find-context profile <character_name>

# キャラクターが登場するエピソードを検索
./cli-tools/find-context/target/release/find-context episode --character <character_name>
```

#### 3. Episode Info Dumper（エピソード情報ダンパー）
LLM分析を使用したエピソードファイルからの自動メタデータ抽出。

**機能:**
- エピソードからキャラクター、テーマ、ログラインを抽出
- 検索可能なJSONインデックス生成
- LLMを活用したコンテンツ分析

**使用方法:**
```bash
cd cli-tools/dump-episode-info
cargo run
```

## はじめに

### 前提条件
- Rust 1.70+（CLIツール用）
- モダンWebブラウザ（Context Weaver WebUI用）
- LLM CLIツール（Episode Info DumperにはClaude CLI推奨）

### インストール
1. リポジトリをクローン
2. CLIツールをビルド:
   ```bash
   cd cli-tools/context-weaver && cargo build --release
   cd ../find-context && cargo build --release
   cd ../dump-episode-info && cargo build --release
   ```

### クイックスタート
1. **コンテンツ閲覧**: `character_profile/`、`episode/`、`official/`ディレクトリを探索
2. **Context Weaver起動**: WebUIを起動してナラティブコンテキストを作成
3. **コンテンツ検索**: `find-context`を使用してキャラクターやエピソード情報を素早く検索
4. **インデックス更新**: 新しいエピソード追加後に`dump-episode-info`を実行

## 設定

プロジェクトはツール設定に`.fcrc`設定ファイルを使用します:

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