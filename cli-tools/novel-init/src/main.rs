use clap::Parser;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "novel-init")]
#[command(about = "Initialize a new novel project")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(help = "Project name")]
    name: String,
    #[arg(long, help = "Skip interactive setup")]
    non_interactive: bool,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    project_type: String,
    genre: String,
    description: String,
    created: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // プロジェクトディレクトリが既に存在するかチェック
    if Path::new(&cli.name).exists() {
        eprintln!("❌ ディレクトリ '{}' は既に存在しています", cli.name);
        std::process::exit(1);
    }
    
    println!("🚀 NovelEnv プロジェクト '{}' を初期化しています...", cli.name);
    
    let config = if cli.non_interactive {
        ProjectConfig {
            name: cli.name.clone(),
            project_type: "novel".to_string(),
            genre: "その他".to_string(),
            description: "新しい小説プロジェクト".to_string(),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        }
    } else {
        interactive_setup(&cli.name)?
    };
    
    create_project_structure(&config)?;
    
    println!("✅ プロジェクト '{}' が正常に作成されました！", config.name);
    println!();
    println!("📁 次の手順:");
    println!("   cd {}", config.name);
    println!("   novel find-context profile --help");
    println!();
    println!("🎉 Happy writing!");
    
    Ok(())
}

fn interactive_setup(name: &str) -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    println!();
    
    let project_types = vec!["長編小説", "短編集", "連作シリーズ", "その他"];
    let project_type_index = Select::new()
        .with_prompt("プロジェクトの種類を選択してください")
        .items(&project_types)
        .default(0)
        .interact()?;
    
    let genres = vec!["SF", "ファンタジー", "ミステリー", "恋愛", "ホラー", "歴史", "現代", "その他"];
    let genre_index = Select::new()
        .with_prompt("ジャンルを選択してください")
        .items(&genres)
        .default(0)
        .interact()?;
    
    let description: String = Input::new()
        .with_prompt("プロジェクトの簡単な説明")
        .default("新しい小説プロジェクト".to_string())
        .interact_text()?;
    
    Ok(ProjectConfig {
        name: name.to_string(),
        project_type: project_types[project_type_index].to_string(),
        genre: genres[genre_index].to_string(),
        description,
        created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
    })
}

fn create_project_structure(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    // メインディレクトリを作成
    fs::create_dir(&config.name)?;
    
    // サブディレクトリを作成
    let directories = [
        "character_profile",
        "episode", 
        "scene_sketch",
        "summary",
        "official",
        "writing_style",
        ".novelenv"
    ];
    
    for dir in &directories {
        fs::create_dir_all(format!("{}/{}", config.name, dir))?;
    }
    
    // 設定ファイルを作成
    let config_content = toml::to_string_pretty(config)?;
    fs::write(
        format!("{}/.novelenv/config.toml", config.name),
        config_content
    )?;
    
    // CLAUDE.mdを生成
    let claude_md = generate_claude_md(config);
    fs::write(format!("{}/CLAUDE.md", config.name), claude_md)?;
    
    // .gitignoreを作成
    let gitignore_content = r#"# NovelEnv generated files
.novelenv/cache/
*.tmp

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db
"#;
    fs::write(format!("{}/.gitignore", config.name), gitignore_content)?;
    
    // README.mdを作成
    let readme_content = format!(r#"# {}

{}

## プロジェクト情報

- **種類**: {}
- **ジャンル**: {}
- **作成日**: {}

## 使用方法

このプロジェクトはNovelEnvで管理されています。

```bash
# キャラクタープロファイルを検索
novel find-context profile <キャラクター名>

# コンテキストウィーバーを起動
novel weave serve --port 3000

# エピソード情報をダンプ
novel dump episodes
```

## ディレクトリ構造

- `character_profile/` - キャラクタープロファイル
- `episode/` - エピソード・章
- `scene_sketch/` - シーンスケッチ・下書き
- `summary/` - 要約・あらすじ
- `official/` - 世界観・設定資料
- `writing_style/` - 文体・スタイルガイド
"#, config.name, config.description, config.project_type, config.genre, config.created);
    
    fs::write(format!("{}/README.md", config.name), readme_content)?;
    
    // writing_style/always.mdを作成
    let always_md_content = generate_always_md(config);
    fs::write(format!("{}/writing_style/always.md", config.name), always_md_content)?;
    
    Ok(())
}

fn generate_claude_md(config: &ProjectConfig) -> String {
    format!(r#"# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Project**: {}
**Type**: {}
**Genre**: {}
**Description**: {}
**Created**: {}

This is a creative writing project managed by NovelEnv v2.

## Available Commands

All commands are prefixed with `novel`:

- `novel find-context profile <name>` - Find character profile
- `novel find-context episode --character <name>` - Find episodes by character
- `novel weave serve --port 3000` - Start the context weaver UI
- `novel weave resolve <id>` - Resolve narrative context
- `novel dump episodes` - Generate episode index

## Repository Structure

```
{}/
├── character_profile/     # Character profiles
├── episode/              # Story episodes or chapters
├── scene_sketch/         # Generated scene drafts and snippets
├── summary/              # Summaries of episodes
├── official/             # World-building, rules, and concept documents
├── writing_style/        # Writing style guidelines
└── .novelenv/           # Project configuration
```

## Common Tasks

### Text Generation
When asked to "generate text" without specific save location instructions:
- Create files in the `scene_sketch/` directory.
- Use descriptive names for the `.md` files.
- **ALWAYS refer to `writing_style/always.md` for consistent writing style guidelines**.
- Follow any additional writing style guidelines found in the `writing_style/` directory.

## Important Notes

- This is a pure content repository; no build, test, or lint commands are expected to exist for the content itself.
- The focus is on narrative consistency.
- When using the Task tool to generate content, always report the exact filename created.

## Project Settings

Genre: {}
Writing Style: [See writing_style/ directory for specific guidelines]
Target Audience: [To be defined in official/ directory]
"#, config.name, config.project_type, config.genre, config.description, config.created, config.name, config.genre)
}

fn generate_always_md(config: &ProjectConfig) -> String {
    format!(r#"# 文体ガイドライン - always.md

このファイルは、すべてのテキスト生成時に適用される基本的な文体・スタイルガイドラインを定義します。

## プロジェクト基本情報

- **プロジェクト名**: {}
- **ジャンル**: {}
- **種類**: {}

## 基本的な文体設定

### 文体・語り手
- **語り手**: 三人称視点
- **時制**: 過去形
- **敬語**: 基本的に使用しない（会話内は除く）

### 文章の特徴
- **文の長さ**: 中程度（20-40文字程度）を基本とし、適度に長短を混在
- **句読点**: 読みやすさを重視し、適切に配置
- **改行**: 段落は適度に改行し、読みやすさを保つ

## ジャンル別の特徴

{}

## 文章作成時の注意点

### 必須事項
- 一貫した視点を保つ
- キャラクターの個性を文体にも反映
- シーンの雰囲気に応じた文体の調整

### 避けるべき表現
- 不自然な敬語の混在
- 視点の混乱（一人称と三人称の混在など）
- 過度に複雑な文構造

## 特別な設定

### 会話文
- 「」を使用
- キャラクターごとの特徴的な話し方を意識

### 描写
- 五感を意識した具体的な描写
- 過度に詳細すぎない、適度な情報量

## このファイルの使用方法

このファイルの内容は、以下の場面で参照されます：
- LLMによるテキスト生成時の基準として
- 既存テキストの校正・編集時の指針として
- 新しいライター・協力者への文体説明として

---

*このファイルはプロジェクトの進行に合わせて更新してください。*
"#, 
    config.name, 
    config.genre, 
    config.project_type,
    match config.genre.as_str() {
        "SF" => "- 科学的な用語を適度に使用\n- 未来的・技術的な雰囲気を演出\n- 論理的で明確な文体",
        "ファンタジー" => "- 詩的で美しい表現を意識\n- 異世界観を表現する独特な語彙\n- 神秘的で幻想的な雰囲気",
        "ミステリー" => "- 緊張感のある簡潔な文体\n- 論理的で冷静な描写\n- 読者の推理を促す情報の提示",
        "恋愛" => "- 感情豊かで繊細な表現\n- 内面描写を重視\n- 美しい情景描写",
        "ホラー" => "- 不安感を煽る表現技法\n- 五感に訴える恐怖描写\n- 短く印象的な文での緊張演出",
        "歴史" => "- 時代考証を意識した語彙選択\n- 品格のある文体\n- 時代背景に応じた敬語使用",
        "現代" => "- 自然で親しみやすい文体\n- 現代的な語彙と表現\n- リアリティのある会話",
        _ => "- プロジェクトの特色を活かした文体\n- 読者に親しみやすい表現\n- 一貫性のある文体"
    }
)
}