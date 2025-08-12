use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use glob::glob;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct Config {
    tools: Option<ToolsConfig>,
    dump_settings: Option<DumpSettings>,
}

#[derive(Deserialize, Debug)]
struct ToolsConfig {
    llm_cli: Option<LlmCliConfig>,
}

#[derive(Deserialize, Debug)]
struct LlmCliConfig {
    command: String,
    prompt_flag: String,
}

#[derive(Deserialize, Debug)]
struct DumpSettings {
    input_dir: String,
    output_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpisodeInfo {
    pub episode_path: String,
    pub characters: Vec<String>,
    pub logline: String,
    pub themes: Vec<String>,
}

fn find_project_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    for path in current_dir.ancestors() {
        if path.join("novelenv.toml").exists() || path.join("find_context.toml").exists() {
            return Some(path.to_path_buf());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_root = find_project_root().ok_or("Error: Could not find project root containing novelenv.toml or find_context.toml file.")?;
    
    // Try novelenv.toml first, then fall back to find_context.toml for backward compatibility
    let novelenv_config_path = project_root.join("novelenv.toml");
    let legacy_config_path = project_root.join("find_context.toml");
    
    let config_path = if novelenv_config_path.exists() {
        novelenv_config_path
    } else if legacy_config_path.exists() {
        legacy_config_path
    } else {
        return Err("No configuration file found (novelenv.toml or find_context.toml)".into());
    };

    let config_bytes = fs::read(&config_path)?;
    let config_str = String::from_utf8(config_bytes)?;
    let config: Config = toml::from_str(&config_str)?;

    let llm_config = config.tools.and_then(|t| t.llm_cli).ok_or("LLM CLI config not found in configuration file")?;
    let dump_settings = config.dump_settings.ok_or("Dump settings not found in configuration file")?;

    let input_pattern = project_root.join(dump_settings.input_dir).join("*.md");
    let output_path = project_root.join(&dump_settings.output_file);
    
    // Create output directory if it doesn't exist
    if let Some(output_dir) = output_path.parent() {
        fs::create_dir_all(output_dir)?;
    }

    let mut all_episode_info = Vec::new();
    let json_regex = Regex::new(r"\{[\s\S]*\}")?;

    println!("Starting episode processing...");

    for entry in glob(input_pattern.to_str().unwrap())? {
        let path = entry?;
        let episode_content = fs::read_to_string(&path)?;
        let relative_path = path.strip_prefix(&project_root)?.to_str().unwrap().to_string();

        println!("Processing: {relative_path}");

        let prompt = format!(
            "Please read the following story, extract the list of characters, a one-sentence logline, and a list of themes. Respond ONLY with a single JSON object, without any other text. The JSON schema is: {}\n\nStory:\n{}",
            "{\"episode_path\":\"<path>\",\"characters\":[\"...\"],\"logline\":\"...\",\"themes\":[\"...\"]}",
            episode_content
        );

        let mut command_parts = llm_config.command.split_whitespace();
        let executable = command_parts.next().unwrap();
        let args = command_parts.collect::<Vec<_>>();

        let child = Command::new(executable)
            .args(args)
            .arg(&llm_config.prompt_flag)
            .arg(&prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            eprintln!("LLM command failed for {}: {}", relative_path, String::from_utf8_lossy(&output.stderr));
            continue;
        }

        let llm_output = String::from_utf8(output.stdout)?;
        
        if let Some(mat) = json_regex.find(&llm_output) {
            match serde_json::from_str::<EpisodeInfo>(mat.as_str()) {
                Ok(mut info) => {
                    info.episode_path = relative_path;
                    all_episode_info.push(info);
                    println!("  -> Success.");
                }
                Err(e) => {
                    eprintln!("Failed to parse extracted JSON for {relative_path}: {e}\nLLM Output was:\n---\n{llm_output}\n---");
                }
            }
        } else {
            eprintln!("Could not find JSON in LLM output for {relative_path}\nLLM Output was:\n---\n{llm_output}\n---");
        }
    }

    let final_json = serde_json::to_string_pretty(&all_episode_info)?;
    fs::write(&output_path, final_json)?;

    println!("\nProcessing complete. Index saved to {}.", output_path.display());

    Ok(())
}