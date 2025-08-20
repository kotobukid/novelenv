use anyhow::Result;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::anychar,
    combinator::{map, recognize},
    multi::many0,
    sequence::delimited,
};
use crate::models::{TextSegment, SegmentType};

pub fn parse_dialogues(text: &str) -> Result<Vec<TextSegment>> {
    let mut segments = Vec::new();
    let mut current_pos = 0;
    let chars: Vec<char> = text.chars().collect();
    
    while current_pos < chars.len() {
        // 会話文の開始を探す
        if let Some(dialogue_start) = find_dialogue_start(&chars[current_pos..]) {
            // 会話文開始前の地の文を追加
            if dialogue_start > 0 {
                let narrative: String = chars[current_pos..current_pos + dialogue_start]
                    .iter()
                    .collect();
                segments.push(TextSegment {
                    start: current_pos,
                    end: current_pos + dialogue_start,
                    content: narrative,
                    segment_type: SegmentType::Narrative,
                });
            }
            
            current_pos += dialogue_start;
            
            // 会話文を抽出
            if let Some(dialogue_end) = find_dialogue_end(&chars[current_pos..]) {
                let dialogue: String = chars[current_pos..current_pos + dialogue_end]
                    .iter()
                    .collect();
                segments.push(TextSegment {
                    start: current_pos,
                    end: current_pos + dialogue_end,
                    content: dialogue,
                    segment_type: SegmentType::Dialogue,
                });
                current_pos += dialogue_end;
            } else {
                // 閉じ括弧が見つからない場合は1文字進める
                current_pos += 1;
            }
        } else {
            // 残りは全て地の文
            if current_pos < chars.len() {
                let narrative: String = chars[current_pos..].iter().collect();
                segments.push(TextSegment {
                    start: current_pos,
                    end: chars.len(),
                    content: narrative,
                    segment_type: SegmentType::Narrative,
                });
            }
            break;
        }
    }
    
    Ok(segments)
}

fn find_dialogue_start(chars: &[char]) -> Option<usize> {
    for (i, &ch) in chars.iter().enumerate() {
        if ch == '「' || ch == '『' {
            return Some(i);
        }
    }
    None
}

fn find_dialogue_end(chars: &[char]) -> Option<usize> {
    if chars.is_empty() {
        return None;
    }
    
    let start_bracket = chars[0];
    let end_bracket = match start_bracket {
        '「' => '」',
        '『' => '』',
        _ => return None,
    };
    
    let mut depth = 0;
    for (i, &ch) in chars.iter().enumerate() {
        if ch == start_bracket {
            depth += 1;
        } else if ch == end_bracket {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
    }
    
    None
}

// nom版のパーサー（将来の拡張用）
pub fn dialogue_nom(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(tag("「"), take_until("」"), tag("」")),
        delimited(tag("『"), take_until("』"), tag("』")),
    ))(input)
}

// 入れ子対応版（Phase 2で使用予定）
pub fn dialogue_nested_nom(input: &str) -> IResult<&str, String> {
    fn parse_content(input: &str) -> IResult<&str, String> {
        let mut result = String::new();
        let mut remaining = input;
        
        while !remaining.is_empty() && !remaining.starts_with('」') && !remaining.starts_with('』') {
            if remaining.starts_with('『') {
                // 内側の括弧を処理
                let (rest, inner) = delimited(
                    tag("『"),
                    take_until("』"),
                    tag("』")
                )(remaining)?;
                result.push_str("『");
                result.push_str(inner);
                result.push_str("』");
                remaining = rest;
            } else {
                // 通常の文字
                let (rest, ch) = anychar(remaining)?;
                result.push(ch);
                remaining = rest;
            }
        }
        
        Ok((remaining, result))
    }
    
    map(
        delimited(tag("「"), parse_content, tag("」")),
        |content| format!("「{}」", content)
    )(input)
}