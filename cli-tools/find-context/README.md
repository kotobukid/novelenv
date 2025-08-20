# `find_context` Tool Specification

## 1. Overview

`find_context` is a CLI tool designed to provide a unified interface for searching context-dependent information within this project. It is intended to be used by both humans and LLMs.

## 2. Command Structure

The tool utilizes a subcommand-based architecture and can be used directly or via NovelEnv integrated CLI.

### NovelEnv Integration (Recommended)

```bash
# Character profile search
novel find-context profile <alias>

# Episode search by character
novel find-context episode --character <name>
```

### Direct Usage

```sh
find-context <subcommand> [arguments...]
```

### 2.1. `profile` Subcommand

Retrieves the content of a character's profile file via an alias.

- **Usage**: `find_context profile <alias>`
- **Action**: 
  1. Reads the configuration file (`find_context.toml`).
  2. Looks up the file path associated with the `<alias>` under the `[profile.aliases]` table.
  3. If found, it prints the entire content of the specified file to standard output.
  4. If not found, it prints an error message to standard error and exits with a non-zero status code.

## 3. Configuration File (`find_context.toml`)

The tool is configured via a `find_context.toml` file located in the project root.

- **Format**: TOML
- **Structure**:

```toml
# find_context.toml - Configuration for find-context tool

# Aliases for the `profile` subcommand
[profile.aliases]
"ルネ" = "character/流音.md"
"アカリ" = "character/アカリ.md"
# ... and so on for other characters

# Future subcommands can have their own tables
# [episode.settings]
# index_path = ".index/episodes"
```

## 4. Output

- **On Success**: The requested content is written to `stdout`. The tool should exit with status code `0`.
- **On Failure** (e.g., alias not found, file not readable): A descriptive error message is written to `stderr`. The tool should exit with a non-zero status code (e.g., `1`).

This design allows an LLM to use the tool efficiently: a single `run_shell_command` call yields the final content directly from `stdout`, minimizing complex interactions.

## 5. Future Vision

The `find_context` tool is designed to be extensible. The core principle is to provide a **unified, single interface for accessing the latest, canonical information** from various data sources, abstracting the underlying complexity from the LLM.

Future subcommands could be added to integrate with other services, always ensuring that the data returned is live and not a stale snapshot.

- **Google Docs**: `find_context gdoc "<document_name>"`
- **Obsidian**: `find_context obsidian --tag "#character" "<note_title>"`
- **External APIs/gRPC**: `find_context grpc get_world_setting "<setting_key>"`

This approach fundamentally avoids the stale data and management issues associated with snapshot-based knowledge systems.
