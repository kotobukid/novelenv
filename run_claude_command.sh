#!/bin/bash

# スクリプトの目的: .claude/commands/ に定義されたカスタムコマンドを解釈・実行する
# 使い方: bash ./cli-tools/run_claude_command.sh [コマンド名] [引数...]

set -e

# --- 引数のチェック ---
if [ -z "$1" ]; then
  echo "エラー: コマンド名が指定されていません。"
  echo "使い方: $0 [コマンド名] [引数...]"
  exit 1
fi

COMMAND_NAME=$1
shift # 第1引数（コマンド名）を消費し、残りを引数として扱う
ARGUMENTS="$@"

# --- コマンド定義ファイルのパス ---
COMMAND_FILE=".claude/commands/${COMMAND_NAME}.md"

if [ ! -f "$COMMAND_FILE" ]; then
  echo "エラー: コマンド定義ファイルが見つかりません: $COMMAND_FILE"
  exit 1
fi

echo "コマンド '$COMMAND_NAME' を実行します..."
echo "引数: $ARGUMENTS"
echo "---"

# --- コマンドごとの処理分岐 ---
case "$COMMAND_NAME" in
  "sketch")
    # sketch.md から実行すべきコマンドラインを抽出して実行する
    # 例: ./cli-tools/context-weaver/target/release/weaver resolve [narrative_id]
    EXEC_TEMPLATE=$(grep -oE '\./cli-tools/context-weaver/target/release/weaver resolve \[narrative_id\]' "$COMMAND_FILE" | head -n 1)

    if [ -z "$EXEC_TEMPLATE" ]; then
      echo "エラー: sketch.md 内で実行可能なコマンド定義が見つかりませんでした。"
      exit 1
    fi

    # プレースホルダーを実際の引数に置換
    FULL_COMMAND=$(echo "$EXEC_TEMPLATE" | sed "s/\[narrative_id\]/$ARGUMENTS/")
    
    echo "実行コマンド: $FULL_COMMAND"
    eval "$FULL_COMMAND"
    ;;

  "char-lint" | "char-slim" | "reality-lint" | "summ")
    # これらのコマンドは、LLMが直接解釈して複数のステップを実行するタイプ
    echo "情報: コマンド '$COMMAND_NAME' は、LLMが直接ステップを実行するタイプです。"
    echo "このスクリプトによる自動実行には現在対応していません。"
    echo "LLMに '$COMMAND_FILE' の内容に従って処理を続けるよう指示してください。"
    # 正常終了とする
    exit 0
    ;;

  *)
    echo "エラー: 未知のコマンドか、スクリプトがまだ対応していないコマンドです: '$COMMAND_NAME'"
    exit 1
    ;;
esac

echo "---"
echo "コマンド '$COMMAND_NAME' の実行が完了しました。"
