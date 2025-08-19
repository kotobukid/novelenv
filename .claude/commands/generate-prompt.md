# Prompt Generation Command

## Command

`/generate-prompt`

## Description

Generates tailored prompts for episode creation based on your project's scale settings, episode position, and specific
requirements.

## Usage

Type `/generate-prompt` with parameters to generate context-aware writing prompts.

### Basic Format

```
/generate-prompt episode:X [type] [options]
```

### Examples

```
/generate-prompt episode:9                    # Generate prompt for episode 9
/generate-prompt episode:5 tone:dramatic     # Episode 5 with dramatic tone
/generate-prompt episode:1 character:alice   # Episode 1 focusing on alice
/generate-prompt danger-zone                 # Special danger zone prompt
/generate-prompt recovery                    # Recovery episode prompt
```

## Prompt Types

### Standard Episode Prompt

For regular episodes within your series structure:

```
/generate-prompt episode:X
```

Generated prompt includes:

- Project scale constraints
- Episode position context
- Character availability
- Tone guidelines
- Scale-appropriate incident suggestions

### Danger Zone Prompt (Special)

For your project's critical episode (typically 70% point):

```
/generate-prompt danger-zone
/generate-prompt episode:9  # (if episode 9 is your danger zone)
```

Includes special constraints:

- Maximum scale enforcement
- Emotional peak guidance
- Recovery preparation requirements
- Explicit violation warnings

### Recovery Episode Prompt

For the episode following your danger zone:

```
/generate-prompt recovery
```

Focuses on:

- Tone normalization
- Relationship healing
- Scale de-escalation
- Reader comfort restoration

### Character-Focused Prompt

```
/generate-prompt episode:X character:name
```

Emphasizes:

- Character development arc
- Relationship dynamics
- Character-appropriate incidents
- Voice consistency

## Generated Prompt Components

### 1. Project Context Section

```
【作品設定】
- ジャンル: [your genre]
- 作品スケールレベル: [X]
- シリーズ構成: [X]話構成
- 現在のエピソード: 第[N]話
```

### 2. Scale Constraints

```
【重要な制約】
- 騒動レベル上限: [X]
- 解決方法: [scale-appropriate methods]
- 影響範囲: [scope limits]
```

### 3. Tone Guidelines

```
【トーン設定】
- 基本トーン: [project baseline]
- このエピソードの重点: [specific to episode]
- 避けるべき要素: [scale violations]
```

### 4. Character Context

```
【キャラクター配置】
- 主要登場人物: [relevant characters]
- 関係性の現状: [current dynamics]
- 成長ポイント: [development opportunities]
```

### 5. Structural Guidelines

```
【構成指示】
- 前話との接続: [continuity notes]
- 次話への布石: [setup requirements]
- 伏線要素: [foreshadowing needs]
```

### 6. Scale-Specific Warnings

```
【絶対NGリスト】
- [Scale-inappropriate keywords]
- [Violation examples specific to your scale]
- [Boundary cases to avoid]
```

## Advanced Options

### Tone Modulation

```
/generate-prompt episode:X tone:light      # Lighter than usual
/generate-prompt episode:X tone:serious    # More serious (within scale)
/generate-prompt episode:X tone:comedic    # Comedy emphasis
```

### Relationship Focus

```
/generate-prompt episode:X focus:friendship    # Friendship dynamics
/generate-prompt episode:X focus:conflict      # Scaled conflict
/generate-prompt episode:X focus:growth        # Character development
```

### Structural Emphasis

```
/generate-prompt episode:X structure:setup     # Episode as setup for future
/generate-prompt episode:X structure:payoff    # Episode as payoff for setup
/generate-prompt episode:X structure:bridge    # Transitional episode
```

## Context Integration

The command automatically pulls from:

- `.novel-config.toml` for project settings
- `writing_style/scale_management.md` for constraints
- `templates/series/episode_generation_prompts.md` for base templates
- Previous episodes via `novel find-context episode`
- Character profiles via `novel find-context profile`

## Output Options

### Console Display

Default behavior - shows prompt in terminal for copy-paste.

### File Save

```
/generate-prompt episode:9 --save
```

Saves to `prompts/episode-9-prompt.md`.

### Clipboard Copy

```
/generate-prompt episode:9 --copy
```

Copies directly to clipboard (if supported).

## Template Customization

### Episode-Specific Templates

The command recognizes special episodes:

- Episode 1: Introduction template
- Midpoint: Turning point template
- Danger zone: Crisis template
- Recovery: Healing template
- Finale: Conclusion template

### Genre-Specific Additions

Adds genre-appropriate elements:

- Daily life: Slice-of-life prompts
- Youth drama: School/coming-of-age elements
- Fantasy: World-building reminders
- Mystery: Clue and revelation guidance

## Batch Generation

```
/generate-prompt batch episode:5-8
```

Generates prompts for multiple episodes with:

- Individual customization
- Arc-level coordination
- Progressive development
- Consistency maintenance

## Integration with Other Commands

Works with:

- `/consult-series` for planning context
- `/check-scale` for incident validation
- `/review-prep` for quality assurance
- `novel find-context` for character data