# Novel-Init - Project Initialization Tool

NovelEnvプロジェクトの初期化を行うCLIツールです。インタラクティブなセットアップでプロジェクト構造の作成、設定ファイルの生成、文体スタイルの選択、サンプルキャラクターのインポートを行います。

## 機能

### 1. インタラクティブプロジェクトセットアップ
- プロジェクト種類の選択（長編小説、短編集、連作シリーズ、その他）
- ジャンル選択（SF、ファンタジー、ミステリー等）
- プロジェクト説明の入力

### 2. ディレクトリ構造の自動作成
以下のディレクトリ構造を自動生成：
```
project-name/
├── character/     # キャラクタープロファイル
├── episode/              # エピソード・章
├── scene_sketch/         # シーンスケッチ・下書き
├── summary/              # 要約・あらすじ
├── environment/          # 世界設定・環境設定
├── notes/               # 概念・ギミック・技術説明
├── writing_style/        # 文体・スタイルガイド
├── .novelenv/           # NovelEnv設定ディレクトリ
└── .claude/commands/    # カスタムスラッシュコマンド
```

### 3. 設定ファイル生成
- `.novelenv/config.toml`: プロジェクト設定
- `CLAUDE.md`: LLM連携用プロジェクト説明
- `README.md`: プロジェクト説明
- `.gitignore`: Git除外設定
- `writing_style/always.md`: 基本文体設定

### 4. 文体スタイル選択機能
- NovelEnvの`writing_style/`ディレクトリから利用可能なスタイルファイルをスキャン
- MultiSelectインターフェースで複数選択可能
- `always.md`は必須として自動選択

### 5. サンプルキャラクターインポート
- アベル.mdとハンナ.mdのサンプルキャラクターファイルをインポート可能
- ユーザー確認後にNovelEnvの`character/`からコピー

### 6. カスタムコマンドコピー
- NovelEnvの`.claude/commands/`からカスタムスラッシュコマンドを自動コピー

## 使用方法

### 基本的な使用

```bash
# インタラクティブモード（推奨）
./target/release/novel-init my-novel

# 非インタラクティブモード
./target/release/novel-init my-novel --non-interactive
```

### オプション

- `--non-interactive`: 対話なしでデフォルト設定でプロジェクトを作成

## ビルド方法

```bash
cd cli-tools/novel-init
cargo build --release
```

## 依存関係

### Rust Crates
- `clap`: コマンドライン引数解析
- `dialoguer`: インタラクティブUI（Select, MultiSelect, Confirm）
- `serde`: 設定ファイルシリアライゼーション
- `toml`: TOML設定ファイル処理
- `chrono`: 日時処理

### 外部依存
- NovelEnvの`writing_style/`ディレクトリ（文体スタイルファイル用）
- NovelEnvの`character/`ディレクトリ（サンプルキャラクター用）
- NovelEnvの`.claude/commands/`ディレクトリ（カスタムコマンド用）

## 設定ファイル例

生成される`.novelenv/config.toml`の例：

```toml
name = "my-novel"
project_type = "長編小説"
genre = "SF"
description = "近未来を舞台にした冒険小説"
created = "2024-01-01"
```

## 統合

このツールは通常、NovelEnvメインCLI（`novel`コマンド）経由で使用されます：

```bash
novel init my-project
```

## 技術仕様

- パス解決：開発環境（`cli-tools/*/target/release/`）と本番環境（`~/.local/bin/`）の両方に対応
- エラーハンドリング：ファイル操作エラーを適切にキャッチして継続処理
- 文字エンコーディング：UTF-8対応、日本語文字境界の適切な処理
- インタラクティブUI：端末接続確認とフォールバック処理