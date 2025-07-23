use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// --- Shared Data Structures ---
#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeInfo {
    pub episode_path: String,
    pub characters: Vec<String>,
    pub logline: String,
    pub themes: Vec<String>,
}

// --- CLI Definition ---
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Profile {
        name: String,
    },
    Episode {
        #[arg(short, long)]
        character: String,
    },
}

// --- Config File Structures ---
#[derive(Deserialize, Debug, Default)]
struct Config {
    profile: Option<ProfileConfig>,
}

#[derive(Deserialize, Debug)]
struct ProfileConfig {
    aliases: HashMap<String, String>,
}

// --- Helper Functions ---
fn find_project_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    for path in current_dir.ancestors() {
        if path.join(".fcrc").exists() || path.join("character_profile").exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}

fn load_config(project_root: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = project_root.join(".fcrc");
    if config_path.exists() {
        let config_bytes = fs::read(&config_path)?;
        let config_str = String::from_utf8(config_bytes)?;
        Ok(toml::from_str(&config_str)?)
    } else {
        Ok(Config::default())
    }
}

// --- Main Logic ---
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let project_root = find_project_root().ok_or("Error: Could not find project root.")?;
    let config = load_config(&project_root)?;

    match cli.command {
        Commands::Profile { name } => handle_profile_command(name, &project_root, &config)?,
        Commands::Episode { character } => {
            handle_episode_command(character, &project_root, &config)?
        }
    }

    Ok(())
}

fn handle_profile_command(
    name: String,
    project_root: &Path,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let profile_path_str = config
        .profile
        .as_ref()
        .and_then(|p| p.aliases.get(&name))
        .cloned();

    let final_path = if let Some(path_str) = profile_path_str {
        project_root.join(path_str)
    } else {
        project_root
            .join("character_profile")
            .join(format!("{name}.md"))
    };

    match fs::read_to_string(&final_path) {
        Ok(content) => {
            print!("{content}");
            Ok(())
        }
        Err(_) => {
            eprintln!("Error: Profile for '{name}' not found.");
            eprintln!("Checked alias and direct path: {}", final_path.display());
            std::process::exit(1);
        }
    }
}

fn handle_episode_command(
    character_name: String,
    project_root: &Path,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let index_path = project_root.join("episode_index.json");
    if !index_path.exists() {
        eprintln!("Error: episode_index.json not found. Please run `dump-episode-info` first.");
        std::process::exit(1);
    }

    // Resolve alias: if the provided name is an alias, get the real name. Otherwise, use the provided name.
    let resolved_name = config
        .profile
        .as_ref()
        .and_then(|p| {
            p.aliases
                .get(&character_name)
                .map(|s| s.trim_end_matches(".md").split('/').next_back().unwrap_or(s))
        })
        .unwrap_or(&character_name);

    let index_content = fs::read_to_string(index_path)?;
    let all_episodes: Vec<EpisodeInfo> = serde_json::from_str(&index_content)?;

    let filtered_episodes: Vec<_> = all_episodes
        .into_iter()
        .filter(|info| info.characters.iter().any(|c| c.contains(resolved_name)))
        .collect();

    if filtered_episodes.is_empty() {
        println!(
            "No episodes found featuring the character: {resolved_name} (resolved from: {character_name})"
        );
    } else {
        println!(
            "Showing episodes for: {resolved_name} (resolved from: {character_name})"
        );
        for info in filtered_episodes {
            println!("{}: {}", info.episode_path, info.logline);
        }
    }

    Ok(())
}
