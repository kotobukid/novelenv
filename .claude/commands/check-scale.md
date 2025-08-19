# Scale Checker Command

## Command

`/check-scale`

## Description

Quick scale validation tool that checks if proposed incidents or plot points fit within your project's defined scale
limits.

## Usage

Type `/check-scale` followed by your incident description to get immediate scale feedback.

### Basic Format

```
/check-scale [incident description]
```

### Examples

```
/check-scale The protagonist accidentally breaks the school's expensive computer
/check-scale Main character gets into a fight and someone gets seriously injured
/check-scale The student council president resigns due to a scandal
```

## What It Does

1. **Loads Project Settings**: Reads your scale level from `.novel-config.toml`
2. **Analyzes Incident**: Evaluates the proposed incident against scale criteria:
    - Impact scope (individual → society)
    - Financial damage
    - Resolution method required
    - Relationship consequences
    - Recovery timeframe
3. **Provides Verdict**:
    - ✅ **APPROVED**: Fits within scale
    - ⚠️ **CAUTION**: At scale limit
    - ❌ **REJECTED**: Exceeds scale
4. **Suggests Alternatives**: If rejected, provides scale-appropriate alternatives

## Response Format

### Approved Incident

```
✅ SCALE CHECK: APPROVED
Incident: [description]
Scale Level: [X] 
Assessment: This incident fits well within your scale limits.
- Impact: [scope analysis]
- Resolution: [method]
- Recovery: [timeframe]
```

### Rejected Incident

```
❌ SCALE CHECK: REJECTED
Incident: [description]
Scale Level: [X] (incident assessed as Level [Y])
Problems:
- [specific issue 1]
- [specific issue 2]

Suggested Alternatives:
1. [scaled-down version 1]
2. [scaled-down version 2]
```

## Context Integration

The command automatically references:

- `writing_style/scale_management.md` for scale definitions
- `templates/series/incident_scale_checker.md` for evaluation criteria
- Project's danger zone episode and current story position
- Character relationships and story context

## Advanced Usage

### Episode-Specific Checking

```
/check-scale episode:9 [incident] 
```

Special handling for danger zone episodes.

### Character-Impact Focus

```
/check-scale character:[name] [incident]
```

Evaluates impact specifically on named character's story arc.

### Quick Batch Check

```
/check-scale batch
1. [incident 1]
2. [incident 2] 
3. [incident 3]
```

## Warning Flags

The checker automatically flags:

- Legal terminology (訴訟, 裁判, etc.)
- Permanent consequences (退学, 永久, etc.)
- Physical harm escalations
- Financial damage exceeding scale
- Authority figure involvement beyond scale

## Integration with Other Commands

Works together with:

- `/consult-series` for detailed planning sessions
- `/review-prep` for pre-review validation
- `novel find-context` for character/episode context