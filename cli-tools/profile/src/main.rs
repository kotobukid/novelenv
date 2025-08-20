use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::io::{self, Read};
use std::fs;
use anyhow::{Result, Context};

mod models;
mod analyzer;
mod parser;
mod report;

use models::TextProfile;
use report::Reporter;

#[derive(Parser)]
#[command(name = "profile")]
#[command(version = "0.1.0")]
#[command(about = "Analyze text style and statistics for novels")]
#[command(long_about = r#"Analyze text style and statistics for novels

ANALYSIS METRICS:

Style Balance:
  - Dialogue ratio: Percentage of text within quotation marks (「」『』)
  - Narrative ratio: Remaining text (descriptions, thoughts, actions)

Sentence Rhythm:
  - Length distribution: How sentence lengths vary throughout the text
  - Tempo patterns:
    * Steady: Consistent sentence lengths (low variation)
    * Varied: Mixed short and long sentences (high variation)
    * Alternating: Regular short-long-short pattern
    * Accelerating: Sentences gradually getting shorter (building tension)
    * Decelerating: Sentences gradually getting longer (slowing pace)

Readability Factors:
  - Hiragana ratio: Higher percentage generally means easier reading
  - Average sentence length: Shorter sentences are typically easier to read
  - Character type distribution: Balance of hiragana/katakana/kanji

FILE FORMAT SUPPORT:

Markdown files (.md):
  - Automatically removes headers (# lines)
  - Normalizes paragraph breaks (double newlines → single)
  - Use --sketch for NovelEnv scene_sketch format

Plain text files (.txt):
  - Analyzed as-is without modifications

EXAMPLES:

Basic analysis:
  novel profile episode/chapter1.md

Compare with reference:
  novel profile my_story.md -- --compare bestseller.txt

NovelEnv sketch analysis:
  novel profile scene_sketch/interaction.md -- --sketch

Detailed output:
  novel profile chapter1.md -- --verbose --format json"#)]
struct Cli {
    /// Input file path (reads from stdin if not specified)
    input: Option<PathBuf>,
    
    /// Compare with another file
    #[arg(short, long, value_name = "FILE")]
    compare: Option<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "terminal")]
    format: OutputFormat,
    
    /// Character encoding for file input
    #[arg(short = 'e', long, default_value = "utf-8")]
    encoding: String,
    
    /// Generate chart visualization
    #[arg(long)]
    chart: bool,
    
    /// Verbose output (show detailed statistics)
    #[arg(short, long)]
    verbose: bool,
    
    /// Treat as NovelEnv sketch format (exclude logline and author's note)
    #[arg(long)]
    sketch: bool,
    
    /// Output raw statistics as JSON (for debugging)
    #[arg(long, hide = true)]
    debug: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    /// Human-readable terminal output with colors
    Terminal,
    /// Machine-readable JSON format
    Json,
    /// CSV format for spreadsheet import
    Csv,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Read input text
    let input_text = read_input(&cli.input, &cli.encoding)?;
    
    // Preprocess text based on file type and options
    let processed_text = preprocess_text(&input_text, &cli)?;
    
    // Analyze processed text
    let profile = analyze_text(&processed_text)?;
    
    // Handle comparison if requested
    let comparison = if let Some(compare_path) = &cli.compare {
        let compare_text = read_file(compare_path, &cli.encoding)?;
        let processed_compare_text = preprocess_text(&compare_text, &cli)?;
        Some(analyze_text(&processed_compare_text)?)
    } else {
        None
    };
    
    // Generate and display report
    let reporter = Reporter::new(cli.format.clone(), cli.verbose, cli.chart);
    reporter.report(&profile, comparison.as_ref(), cli.input.as_deref())?;
    
    Ok(())
}

fn read_input(path: &Option<PathBuf>, encoding: &str) -> Result<String> {
    match path {
        Some(p) => read_file(p, encoding),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn read_file(path: &PathBuf, encoding: &str) -> Result<String> {
    // For now, we only support UTF-8
    // Future: Add encoding_rs for other encodings
    if encoding != "utf-8" {
        eprintln!("Warning: Only UTF-8 encoding is currently supported. Ignoring encoding parameter.");
    }
    
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

fn preprocess_text(text: &str, cli: &Cli) -> Result<String> {
    let mut processed = text.to_string();
    
    // Detect file type from extension or content
    let is_markdown = if let Some(path) = &cli.input {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase() == "md")
            .unwrap_or(false)
    } else {
        // If reading from stdin, detect from content (simple heuristic)
        text.contains("# ") || text.contains("## ")
    };
    
    if is_markdown {
        processed = preprocess_markdown(&processed, cli.sketch)?;
    }
    
    Ok(processed)
}

fn preprocess_markdown(text: &str, is_sketch: bool) -> Result<String> {
    let mut lines: Vec<&str> = text.lines().collect();
    
    // Remove Markdown headers (lines starting with #)
    lines.retain(|line| !line.trim_start().starts_with('#'));
    
    // Handle NovelEnv sketch format
    if is_sketch {
        lines = remove_sketch_metadata(lines)?;
    }
    
    let processed = lines.join("\n");
    
    // Convert Markdown paragraph breaks (double newlines) to single newlines
    let processed = processed
        .split("\n\n")
        .filter(|para| !para.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    Ok(processed)
}

fn remove_sketch_metadata(lines: Vec<&str>) -> Result<Vec<&str>> {
    let mut result = Vec::new();
    let mut in_metadata = false;
    let mut skip_until_content = true;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Skip logline section (marked with **ログライン**)
        if trimmed.starts_with("**ログライン**") {
            skip_until_content = true;
            continue;
        }
        
        // Skip separator lines
        if trimmed == "---" {
            skip_until_content = false;
            continue;
        }
        
        // Skip author's note section (usually at the end)
        if trimmed.starts_with("**作者ノート**") || 
           trimmed.starts_with("**Author's Note**") {
            break;
        }
        
        // Skip empty lines at the beginning
        if skip_until_content && trimmed.is_empty() {
            continue;
        }
        
        // Start including content after separator
        if !skip_until_content || !trimmed.is_empty() {
            skip_until_content = false;
            result.push(line);
        }
    }
    
    Ok(result)
}

fn analyze_text(text: &str) -> Result<TextProfile> {
    // This will be implemented in analyzer module
    analyzer::analyze(text)
}
