# 設定ファイル名変更のお知らせ

## 変更内容

NovelEnv v2では、設定ファイル名を `.fcrc` から `find_context.toml` に変更しました。

### 変更理由

- ファイル内容がTOML形式であることを明確にするため
- ツール名（find-context）との関連性を明確にするため
- 拡張子によってエディタのシンタックスハイライトが自動的に適用されるため

## 移行方法

既存のプロジェクトで `.fcrc` を使用している場合は、以下のコマンドで移行してください：

```bash
# プロジェクトルートで実行
mv .fcrc find_context.toml
```

## 影響を受けるツール

以下のツールが新しいファイル名を使用するよう更新されました：

- `find-context` (novel find-context)
- `dump-episode-info` (novel dump episodes)

## 設定ファイルの内容

ファイル名以外の変更はありません。設定内容はそのまま使用できます。

```toml
# find_context.toml - Configuration for find-context tool

[profile.aliases]
"アベル" = "character/アベル.md"
"ハンナ" = "character/ハンナ.md"

[tools.llm_cli]
command = "claude"
prompt_flag = "--prompt"

[dump_settings]
input_dir = "episode"
output_file = "episode_index.json"
```