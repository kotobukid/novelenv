---
description: キャラクターの言動を設定と照合してチェックする
---

# Character Consistency Check

Usage: `/char-lint <story_file> <character_name>`

Extract the story file (first argument) and character name (second argument) from `$ARGUMENTS`.

## Character Profile

@character_profile/[character_name].md

## Target Story

@[story_file]

## Task

Cross-reference the character's actions, dialogue, and behavior in the story against their established profile. Check
for:

1. **Personality Consistency**: Does the character act according to their established personality traits?
2. **Speech Patterns**: Are their dialogue style and vocabulary consistent with their profile?
3. **Behavioral Patterns**: Do their actions align with their known preferences, habits, and tendencies?
4. **Relationship Dynamics**: Are their interactions with other characters consistent with established relationships?
5. **Skill/Ability Usage**: Do they use abilities or knowledge consistent with their background?
6. **Character Arc**: Are any character developments logical and well-motivated?

For each finding:

- Quote the specific passage from the story
- Reference the relevant profile information
- Explain the consistency or inconsistency
- If inconsistent, suggest what would be more appropriate

**IMPORTANT: Output the analysis results in Japanese (日本語で分析結果を出力してください)**

**NOTE: If no inconsistencies are found, clearly state that the character portrayal is consistent with their established
profile.**