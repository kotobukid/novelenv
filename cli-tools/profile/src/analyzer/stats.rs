use anyhow::Result;
use crate::models::{BasicStats, CharTypeDistribution};

pub struct BasicStatsAnalyzer;

impl BasicStatsAnalyzer {
    pub fn analyze(text: &str) -> Result<BasicStats> {
        let total_chars = text.chars().count();
        let total_paragraphs = count_paragraphs(text);
        let char_types = analyze_char_types(text);
        
        // 簡易的な文数カウント（。！？で分割）
        let total_sentences = text
            .chars()
            .filter(|&c| c == '。' || c == '！' || c == '？')
            .count()
            .max(1); // 最低1文
        
        Ok(BasicStats {
            total_chars,
            total_sentences,
            total_paragraphs,
            char_types,
        })
    }
}

fn count_paragraphs(text: &str) -> usize {
    text.split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .count()
        .max(1)
}

fn analyze_char_types(text: &str) -> CharTypeDistribution {
    let mut dist = CharTypeDistribution {
        hiragana: 0,
        katakana: 0,
        kanji: 0,
        alphabet: 0,
        number: 0,
        punctuation: 0,
        other: 0,
    };
    
    for ch in text.chars() {
        match ch {
            'ぁ'..='ん' => dist.hiragana += 1,
            'ァ'..='ヶ' | 'ー' => dist.katakana += 1,
            '一'..='龥' | '々' => dist.kanji += 1,
            'a'..='z' | 'A'..='Z' => dist.alphabet += 1,
            '0'..='9' | '０'..='９' => dist.number += 1,
            '。' | '、' | '！' | '？' | '「' | '」' | '『' | '』' |
            '（' | '）' | '・' | '…' | '―' | '—' => dist.punctuation += 1,
            '\n' | '\r' | '\t' | ' ' | '　' => {}, // 空白類は除外
            _ => dist.other += 1,
        }
    }
    
    dist
}