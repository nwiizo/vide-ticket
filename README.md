# vibe-ticket

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
- **Task Management**: Break tickets into trackable tasks
- **Flexible Search**: Find tickets with powerful filters
- **Export/Import**: JSON, YAML, CSV, and Markdown formats
- **AI Integration**: Claude Code support with CLAUDE.md generation

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
- [Git Worktree Guide](docs/git-worktree.md)
- [Claude Integration](docs/claude-integration.md)
- [Data Formats](docs/data-formats.md)

## AI Assistant Setup

```bash
# Generate CLAUDE.md for AI assistance
vibe-ticket init --claude-md

# Add strict AI rules
curl https://raw.githubusercontent.com/nwiizo/vibe-ticket/main/rules/agent.md >> CLAUDE.md
```

## Installation

### From Source

```bash
git clone https://github.com/nwiizo/vibe-ticket.git
cd vibe-ticket
cargo build --release
cargo install --path .
```

### Prerequisites

- Rust 1.70+
- Git (for branch/worktree features)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT License](LICENSE)