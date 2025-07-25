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
            description: "新しい小説プロジェクト".to_string(),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        };
        // 非対話モードでは always.md のみ、サンプルキャラクター無し
        (config, vec!["always.md".to_string()], false)
    } else {
        interactive_setup(&cli.name)?
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
        "official",
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
    
    // カスタムスラッシュコマンドをコピー
    copy_custom_commands(config)?;
    
    // 選択されたwriting_styleファイルをコピー
    copy_selected_writing_styles(config, selected_styles)?;
    
    // サンプルキャラクターをコピー
    if import_characters {
        copy_sample_characters(config)?;
    }
    
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
    
    let selections = MultiSelect::new()
        .with_prompt("含めるファイルを選択")
        .items(&items)
        .defaults(&defaults)
        .interact()?;
    
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
    let import = Confirm::new()
        .with_prompt("サンプルキャラクターの情報をインポートしますか？")
        .default(true)
        .interact()?;
    
    Ok(import)
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