use clap::Parser;
use dialoguer::{Input, Select, MultiSelect, Confirm};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

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
    // Series configuration
    series_type: String,  // short, medium, long, epic
    total_episodes: usize,
    // Scale management
    scale_level: u8,
    enable_scale_management: bool,
}

#[derive(Debug, Clone)]
struct WritingStyleFile {
    filename: String,
    display_name: String,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // プロジェクトディレクトリが既に存在するかチェック
    if Path::new(&cli.name).exists() {
        eprintln!("❌ ディレクトリ '{}' は既に存在しています", cli.name);
        std::process::exit(1);
    }
    
    println!("🚀 NovelEnv プロジェクト '{}' を初期化しています...", cli.name);
    
    let (config, selected_styles, import_characters) = if cli.non_interactive {
        let config = ProjectConfig {
            name: cli.name.clone(),
            project_type: "novel".to_string(),
            genre: "その他".to_string(),
            series_type: "medium".to_string(),
            total_episodes: 6,
            scale_level: 3,
            enable_scale_management: false,
            description: "新しい小説プロジェクト".to_string(),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        };
        // 非対話モードでは always.md のみ、サンプルキャラクター無し
        (config, vec!["always.md".to_string()], false)
    } else {
        // 対話セットアップを試みて、端末エラーの場合は非対話モードにフォールバック
        match interactive_setup(&cli.name) {
            Ok(result) => result,
            Err(_) => {
                println!("📝 端末接続エラーのため、デフォルト設定でプロジェクトを作成します");
                let config = ProjectConfig {
                    name: cli.name.clone(),
                    project_type: "novel".to_string(),
                    genre: "その他".to_string(),
                    description: "新しい小説プロジェクト".to_string(),
                    created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                    series_type: "medium".to_string(),
                    total_episodes: 6,
                    scale_level: 3,
                    enable_scale_management: false,
                };
                (config, vec!["always.md".to_string()], false)
            }
        }
    };
    
    create_project_structure(&config, &selected_styles, import_characters)?;
    
    println!("✅ プロジェクト '{}' が正常に作成されました！", config.name);
    println!();
    println!("📁 次の手順:");
    println!("   cd {}", config.name);
    println!("   novel find-context profile --help");
    println!();
    println!("🎉 Happy writing!");
    
    Ok(())
}

fn interactive_setup(name: &str) -> Result<(ProjectConfig, Vec<String>, bool), Box<dyn std::error::Error>> {
    println!();
    
    let project_types = vec!["長編小説", "短編集", "連作シリーズ", "その他"];
    let project_type_index = Select::new()
        .with_prompt("プロジェクトの種類を選択してください")
        .items(&project_types)
        .default(0)
        .interact()?;
    
    // シリーズ構成の選択
    let series_types = vec!["短編（1話完結）", "中編（4-6話）", "長編（12話程度）", "超長編（24話以上）"];
    let series_type_index = Select::new()
        .with_prompt("シリーズ構成を選択してください")
        .items(&series_types)
        .default(1)
        .interact()?;
    
    let (series_type, total_episodes) = match series_type_index {
        0 => ("short", 1),
        1 => ("medium", 6),
        2 => ("long", 12),
        3 => ("epic", 24),
        _ => ("medium", 6),
    };
    
    let genres = vec!["日常系", "青春ドラマ", "SF", "ファンタジー", "ミステリー", "恋愛", "ホラー", "アクション", "シリアス", "その他"];
    let genre_index = Select::new()
        .with_prompt("ジャンルを選択してください")
        .items(&genres)
        .default(0)
        .interact()?;
    
    // ジャンルに基づくスケールレベルの推奨値
    let (recommended_scale, enable_scale_mgmt) = match genre_index {
        0 => (2, true),  // 日常系
        1 => (3, true),  // 青春ドラマ
        7 => (4, false), // アクション
        8 => (5, false), // シリアス
        _ => (3, false), // その他
    };
    
    // スケール管理の確認（日常系・青春＋長編の場合は強く推奨）
    let should_recommend_scale_mgmt = (genre_index <= 1) && (series_type == "long" || series_type == "epic");
    let enable_scale_management = if should_recommend_scale_mgmt {
        Confirm::new()
            .with_prompt("作品スケール管理システムを有効にしますか？（日常系・長編には強く推奨）")
            .default(true)
            .interact()?
    } else {
        Confirm::new()
            .with_prompt("作品スケール管理システムを有効にしますか？")
            .default(enable_scale_mgmt)
            .interact()?
    };
    
    let scale_level = if enable_scale_management {
        let scale_options = vec![
            "レベル1: 日常系（宿題忘れ程度）",
            "レベル2: 軽いドラマ（部活トラブル程度）",
            "レベル3: 青春ドラマ（大会失敗程度）",
            "レベル4: シリアス（退学危機・事故）",
            "レベル5: 重厚（生死・犯罪レベル）"
        ];
        let scale_index = Select::new()
            .with_prompt("作品のスケールレベルを選択してください")
            .items(&scale_options)
            .default((recommended_scale - 1) as usize)
            .interact()?;
        (scale_index + 1) as u8
    } else {
        recommended_scale
    };
    
    let description: String = Input::new()
        .with_prompt("プロジェクトの簡単な説明")
        .default("新しい小説プロジェクト".to_string())
        .interact_text()?;
    
    // writing_styleファイルの選択
    let available_styles = scan_writing_styles()?;
    let selected_styles = if !available_styles.is_empty() {
        select_writing_styles(&available_styles)?
    } else {
        println!("📝 利用可能なwriting_styleファイルが見つかりませんでした");
        vec!["always.md".to_string()]
    };
    
    // サンプルキャラクターのインポート確認
    let import_sample_characters = ask_sample_characters_import()?;
    
    let config = ProjectConfig {
        name: name.to_string(),
        project_type: project_types[project_type_index].to_string(),
        genre: genres[genre_index].to_string(),
        description,
        created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        series_type: series_type.to_string(),
        total_episodes,
        scale_level,
        enable_scale_management,
    };
    
    Ok((config, selected_styles, import_sample_characters))
}

fn create_project_structure(config: &ProjectConfig, selected_styles: &[String], import_characters: bool) -> Result<(), Box<dyn std::error::Error>> {
    // メインディレクトリを作成
    fs::create_dir(&config.name)?;
    
    // サブディレクトリを作成
    let directories = [
        "character_profile",
        "episode", 
        "scene_sketch",
        "summary",
        "environment",      // official → environment (作品世界設定)
        "notes",           // 新規追加 (概念・ギミック・技術説明)
        "writing_style",
        ".novelenv",
        ".claude",
        ".claude/commands"
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

# IDE files
.idea/
.vscode/
*.swp
*.swo
*~

# Temporary files
*.bak
*.backup
*.old
*.orig
*.rej
*.temp
*.temporary

# Logs
*.log
logs/
"#;
    fs::write(format!("{}/.gitignore", config.name), gitignore_content)?;
    
    // .wvignoreを作成（Context Weaver用）
    let wvignore_content = r#"# Git files
.git/
.gitignore

# NovelEnv internal files
.novelenv/
.weaver-narratives.json

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Claude Code files
.claude/

# Temporary files
*.tmp
*.temp
*.log

# Build artifacts
target/
node_modules/
"#;
    fs::write(format!("{}/.wvignore", config.name), wvignore_content)?;
    
    // novelenv.tomlを作成（NovelEnv CLI tools用設定）
    let novelenv_content = generate_novelenv_toml(config, import_characters);
    fs::write(format!("{}/novelenv.toml", config.name), novelenv_content)?;
    
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
- `environment/` - 世界観・環境設定
- `notes/` - 概念・ギミック・技術説明
- `writing_style/` - 文体・スタイルガイド
"#, config.name, config.description, config.project_type, config.genre, config.created);
    
    fs::write(format!("{}/README.md", config.name), readme_content)?;
    
    // writing_style/always.mdを作成
    let always_md_content = generate_always_md(config);
    fs::write(format!("{}/writing_style/always.md", config.name), always_md_content)?;
    
    // カスタムスラッシュコマンドをコピー
    copy_custom_commands(config)?;
    
    // 選択されたwriting_styleファイルをコピー
    copy_selected_writing_styles(config, selected_styles)?;
    
    // サンプルキャラクターをコピー
    if import_characters {
        copy_sample_characters(config)?;
    }
    
    // プロジェクトタイプに応じたテンプレートをコピー
    copy_project_templates(config)?;
    
    // スケール管理システムのファイルをコピー（有効な場合）
    if config.enable_scale_management {
        copy_scale_management_files(config)?;
    }
    
    Ok(())
}

fn generate_template_descriptions(project_type: &str) -> (String, String) {
    match project_type {
        "連作シリーズ" => (
            "\n- **Series Planning**: Use `summary/series_plan.md` for overall series structure, episode outlines, and foreshadowing management\n- **Episode Creation**: Use `episode/episode_template.md` as the template for individual episodes\n- **Character Development**: Use `notes/character_arc.md` to track character growth across the series".to_string(),
            "\n- `summary/series_plan.md` - Overall series structure and episode planning\n- `episode/episode_template.md` - Template for individual episodes  \n- `notes/character_arc.md` - Character development tracking\n- `environment/worldbuilding.md` - World setting and rules\n- `character_profile/character_sheet.md` - Detailed character profiles".to_string()
        ),
        "長編小説" => (
            "\n- **Plot Planning**: Use `summary/plot_outline.md` for overall story structure and three-act planning\n- **Chapter Creation**: Use `episode/chapter_template.md` as the template for individual chapters".to_string(),
            "\n- `summary/plot_outline.md` - Overall plot structure and planning\n- `episode/chapter_template.md` - Template for individual chapters\n- `environment/worldbuilding.md` - World setting and rules\n- `character_profile/character_sheet.md` - Detailed character profiles".to_string()
        ),
        "短編集" => (
            "\n- **Collection Planning**: Use `summary/story_list.md` for managing multiple stories and their themes\n- **Story Creation**: Use `episode/story_template.md` as the template for individual short stories".to_string(),
            "\n- `summary/story_list.md` - Collection planning and story management\n- `episode/story_template.md` - Template for individual short stories\n- `environment/worldbuilding.md` - World setting and rules\n- `character_profile/character_sheet.md` - Detailed character profiles".to_string()
        ),
        _ => (
            "\n- No specific templates for this project type".to_string(),
            "\n- `environment/worldbuilding.md` - World setting and rules (if added)\n- `character_profile/character_sheet.md` - Detailed character profiles (if added)".to_string()
        )
    }
}

fn generate_claude_md(config: &ProjectConfig) -> String {
    let (template_description, template_locations) = generate_template_descriptions(&config.project_type);
    
    // スケール管理システムのセクションを追加
    let scale_management_section = if config.enable_scale_management {
        format!(r#"

## 作品スケール管理システム

このプロジェクトではスケール管理システムが有効になっています。

### 設定値
- **作品スケールレベル**: {}
- **シリーズ構成**: {}話構成
- **最大騒動レベル**: {}
- **危険ゾーン**: 第{}話付近（全体の70%地点）
- **緩和話**: 第{}話（危険ゾーンの次話）

### 重要な制約
- 騒動レベルが作品スケールを超えないよう注意してください
- 第{}話では感情的な山場を作りつつ、物理的破綻は避けてください
- 法的問題、損害賠償、キャラクター退場などは避けてください

### 参考ファイル
- `writing_style/scale_management.md` - スケールレベルの定義
- `templates/series/series_plan_with_scale.md` - スケール管理付きシリーズ構成
- `templates/series/incident_scale_checker.md` - 騒動レベルチェッカー
- `templates/series/episode_generation_prompts.md` - 各話用プロンプト
"#,
            config.scale_level,
            config.total_episodes,
            config.scale_level,
            (config.total_episodes as f32 * 0.7).round() as usize,
            (config.total_episodes as f32 * 0.7).round() as usize + 1,
            (config.total_episodes as f32 * 0.7).round() as usize,
        )
    } else {
        String::new()
    };
    
    format!(r#"# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Project**: {}
**Type**: {}
**Genre**: {}
**Description**: {}
**Created**: {}

This is a creative writing project managed by NovelEnv v2.{}

**🚨 IMPORTANT: You are working inside a NovelEnv v2 project directory. All tool commands must use the unified `novel` CLI. Do NOT use direct paths to `cli-tools/` or `target/release/` - these will fail in this context.**

## Available Commands

**IMPORTANT: This is a NovelEnv v2 project. Always use the `novel` command, never direct tool paths.**

All commands must be prefixed with `novel`:

### find-context - Character and Content Search Tool

The `find-context` tool is your primary method for searching character profiles and episodes. It supports aliases and provides fast, reliable content discovery.

#### Character Profile Search

```bash
# Basic usage - find character by name
novel find-context profile <character_name>

# Examples
novel find-context profile alice          # Find alice.md in character_profile/
novel find-context profile "田中太郎"      # Japanese names work too
novel find-context profile protagonist    # Can use aliases defined in novelenv.toml
```

**Features:**
- Searches `character_profile/` directory for exact matches
- Supports aliases configured in `novelenv.toml`
- Returns full character profile content
- Case-sensitive by default

#### Episode Search

```bash
# Find episodes featuring a specific character
novel find-context episode --character <character_name>

# Examples
novel find-context episode --character alice
novel find-context episode --character "山田花子"
```

**Requirements:**
- Requires `episode_index.json` to be generated first
- Run `novel dump episodes` if search returns no results

### context-weaver - Narrative Context Management

Context Weaver helps you manage complex narrative contexts by combining multiple files into cohesive reference documents.

#### Start Web UI

```bash
# Start the web interface
novel weave serve --port <port_number>

# Examples
novel weave serve --port 3000    # Default port
novel weave serve --port 8080    # Custom port
```

**Web UI Features:**
- Visual file browser with tree view
- Drag-and-drop interface for building contexts
- Save narratives with unique IDs
- Export resolved contexts

#### Resolve Narrative Context

```bash
# Resolve a saved narrative by ID
novel weave resolve <narrative_id>

# Example
novel weave resolve 123e4567-e89b-12d3-a456-426614174000
```

**Output:**
- Concatenates all files in the narrative
- Respects include ranges (if specified)
- Outputs to stdout for piping or redirection

### dump-episode-info - Episode Index Generator

This tool analyzes all episodes using an LLM to generate searchable metadata.

```bash
# Generate episode index
novel dump episodes

# What it does:
# 1. Scans all files in episode/ directory
# 2. Uses LLM to extract character appearances and plot summaries
# 3. Creates/updates episode_index.json
```

**Important Notes:**
- Requires LLM CLI tool (configured in novelenv.toml)
- May take several minutes for large projects
- Required before using `novel find-context episode`

### pick-name - Character Name Generator

This tool generates random character names for temporary/mob characters in scene sketches, avoiding repetitive LLM name patterns and maintaining project-level history to prevent duplicates.

```bash
# Given names only (default)
novel pick-name -- --genre fantasy --gender male                    # → "セラス"
novel pick-name -- --genre fantasy --gender female                  # → "エラ"
novel pick-name -- --genre japanese --gender male                   # → "蓮司"
novel pick-name -- --genre japanese --gender female                 # → "千晴"
novel pick-name -- --genre modern --gender male                     # → "Blake"
novel pick-name -- --genre modern --gender female                   # → "Harper"

# Full names (given + family)
novel pick-name -- --genre fantasy --gender male --format full      # → "セラス ドラゴンハート"
novel pick-name -- --genre fantasy --gender female --format full    # → "エラ シルバームーン"
novel pick-name -- --genre japanese --gender male --format full     # → "高瀬 蓮司"
novel pick-name -- --genre japanese --gender female --format full   # → "蒼井 千晴"
novel pick-name -- --genre modern --gender male --format full       # → "Blake Johnson"
novel pick-name -- --genre modern --gender female --format full     # → "Harper Smith"

# History management
novel pick-name -- --show-history                     # View recent name usage
novel pick-name -- --clear-history                    # Clear usage history
novel pick-name -- --genre fantasy --gender male --ignore-history  # Ignore history
```

**Features:**
- **Flexible Format**: Generate given names only or full names (given + family)
- **Smart History**: Avoids recently used names within the same project
- **Project-Specific**: History is stored in `.pick-name-history` per project
- **Auto-Rotation**: Maintains up to 50 recent names, automatically removing older entries
- **Intelligent Combinations**: Combines names from separate lists for maximum variety
- **Fallback**: When all names are used, warns and falls back to full list

**When to Use:**
- For temporary characters in scene sketches
- Background characters needing quick names
- To avoid repetitive LLM naming patterns
- **NOT for main characters** (humans should name important characters)

**History System:**
- Names are tracked per genre/gender combination (e.g., fantasy_male, japanese_female)
- Recently used names are automatically avoided
- History is project-specific and stored in `.pick-name-history`
- Use `--ignore-history` to bypass history checking when needed

### Common Workflows

#### Finding a Character and Their Story Arc

```bash
# 1. Get character profile
novel find-context profile alice

# 2. Generate episode index (if needed)
novel dump episodes

# 3. Find all episodes with this character
novel find-context episode --character alice
```

#### Creating a Complex Context for Writing

```bash
# 1. Start the context weaver UI
novel weave serve --port 3000

# 2. In browser:
#    - Navigate to http://localhost:3000
#    - Select relevant files (character profiles, episodes, world-building)
#    - Save as a narrative

# 3. Later, resolve the narrative
novel weave resolve <narrative-id> > context.md
```

#### Updating Project Indices

```bash
# When you've added new episodes or modified existing ones
novel dump episodes

# This ensures episode searches return current results
```

#### Generating Scene Sketches with Fresh Character Names

```bash
# 1. Generate a fresh character name for your scene
novel pick-name -- --genre fantasy --gender female
# Output: "セラフィーナ"

# 2. Generate a full name for a more formal character
novel pick-name -- --genre japanese --gender male --format full
# Output: "高瀬 蓮司"

# 3. Use the names in your scene creation
# (Create scene with these characters)

# 4. Check history if needed
novel pick-name -- --show-history

# 5. Clear history if starting a new story arc
novel pick-name -- --clear-history
```

### Troubleshooting

**"Command not found" errors:**
- Ensure you're using `novel` prefix, not direct tool paths
- Check that NovelEnv is properly installed

**Episode search returns no results:**
- Run `novel dump episodes` to generate/update the index
- Check that episodes exist in the `episode/` directory

**Context weaver won't start:**
- Check if the port is already in use
- Try a different port number

**Pick-name shows "Not in a novel project directory":**
- Ensure you're running the command from within a NovelEnv project
- Check that `.novelenv/` directory exists in the project root
- Use `--ignore-history` if you need names outside a project context

**Do NOT use direct tool paths like:**
- ❌ `./cli-tools/find-context/target/release/find-context`
- ❌ `./cli-tools/context-weaver/target/release/weaver`
- ❌ `./cli-tools/pick-name/target/release/pick-name`
- ❌ Any path containing `cli-tools/` or `target/release/`

**Always use the unified `novel` command instead:**
- ✅ `novel find-context`
- ✅ `novel weave`
- ✅ `novel dump`
- ✅ `novel pick-name`

## Repository Structure

```
{}/
├── character_profile/     # Character profiles and development
├── episode/              # Story episodes and chapters
├── scene_sketch/         # Generated scene drafts and snippets
├── summary/              # Episode summaries and story outlines
├── environment/          # World-building and environmental settings
├── notes/               # Concepts, mechanisms, and technical explanations
├── writing_style/        # Writing style guidelines and templates
└── .novelenv/           # NovelEnv v2 configuration
```

## Directory Usage Guidelines

When users ask you to create or save content, guide them to the appropriate directories:

### **Character Development (`character_profile/`)**
- **Character profiles and basic information**: Main character data, backstories, relationships
- **Character development arcs**: Evolution tracking, personality changes over time
- **Character reference sheets**: Physical descriptions, mannerisms, speech patterns
- **Relationship charts**: Character connections, family trees, social networks
- **Character interviews**: In-depth personality exploration through Q&A format

**This directory is your go-to for any character-related content. LLMs should freely create and update character files here.**

### **Story Content**

#### **Episodes & Chapters (`episode/`)**
- **Main story content**: Published chapters, complete episodes
- **Story arcs**: Multi-part storylines, season structures
- **Narrative sequences**: Connected story events, plot progressions
- **Final drafts**: Polished content ready for publication

#### **Scene Sketches (`scene_sketch/`)**
- **Creative experiments**: "What if" scenarios, alternative scenes
- **Scene drafts**: Rough versions before integration into episodes
- **Dialogue practice**: Character conversation development
- **Atmospheric pieces**: Setting mood, tension building exercises
- **Inspiration captures**: Spontaneous creative moments, fragment collections

#### **Summaries (`summary/`)**
- **Episode summaries**: Concise plot recaps for LLM context
- **Story outlines**: Plot progression maps, story structure planning
- **Character arc summaries**: Development tracking across episodes
- **World state updates**: Changes in setting, politics, relationships between episodes

### **World Building & Documentation**

#### **Environment Settings (`environment/`)**
- **Physical locations**: Cities, buildings, landscapes, geographical features
- **World rules**: Physics, magic systems, technological limitations
- **Political systems**: Governments, organizations, power structures
- **Cultural elements**: Traditions, languages, social customs
- **Environmental conditions**: Climate, seasons, natural phenomena

#### **Conceptual Notes (`notes/`)**
- **Abstract concepts**: Philosophical themes, symbolic meanings
- **Technical explanations**: How things work, system mechanics
- **Design documents**: Game-like mechanics, rule systems
- **Research materials**: Real-world references, inspiration sources
- **Meta-commentary**: Author notes, development thoughts

### **Writing Style (`writing_style/`)**
- **Genre-specific guidelines**: Horror, romance, SF writing techniques
- **Tone instructions**: Formal, casual, poetic approaches
- **Technical requirements**: POV rules, tense consistency
- **Voice development**: Character-specific writing styles

**Always reference `writing_style/always.md` first, then apply genre-specific styles as needed.**

## Content Creation Guidelines

**🚨 CRITICAL: Use Existing Template Files First**

This project includes pre-configured template files based on the project type. **ALWAYS check for and use existing template files before creating new ones:**

**For {} projects:**{}

**Template File Locations:**
{}

**Before creating ANY new file, check if a relevant template already exists in the project directories. If a template file exists, USE IT and fill it out rather than creating a new file with a different name.**

**LLMs should proactively:**
- Create character profiles when new characters are introduced
- Generate scene sketches for creative exploration
- Update summaries after major story events
- Expand environment details when new locations appear
- Document new concepts and mechanics in notes

**Always suggest the most appropriate directory and explain why that location suits the content type.**

## Common Tasks

### Text Generation
When asked to "generate text" without specific save location instructions:
- Create files in the `scene_sketch/` directory.
- Use descriptive names for the `.md` files.
- **ALWAYS refer to `writing_style/always.md` for consistent writing style guidelines**.
- Follow any additional writing style guidelines found in the `writing_style/` directory.

### Character and Context Search
**IMPORTANT: When searching for context-dependent information (e.g., character profiles, configuration files), you should try the `novel find-context` tool first.**

The `novel find-context` tool is the preferred method for resolving project-specific aliases and file structures.

**Search Strategy**
- **First**: Try `novel find-context` for NovelEnv-specific searches
- **If novel command fails or returns "not found"**: Use general-purpose search tools like `Grep`, `Read`, `Glob` as fallback
- **If both approaches fail**: Report that the information was not found

**Usage Examples (always use `novel` command):**
```bash
# Character profile search - CORRECT
novel find-context profile <character_name>

# Episode search by character - CORRECT  
novel find-context episode --character <character_name>

# WRONG - Do NOT use direct paths like this:
# ❌ ./cli-tools/find-context/target/release/find-context profile <character_name>
```

## Important Notes

- This is a pure content repository; no build, test, or lint commands are expected to exist for the content itself.
- The focus is on narrative consistency.
- When using the Task tool to generate content, always report the exact filename created.
- **ALWAYS use `novel find-context` for character and context searches - never use direct file access tools**
- **CRITICAL: This is a NovelEnv v2 managed project. NEVER attempt to run tools directly from `cli-tools/` paths**
- **ONLY use the `novel` command for all tool operations within this project**

## Project Settings

Genre: {}
Writing Style: [See writing_style/ directory for specific guidelines]
Target Audience: [To be defined in official/ directory]
"#, config.name, config.project_type, config.genre, config.description, config.created, scale_management_section, config.name, config.project_type, template_description, template_locations, config.genre)
}

fn generate_novelenv_toml(config: &ProjectConfig, import_characters: bool) -> String {
    let mut profile_aliases = String::new();
    
    if import_characters {
        profile_aliases.push_str(r#""アベル" = "character_profile/アベル.md"
"ハンナ" = "character_profile/ハンナ.md"
"#);
    } else {
        profile_aliases.push_str(r#"# Example entries - update these based on your actual character files:
# "アベル" = "character_profile/アベル.md"
# "ハンナ" = "character_profile/ハンナ.md"
# "主人公" = "character_profile/protagonist.md"
"#);
    }
    
    format!(r#"# novelenv.toml - Configuration for NovelEnv CLI tools
# This file configures the NovelEnv CLI tools for this project

# Aliases for the `profile` subcommand (find-context tool)
[profile.aliases]
{}

# LLM CLI configuration for dump-episode-info
[tools.llm_cli]
command = "claude"
prompt_flag = "--prompt"

# Dump settings for episode index generation
[dump_settings]
input_dir = "episode"
output_file = ".novelenv/episode_index.json"

# Machine-generated data storage configuration
[storage]
data_dir = ".novelenv"

# Context weaver settings
[context_weaver]
narratives_file = "narratives.json"

# Name picker settings
[name_picker]
history_file = "name_history.json"

# Project metadata
[project]
name = "{}"
type = "{}"
genre = "{}"
created = "{}"
"#, profile_aliases, config.name, config.project_type, config.genre, config.created)
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

fn copy_custom_commands(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    // NovelEnvのメインディレクトリを探す
    let novelenv_commands_dir = find_novelenv_commands_dir();
    
    if let Some(source_dir) = novelenv_commands_dir {
        println!("📝 カスタムスラッシュコマンドをコピー中...");
        
        // ソースディレクトリ内の.mdファイルをすべてコピー
        if let Ok(entries) = fs::read_dir(&source_dir) {
            let mut copied_count = 0;
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.extension().map_or(false, |ext| ext == "md") {
                        if let Some(filename) = path.file_name() {
                            let dest_path = format!("{}/.claude/commands/{}", 
                                config.name, 
                                filename.to_string_lossy());
                            
                            if let Err(e) = fs::copy(&path, &dest_path) {
                                eprintln!("⚠️  コマンドファイル {} のコピーに失敗: {}", 
                                    filename.to_string_lossy(), e);
                            } else {
                                copied_count += 1;
                            }
                        }
                    }
                }
            }
            
            if copied_count > 0 {
                println!("✅ {} 個のカスタムコマンドをコピーしました", copied_count);
            } else {
                println!("📝 コピーするカスタムコマンドが見つかりませんでした");
            }
        }
    } else {
        // NovelEnvのコマンドディレクトリが見つからない場合はスキップ
        println!("📝 NovelEnvのカスタムコマンドディレクトリが見つかりません（スキップ）");
    }
    
    Ok(())
}

fn find_novelenv_root() -> Option<PathBuf> {
    // 実行ファイルから相対的に探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // 開発環境: cli-tools/novel-init/target/release から探す
            if let Some(root_dir) = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // novel-init
                .and_then(|p| p.parent()) // cli-tools
                .and_then(|p| p.parent()) // novelenv root
            {
                if root_dir.join("writing_style").exists() {
                    return Some(root_dir.to_path_buf());
                }
            }
            
            // インストール済み環境: ~/.local/bin から上位ディレクトリを探す
            let installed_root = exe_dir
                .parent() // .local
                .and_then(|p| p.parent()) // home
                .map(|p| p.join("projects").join("novel"));
            
            if let Some(root) = installed_root {
                if root.join("writing_style").exists() {
                    return Some(root);
                }
            }
        }
    }
    
    // フォールバック: 現在のディレクトリから探す
    let current_dir = env::current_dir().ok()?;
    if current_dir.join("writing_style").exists() {
        return Some(current_dir);
    }
    
    None
}

fn find_novelenv_commands_dir() -> Option<PathBuf> {
    // 実行ファイルから相対的に探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // インストール済み環境: ~/.local/bin から上位ディレクトリを探す
            let installed_commands = exe_dir
                .parent()? // .local
                .parent()? // home
                .join("projects")
                .join("novel")
                .join(".claude")
                .join("commands");
            
            if installed_commands.exists() {
                return Some(installed_commands);
            }
            
            // 開発環境: cli-tools/novel-init/target/release から探す
            if let Some(cli_tools_dir) = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // release
                .and_then(|p| p.parent()) // novel-init
                .and_then(|p| p.parent()) // cli-tools
            {
                let dev_commands = cli_tools_dir
                    .join(".claude")
                    .join("commands");
                
                if dev_commands.exists() {
                    return Some(dev_commands);
                }
            }
        }
    }
    
    // フォールバック: 現在のディレクトリから探す
    let fallback_commands = PathBuf::from(".claude/commands");
    if fallback_commands.exists() {
        return Some(fallback_commands);
    }
    
    None
}

fn scan_writing_styles() -> Result<Vec<WritingStyleFile>, Box<dyn std::error::Error>> {
    let writing_style_dir = find_novelenv_writing_style_dir();
    let mut styles = Vec::new();
    
    if let Some(dir) = writing_style_dir {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "md") {
                        if let Some(filename) = path.file_name() {
                            let filename_str = filename.to_string_lossy().to_string();
                            
                            // ファイルの最初の行（ヘッダー）を読み取る
                            let (display_name, description) = extract_header_info(&path)?;
                            
                            styles.push(WritingStyleFile {
                                filename: filename_str,
                                display_name,
                                description,
                            });
                        }
                    }
                }
            }
        }
    }
    
    // always.mdを最初に並べ替え
    styles.sort_by(|a, b| {
        if a.filename == "always.md" {
            std::cmp::Ordering::Less
        } else if b.filename == "always.md" {
            std::cmp::Ordering::Greater
        } else {
            a.display_name.cmp(&b.display_name)
        }
    });
    
    Ok(styles)
}

fn extract_header_info(path: &Path) -> Result<(String, String), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut display_name = path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let mut description = String::new();
    
    // 最初の # ヘッダーを探す
    for line in lines.iter() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            display_name = trimmed[2..].trim().to_string();
            break;
        }
    }
    
    // 説明文を探す（2-3行目あたり）
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if i > 0 && i < 5 && !trimmed.is_empty() && !trimmed.starts_with("#") {
            description = if trimmed.chars().count() > 50 {
                let truncated: String = trimmed.chars().take(47).collect();
                format!("{}...", truncated)
            } else {
                trimmed.to_string()
            };
            break;
        }
    }
    
    Ok((display_name, description))
}

fn select_writing_styles(available_styles: &[WritingStyleFile]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!();
    println!("📝 プロジェクトに含めるwriting_styleファイルを選択してください:");
    println!("   (スペースキーで選択/解除、Enterで確定)");
    
    let items: Vec<String> = available_styles.iter().map(|style| {
        format!("{} ({})", style.filename.trim_end_matches(".md"), style.display_name)
    }).collect();
    
    // always.mdは最初にあって、デフォルトで選択状態
    let mut defaults = vec![false; available_styles.len()];
    if let Some(pos) = available_styles.iter().position(|s| s.filename == "always.md") {
        defaults[pos] = true;
    }
    
    match MultiSelect::new()
        .with_prompt("含めるファイルを選択")
        .items(&items)
        .defaults(&defaults)
        .interact() 
    {
        Ok(selections) => {
            let selected_files: Vec<String> = selections.iter()
                .map(|&i| available_styles[i].filename.clone())
                .collect();
            
            // always.mdが選択されていない場合は強制的に追加
            let mut result = selected_files;
            if !result.contains(&"always.md".to_string()) {
                result.insert(0, "always.md".to_string());
                println!("📝 always.md は必須のため自動的に含まれます");
            }
            
            Ok(result)
        }
        Err(_) => {
            println!("📝 writing_style選択をスキップします");
            Ok(vec!["always.md".to_string()])
        }
    }
}

fn find_novelenv_writing_style_dir() -> Option<PathBuf> {
    // NovelEnvのwriting_styleディレクトリを探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // インストール済み環境
            let installed_writing_style = exe_dir
                .parent()? // .local
                .parent()? // home
                .join("projects")
                .join("novel")
                .join("writing_style");
            
            if installed_writing_style.exists() {
                return Some(installed_writing_style);
            }
            
            // 開発環境
            if let Some(cli_tools_dir) = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // release
                .and_then(|p| p.parent()) // novel-init
                .and_then(|p| p.parent()) // cli-tools
            {
                let dev_writing_style = cli_tools_dir
                    .join("writing_style");
                
                if dev_writing_style.exists() {
                    return Some(dev_writing_style);
                }
            }
        }
    }
    
    // フォールバック
    let fallback = PathBuf::from("writing_style");
    if fallback.exists() {
        return Some(fallback);
    }
    
    None
}

fn copy_selected_writing_styles(config: &ProjectConfig, selected_styles: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = find_novelenv_writing_style_dir();
    
    if let Some(source_dir) = source_dir {
        println!("📝 選択されたwriting_styleファイルをコピー中...");
        
        let mut copied_count = 0;
        let mut skipped_always = false;
        
        for filename in selected_styles {
            let source_path = source_dir.join(filename);
            let dest_path = format!("{}/writing_style/{}", config.name, filename);
            
            // always.mdは既に generate_always_md で作成済みなのでスキップ
            if filename == "always.md" {
                skipped_always = true;
                continue;
            }
            
            if source_path.exists() {
                if let Err(e) = fs::copy(&source_path, &dest_path) {
                    eprintln!("⚠️  writing_styleファイル {} のコピーに失敗: {}", filename, e);
                } else {
                    copied_count += 1;
                }
            } else {
                eprintln!("⚠️  writing_styleファイル {} が見つかりません", filename);
            }
        }
        
        if copied_count > 0 {
            println!("✅ {} 個のwriting_styleファイルをコピーしました", copied_count);
        }
        
        if skipped_always {
            println!("📝 always.md は自動生成されました");
        }
        
    } else {
        println!("📝 NovelEnvのwriting_styleディレクトリが見つかりません（スキップ）");
    }
    
    Ok(())
}

fn ask_sample_characters_import() -> Result<bool, Box<dyn std::error::Error>> {
    // 端末接続エラーの場合はデフォルト値を使用
    match Confirm::new()
        .with_prompt("サンプルキャラクターの情報をインポートしますか？")
        .default(true)
        .interact() 
    {
        Ok(import) => Ok(import),
        Err(_) => {
            println!("📝 非対話環境のため、サンプルキャラクターのインポートはスキップします");
            Ok(false)
        }
    }
}

fn copy_sample_characters(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    // NovelEnvのcharacter_profileディレクトリを探す
    let novelenv_character_dir = find_novelenv_character_profile_dir();
    
    if let Some(source_dir) = novelenv_character_dir {
        println!("👥 サンプルキャラクターをコピー中...");
        
        let sample_characters = ["アベル.md", "ハンナ.md"];
        let mut copied_count = 0;
        
        for character_file in &sample_characters {
            let source_path = source_dir.join(character_file);
            let dest_path = format!("{}/character_profile/{}", config.name, character_file);
            
            if source_path.exists() {
                if let Err(e) = fs::copy(&source_path, &dest_path) {
                    eprintln!("⚠️  キャラクターファイル {} のコピーに失敗: {}", character_file, e);
                } else {
                    copied_count += 1;
                }
            } else {
                eprintln!("⚠️  キャラクターファイル {} が見つかりません", character_file);
            }
        }
        
        if copied_count > 0 {
            println!("✅ {} 人のサンプルキャラクターをコピーしました", copied_count);
        } else {
            println!("📝 コピーするサンプルキャラクターが見つかりませんでした");
        }
    } else {
        // NovelEnvのキャラクターディレクトリが見つからない場合はスキップ
        println!("📝 NovelEnvのキャラクターディレクトリが見つかりません（スキップ）");
    }
    
    Ok(())
}

fn find_novelenv_character_profile_dir() -> Option<PathBuf> {
    // NovelEnvのcharacter_profileディレクトリを探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // インストール済み環境
            let installed_character_profile = exe_dir
                .parent()? // .local
                .parent()? // home
                .join("projects")
                .join("novel")
                .join("character_profile");
            
            if installed_character_profile.exists() {
                return Some(installed_character_profile);
            }
            
            // 開発環境
            if let Some(cli_tools_dir) = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // release
                .and_then(|p| p.parent()) // novel-init
                .and_then(|p| p.parent()) // cli-tools
            {
                let dev_character_profile = cli_tools_dir
                    .join("character_profile");
                
                if dev_character_profile.exists() {
                    return Some(dev_character_profile);
                }
            }
        }
    }
    
    // フォールバック
    let fallback = PathBuf::from("character_profile");
    if fallback.exists() {
        return Some(fallback);
    }
    
    None
}

fn find_novelenv_templates_dir() -> Option<PathBuf> {
    // NovelEnvのtemplatesディレクトリを探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // インストール済み環境
            let installed_templates = exe_dir
                .parent()? // .local
                .parent()? // home
                .join("projects")
                .join("novelenv")
                .join("templates");
            
            if installed_templates.exists() {
                return Some(installed_templates);
            }
            
            // 開発環境  
            if let Some(novel_init_dir) = exe_dir
                .parent() // target
                .and_then(|p| p.parent()) // novel-init
            {
                if let Some(cli_tools_dir) = novel_init_dir.parent() {
                    if let Some(novelenv_root) = cli_tools_dir.parent() {
                        let dev_templates = novelenv_root
                            .join("templates");
                
                        if dev_templates.exists() {
                            return Some(dev_templates);
                        }
                    }
                }
            }
        }
    }
    
    // フォールバック: プロジェクトルートのtemplatesディレクトリ
    let fallback = PathBuf::from("templates");
    if fallback.exists() {
        return Some(fallback);
    }
    
    None
}

fn copy_scale_management_files(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 スケール管理システムのファイルをコピー中...");
    
    // templatesディレクトリを作成
    fs::create_dir_all(format!("{}/templates/series", config.name))?;
    
    // コピーするファイルのリスト
    let scale_files = vec![
        ("writing_style/scale_management.md", "writing_style/scale_management.md"),
        ("templates/series/series_plan_with_scale.md", "templates/series/series_plan_with_scale.md"),
        ("templates/series/incident_scale_checker.md", "templates/series/incident_scale_checker.md"),
        ("templates/series/episode_generation_prompts.md", "templates/series/episode_generation_prompts.md"),
    ];
    
    let source_dir = find_novelenv_root();
    
    if let Some(source_dir) = source_dir {
        for (src_path, dest_path) in scale_files {
            let source_file = source_dir.join(src_path);
            let dest_file = format!("{}/{}", config.name, dest_path);
            
            if source_file.exists() {
                // ディレクトリが存在しない場合は作成
                if let Some(parent) = PathBuf::from(&dest_file).parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // ファイルをコピーして変数を置換
                let content = fs::read_to_string(&source_file)?;
                let customized_content = customize_scale_template(&content, config);
                fs::write(&dest_file, customized_content)?;
                
                println!("  ✓ {} をコピーしました", dest_path);
            }
        }
        
        println!("✅ スケール管理システムのファイルをコピーしました");
    } else {
        println!("📝 スケール管理ファイルが見つかりません（スキップ）");
    }
    
    Ok(())
}

fn customize_scale_template(content: &str, config: &ProjectConfig) -> String {
    let danger_zone = (config.total_episodes as f32 * 0.7).round() as usize;
    let recovery = danger_zone + 1;
    
    content
        .replace("{{SERIES_TITLE}}", &config.name)
        .replace("{{TOTAL_EPISODES}}", &config.total_episodes.to_string())
        .replace("{{GENRE}}", &config.genre)
        .replace("{{SCALE_LEVEL}}", &config.scale_level.to_string())
        .replace("{{MAX_INCIDENT_LEVEL}}", &config.scale_level.to_string())
        .replace("{{DANGER_ZONE_EPISODE}}", &danger_zone.to_string())
        .replace("{{RECOVERY_EPISODE}}", &recovery.to_string())
        .replace("{{DATE}}", &config.created)
}

fn copy_project_templates(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let templates_dir = find_novelenv_templates_dir();
    
    if let Some(templates_dir) = templates_dir {
        println!("📄 プロジェクトタイプに応じたテンプレートをコピー中...");
        
        // プロジェクトタイプに応じたディレクトリを決定
        let type_dir = match config.project_type.as_str() {
            "長編小説" => "novel",
            "短編集" => "anthology",
            "連作シリーズ" => "series",
            _ => return Ok(()), // その他の場合はテンプレートをコピーしない
        };
        
        let type_templates_dir = templates_dir.join(type_dir);
        let shared_templates_dir = templates_dir.join("shared");
        
        let mut copied_count = 0;
        
        // プロジェクトタイプ固有のテンプレートをコピー
        if type_templates_dir.exists() {
            if let Ok(entries) = fs::read_dir(&type_templates_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        
                        if path.extension().map_or(false, |ext| ext == "md") {
                            if let Some(filename) = path.file_name() {
                                let dest_dir = determine_template_destination(&filename.to_string_lossy());
                                let dest_path = format!("{}/{}/{}", 
                                    config.name, 
                                    dest_dir,
                                    filename.to_string_lossy());
                                
                                // テンプレートの内容を読み込んで変数を置換
                                if let Ok(content) = fs::read_to_string(&path) {
                                    let processed_content = process_template_content(&content, config);
                                    
                                    if let Err(e) = fs::write(&dest_path, processed_content) {
                                        eprintln!("⚠️  テンプレートファイル {} のコピーに失敗: {}", 
                                            filename.to_string_lossy(), e);
                                    } else {
                                        copied_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // 共有テンプレートをコピー
        if shared_templates_dir.exists() {
            if let Ok(entries) = fs::read_dir(&shared_templates_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        
                        if path.extension().map_or(false, |ext| ext == "md") {
                            if let Some(filename) = path.file_name() {
                                let dest_dir = determine_template_destination(&filename.to_string_lossy());
                                let dest_path = format!("{}/{}/{}", 
                                    config.name, 
                                    dest_dir,
                                    filename.to_string_lossy());
                                
                                // テンプレートの内容を読み込んで変数を置換
                                if let Ok(content) = fs::read_to_string(&path) {
                                    let processed_content = process_template_content(&content, config);
                                    
                                    if let Err(e) = fs::write(&dest_path, processed_content) {
                                        eprintln!("⚠️  共有テンプレートファイル {} のコピーに失敗: {}", 
                                            filename.to_string_lossy(), e);
                                    } else {
                                        copied_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if copied_count > 0 {
            println!("✅ {} 個のテンプレートファイルをプロジェクトに追加しました", copied_count);
        }
    } else {
        println!("📄 NovelEnvのテンプレートディレクトリが見つかりません（スキップ）");
    }
    
    Ok(())
}

fn determine_template_destination(filename: &str) -> &'static str {
    match filename {
        // summaryディレクトリに配置
        "series_plan.md" | "plot_outline.md" | "story_list.md" => "summary",
        // episodeディレクトリに配置
        "episode_template.md" | "chapter_template.md" | "story_template.md" => "episode",
        // notesディレクトリに配置
        "character_arc.md" => "notes",
        // environmentディレクトリに配置
        "worldbuilding.md" => "environment",
        // character_profileディレクトリに配置
        "character_sheet.md" => "character_profile",
        // デフォルトはnotesディレクトリ
        _ => "notes",
    }
}

fn process_template_content(content: &str, config: &ProjectConfig) -> String {
    content
        .replace("{{PROJECT_NAME}}", &config.name)
        .replace("{{GENRE}}", &config.genre)
        .replace("{{DATE}}", &config.created)
        .replace("{{EPISODE_NUMBER}}", "1")
        .replace("{{CHAPTER_NUMBER}}", "1")
}