# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vide-ticket is a high-performance ticket management system written in Rust, designed to replace the existing `ticket.sh` with 10x performance improvement while maintaining backwards compatibility.

## Common Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run

# Run with specific features
cargo run --features api
cargo run --features database
cargo run --all-features
```

### Testing and Quality Checks
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check

# Run clippy linting
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix

# Detect semantic code similarities
similarity-rs .

# Analyze duplicate code patterns and create refactoring plan
# Check detailed options with: similarity-rs -h
```

### Documentation
```bash
# Generate and open documentation
cargo doc --no-deps --open

# Generate docs with private items
cargo doc --no-deps --document-private-items
```

### Publishing to crates.io
```bash
# Verify package metadata and structure
cargo publish --dry-run

# Check package contents that will be uploaded
cargo package --list

# Publish to crates.io (requires authentication)
cargo publish

# Publish with specific version
cargo publish --package vide-ticket

# Login to crates.io (first time setup)
cargo login

# Owner management
cargo owner --add username
cargo owner --remove username
cargo owner --list
```

## Architecture

### Module Structure
- **`core`**: Central business logic containing ticket models, validation rules, and core operations
- **`storage`**: Abstraction layer for data persistence with implementations for file-based and optional database storage
- **`cli`**: Command-line interface using clap, handles user interactions and command parsing
- **`api`**: Optional REST API using axum framework (enabled with `api` feature)
- **`config`**: Configuration management supporting YAML/JSON formats and environment variables
- **`plugins`**: Plugin system for extending functionality without modifying core code

### Key Design Decisions
1. **Feature Flags**: API and database support are optional features to keep the core binary lightweight
2. **Error Handling**: Uses `anyhow` for application errors and `thiserror` for library errors
3. **Async Runtime**: Tokio with full features for concurrent operations
4. **Backwards Compatibility**: Must maintain compatibility with existing `ticket.sh` commands

### Development Roadmap (from TODO.md)
The project follows a three-phase implementation plan:
1. **Phase 1 (MVP)**: Basic ticket operations, Git integration, CLI implementation
2. **Phase 2 (Quality)**: Testing, error handling, performance optimization
3. **Phase 3 (Extensions)**: Plugin system, API server, advanced features

## Important Configurations

### Rust Edition and MSRV
- Rust Edition: 2021
- Minimum Supported Rust Version (MSRV): 1.70.0

### Performance Settings
Release builds are optimized with:
- LTO (Link Time Optimization) enabled
- Optimization level 3
- Single codegen unit for maximum optimization

### Code Style
- Line width: 100 characters
- Import style: Vertical with grouping
- Unix line endings enforced
- Comprehensive clippy rules for code quality

## Claude Code Best Practices

### Memory Management (CLAUDE.md)
- **Be Specific**: Write actionable instructions ("Use 2-space indentation" vs "Format code properly")
- **Structure Content**: Use clear markdown headings and bullet points for organization
- **Modular Organization**: Use imports with `@path/to/file` syntax for complex projects
- **Regular Updates**: Review and update memories as the project evolves
- **Quick Memory**: Start a line with `#` to quickly add a memory during conversation

### Effective Claude Code Usage

#### CLI Commands
```bash
# Start interactive session
claude

# One-shot query (non-interactive)
claude -p "analyze this error pattern"

# Continue previous conversation
claude -c

# Work across multiple directories
claude --add-dir ../related-project

# Skip permissions for automation
claude --dangerously-skip-permissions -p "run tests"
```

#### Keyboard Shortcuts
- `Ctrl+C` - Cancel current operation
- `Ctrl+D` - Exit session
- `Ctrl+L` - Clear screen
- `Esc + Esc` - Edit previous message
- `\` + `Enter` - Multiline input (recommended)
- `↑/↓` - Navigate command history

#### Workflow Patterns
1. **Codebase Exploration**: Start high-level → dive into specifics
2. **Debugging**: Share error → get fix → apply → verify
3. **Feature Development**: Describe in plain English → iterate on implementation
4. **Testing**: Identify gaps → generate tests → verify coverage
5. **Extended Thinking**: Use "think harder" for complex problems

#### Session Management
- Use `--continue` to resume previous conversations
- Leverage git worktrees for parallel task isolation
- Batch multiple file operations for better performance

### Project-Specific Instructions
When writing CLAUDE.md for your project:
1. Include common commands and their purposes
2. Document project-specific conventions
3. List critical files and their roles
4. Define testing and quality standards
5. Specify security requirements

### Integration Tips
- Configure GitHub Actions behavior through CLAUDE.md
- Use slash commands (`/memory`, `/config`) for quick access
- Enable appropriate tools with `--allowedTools` flag
- Set permission mode based on workflow needs