use anyhow::Result;

pub struct SentenceAnalyzer;

impl SentenceAnalyzer {
    pub fn analyze(text: &str) -> Result<Vec<String>> {
        let mut sentences = Vec::new();
        let mut current = String::new();
        let mut in_dialogue = false;
        
        for ch in text.chars() {
            current.push(ch);
            
            // 会話文の開始/終了を追跡
            match ch {
                '「' | '『' => in_dialogue = true,
                '」' | '』' => in_dialogue = false,
                _ => {}
            }
            
            // 文末記号での分割（会話文内は除外）
            if !in_dialogue && (ch == '。' || ch == '！' || ch == '？') {
                let sentence = current.trim().to_string();
                if !sentence.is_empty() {
                    sentences.push(sentence);
                }
                current.clear();
            }
        }
        
        // 最後の文を追加
        let sentence = current.trim().to_string();
        if !sentence.is_empty() {
            sentences.push(sentence);
        }
        
        // 文が一つもない場合は全体を1文として扱う
        if sentences.is_empty() && !text.trim().is_empty() {
            sentences.push(text.trim().to_string());
        }
        
        Ok(sentences)
    }
}