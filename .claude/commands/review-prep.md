# Review Preparation Command

## Command

`/review-prep`

## Description

Prepares your story content for review by performing comprehensive checks and generating a review-ready summary.

## Usage

Type `/review-prep` with optional parameters to prepare specific content for review.

### Basic Format

```
/review-prep [episode:X] [character:name] [full]
```

### Examples

```
/review-prep episode:9          # Prepare episode 9 for review
/review-prep character:alice    # Review all content featuring alice
/review-prep full              # Full project review preparation
```

## What It Does

### 1. Scale Compliance Check

- Validates all incidents against project scale level
- Flags potential scale violations
- Suggests corrections for over-scale content

### 2. Consistency Verification

- Character behavior alignment
- Timeline consistency
- Setting/world-building coherence
- Previously established facts

### 3. Narrative Structure Analysis

- Plot progression appropriateness
- Character arc development
- Theme reinforcement
- Pacing evaluation

### 4. Review Document Generation

Creates a structured document for reviewer containing:

- Content summary
- Key scenes and incidents
- Character interactions
- Scale compliance statement
- Potential concerns flagged
- Reviewer questions prepared

## Output Format

### Episode Review Prep

```
📋 REVIEW PREPARATION: Episode X

## Summary
[Concise plot summary]

## Scale Compliance
✅ Scale Level: [X] - All incidents within limits
⚠️ Caution: [Any borderline content]
❌ Violations: [Any over-scale content with corrections]

## Key Incidents
1. [Incident 1] - Scale Level [X]
2. [Incident 2] - Scale Level [X]

## Character Arcs
- [Character A]: [Development in this episode]
- [Character B]: [Role and growth]

## Consistency Check
✅ Character behavior
✅ Timeline accuracy  
✅ Setting details
⚠️ [Any flagged inconsistencies]

## Reviewer Focus Points
Please evaluate:
1. [Specific question about plot choice]
2. [Concern about character development]
3. [Scale appropriateness of key scene]

## Context References
- Previous episodes: [relevant episodes]
- Character profiles: [affected characters]
- World-building elements: [relevant settings]
```

## Advanced Features

### Pre-Review Self-Assessment

```
/review-prep self-check episode:X
```

Runs through common review failure points:

- Scale violations
- Character inconsistencies
- Plot holes
- Tonal mismatches

### Reviewer Question Generator

```
/review-prep questions episode:X
```

Generates specific questions to ask the reviewer:

- "Is the incident in Scene 3 appropriate for the scale?"
- "Does Character A's reaction align with their established personality?"
- "Are there any plot holes you notice?"

### Comparison Mode

```
/review-prep compare episode:X episode:Y
```

Compares two episodes for consistency in:

- Character behavior
- World-building details
- Tone and style
- Scale handling

## Integration with Scale Management

Automatically references:

- Project scale level from `.novel-config.toml`
- Scale definitions from `writing_style/scale_management.md`
- Episode templates from `templates/series/`
- Incident checkers and warning lists

## Batch Processing

```
/review-prep batch episode:1-5
```

Prepares multiple episodes simultaneously with:

- Individual episode prep sheets
- Cross-episode consistency checks
- Arc-level analysis
- Cumulative scale tracking

## Pre-Review Checklist

The command generates a final checklist:

- [ ] All incidents within scale limits
- [ ] Character behavior consistent
- [ ] Timeline makes sense
- [ ] No contradictions with previous episodes
- [ ] Tone appropriate for project
- [ ] Reviewer questions prepared
- [ ] Context documents ready

## File Output

Can optionally save prep documents:

```
/review-prep episode:9 --save
```

Creates `review-prep-episode-9.md` in project root with full analysis.

## Integration with Other Commands

Works with:

- `/check-scale` for incident validation
- `/consult-series` for planning context
- `novel find-context` for character/episode data
- `novel weave` for context compilation