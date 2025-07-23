CLAUDE.mdを参照し従う

エラーや誤りがあった場合、謝罪やへりくだった表現はせず、原因と対策を簡潔に述べること

---

## カスタムコマンドの実行について

このリポジトリには、`.claude/commands/` ディレクトリに定義された、`/` から始まるカスタムコマンドが存在します。
これらのコマンドがユーザーから指示された場合、あなたは `./run_claude_command.sh` スクリプトを使用してそれを実行する必要があります。

### ルール

1.  ユーザーが `/command_name [arguments]` の形式で指示を出した場合、あなたは以下の形式でシェルコマンドを実行してください。
    ```bash
    bash ./cli-tools/run_claude_command.sh command_name [arguments]
    ```

2.  スクリプトが「LLMが直接ステップを実行するタイプです」という情報を返した場合、それはスクリプトによる自動実行が未対応のコマンドです。その場合は、指示された通り、対応する `.md` ファイル（例: `.claude/commands/char-lint.md`）の内容を読み解き、そこに書かれているタスクをあなたが直接実行してください。

### 実行例

- **ユーザーの指示:** `/sketch 22823daf-4a80-4c54-b04a-c7a6c2669e71`
- **あなたが実行するべきコマンド:** `bash ./cli-tools/run_claude_command.sh sketch 22823daf-4a80-4c54-b04a-c7a6c2669e71`

- **ユーザーの指示:** `/char-lint episode/ep1.md アベル`
- **あなたが実行するべきコマンド:** `bash ./cli-tools/run_claude_command.sh char-lint episode/ep1.md アベル`

