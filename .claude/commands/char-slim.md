---
description: キャラクター設定ファイルの重複・冗長記述を削減する
---

# Character Profile Slimming

Usage: `/char-slim <character_name> [--fix]`

Extract the character name and options from `$ARGUMENTS`.

- First argument: character name
- Optional flag: `--fix` to apply changes (default: analysis only)

## Character Profile

@character/[character_name].md

## Task

Analyze the character profile file to identify and reduce redundant or verbose descriptions while preserving all
essential information.

### Reduction Targets

1. **Duplicate Explanations**: Same concept explained multiple times in different sections
2. **Redundant Descriptions**: Semantically similar content using different words
3. **Excessive Hierarchy**: Simplify structures deeper than 2 levels
4. **Similar Sections**: Merge sections with overlapping purposes (e.g., 背景/経歴/生い立ち → 背景)
5. **Excessive Examples**: Reduce lists with more than 3 examples to 2 representative ones
6. **Overlapping Details**: Remove descriptions that repeat section headings

### Information to Preserve

- Core character traits and personality
- Proper nouns and numerical data
- Relationships with other characters
- Representative catchphrases and quotes
- Unique abilities or skills
- Character arc and growth trajectory

### Analysis Process

1. **Scan for Redundancies**:
    - Identify sections discussing the same topic
    - Find repeated explanations of the same concept
    - Locate overly detailed descriptions

2. **Semantic Clustering**:
    - Group related content by meaning, not just keywords
    - Identify conceptual overlaps across different sections

3. **Structure Optimization**:
    - Flatten deep hierarchies
    - Merge similar subsections
    - Remove unnecessary formatting layers

### Output

**If `--fix` is NOT specified (default)**:

- Calculate reduction percentage: `(original_length - estimated_new_length) / original_length * 100`
- List major redundancies found with line numbers
- Provide summary of proposed changes
- Show estimated character count reduction

**If `--fix` IS specified**:

- Apply all identified reductions
- Preserve file structure readability
- Maintain markdown formatting
- Report actual reduction achieved

**IMPORTANT: Output all messages in Japanese (日本語で出力してください)**

**NOTE: When applying fixes, ensure the file remains coherent and all essential character information is retained.**