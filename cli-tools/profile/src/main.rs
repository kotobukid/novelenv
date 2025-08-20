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
#[command(long_about = None)]
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
    
    // Analyze main text
    let profile = analyze_text(&input_text)?;
    
    // Handle comparison if requested
    let comparison = if let Some(compare_path) = &cli.compare {
        let compare_text = read_file(compare_path, &cli.encoding)?;
        Some(analyze_text(&compare_text)?)
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

fn analyze_text(text: &str) -> Result<TextProfile> {
    // This will be implemented in analyzer module
    analyzer::analyze(text)
}
