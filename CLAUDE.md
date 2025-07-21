# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vide-ticket is a high-performance ticket management system written in Rust for the Vide Coding environment. Built for maximum speed and reliability with comprehensive features for modern development workflows.

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
4. **Timestamp-based Slugs**: All tickets are prefixed with YYYYMMDDHHMM for chronological ordering

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

## CLAUDE.md Generation Features

vide-ticket includes built-in support for generating and maintaining CLAUDE.md files to enhance Claude Code integration.

### Generating CLAUDE.md

#### During Project Initialization
```bash
# Initialize project with CLAUDE.md
vide-ticket init --claude-md
# or
vide-ticket init --claude
```

This creates a CLAUDE.md file with:
- Project overview and metadata
- Common vide-ticket commands
- Current configuration settings
- Workflow guidelines
- The initialization command itself is recorded in the file

#### For Existing Projects
```bash
# Generate basic CLAUDE.md
vide-ticket config claude

# Generate with advanced template
vide-ticket config claude --template advanced

# Append to existing CLAUDE.md
vide-ticket config claude --append

# Custom output path
vide-ticket config claude --output ./docs/CLAUDE.md
```

### Template Options

#### Basic Template
- Project name and description
- Essential vide-ticket commands
- Current project configuration (Git integration, default priority, etc.)
- Project statistics (total tickets, active tickets)
- Basic workflow guidelines
- Best practices

#### Advanced Template  
- Everything from basic template plus:
- Git worktree support examples
- Advanced search and filtering
- Export/import functionality
- Environment variables documentation
- Git hooks integration examples
- Troubleshooting guide

### Dynamic Content
The generated CLAUDE.md includes dynamically populated information:
- Project name from configuration
- Current date of generation
- Real-time ticket statistics
- Active project settings
- Git integration status

### Usage Workflow

1. **New Projects**: Use `vide-ticket init --claude-md` to start with AI assistance ready
2. **Existing Projects**: Run `vide-ticket config claude` to add CLAUDE.md
3. **Updates**: Use `--append` flag when project configuration changes
4. **Customization**: Add project-specific instructions after generation

### Benefits for Claude Code
- Understands project-specific commands and workflows
- Provides context-aware suggestions
- Follows established project conventions
- Assists with ticket management best practices

## vide-ticket Command Reference

### Quick Start
```bash
# Initialize a new project
vide-ticket init --claude-md

# Create a new ticket (note: -P for priority, not -p)
vide-ticket new "implement-auth" -t "Add user authentication" -P high

# List all open tickets
vide-ticket open

# Start working on a ticket
vide-ticket start implement-auth

# Close current ticket
vide-ticket close -m "Implemented OAuth2 authentication"
```

### Essential Commands

#### Ticket Management
```bash
# Create ticket with automatic timestamp prefix
vide-ticket new "feature-name" -t "Title" -d "Description" -P high --tags "backend,api"

# List tickets with filters
vide-ticket list --status todo --priority high
vide-ticket list --open  # Show only todo/doing tickets
vide-ticket open         # Alias for list --open

# Search tickets
vide-ticket search "authentication" --regex
vide-ticket search "bug" --tags

# Update ticket
vide-ticket edit <ticket> -t "New Title" -P critical --add-tags "urgent"
```

#### Workflow Commands
```bash
# Start working (creates Git branch)
vide-ticket start <ticket>

# Check current status
vide-ticket check --detailed

# Complete and archive
vide-ticket close <ticket> -m "Completion message" --archive
```

#### Task Management
```bash
# Add tasks to current ticket
vide-ticket task add "Write unit tests"
vide-ticket task add "Update documentation"

# Complete tasks
vide-ticket task complete 1
vide-ticket task list --incomplete
```

#### Data Management
```bash
# Export for backup
vide-ticket export --format json -o backup.json
vide-ticket export --format csv -o tickets.csv --include-archived

# Import tickets
vide-ticket import backup.json --dry-run
vide-ticket import tickets.csv
```

#### Configuration
```bash
# View configuration
vide-ticket config show

# Set configuration values
vide-ticket config set git.auto_branch true
vide-ticket config set project.default_priority high
vide-ticket config set ui.emoji true

# Generate/update CLAUDE.md
vide-ticket config claude --template advanced --append
```

### Important Notes

1. **Priority Flag**: Use `-P` or `--priority` for priority (not `-p`, which is for project path)
2. **Timestamps**: All tickets get YYYYMMDDHHMM prefix automatically
3. **Git Integration**: Branches are created as `ticket/<timestamp>-<slug>` by default
4. **Open Tickets**: Use `vide-ticket open` for quick view of active work

### Common Workflows

#### Bug Fix Workflow
```bash
vide-ticket new "fix-login-error" -t "Fix login validation error" -P high --tags "bug,auth"
vide-ticket start fix-login-error
# ... fix the bug ...
vide-ticket close -m "Fixed validation regex pattern"
```

#### Feature Development
```bash
vide-ticket new "api-endpoints" -t "Implement REST API endpoints" -P medium
vide-ticket start api-endpoints
vide-ticket task add "Design API schema"
vide-ticket task add "Implement GET endpoints"
vide-ticket task add "Implement POST endpoints"
vide-ticket task add "Add authentication"
vide-ticket task add "Write API tests"
# ... complete tasks one by one ...
vide-ticket task complete 1
```

#### Daily Standup
```bash
# Check what you're working on
vide-ticket check

# See all open tickets
vide-ticket open

# Review high priority items
vide-ticket list --open --priority high
```