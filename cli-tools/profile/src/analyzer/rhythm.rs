use anyhow::Result;
use crate::models::{RhythmMetrics, TempoPattern};

pub struct RhythmAnalyzer;

impl RhythmAnalyzer {
    pub fn analyze(sentences: &[String]) -> Result<RhythmMetrics> {
        if sentences.is_empty() {
            return Ok(RhythmMetrics {
                sentence_lengths: vec![],
                avg_sentence_length: 0.0,
                variation_coefficient: 0.0,
                tempo_pattern: TempoPattern::Steady,
            });
        }
        
        let sentence_lengths: Vec<usize> = sentences
            .iter()
            .map(|s| s.chars().count())
            .collect();
        
        let sum: usize = sentence_lengths.iter().sum();
        let avg_sentence_length = sum as f32 / sentence_lengths.len() as f32;
        
        // 変動係数の計算（標準偏差 / 平均）
        let variance = sentence_lengths
            .iter()
            .map(|&len| {
                let diff = len as f32 - avg_sentence_length;
                diff * diff
            })
            .sum::<f32>() / sentence_lengths.len() as f32;
        
        let std_dev = variance.sqrt();
        let variation_coefficient = if avg_sentence_length > 0.0 {
            std_dev / avg_sentence_length
        } else {
            0.0
        };
        
        let tempo_pattern = analyze_tempo_pattern(&sentence_lengths);
        
        Ok(RhythmMetrics {
            sentence_lengths,
            avg_sentence_length,
            variation_coefficient,
            tempo_pattern,
        })
    }
}

fn analyze_tempo_pattern(lengths: &[usize]) -> TempoPattern {
    if lengths.len() < 3 {
        return TempoPattern::Steady;
    }
    
    // 変化のパターンを分析
    let mut increasing = 0;
    let mut decreasing = 0;
    let mut alternating = 0;
    
    for window in lengths.windows(2) {
        if window[1] > window[0] {
            increasing += 1;
        } else if window[1] < window[0] {
            decreasing += 1;
        }
    }
    
    // 長短の交互パターンをチェック
    for window in lengths.windows(3) {
        if (window[1] > window[0] && window[2] < window[1]) ||
           (window[1] < window[0] && window[2] > window[1]) {
            alternating += 1;
        }
    }
    
    let total_transitions = lengths.len() - 1;
    
    if alternating as f32 / total_transitions.max(1) as f32 > 0.6 {
        TempoPattern::Alternating
    } else if increasing as f32 / total_transitions as f32 > 0.7 {
        TempoPattern::Accelerating
    } else if decreasing as f32 / total_transitions as f32 > 0.7 {
        TempoPattern::Decelerating
    } else if (increasing + decreasing) as f32 / total_transitions as f32 > 0.8 {
        TempoPattern::Varied
    } else {
        TempoPattern::Steady
    }
}