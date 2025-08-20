use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

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
        if path.join("novelenv.toml").exists() || path.join("find_context.toml").exists() || path.join("character").exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}

fn load_config(project_root: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    // Try novelenv.toml first, then fall back to find_context.toml for backward compatibility
    let novelenv_config_path = project_root.join("novelenv.toml");
    let legacy_config_path = project_root.join("find_context.toml");
    
    let config_path = if novelenv_config_path.exists() {
        novelenv_config_path
    } else if legacy_config_path.exists() {
        legacy_config_path
    } else {
        return Ok(Config::default());
    };
    
    let config_bytes = fs::read(&config_path)?;
    let config_str = String::from_utf8(config_bytes)?;
    Ok(toml::from_str(&config_str)?)
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
    // Check if the name is an alias first
    let profile_path_str = config
        .profile
        .as_ref()
        .and_then(|p| p.aliases.get(&name))
        .cloned();

    // If it's an alias, use the alias path directly
    if let Some(path_str) = profile_path_str {
        let final_path = project_root.join(path_str);
        if debug {
            eprintln!("[DEBUG] Using alias path: {}", final_path.display());
        }
        return try_read_profile(&final_path, &name, debug);
    }

    // Otherwise, try to find the profile with subdirectory support
    let profile_dir = project_root.join("character");
    let paths_to_try = generate_profile_paths(&profile_dir, &name);
    
    if debug {
        eprintln!("[DEBUG] Searching for profile: {}", name);
        eprintln!("[DEBUG] Paths to try:");
        for path in &paths_to_try {
            eprintln!("[DEBUG]   - {}", path.display());
        }
    }

    // Try each path in order
    for path in &paths_to_try {
        if path.exists() {
            if debug {
                eprintln!("[DEBUG] Found at: {}", path.display());
            }
            return try_read_profile(path, &name, debug);
        }
    }

    // If not found, check for suggestions
    if let Some(suggestions) = find_similar_profiles(&profile_dir, &name) {
        if suggestions.len() == 1 {
            // If there's exactly one suggestion, use it directly
            let suggestion = &suggestions[0];
            let suggested_path = profile_dir.join(format!("{}.md", suggestion));
            
            if debug {
                eprintln!("[DEBUG] Auto-selecting single suggestion: {}", suggestion);
                eprintln!("[DEBUG] Suggested path: {}", suggested_path.display());
            }
            
            // Show a brief message about the auto-selection
            eprintln!("Profile '{}' not found. Using '{}':", name, suggestion);
            return try_read_profile(&suggested_path, suggestion, debug);
        } else if !suggestions.is_empty() {
            // Multiple suggestions - show them as before
            eprintln!("Error: Failed to read profile for '{name}'");
            eprintln!("Tried the following paths:");
            for path in &paths_to_try {
                eprintln!("  - {}", path.display());
            }
            eprintln!("\nDid you mean one of these?");
            for suggestion in suggestions {
                eprintln!("  - {}", suggestion);
            }
            std::process::exit(1);
        }
    }
    
    // No suggestions found - show error
    eprintln!("Error: Failed to read profile for '{name}'");
    eprintln!("Tried the following paths:");
    for path in &paths_to_try {
        eprintln!("  - {}", path.display());
    }
    
    std::process::exit(1);
}

// Helper function to generate possible profile paths based on input
fn generate_profile_paths(profile_dir: &Path, name: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    // Check if the name contains a path separator
    if name.contains('/') {
        // Split the path and construct the full path
        let parts: Vec<&str> = name.split('/').collect();
        let mut path = profile_dir.to_path_buf();
        
        // Build the subdirectory path
        for (i, part) in parts.iter().enumerate() {
            if i < parts.len() - 1 {
                path.push(part);
            } else {
                // Last part is the filename
                path.push(format!("{}.md", part));
            }
        }
        paths.push(path);
        
        // Also try without subdirectory as fallback
        if let Some(base_name) = parts.last() {
            paths.push(profile_dir.join(format!("{}.md", base_name)));
        }
    } else {
        // No subdirectory, just simple name
        paths.push(profile_dir.join(format!("{}.md", name)));
    }
    
    paths
}

// Helper function to read and print profile content
fn try_read_profile(path: &Path, name: &str, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut final_path = path.to_path_buf();
    
    // Try to canonicalize the path to resolve symlinks
    if let Ok(canonical_path) = final_path.canonicalize() {
        if debug {
            eprintln!("[DEBUG] Canonicalized path: {}", canonical_path.display());
        }
        final_path = canonical_path;
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
                io::ErrorKind::NotFound => eprintln!("Reason: File not found"),
                io::ErrorKind::PermissionDenied => eprintln!("Reason: Permission denied"),
                _ => eprintln!("Reason: {}", e),
            }
            
            std::process::exit(1);
        }
    }
}

// Helper function to find similar profile names with improved partial matching
fn find_similar_profiles(profile_dir: &Path, target: &str) -> Option<Vec<String>> {
    let target_lower = target.to_lowercase();
    
    // Extract base name if target contains path separator
    let base_name = if target.contains('/') {
        target.split('/').last().unwrap_or(target)
    } else {
        target
    };
    let base_name_lower = base_name.to_lowercase();
    
    #[derive(Debug, Clone)]
    struct MatchCandidate {
        full_name: String,
        match_score: u8, // Higher score = better match
    }
    
    let mut candidates = Vec::new();
    
    // Recursively search for profiles with scoring
    fn search_dir(dir: &Path, base_name_lower: &str, target_lower: &str, candidates: &mut Vec<MatchCandidate>, prefix: &str) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    // Recursively search subdirectories
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    let new_prefix = if prefix.is_empty() {
                        dir_name
                    } else {
                        format!("{}/{}", prefix, dir_name)
                    };
                    search_dir(&path, base_name_lower, target_lower, candidates, &new_prefix);
                } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let stem_lower = stem.to_lowercase();
                        let full_name = if prefix.is_empty() {
                            stem.to_string()
                        } else {
                            format!("{}/{}", prefix, stem)
                        };
                        let full_name_lower = full_name.to_lowercase();
                        
                        // Calculate match score based on different criteria
                        let mut score = 0u8;
                        
                        // Exact base name match (highest priority)
                        if stem_lower == base_name_lower {
                            score = 100;
                        }
                        // Exact full path match
                        else if full_name_lower == target_lower {
                            score = 95;
                        }
                        // Base name starts with target
                        else if stem_lower.starts_with(&base_name_lower) {
                            score = 80;
                        }
                        // Base name ends with target  
                        else if stem_lower.ends_with(&base_name_lower) {
                            score = 75;
                        }
                        // Target starts with base name
                        else if base_name_lower.starts_with(&stem_lower) {
                            score = 70;
                        }
                        // Base name contains target
                        else if stem_lower.contains(&base_name_lower) {
                            score = 60;
                        }
                        // Target contains base name
                        else if base_name_lower.contains(&stem_lower) {
                            score = 55;
                        }
                        // Full name contains target
                        else if full_name_lower.contains(&target_lower) {
                            score = 40;
                        }
                        // Target contains full name
                        else if target_lower.contains(&full_name_lower) {
                            score = 35;
                        }
                        // Character distance match (for typos like 太朗 → 太郎)
                        else if character_distance(&stem_lower, &base_name_lower) <= 1 {
                            score = 25;
                        }
                        // Fuzzy character match (at least 50% common characters)
                        else if has_significant_overlap(&stem_lower, &base_name_lower, 0.5) {
                            score = 20;
                        }
                        
                        // Add to candidates if any match found
                        if score > 0 {
                            candidates.push(MatchCandidate {
                                full_name,
                                match_score: score,
                            });
                        }
                    }
                }
            }
        }
    }
    
    search_dir(profile_dir, &base_name_lower, &target_lower, &mut candidates, "");
    
    // Sort by match score (descending) and then alphabetically
    candidates.sort_by(|a, b| {
        b.match_score.cmp(&a.match_score)
            .then(a.full_name.cmp(&b.full_name))
    });
    
    // Take top 5 suggestions
    let suggestions: Vec<String> = candidates.into_iter()
        .take(5)
        .map(|c| c.full_name)
        .collect();
    
    if suggestions.is_empty() {
        None
    } else {
        Some(suggestions)
    }
}

// Helper function to calculate character-level edit distance (Levenshtein distance)
fn character_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    
    // Initialize first row and column
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }
    
    // Fill the dp table
    for i in 1..=len1 {
        for j in 1..=len2 {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                    dp[i - 1][j - 1]
                );
            }
        }
    }
    
    dp[len1][len2]
}

// Helper function to check if two strings have significant character overlap
fn has_significant_overlap(s1: &str, s2: &str, threshold: f32) -> bool {
    if s1.is_empty() || s2.is_empty() {
        return false;
    }
    
    let chars1: HashSet<char> = s1.chars().collect();
    let chars2: HashSet<char> = s2.chars().collect();
    
    let intersection = chars1.intersection(&chars2).count();
    let union = chars1.union(&chars2).count();
    
    if union == 0 {
        return false;
    }
    
    (intersection as f32 / union as f32) >= threshold
}

fn handle_episode_command(
    character_name: String,
    project_root: &Path,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Try .novelenv/episode_index.json first, then fall back to legacy location
    let novelenv_index_path = project_root.join(".novelenv").join("episode_index.json");
    let legacy_index_path = project_root.join("episode_index.json");
    
    let index_path = if novelenv_index_path.exists() {
        novelenv_index_path
    } else if legacy_index_path.exists() {
        legacy_index_path
    } else {
        eprintln!("Error: episode_index.json not found in .novelenv/ or project root.");
        eprintln!("Please run `dump-episode-info` first.");
        std::process::exit(1);
    };

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
