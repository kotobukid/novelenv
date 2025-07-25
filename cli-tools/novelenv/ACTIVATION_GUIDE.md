# NovelEnv v2 環境切り替えガイド

## 環境のアクティベート

プロジェクトディレクトリ内で以下のコマンドを実行：

```bash
source <(novel activate)
```

## 使用例

```bash
# 1. プロジェクトを作成
novel init my-novel
cd my-novel

# 2. 環境をアクティベート
source <(novel activate)
# 📝 NovelEnv activated: my-novel

# 3. プロンプトが変更される
# 通常: user@host:~/path $
# 変更後: (my-novel) user@host:~/path $

# 4. 環境変数の確認
echo $NOVELENV_ACTIVE        # => my-novel
echo $NOVELENV_PROJECT_ROOT  # => /path/to/my-novel

# 5. どのサブディレクトリからでも動作
cd episode
source <(novel activate)    # 親ディレクトリのプロジェクトを自動検出
```

## 自動化

### bashrcに追加（自動アクティベート）

```bash
# ~/.bashrc に追加
novelenv_auto_activate() {
    if [ -d ".novelenv" ] && [ -z "$NOVELENV_ACTIVE" ]; then
        eval "$(novel activate 2>/dev/null)"
    fi
}

# ディレクトリ変更時に自動実行
cd() {
    builtin cd "$@" && novelenv_auto_activate
}
```

### エイリアス追加

```bash
# ~/.bashrc に追加
alias nv='source <(novel activate)'
alias nvi='novel init'
alias nvf='novel find-context'
```

## 環境の無効化

現在のセッションでは環境変数を直接解除：

```bash
unset NOVELENV_ACTIVE
unset NOVELENV_PROJECT_ROOT
# プロンプトは新しいシェルセッションで元に戻ります
```

## 複数プロジェクトの切り替え

```bash
# プロジェクトA
cd ~/novels/sci-fi-project
source <(novel activate)
# (sci-fi-project) $ 

# プロジェクトB
cd ~/novels/fantasy-project  
source <(novel activate)
# (fantasy-project) $
```

## トラブルシューティング

### "NovelEnvプロジェクトが見つかりません"エラー

- `.novelenv` ディレクトリが存在するか確認
- `novel init` でプロジェクトを初期化したか確認
- プロジェクトディレクトリ内またはサブディレクトリで実行しているか確認

### プロンプトが変更されない

- `source <(novel activate)` の `source` が重要（`novel activate` だけでは無効）
- シェルがbashまたはzshであることを確認
- 新しいシェルセッションを開始してみる