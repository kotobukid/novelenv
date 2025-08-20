use anyhow::Result;
use colored::*;
use comfy_table::{Table, Cell, Attribute, Color as TableColor};
use std::path::Path;
use crate::models::{TextProfile, TempoPattern};

pub struct TerminalReporter {
    verbose: bool,
    chart: bool,
}

impl TerminalReporter {
    pub fn new(verbose: bool, chart: bool) -> Self {
        Self { verbose, chart }
    }
    
    pub fn report(
        &self,
        profile: &TextProfile,
        comparison: Option<&TextProfile>,
        input_path: Option<&Path>,
    ) -> Result<()> {
        // ヘッダー
        self.print_header(input_path);
        
        if let Some(compare) = comparison {
            self.print_comparison(profile, compare);
        } else {
            self.print_single(profile);
        }
        
        Ok(())
    }
    
    fn print_header(&self, path: Option<&Path>) {
        let filename = path
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("stdin");
        
        println!("\n{} Text Profile: {}", "📊".blue(), filename.cyan());
        println!("{}", "━".repeat(50).dimmed());
    }
    
    fn print_single(&self, profile: &TextProfile) {
        // 基本統計
        println!(
            "Total: {} chars | {} sentences",
            format!("{}", profile.basic_stats.total_chars).yellow(),
            format!("{}", profile.basic_stats.total_sentences).yellow()
        );
        
        println!();
        
        // 文体バランス
        println!("{} Style Balance", "📝".blue());
        let dialogue_percent = (profile.dialogue_stats.dialogue_ratio * 100.0) as usize;
        let narrative_percent = 100 - dialogue_percent;
        
        self.print_bar("Dialogue", dialogue_percent, "█", "░");
        self.print_bar("Narrative", narrative_percent, "█", "░");
        
        println!();
        
        // 文のリズム
        if self.chart {
            self.print_rhythm_chart(profile);
        }
        
        // 読みやすさ
        println!("{} Readability", "⚡".yellow());
        let hiragana_percent = (profile.basic_stats.char_types.hiragana_ratio() * 100.0) as usize;
        self.print_bar("Hiragana", hiragana_percent, "█", "░");
        
        println!(
            "  Avg sentence: {} chars",
            format!("{:.1}", profile.rhythm_metrics.avg_sentence_length).cyan()
        );
        
        // テンポパターン
        let tempo_desc = match profile.rhythm_metrics.tempo_pattern {
            TempoPattern::Steady => "Steady",
            TempoPattern::Varied => "Varied",
            TempoPattern::Accelerating => "Accelerating",
            TempoPattern::Decelerating => "Decelerating",
            TempoPattern::Alternating => "Alternating",
        };
        println!("  Tempo: {}", tempo_desc.green());
        
        // 詳細モード
        if self.verbose {
            self.print_detailed_stats(profile);
        }
    }
    
    fn print_comparison(&self, profile: &TextProfile, compare: &TextProfile) {
        println!("{} Comparison Report", "📊".blue());
        println!();
        
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("Metric"),
            Cell::new("Your Text"),
            Cell::new("Reference"),
            Cell::new("Diff"),
        ]);
        
        // 文字数
        let chars_diff = profile.basic_stats.total_chars as i32 - compare.basic_stats.total_chars as i32;
        table.add_row(vec![
            Cell::new("Total Chars"),
            Cell::new(profile.basic_stats.total_chars),
            Cell::new(compare.basic_stats.total_chars),
            format_diff(chars_diff),
        ]);
        
        // 会話率
        let dialogue_diff = (profile.dialogue_stats.dialogue_ratio - compare.dialogue_stats.dialogue_ratio) * 100.0;
        table.add_row(vec![
            Cell::new("Dialogue %"),
            Cell::new(format!("{:.1}%", profile.dialogue_stats.dialogue_ratio * 100.0)),
            Cell::new(format!("{:.1}%", compare.dialogue_stats.dialogue_ratio * 100.0)),
            format_diff_percent(dialogue_diff),
        ]);
        
        // 平均文長
        let sentence_diff = profile.rhythm_metrics.avg_sentence_length - compare.rhythm_metrics.avg_sentence_length;
        table.add_row(vec![
            Cell::new("Avg Sentence"),
            Cell::new(format!("{:.1}", profile.rhythm_metrics.avg_sentence_length)),
            Cell::new(format!("{:.1}", compare.rhythm_metrics.avg_sentence_length)),
            format_diff_float(sentence_diff),
        ]);
        
        // ひらがな率
        let hiragana_diff = (profile.basic_stats.char_types.hiragana_ratio() - 
                            compare.basic_stats.char_types.hiragana_ratio()) * 100.0;
        table.add_row(vec![
            Cell::new("Hiragana %"),
            Cell::new(format!("{:.1}%", profile.basic_stats.char_types.hiragana_ratio() * 100.0)),
            Cell::new(format!("{:.1}%", compare.basic_stats.char_types.hiragana_ratio() * 100.0)),
            format_diff_percent(hiragana_diff),
        ]);
        
        println!("{}", table);
    }
    
    fn print_bar(&self, label: &str, percent: usize, filled: &str, empty: &str) {
        let bar_width = 20;
        let filled_count = (percent * bar_width / 100).min(bar_width);
        let empty_count = bar_width - filled_count;
        
        let bar = format!(
            "{}{}",
            filled.repeat(filled_count).green(),
            empty.repeat(empty_count).dimmed()
        );
        
        println!("  {:10} {:3}% {}", label, percent, bar);
    }
    
    fn print_rhythm_chart(&self, profile: &TextProfile) {
        println!("{} Sentence Rhythm", "📈".blue());
        
        // 文長を範囲ごとに集計
        let mut short = 0;  // 1-20
        let mut medium = 0; // 21-40
        let mut long = 0;   // 41-60
        let mut very_long = 0; // 61+
        
        for &len in &profile.rhythm_metrics.sentence_lengths {
            match len {
                1..=20 => short += 1,
                21..=40 => medium += 1,
                41..=60 => long += 1,
                _ => very_long += 1,
            }
        }
        
        self.print_histogram("Short (1-20)", short);
        self.print_histogram("Medium (21-40)", medium);
        self.print_histogram("Long (41-60)", long);
        self.print_histogram("Very Long (61+)", very_long);
        
        println!();
    }
    
    fn print_histogram(&self, label: &str, count: usize) {
        let bar = "█".repeat(count.min(20)).cyan();
        println!("  {:15} {} {}", label, bar, count);
    }
    
    fn print_detailed_stats(&self, profile: &TextProfile) {
        println!();
        println!("{}", "Detailed Statistics".bold());
        println!("{}", "─".repeat(30).dimmed());
        
        println!("Paragraphs: {}", profile.basic_stats.total_paragraphs);
        println!("Dialogue segments: {}", profile.dialogue_stats.dialogue_count);
        println!("Variation coefficient: {:.2}", profile.rhythm_metrics.variation_coefficient);
        
        // 文字種詳細
        println!();
        println!("Character Types:");
        let types = &profile.basic_stats.char_types;
        println!("  Hiragana: {}", types.hiragana);
        println!("  Katakana: {}", types.katakana);
        println!("  Kanji: {}", types.kanji);
        println!("  Alphabet: {}", types.alphabet);
        println!("  Number: {}", types.number);
        println!("  Punctuation: {}", types.punctuation);
    }
}

fn format_diff(diff: i32) -> Cell {
    let text = if diff > 0 {
        format!("+{}", diff).green().to_string()
    } else if diff < 0 {
        format!("{}", diff).red().to_string()
    } else {
        "≈".dimmed().to_string()
    };
    Cell::new(text)
}

fn format_diff_percent(diff: f32) -> Cell {
    let text = if diff > 0.5 {
        format!("+{:.1}%", diff).green().to_string()
    } else if diff < -0.5 {
        format!("{:.1}%", diff).red().to_string()
    } else {
        "≈".dimmed().to_string()
    };
    Cell::new(text)
}

fn format_diff_float(diff: f32) -> Cell {
    let text = if diff > 0.5 {
        format!("+{:.1}", diff).green().to_string()
    } else if diff < -0.5 {
        format!("{:.1}", diff).red().to_string()
    } else {
        "≈".dimmed().to_string()
    };
    Cell::new(text)
}