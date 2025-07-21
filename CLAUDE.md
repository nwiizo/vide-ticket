# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vibe-ticket is a high-performance ticket management system written in Rust for the Vibe Coding environment. Built for maximum speed and reliability with comprehensive features for modern development workflows.

### Key Features
- Timestamp-based ticket naming (YYYYMMDDHHMM prefix)
- Git integration with automatic branch management
- Task management within tickets
- Spec-driven development support
- Multiple export/import formats (JSON, YAML, CSV, Markdown)
- Archive system for completed tickets
- Powerful search and filtering capabilities

## Common Development Commands

### Build and Test
```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run -- <command>

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Code Quality
```bash
# Format code (REQUIRED before commit)
cargo fmt

# Run clippy linting (REQUIRED - must pass with no warnings)
cargo clippy -- -D warnings

# Fix clippy suggestions automatically
cargo clippy --fix

# Check for security vulnerabilities
cargo audit

# Generate documentation
cargo doc --no-deps --open
```

### CI/CD Workflow
```bash
# Run full CI checks locally (mimics GitHub Actions)
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test

# Build release artifacts
./scripts/build-release.sh

# Install locally
./scripts/install.sh
```

## Architecture

### Module Structure
```
src/
├── core/           # Business logic (Ticket, Task, Status, Priority)
├── storage/        # Data persistence layer (FileStorage, Repository trait)
├── cli/            # CLI interface and command handlers
├── config/         # Configuration management
├── specs/          # Spec-driven development support
├── error.rs        # Error types and handling
└── main.rs         # Entry point
```

### Key Components

#### Core Module (`src/core/`)
- `ticket.rs`: Main Ticket struct with validation and business rules
- `task.rs`: Task management within tickets
- `status.rs`: Ticket status enum (Todo, Doing, Done, Blocked, Review)
- `priority.rs`: Priority levels (Low, Medium, High, Critical)
- `id.rs`: UUID-based ticket ID generation

#### Storage Module (`src/storage/`)
- `repository.rs`: Repository trait defining storage interface
- `file.rs`: YAML-based file storage implementation
- Files stored in `.vibe-ticket/tickets/` directory

#### CLI Module (`src/cli/`)
- `commands.rs`: Command definitions using clap
- `handlers/`: Individual command implementations
- `output.rs`: Output formatting (JSON, colored terminal)

### Important Design Decisions

1. **Timestamp Prefixes**: All tickets get YYYYMMDDHHMM prefix for chronological ordering
2. **File-based Storage**: Uses YAML for human readability and Git compatibility
3. **Active Ticket**: Stored in `.vibe-ticket/active_ticket` file
4. **Error Handling**: User-friendly errors with suggestions
5. **Git Integration**: Optional but recommended for branch management

## Development Guidelines

### Testing
```bash
# Run unit tests for a specific module
cargo test core::
cargo test storage::
cargo test cli::

# Run integration tests
cargo test --test '*'

# Test with different feature flags
cargo test --no-default-features
cargo test --all-features
```

### Error Handling Pattern
```rust
use crate::error::{Result, VibeTicketError};

pub fn function_name() -> Result<ReturnType> {
    // Use ? operator for error propagation
    let data = operation_that_might_fail()?;
    
    // Custom error with context
    data.validate()
        .map_err(|e| VibeTicketError::Validation(format!("Invalid data: {}", e)))?;
    
    Ok(data)
}
```

### Adding New Commands

1. Define command in `src/cli/commands.rs`
2. Create handler in `src/cli/handlers/<command>.rs`
3. Add handler to `src/cli/handlers/mod.rs`
4. Wire up in `src/main.rs` match statement
5. Add tests in handler file

## Common vibe-ticket Workflows

### Creating and Managing Tickets
```bash
# Create new ticket (note: -P for priority, not -p)
vibe-ticket new "feature-name" -t "Add feature X" -P high --tags "backend,api"

# List open tickets (todo/doing status)
vibe-ticket open

# Start working (creates Git branch)
vibe-ticket start 202507201345-feature-name

# Check current work
vibe-ticket check

# Close ticket
vibe-ticket close -m "Implemented feature X"
```

### Task Management
```bash
# Add tasks to current ticket
vibe-ticket task add "Write unit tests"
vibe-ticket task add "Update documentation"

# List tasks
vibe-ticket task list

# Complete task (1-based index)
vibe-ticket task complete 1
```

### Search and Filter
```bash
# Search by text
vibe-ticket search "authentication"

# Search with regex
vibe-ticket search "fix.*bug" --regex

# Filter by status and priority
vibe-ticket list --status todo --priority high

# List archived tickets
vibe-ticket list --archived
```

### Data Export/Import
```bash
# Export all tickets
vibe-ticket export json -o backup.json

# Export as CSV
vibe-ticket export csv -o tickets.csv --include-archived

# Import tickets
vibe-ticket import backup.json --dry-run
vibe-ticket import backup.json
```

## Configuration

Configuration file: `.vibe-ticket/config.yaml`

### Key Settings
- `project.name`: Project name
- `project.default_priority`: Default priority for new tickets
- `git.enabled`: Enable Git integration
- `git.auto_branch`: Auto-create branches on ticket start
- `git.branch_prefix`: Branch naming prefix (default: "ticket/")
- `ui.emoji`: Enable emoji in output
- `archive.auto_archive`: Auto-archive closed tickets

### Managing Configuration
```bash
# View all settings
vibe-ticket config show

# Set a value
vibe-ticket config set git.auto_branch true

# Get specific value
vibe-ticket config get project.name

# Generate/update CLAUDE.md
vibe-ticket config claude --template advanced
```

## Troubleshooting

### Common Issues

1. **"Project not initialized"**
   - Run `vibe-ticket init` in project root
   - Check for `.vibe-ticket/` directory

2. **Git integration not working**
   - Ensure you're in a Git repository
   - Check `git.enabled` in config
   - Verify Git is installed

3. **Permission errors**
   - Check file permissions in `.vibe-ticket/`
   - Ensure write access to project directory

### Debug Mode
```bash
# Enable verbose logging
RUST_LOG=debug vibe-ticket <command>

# Check version and build info
vibe-ticket --version
```

## Performance Considerations

- File-based storage is efficient for up to ~10,000 tickets
- Use `--limit` flag for large ticket lists
- Archive completed tickets regularly
- CSV export is fastest for large datasets

## Security Notes

- No sensitive data in ticket descriptions
- Git branch names are sanitized
- File permissions set to user-only by default
- No network access required (purely local)

## Contributing

When modifying vibe-ticket:
1. Run `cargo fmt` before committing
2. Ensure `cargo clippy` passes with no warnings
3. Add tests for new functionality
4. Update documentation as needed
5. Follow existing code patterns

## Release Process

1. Update version in `Cargo.toml`
2. Run `cargo test` and `cargo clippy`
3. Build release: `./scripts/build-release.sh`
4. Test installation: `./scripts/install.sh`
5. Tag release: `git tag -a v0.x.x -m "Release v0.x.x"`
6. Push tag: `git push origin v0.x.x`

## Future Enhancements (from TODO.md)

- REST API server (optional feature)
- Plugin system for extensibility
- Database backend support
- Enhanced Git integration (PR creation)
- Web UI (separate project)