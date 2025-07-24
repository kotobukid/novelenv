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
    
    #[arg(long, global = true, help = "Enable debug output")]
    debug: bool,
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
        Commands::Profile { name } => handle_profile_command(name, &project_root, &config, cli.debug)?,
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
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let profile_path_str = config
        .profile
        .as_ref()
        .and_then(|p| p.aliases.get(&name))
        .cloned();

    let mut final_path = if let Some(path_str) = profile_path_str {
        project_root.join(path_str)
    } else {
        project_root
            .join("character_profile")
            .join(format!("{name}.md"))
    };
    
    if debug {
        eprintln!("[DEBUG] Initial path: {}", final_path.display());
    }
    
    // Try to canonicalize the path to resolve symlinks
    if let Ok(canonical_path) = final_path.canonicalize() {
        if debug {
            eprintln!("[DEBUG] Canonicalized path: {}", canonical_path.display());
        }
        final_path = canonical_path;
    } else if debug {
        eprintln!("[DEBUG] Could not canonicalize path (file may not exist)");
    }

    match fs::read_to_string(&final_path) {
        Ok(content) => {
            print!("{content}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: Failed to read profile for '{name}'");
            eprintln!("Path: {}", final_path.display());
            
            // Check if it's a symlink issue
            if let Ok(metadata) = fs::symlink_metadata(&final_path) {
                if metadata.file_type().is_symlink() {
                    eprintln!("Note: The file is a symbolic link");
                    match fs::read_link(&final_path) {
                        Ok(target) => eprintln!("  -> Points to: {}", target.display()),
                        Err(_) => eprintln!("  -> Unable to read symlink target"),
                    }
                }
            }
            
            // Provide specific error details
            match e.kind() {
                std::io::ErrorKind::NotFound => eprintln!("Reason: File not found"),
                std::io::ErrorKind::PermissionDenied => eprintln!("Reason: Permission denied"),
                _ => eprintln!("Reason: {}", e),
            }
            
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
