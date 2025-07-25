#!/bin/bash
# NovelEnv v2 永続インストールスクリプト

set -e

echo "🚀 NovelEnv v2 を永続インストールしています..."

# インストール先ディレクトリを作成
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# バイナリをコピー
echo "📦 バイナリをコピー中..."
cp cli-tools/novelenv/target/release/novel "$INSTALL_DIR/"
cp cli-tools/find-context/target/release/find-context "$INSTALL_DIR/"
cp cli-tools/context-weaver/target/release/weaver "$INSTALL_DIR/"
cp cli-tools/dump-episode-info/target/release/dump-episode-info "$INSTALL_DIR/"
cp cli-tools/novel-init/target/release/novel-init "$INSTALL_DIR/"

# staticディレクトリをコピー（context-weaver用）
echo "📦 静的ファイルをコピー中..."
if [ -d "cli-tools/context-weaver/static" ]; then
    cp -r cli-tools/context-weaver/static "$INSTALL_DIR/"
    echo "✅ context-weaver用静的ファイルをコピーしました"
else
    echo "⚠️  context-weaver/static ディレクトリが見つかりません"
fi

# 実行権限を付与
chmod +x "$INSTALL_DIR"/{novel,find-context,weaver,dump-episode-info,novel-init}

echo "✅ バイナリのインストール完了!"

# シェル設定ファイルを検出
SHELL_CONFIG=""
if [[ "$SHELL" == *"zsh"* ]] && [[ -f "$HOME/.zshrc" ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
    SHELL_NAME="zsh"
elif [[ "$SHELL" == *"bash"* ]] && [[ -f "$HOME/.bashrc" ]]; then
    SHELL_CONFIG="$HOME/.bashrc"
    SHELL_NAME="bash"
elif [[ -f "$HOME/.profile" ]]; then
    SHELL_CONFIG="$HOME/.profile"
    SHELL_NAME="profile"
else
    echo "⚠️  シェル設定ファイルが見つかりません"
    echo "   手動で以下を設定ファイルに追加してください:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    exit 1
fi

# PATH設定の確認と追加
PATH_LINE='export PATH="$HOME/.local/bin:$PATH"'
if grep -q "\.local/bin" "$SHELL_CONFIG"; then
    echo "📝 PATH設定は既に存在します ($SHELL_CONFIG)"
else
    echo "📝 PATH設定を追加中... ($SHELL_CONFIG)"
    echo "" >> "$SHELL_CONFIG"
    echo "# NovelEnv PATH" >> "$SHELL_CONFIG"
    echo "$PATH_LINE" >> "$SHELL_CONFIG"
    echo "✅ PATH設定を追加しました"
fi

echo ""
echo "🎉 インストール完了!"
echo ""
echo "🔧 次の手順:"
echo "1. 新しいターミナルを開く、または:"
echo "   source $SHELL_CONFIG"
echo ""
echo "2. 動作確認:"
echo "   novel --version"
echo ""
echo "3. プロジェクト作成:"
echo "   novel init my-project"
echo "   cd my-project"
echo "   source <(novel activate)"
echo ""
echo "📖 詳細は以下を参照:"
echo "   cat cli-tools/novelenv/ACTIVATION_GUIDE.md"