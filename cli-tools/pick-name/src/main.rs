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
    genre: Genre,
    
    #[arg(long, help = "Gender of the name")]
    gender: Gender,
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

fn get_name_file(genre: &Genre, gender: &Gender) -> String {
    match (genre, gender) {
        (Genre::Fantasy, Gender::Male) => "fantasy_male.txt".to_string(),
        (Genre::Fantasy, Gender::Female) => "fantasy_female.txt".to_string(),
        (Genre::Japanese, Gender::Male) => "japanese_male.txt".to_string(),
        (Genre::Japanese, Gender::Female) => "japanese_female.txt".to_string(),
        (Genre::Modern, Gender::Male) => "modern_western.txt".to_string(),
        (Genre::Modern, Gender::Female) => "modern_western.txt".to_string(),
    }
}

fn main() {
    let cli = Cli::parse();
    
    let data_path = get_data_path();
    let name_file = get_name_file(&cli.genre, &cli.gender);
    let file_path = data_path.join(&name_file);
    
    let names_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading name file {}: {}", file_path.display(), e);
            exit(1);
        });
    
    let names: Vec<&str> = names_content.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    
    if names.is_empty() {
        eprintln!("Error: No names found in {}", file_path.display());
        exit(1);
    }
    
    let mut rng = rand::thread_rng();
    let selected_name = names.choose(&mut rng)
        .expect("Failed to select random name");
    
    println!("{}", selected_name);
}