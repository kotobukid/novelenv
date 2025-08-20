use anyhow::Result;
use std::path::Path;
use crate::models::TextProfile;

mod display;
mod json;
mod chart;

use self::display::TerminalReporter;
use self::json::JsonReporter;

#[derive(Clone)]
pub struct Reporter {
    format: crate::OutputFormat,
    verbose: bool,
    chart: bool,
}

impl Reporter {
    pub fn new(format: crate::OutputFormat, verbose: bool, chart: bool) -> Self {
        Self {
            format,
            verbose,
            chart,
        }
    }
    
    pub fn report(
        &self,
        profile: &TextProfile,
        comparison: Option<&TextProfile>,
        input_path: Option<&Path>,
    ) -> Result<()> {
        match self.format {
            crate::OutputFormat::Terminal => {
                let reporter = TerminalReporter::new(self.verbose, self.chart);
                reporter.report(profile, comparison, input_path)
            }
            crate::OutputFormat::Json => {
                let reporter = JsonReporter::new();
                reporter.report(profile, comparison)
            }
            crate::OutputFormat::Csv => {
                // TODO: CSV出力実装
                eprintln!("CSV format not yet implemented, falling back to JSON");
                let reporter = JsonReporter::new();
                reporter.report(profile, comparison)
            }
        }
    }
}