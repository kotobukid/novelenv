use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(name = "pick-name")]
#[command(about = "Random name picker for novel characters")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(long, help = "Genre of the name")]
    genre: Option<Genre>,
    
    #[arg(long, help = "Gender of the name")]
    gender: Option<Gender>,
    
    #[arg(long, help = "Format of the name (given or full)", default_value = "given")]
    format: NameFormat,
    
    #[arg(long, help = "Ignore history and pick any name")]
    ignore_history: bool,
    
    #[arg(long, help = "Show usage history")]
    show_history: bool,
    
    #[arg(long, help = "Clear usage history")]
    clear_history: bool,
}

#[derive(Clone, ValueEnum)]
enum Genre {
    Fantasy,
    Japanese,
    Modern,
}

#[derive(Clone, ValueEnum)]
enum Gender {
    Male,
    Female,
}

#[derive(Clone, ValueEnum)]
enum NameFormat {
    Given,
    Full,
}

fn get_data_path() -> PathBuf {
    // 現在の実行ファイルの場所を取得
    let current_exe = env::current_exe()
        .expect("Failed to get current executable path");
    
    let exe_dir = current_exe.parent()
        .expect("Failed to get executable directory");
    
    // まずインストール済み環境のパスを試す（実行ファイルと同じディレクトリのdata/）
    let installed_data_path = exe_dir.join("data");
    if installed_data_path.exists() {
        return installed_data_path;
    }
    
    // 開発環境のパスを試す
    // 想定構造: cli-tools/pick-name/target/release/pick-name
    //          cli-tools/pick-name/data/
    if let Some(target_dir) = exe_dir.parent() { // release -> target
        if let Some(project_dir) = target_dir.parent() { // target -> pick-name
            let dev_data_path = project_dir.join("data");
            if dev_data_path.exists() {
                return dev_data_path;
            }
        }
    }
    
    eprintln!("Error: data directory not found");
    eprintln!("Looked in:");
    eprintln!("  - {}", installed_data_path.display());
    if let Some(project_dir) = exe_dir.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
        eprintln!("  - {}", project_dir.join("data").display());
    }
    exit(1);
}

fn get_given_name_file(genre: &Genre, gender: &Gender) -> String {
    match (genre, gender) {
        (Genre::Fantasy, Gender::Male) => "fantasy_male.txt".to_string(),
        (Genre::Fantasy, Gender::Female) => "fantasy_female.txt".to_string(),
        (Genre::Japanese, Gender::Male) => "japanese_male.txt".to_string(),
        (Genre::Japanese, Gender::Female) => "japanese_female.txt".to_string(),
        (Genre::Modern, Gender::Male) => "modern_western.txt".to_string(),
        (Genre::Modern, Gender::Female) => "modern_western.txt".to_string(),
    }
}

fn get_family_name_file(genre: &Genre) -> String {
    match genre {
        Genre::Fantasy => "fantasy_family.txt".to_string(),
        Genre::Japanese => "japanese_family.txt".to_string(),
        Genre::Modern => "modern_family.txt".to_string(),
    }
}

fn load_names_from_file(file_path: &PathBuf) -> Vec<String> {
    let names_content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading name file {}: {}", file_path.display(), e);
            exit(1);
        });
    
    names_content.lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn find_project_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    
    // 現在のディレクトリから上に向かって.novelenvディレクトリを探す
    let mut dir = current_dir.as_path();
    loop {
        let novelenv_dir = dir.join(".novelenv");
        if novelenv_dir.exists() && novelenv_dir.is_dir() {
            return Some(dir.to_path_buf());
        }
        
        dir = dir.parent()?;
    }
}

fn get_history_path() -> Option<PathBuf> {
    find_project_root().map(|root| {
        let novelenv_dir = root.join(".novelenv");
        // Create .novelenv directory if it doesn't exist
        if !novelenv_dir.exists() {
            let _ = fs::create_dir_all(&novelenv_dir);
        }
        novelenv_dir.join("name_history.json")
    })
}

fn get_category_key(genre: &Genre, gender: &Gender) -> String {
    let genre_str = match genre {
        Genre::Fantasy => "fantasy",
        Genre::Japanese => "japanese", 
        Genre::Modern => "modern",
    };
    let gender_str = match gender {
        Gender::Male => "male",
        Gender::Female => "female",
    };
    format!("{}_{}", genre_str, gender_str)
}

fn read_history() -> Vec<String> {
    if let Some(history_path) = get_history_path() {
        if let Ok(content) = fs::read_to_string(&history_path) {
            return content.lines().map(|s| s.to_string()).collect();
        }
    }
    Vec::new()
}

fn write_history(lines: &[String]) {
    if let Some(history_path) = get_history_path() {
        let content = lines.join("\n");
        if let Err(e) = fs::write(&history_path, content) {
            eprintln!("Warning: Failed to write history: {}", e);
        }
    }
}

fn get_used_names(category: &str) -> Vec<String> {
    let history = read_history();
    let prefix = format!("{}:", category);
    
    history.iter()
        .filter(|line| line.starts_with(&prefix))
        .filter_map(|line| line.split(':').nth(1))
        .map(|s| s.to_string())
        .collect()
}

fn add_to_history(category: &str, name: &str) {
    let mut history = read_history();
    
    // 新しいエントリを追加
    history.push(format!("{}:{}", category, name));
    
    // 50行を超えた場合は先頭から削除
    if history.len() > 50 {
        let len = history.len();
        history = history.into_iter().skip(len - 50).collect();
    }
    
    write_history(&history);
}

fn show_history_command() {
    let history = read_history();
    if history.is_empty() {
        println!("No usage history found.");
        return;
    }
    
    println!("Name usage history:");
    for line in &history {
        println!("  {}", line);
    }
    println!("\nTotal entries: {}", history.len());
}

fn clear_history_command() {
    if let Some(history_path) = get_history_path() {
        if history_path.exists() {
            if let Err(e) = fs::remove_file(&history_path) {
                eprintln!("Error clearing history: {}", e);
                exit(1);
            }
            println!("History cleared.");
        } else {
            println!("No history file found.");
        }
    } else {
        println!("Not in a novel project directory.");
    }
}

fn main() {
    let cli = Cli::parse();
    
    // Handle special commands first
    if cli.show_history {
        show_history_command();
        return;
    }
    
    if cli.clear_history {
        clear_history_command();
        return;
    }
    
    // Ensure genre and gender are provided for name generation
    let genre = cli.genre.unwrap_or_else(|| {
        eprintln!("Error: --genre is required for name generation");
        exit(1);
    });
    
    let gender = cli.gender.unwrap_or_else(|| {
        eprintln!("Error: --gender is required for name generation");
        exit(1);
    });
    
    let data_path = get_data_path();
    
    // Load given names
    let given_name_file = get_given_name_file(&genre, &gender);
    let given_name_path = data_path.join(&given_name_file);
    let given_names = load_names_from_file(&given_name_path);
    
    if given_names.is_empty() {
        eprintln!("Error: No given names found in {}", given_name_path.display());
        exit(1);
    }
    
    // For full name format, also load family names
    let family_names = if matches!(cli.format, NameFormat::Full) {
        let family_name_file = get_family_name_file(&genre);
        let family_name_path = data_path.join(&family_name_file);
        let names = load_names_from_file(&family_name_path);
        
        if names.is_empty() {
            eprintln!("Error: No family names found in {}", family_name_path.display());
            exit(1);
        }
        
        Some(names)
    } else {
        None
    };
    
    let mut rng = rand::thread_rng();
    
    // Generate the final name
    let final_name = match cli.format {
        NameFormat::Given => {
            let category = get_category_key(&genre, &gender);
            
            // Filter out used given names unless ignore_history is set
            let available_names: Vec<&String> = if cli.ignore_history {
                given_names.iter().collect()
            } else {
                let used_names = get_used_names(&category);
                given_names.iter()
                    .filter(|name| !used_names.contains(name))
                    .collect()
            };
            
            // If all names are used, fall back to all names
            let names_to_use = if available_names.is_empty() {
                eprintln!("Warning: All given names in this category have been used recently. Falling back to full list.");
                given_names.iter().collect()
            } else {
                available_names
            };
            
            let selected_name = names_to_use.choose(&mut rng)
                .expect("Failed to select random given name");
            
            // Add to history unless ignore_history is set
            if !cli.ignore_history {
                add_to_history(&category, selected_name);
            }
            
            (*selected_name).clone()
        }
        NameFormat::Full => {
            let family_names = family_names.as_ref().unwrap();
            let category = format!("{}_full", get_category_key(&genre, &gender));
            
            // For full names, we generate a combination and check against history
            let mut attempts = 0;
            let max_attempts = 100;
            
            let final_full_name = loop {
                let selected_given = given_names.choose(&mut rng)
                    .expect("Failed to select random given name");
                let selected_family = family_names.choose(&mut rng)
                    .expect("Failed to select random family name");
                
                let full_name = match genre {
                    Genre::Japanese => format!("{} {}", selected_family, selected_given),
                    _ => format!("{} {}", selected_given, selected_family),
                };
                
                // Check if this combination has been used recently
                if cli.ignore_history {
                    break full_name;
                }
                
                let used_names = get_used_names(&category);
                if !used_names.contains(&full_name) {
                    break full_name;
                }
                
                attempts += 1;
                if attempts >= max_attempts {
                    eprintln!("Warning: Unable to find unused full name combination after {} attempts. Using random combination.", max_attempts);
                    break full_name;
                }
            };
            
            // Add to history unless ignore_history is set
            if !cli.ignore_history {
                add_to_history(&category, &final_full_name);
            }
            
            final_full_name
        }
    };
    
    println!("{}", final_name);
}