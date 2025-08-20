use anyhow::Result;
use serde_json;
use crate::models::TextProfile;

pub struct JsonReporter;

impl JsonReporter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn report(&self, profile: &TextProfile, comparison: Option<&TextProfile>) -> Result<()> {
        if let Some(compare) = comparison {
            let output = serde_json::json!({
                "main": profile,
                "comparison": compare,
                "diff": {
                    "total_chars": profile.basic_stats.total_chars as i32 - compare.basic_stats.total_chars as i32,
                    "dialogue_ratio": profile.dialogue_stats.dialogue_ratio - compare.dialogue_stats.dialogue_ratio,
                    "avg_sentence_length": profile.rhythm_metrics.avg_sentence_length - compare.rhythm_metrics.avg_sentence_length,
                    "hiragana_ratio": profile.basic_stats.char_types.hiragana_ratio() - compare.basic_stats.char_types.hiragana_ratio(),
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        
        Ok(())
    }
}