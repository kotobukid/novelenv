# Series Consultation Mode

## Command

`/consult-series`

## Description

Activate series consultation mode for scale-managed projects. This command helps you plan storylines, check incident
scales, and maintain narrative consistency within your defined scale limits.

## Usage

Type `/consult-series` to enter consultation mode. The assistant will:

1. Load your project's scale configuration from `.novel-config.toml`
2. Reference scale management guidelines from `writing_style/scale_management.md`
3. Use incident checkers from `templates/series/incident_scale_checker.md`
4. Apply episode-specific prompt templates

## Available Sub-commands

### Plot Development

- `plot`: Discuss overall plot structure within scale limits
- `episode <number>`: Plan specific episode content
- `arc <number>`: Plan story arc development
- `character-arc <name>`: Discuss character development paths

### Scale Management

- `check-incident <description>`: Evaluate proposed incident against scale limits
- `suggest-alternatives <description>`: Get scale-appropriate alternatives
- `tone-check <episode>`: Verify tonal consistency

### Crisis Navigation

- `danger-zone`: Special consultation for your project's danger zone episode
- `recovery`: Plan the recovery episode after crisis
- `escalation-warning`: Check for scale escalation risks

## Context Loading

The command automatically loads:

- Project configuration (scale level, episode count, genre)
- Character profiles via `novel find-context profile`
- Previous episodes via `novel find-context episode`
- Scale management rules and templates

## Example Session

```
User: /consult-series
Assistant: Entering series consultation mode for [Project Name]
- Scale Level: 2 (軽いドラマ)
- Episodes: 12
- Danger Zone: Episode 9
- Current context loaded

How can I help with your series development?

User: I want episode 7 to have some drama
Assistant: Let's plan episode 7 drama within Scale Level 2 limits...
[References scale_management.md and provides appropriate suggestions]
```

## Integration

This command works with:

- `.novel-config.toml` for project settings
- `writing_style/scale_management.md` for scale definitions
- `templates/series/` for planning templates
- `novel find-context` for character/episode data