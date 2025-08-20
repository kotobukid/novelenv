use anyhow::Result;
use crate::models::{DialogueStats, TextSegment, SegmentType};
use crate::parser::brackets::parse_dialogues;

pub struct DialogueAnalyzer;

impl DialogueAnalyzer {
    pub fn analyze(text: &str) -> Result<DialogueStats> {
        let segments = parse_dialogues(text)?;
        
        let dialogue_chars: usize = segments
            .iter()
            .filter(|s| s.segment_type == SegmentType::Dialogue)
            .map(|s| s.content.chars().count())
            .sum();
        
        let dialogue_count = segments
            .iter()
            .filter(|s| s.segment_type == SegmentType::Dialogue)
            .count();
        
        let total_chars = text.chars().count();
        let narrative_chars = total_chars.saturating_sub(dialogue_chars);
        
        let dialogue_ratio = if total_chars > 0 {
            dialogue_chars as f32 / total_chars as f32
        } else {
            0.0
        };
        
        Ok(DialogueStats {
            dialogue_chars,
            narrative_chars,
            dialogue_ratio,
            dialogue_count,
        })
    }
}