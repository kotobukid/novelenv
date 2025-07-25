# Episode Info Dumper

## Overview

`dump-episode-info` は、小説プロジェクトのエピソードファイルを解析し、キャラクター、ログライン、テーマを抽出してJSONインデックスファイルを生成するツールです。LLMを活用してMarkdownファイルの内容を構造化データに変換します。

## Purpose

このツールは以下の目的で作成されています：
- エピソードファイルから自動的にメタデータを抽出
- `find-context` ツールが使用するエピソードインデックスの生成
- 創作プロジェクトの管理とナビゲーションの支援

## Configuration

ツールは `.fcrc` 設定ファイルを使用します。設定例：

```toml
[tools.llm_cli]
command = "claude"
prompt_flag = "--prompt"

[dump_settings]
input_dir = "episode"
output_file = "episode_index.json"
```

### 設定項目

- `tools.llm_cli.command`: 使用するLLMのCLIコマンド
- `tools.llm_cli.prompt_flag`: プロンプトを渡すためのフラグ
- `dump_settings.input_dir`: エピソードファイルが格納されているディレクトリ
- `dump_settings.output_file`: 出力JSONファイルのパス

## Usage

### NovelEnv統合CLI経由（推奨）

```bash
# NovelEnvプロジェクト内で実行
novel dump episodes
```

### 直接実行

プロジェクトルートから以下のコマンドを実行：

```bash
cd cli-tools/dump-episode-info
cargo run
```

## Output Format

生成される `episode_index.json` の形式：

```json
[
  {
    "episode_path": "episode/ep1.md",
    "characters": ["主人公", "ヒロイン", "悪役"],
    "logline": "主人公が謎の事件に巻き込まれる冒険の始まり。",
    "themes": ["友情", "成長", "冒険"]
  }
]
```

## Requirements

- Rust 1.70+
- LLMのCLIツール（Claude CLI、OpenAI CLI等）
- `.fcrc` 設定ファイル

## Dependencies

- `serde`: JSON/TOML serialization
- `glob`: ファイルパターンマッチング
- `regex`: JSON抽出用正規表現
- `toml`: 設定ファイル解析

## Technical Notes

- プロジェクトルートは `.fcrc` ファイルの存在で判定
- LLMレスポンスから正規表現でJSONを抽出
- パースエラーやLLM呼び出し失敗は個別にログ出力して継続処理
- 各エピソードファイルに対して個別にLLMを呼び出すため、処理時間が長くなる場合があります

## Integration

このツールで生成されたインデックスは、`find-context episode --character <name>` コマンドで使用されます。