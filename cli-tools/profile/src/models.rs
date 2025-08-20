use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextProfile {
    pub basic_stats: BasicStats,
    pub dialogue_stats: DialogueStats,
    pub rhythm_metrics: RhythmMetrics,
    pub readability: ReadabilityScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicStats {
    pub total_chars: usize,
    pub total_sentences: usize,
    pub total_paragraphs: usize,
    pub char_types: CharTypeDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharTypeDistribution {
    pub hiragana: usize,
    pub katakana: usize,
    pub kanji: usize,
    pub alphabet: usize,
    pub number: usize,
    pub punctuation: usize,
    pub other: usize,
}

impl CharTypeDistribution {
    pub fn hiragana_ratio(&self) -> f32 {
        let total = self.total();
        if total == 0 { 0.0 } else { self.hiragana as f32 / total as f32 }
    }
    
    pub fn total(&self) -> usize {
        self.hiragana + self.katakana + self.kanji + 
        self.alphabet + self.number + self.punctuation + self.other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueStats {
    pub dialogue_chars: usize,
    pub narrative_chars: usize,
    pub dialogue_ratio: f32,
    pub dialogue_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmMetrics {
    pub sentence_lengths: Vec<usize>,
    pub avg_sentence_length: f32,
    pub variation_coefficient: f32,
    pub tempo_pattern: TempoPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TempoPattern {
    Steady,      // 一定のリズム
    Varied,      // 変化に富む
    Accelerating, // 徐々に短く
    Decelerating, // 徐々に長く
    Alternating,  // 長短交互
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityScore {
    pub score: f32,  // 0-100
    pub factors: ReadabilityFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadabilityFactors {
    pub hiragana_ratio: f32,
    pub sentence_complexity: f32,
    pub punctuation_density: f32,
    pub paragraph_structure: f32,
}

#[derive(Debug, Clone)]
pub struct TextSegment {
    pub start: usize,
    pub end: usize,
    pub content: String,
    pub segment_type: SegmentType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentType {
    Dialogue,
    Narrative,
    Unknown,
}