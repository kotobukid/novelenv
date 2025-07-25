#!/bin/bash
# NovelEnv v2 インストールスクリプト

set -e

echo "🚀 NovelEnv v2 をインストールしています..."

# インストール先ディレクトリを作成
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# バイナリをコピー
echo "📦 バイナリをコピー中..."
cp cli-tools/novelenv/target/release/novel "$INSTALL_DIR/"
cp cli-tools/find-context/target/release/find-context "$INSTALL_DIR/"
cp cli-tools/context-weaver/target/release/weaver "$INSTALL_DIR/"
cp cli-tools/dump-episode-info/target/release/dump-episode-info "$INSTALL_DIR/"

# 実行権限を付与
chmod +x "$INSTALL_DIR"/{novel,find-context,weaver,dump-episode-info}

echo "✅ インストール完了!"
echo ""
echo "🔧 次の手順を実行してください:"
echo ""
echo "1. PATHを設定:"
echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "2. シェル設定ファイルに追加（永続化）:"
if [[ "$SHELL" == *"zsh"* ]]; then
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
    echo "   source ~/.zshrc"
elif [[ "$SHELL" == *"bash"* ]]; then
    echo "   echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo "   source ~/.bashrc"
else
    echo "   シェル設定ファイル（~/.bashrc や ~/.zshrc など）に上記のexport文を追加"
fi
echo ""
echo "3. 動作確認:"
echo "   novel --version"
echo ""
echo "🎉 準備完了！ 'novel init my-project' でプロジェクトを開始できます。"