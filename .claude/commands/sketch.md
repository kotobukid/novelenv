# /sketch command

Generate a scene fragment and save it as a .md file in the scene_sketch/ directory.

Usage: /sketch [narrative_id] [additional description...]
/sketch [description of scene, characters, or any other details...]

## Context Resolution

When the first argument is a UUID (narrative_id), automatically resolve context using:

```bash
./cli-tools/context-weaver/target/release/weaver resolve [narrative_id]
```

This provides pre-curated context including character profiles, world settings, and style guides relevant to the
specific scene.

## Operation Modes

### 1. Context-Enhanced Mode (with narrative_id)

When narrative_id is provided:

1. Resolve context using weaver.
2. Check if the provided information is sufficient for scene sketch generation.
3. If insufficient, proceed to Requirements Gathering.
4. If sufficient, proceed to Direct Generation.

### 2. Requirements Gathering Mode (for vague requests)

When user instructions are insufficient, act as an editor and ask for necessary information to create a better scene.

### 3. Direct Generation Mode (for specific requests)

When sufficient information is provided, directly generate the scene in `scene_sketch/`.

## Information Sufficiency Check (for Context-Enhanced Mode)

When using a `narrative_id`, verify the following information is available in the context:

**Scene Sketch Essentials:**

1. **Length**: Target word count (default: 2000 characters).
2. **Tone**: The desired balance of tones (e.g., Comedy, Serious, Heartwarming).
3. **Setting Freedom**: Can new locations or rules be created within the world's constraints?
4. **Scene Purpose**: What should this scene accomplish (e.g., character development, plot advancement)?
5. **Character Focus**: Which characters are central to this scene?

**If any essential information is missing, ask the user clarifying questions, such as:**

- "この場面の長さはどの程度を想定していますか？（デフォルト: 2000文字程度）"
- "どのような雰囲気（例：コメディ、シリアス）を希望しますか？"
- "世界観に新しい設定（施設、ルールなど）を追加してもよろしいですか？"
- "このシーンの目的（例：キャラクターの関係構築、日常の一コマ）は何ですか？"

## Requirements Gathering Process

If the request is too vague, act as an editor and confirm the following:

**Basic Information:**

- **Characters involved**: Use the `find-context` tool for existing characters if applicable.
- **Scene purpose**: What should be depicted?
- **Timeline position**: Where does this scene fit in the overall story?

**Scene Details:**

- **Setting**: Location, time, season, weather.
- **Central events or themes**: What is the core of the scene?
- **Character emotional changes**: How do the characters' feelings evolve?
- **Scene structure**: How does the scene begin and end?

## Generation Process

The generated content should:

- Follow the writing style defined in `writing_style/always.md` if it exists.
- Use the `find-context` tool to retrieve character profiles when characters are mentioned.
- Be saved with a descriptive filename in the `scene_sketch/` directory.
- Be written in the project's primary language (e.g., Japanese).

## Optional Parameters (defaults if not specified)

This command is designed for idea generation. If parameters are not specified, the LLM can decide freely:

- **Atmosphere/Mood**: LLM's choice (e.g., heartwarming, serious, comedy, mysterious).
- **Time of day**: LLM's choice (e.g., morning, afternoon, evening, night).
- **Location**: LLM's choice from the established world setting.
- **Additional characters**: LLM's choice, may include other characters or focus on solo scenes.

## File Naming Convention

- Use a pattern like: `[main_character]_[situation_key_words].md`
- Example: `character_A_morning_breakfast.md`, `character_B_and_C_first_encounter.md`

## Content Structure

- Always include a brief logline at the beginning.
- The logline should summarize the scene in 1-2 sentences.
- **Always include a "作者ノート" (Author's Note) section immediately after the logline, before the main content** to
  document:
    - Key narrative choices made during writing
    - Character development aspects focused on
    - Thematic elements emphasized
    - Connection points to other scenes or story elements
    - Any creative challenges overcome

## Purpose of Author's Notes (Placed at Beginning)

Scene sketches are raw materials for later compilation into finished works. Placing author's notes at the beginning
ensures that:

- LLMs processing the content understand the creative intent before reading the main text
- Future editing and compilation can build on documented insights from the start
- The thought process behind character and plot decisions guides subsequent processing
- Connections between scenes are immediately apparent when reviewing multiple sketches
- The processing flow aligns with LLM's single-pass reading behavior

## Important for Task Usage

When executed via Task tool, always report the exact filename created:

- Example: "Created scene at: ./scene_sketch/character_situation.md"

Example: `/sketch A conversation between two characters at a cafe.`
