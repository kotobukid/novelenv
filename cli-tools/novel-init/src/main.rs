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
- Follow any writing style guidelines found in the `writing_style/` directory.

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