#!/bin/bash

# NovelEnv Installation Script
# This script builds all tools and creates symbolic links instead of copying binaries

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="$HOME/.local/bin"

echo "🚀 NovelEnv インストールを開始します..."

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Build all tools
echo "📦 すべてのツールをビルド中..."
(cd "$SCRIPT_DIR/cli-tools/find-context" && cargo build --release)
(cd "$SCRIPT_DIR/cli-tools/context-weaver" && cargo build --release)
(cd "$SCRIPT_DIR/cli-tools/dump-episode-info" && cargo build --release)
(cd "$SCRIPT_DIR/cli-tools/novel-init" && cargo build --release)
(cd "$SCRIPT_DIR/cli-tools/novelenv" && cargo build --release)

# Create symbolic links instead of copying
echo "🔗 シンボリックリンクを作成中..."
ln -sf "$SCRIPT_DIR/cli-tools/novelenv/target/release/novel" "$INSTALL_DIR/novel"
ln -sf "$SCRIPT_DIR/cli-tools/novel-init/target/release/novel-init" "$INSTALL_DIR/novel-init"
ln -sf "$SCRIPT_DIR/cli-tools/find-context/target/release/find-context" "$INSTALL_DIR/find-context"
ln -sf "$SCRIPT_DIR/cli-tools/context-weaver/target/release/weaver" "$INSTALL_DIR/weaver"
ln -sf "$SCRIPT_DIR/cli-tools/dump-episode-info/target/release/dump-episode-info" "$INSTALL_DIR/dump-episode-info"

echo "✅ インストール完了！"
echo ""
echo "📝 以下をシェルの設定ファイル（~/.bashrc や ~/.zshrc）に追加してください："
echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "🎉 novel コマンドが使用できるようになりました！"