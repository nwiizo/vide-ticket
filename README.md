# vibe-ticket

[![Crates.io](https://img.shields.io/crates/v/vibe-ticket.svg)](https://crates.io/crates/vibe-ticket)
[![Documentation](https://docs.rs/vibe-ticket/badge.svg)](https://docs.rs/vibe-ticket)
[![CI](https://github.com/nwiizo/vibe-ticket/workflows/CI/badge.svg)](https://github.com/nwiizo/vibe-ticket/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance ticket management system for developers, built with Rust. Features Git worktree integration for parallel development workflows.

## Quick Start

```bash
# Install
cargo install vibe-ticket

# Initialize project
vibe-ticket init

# Create and start a ticket
vibe-ticket new fix-bug --title "Fix login issue" --priority high
vibe-ticket start fix-bug  # Creates Git worktree by default

# Work in the worktree
cd ./my-project-vibeticket-fix-bug/

# Track progress
vibe-ticket task add "Fix null check"
vibe-ticket task complete 1

# Complete ticket
vibe-ticket close fix-bug --message "Fixed login issue"
```

## Key Features

- **Git Worktree Support**: Work on multiple tickets simultaneously
- **Concurrent Edit Protection**: Safe multi-user/multi-process ticket access with automatic lock management
- **Spec-Driven Development**: Three-phase development with requirements, design, and tasks
- **Task Management**: Break tickets into trackable tasks
- **Flexible Search**: Find tickets with powerful filters
- **Export/Import**: JSON, YAML, CSV, and Markdown formats
- **AI Integration**: Claude Code support with CLAUDE.md generation
- **MCP Server**: Run as Model Context Protocol server for AI assistants

## Essential Commands

```bash
vibe-ticket --help              # Show help for any command
vibe-ticket check               # Check current status
vibe-ticket list --open         # Show active tickets
vibe-ticket search "keyword"    # Search tickets
vibe-ticket worktree list       # List all worktrees
```

## Configuration

```yaml
# .vibe-ticket/config.yaml
git:
  worktree_default: true        # Create worktrees by default
  worktree_prefix: "./{project}-vibeticket-"
project:
  default_priority: medium
```

## Documentation

- [Command Reference](docs/commands.md)
- [Configuration](docs/configuration.md)
- [Spec-Driven Development](docs/spec-driven-development.md)
- [Git Worktree Guide](docs/git-worktree.md)
- [Claude Integration](docs/claude-integration.md)
- [MCP Integration Guide](docs/mcp-guide.md)
- [Data Formats](docs/data-formats.md)

## AI Assistant Setup

```bash
# Generate CLAUDE.md for AI assistance
vibe-ticket init --claude-md

# Add strict AI rules
curl https://raw.githubusercontent.com/nwiizo/vibe-ticket/main/rules/agent.md >> CLAUDE.md
```

### MCP (Model Context Protocol) Support

vibe-ticket can run as an MCP server for AI assistants like Claude:

```bash
# Install (MCP is now included by default)
cargo install vibe-ticket

# Add to Claude Code (global)
claude mcp add vibe-ticket ls $HOME/.cargo/bin/vibe-ticket --scope local -- mcp serve

# Test the server
vibe-ticket mcp serve
```

#### AI Assistant Integration

When using vibe-ticket with AI assistants via MCP:

1. **All CLI operations are available through MCP** - AI can create tickets, manage tasks, search, and more
2. **Suggest MCP tools for improvements** - If you notice patterns or repetitive tasks, ask your AI assistant to use vibe-ticket MCP tools to automate them
3. **Integrated workflow** - AI can seamlessly switch between code editing and ticket management

Example AI interactions:
```
"Create a ticket for the bug we just found"
"Add a task to track the performance optimization"
"Search for tickets related to authentication"
"Show me all open high-priority tickets"
```

See [MCP Integration Guide](docs/mcp-guide.md) for detailed setup and usage.

## Best Practices

### Ticket Management
- Always create a ticket before starting work
- Use meaningful ticket slugs that describe the task
- Update ticket status as work progresses
- Close tickets with descriptive completion messages

### Git Worktree Workflow
- Each ticket gets its own isolated worktree directory
- Work in `./project-vibeticket-<slug>/` directories
- Clean up worktrees after closing tickets: `vibe-ticket worktree remove <ticket>`
- Use `vibe-ticket worktree list` to track active worktrees

### Documentation Testing
- Run `cargo test --doc` regularly to ensure examples work
- Keep documentation examples up-to-date with code changes
- Doc tests prevent documentation drift

### Active Development Tips
- Check current context with `vibe-ticket check`
- Use `vibe-ticket list --status doing` to see active work
- Break complex work into tasks within tickets
- Conduct retrospectives after major tasks

### Concurrent Access Safety
- vibe-ticket automatically handles multiple users/processes accessing tickets
- File locking prevents data corruption during concurrent modifications
- Stale locks are automatically cleaned up after 30 seconds
- Operations retry automatically if a file is temporarily locked

## Installation

### From Source

```bash
git clone https://github.com/nwiizo/vibe-ticket.git
cd vibe-ticket
cargo build --release
cargo install --path .

# With MCP support
cargo build --release --features mcp
cargo install --path . --features mcp
```

### Prerequisites

- Rust 1.70+
- Git (for branch/worktree features)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT License](LICENSE)
