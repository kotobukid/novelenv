# Context Weaver

## Overview

Context Weaver は、創作プロジェクト向けのナラティブコンテキスト管理ツールです。複数のファイルから必要な部分を選択して組み合わせ、統合されたコンテキストとして保存・解決できるWebベースのツールです。

## Purpose

このツールは以下の目的で作成されています：
- 複雑な創作プロジェクトでのコンテキスト管理
- 複数ファイルの部分的な内容を組み合わせたナラティブの作成
- LLMとの連携時に必要なコンテキストの効率的な生成
- プロジェクトファイルの構造化されたナビゲーション

## Features

### Web UI
- **ファイルエクスプローラー**: プロジェクトファイルのツリービュー表示
- **ドラッグ&ドロップ**: 直感的なファイル選択とコンテキスト構築
- **ナラティブ管理**: 作成したコンテキストの保存・編集・削除
- **プレビュー機能**: ファイル内容の事前確認

### CLI機能
- **Webサーバー起動**: `serve` コマンドでWeb UIを提供
- **コンテキスト解決**: `resolve` コマンドで保存されたナラティブを出力

## Usage

### Web UI Server の起動

```bash
./cli-tools/context-weaver/target/release/weaver serve --port 3000 --path /path/to/project
```

デフォルトで http://localhost:3000 でアクセス可能になります。

#### オプション
- `--port, -p`: サーバーポート（デフォルト: 3000）
- `--path, -p`: プロジェクトルートパス（デフォルト: 現在のディレクトリ）

### CLI からのナラティブ解決

```bash
./cli-tools/context-weaver/target/release/weaver resolve <NARRATIVE_ID> --path /path/to/project
```

保存されたナラティブの統合コンテキストを標準出力に出力します。

## Data Structure

### ナラティブデータ

```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Episode 1 Context",
  "description": "Main character introduction scene",
  "contexts": [
    {
      "path": "character_profile/protagonist.md",
      "include_type": { "type": "Full" },
      "order": 0
    },
    {
      "path": "episode/ep1.md",
      "include_type": { "type": "Section", "section": "Scene 1" },
      "order": 1
    },
    {
      "path": "world_setting/magic_system.md",
      "include_type": { "type": "Lines", "start": 10, "end": 50 },
      "order": 2
    }
  ],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### インクルードタイプ

- **Full**: ファイル全体を含める
- **Section**: 特定のセクション（Markdownの見出し）を含める
- **Lines**: 指定行範囲を含める

## Storage

ナラティブデータは `.weaver-narratives.json` ファイルに保存されます。このファイルはプロジェクトルートに作成され、すべての保存済みナラティブが含まれます。

## Web UI の使い方

1. **サーバー起動**: `weaver serve` コマンドでWebサーバーを開始
2. **ファイル選択**: 左側のファイルツリーから必要なファイルを選択
3. **コンテキスト構築**: 中央のエリアにファイルをドラッグ&ドロップ
4. **ナラティブ保存**: 名前と説明を入力して「Save」ボタンをクリック
5. **管理**: 右側のパネルで保存済みナラティブの一覧・編集・削除

## Technical Requirements

- Rust 1.70+
- Tokio async runtime
- Modern web browser (JavaScript対応)

## Dependencies

- `axum`: Web framework
- `serde`: Serialization
- `uuid`: ユニークID生成
- `chrono`: 日時処理
- `tower-http`: HTTP middleware
- `anyhow`: エラーハンドリング

## API Endpoints

- `GET /api/files` - ファイル一覧取得
- `GET /api/narratives` - ナラティブ一覧取得
- `POST /api/narratives` - ナラティブ作成
- `GET /api/narratives/:id` - 特定ナラティブ取得
- `PUT /api/narratives/:id` - ナラティブ更新
- `DELETE /api/narratives/:id` - ナラティブ削除
- `GET /api/narratives/:id/resolve` - ナラティブコンテキスト解決

## Integration

このツールで作成・保存されたナラティブは、NARRATIVE_IDを使って他のツールやLLMとの連携で活用できます。特にClaude Codeなどのツールとの組み合わせで、複雑なコンテキストを効率的に管理できます。