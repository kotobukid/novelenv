use anyhow::Result;
use crate::models::*;

mod stats;
mod dialogue;
mod sentence;
mod rhythm;

use stats::BasicStatsAnalyzer;
use dialogue::DialogueAnalyzer;
use sentence::SentenceAnalyzer;
use rhythm::RhythmAnalyzer;

pub fn analyze(text: &str) -> Result<TextProfile> {
    // 前処理：改行の正規化
    let normalized = normalize_text(text);
    
    // 各分析を実行
    let basic_stats = BasicStatsAnalyzer::analyze(&normalized)?;
    let sentences = SentenceAnalyzer::analyze(&normalized)?;
    let dialogue_stats = DialogueAnalyzer::analyze(&normalized)?;
    let rhythm_metrics = RhythmAnalyzer::analyze(&sentences)?;
    let readability = calculate_readability(&basic_stats, &rhythm_metrics);
    
    Ok(TextProfile {
        basic_stats,
        dialogue_stats,
        rhythm_metrics,
        readability,
    })
}

fn normalize_text(text: &str) -> String {
    // 改行コードの統一（CRLF -> LF）
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
}

fn calculate_readability(stats: &BasicStats, rhythm: &RhythmMetrics) -> ReadabilityScore {
    let hiragana_ratio = stats.char_types.hiragana_ratio();
    
    // シンプルな読みやすさスコア計算
    // ひらがな率が高く、文が短めで、適度な句読点があると読みやすい
    let hiragana_score = (hiragana_ratio * 100.0).min(50.0) * 2.0; // 最大50点
    let sentence_score = if rhythm.avg_sentence_length < 40.0 { 
        30.0 
    } else if rhythm.avg_sentence_length < 60.0 { 
        20.0 
    } else { 
        10.0 
    };
    
    let punctuation_density = stats.total_sentences as f32 / stats.total_chars.max(1) as f32 * 100.0;
    let punctuation_score = if punctuation_density > 2.0 && punctuation_density < 5.0 {
        20.0
    } else {
        10.0
    };
    
    let score = (hiragana_score + sentence_score + punctuation_score).min(100.0);
    
    ReadabilityScore {
        score,
        factors: ReadabilityFactors {
            hiragana_ratio,
            sentence_complexity: rhythm.avg_sentence_length,
            punctuation_density,
            paragraph_structure: 0.0, // TODO: 実装
        },
    }
}