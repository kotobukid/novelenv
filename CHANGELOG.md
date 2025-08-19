# Changelog

All notable changes to NovelEnv will be documented in this file.

## [v2.0.0] - 2024-12-27

### 🎉 Major Release: NovelEnv v2

Complete rewrite with unified CLI architecture and improved project management.

### ✨ New Features

- **Unified CLI**: All tools accessible through `novel` command
- **Project Initialization**: Interactive setup wizard with `novel init`
- **Environment Activation**: Python venv-style project activation
- **Context Weaver**: Web-based narrative context management with file refresh
- **Find Context**: Centralized information retrieval with alias support
- **Episode Dumper**: Automated episode metadata extraction using LLM
- **Writing Style Management**: Multiple style files with genre-specific guidelines

### 📁 Directory Structure Changes

- **Renamed**: `official/` → `environment/` (world settings)
- **Added**: `notes/` directory for concepts and technical explanations
- **Enhanced**: Comprehensive usage guidelines for each directory

### 🔧 Configuration Updates

- **Renamed**: `.fcrc` → `find_context.toml` for clarity
- **Added**: `find_context.toml.example` with detailed configuration examples
- **Improved**: TOML-based configuration with better documentation

### 📝 Documentation

- **Enhanced**: Comprehensive directory usage guidelines
- **Added**: Migration guide for `.fcrc` → `find_context.toml`
- **Updated**: All READMEs with new command syntax and structure
- **Added**: Genre-specific writing style guides (horror, psycho horror)

### 🛠 Technical Improvements

- **Symlink Installation**: Development-friendly installation via symbolic links
- **File System Refresh**: Real-time file list updates in Context Weaver
- **Error Handling**: Improved error messages and fallback behaviors
- **Tool Integration**: Seamless integration between all CLI tools

### 🏗 Architecture

- **Rust-based**: All CLI tools built with Rust for performance and reliability
- **Web UI**: Modern HTML5/JS interface for Context Weaver
- **TOML Configuration**: Human-readable configuration format
- **Modular Design**: Each tool can be used independently or through unified CLI

### 📋 Usage

```bash
# Initialize new project
novel init my-project

# Activate project environment  
novel activate my-project

# Find character information
novel find-context profile character-name

# Start context weaver
novel weave serve --port 3000

# Generate episode index
novel dump episodes
```

### 🔄 Migration from v1

- Move `.fcrc` files to `find_context.toml`
- Update any direct tool paths to use `novel` command
- Review directory structure changes (`official` → `environment`)

---

## Development Notes

### Project Status

- **Version**: 2.0.0
- **Status**: Production Ready
- **Rust Version**: 1.70+
- **Dependencies**: Modern, stable crates only

### Architecture Decisions

- Unified CLI for better user experience
- TOML configuration for human readability
- Symlink installation for development workflow
- Modular tool design for flexibility

### Future Roadmap

- Integration with external tools (Google Docs, Obsidian)
- Advanced search and filtering capabilities
- Template system expansion
- Multi-language support