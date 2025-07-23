---
description: エピソードファイルをサマライズしてMarkdown形式で出力する
---

# Episode Summarizer

Usage: `/summ <episode_file>`

Extract the episode file path from `$ARGUMENTS`.
- Episode file should be in the format: `@episode/filename.md`
- Output will be created as: `summary/filename.md`

## Source Episode

@[episode_file]

## Pre-processing Check

**IMPORTANT: Before generating the summary, check if the output file already exists.**

Check if the file `summary/[filename].md` already exists:
- If the file exists, respond with: "サマリーファイル 'summary/[filename].md' は既に存在します。上書きを避けるため、処理を中止します。"
- If the file does not exist, proceed with summarization.

## Summarization Task

Create a comprehensive but concise summary in Japanese using the following Markdown structure:

```markdown
# エピソード概要
[1-2文での簡潔な要約]

## 基本情報
- **タイトル**: [エピソードのタイトル]
- **シリーズ**: [温泉シリーズ/大学シリーズ等]
- **文字数**: 約[X]文字
- **読了時間**: 約[X]分

## 主要キャラクター
- **[キャラクター名]**: [役割・特徴・この話での位置づけ]
- **[キャラクター名]**: [役割・特徴・この話での位置づけ]
[登場する主要キャラクターをリストアップ]

## あらすじ
### [第一章/章タイトル]
[章の内容を2-3文で要約]

### [第二章/章タイトル]
[章の内容を2-3文で要約]

[各章ごとに要約を記載]

## 重要な出来事
- [重要な出来事1]
- [重要な出来事2]
- [重要な出来事3]
[ストーリーの転換点や重要なシーンをリストアップ]

## テーマ・モチーフ
[この話で扱われているテーマ、科学的概念、心理学的要素等]

## 次回への繋がり
[続編への伏線や関係性の発展等があれば記載]
```

## Output Instructions

**If the output file does not exist:**
- Generate the summary using the structure above
- Write the content to `summary/[filename].md`
- Confirm successful creation

**If the output file exists:**
- Do not generate any summary
- Display the warning message in Japanese
- Do not create or modify any files

**IMPORTANT: All output should be in Japanese (日本語で出力してください)**

**NOTE: Ensure the summary captures the essence of the story while being concise and useful for quick reference.**