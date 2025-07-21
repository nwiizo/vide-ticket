# vide-ticket

A high-performance ticket management system built with Rust, designed to replace shell-based ticket management with a robust, type-safe solution.

## Features

- **Fast Performance**: Built with Rust for maximum speed and reliability
- **Git Integration**: Automatic branch creation and management
- **Flexible Storage**: YAML-based storage with efficient file handling
- **Rich CLI**: Intuitive command-line interface with comprehensive options
- **Task Management**: Built-in task tracking within tickets
- **Export/Import**: Support for JSON, YAML, CSV, and Markdown formats
- **Search & Filter**: Powerful search capabilities with regex support
- **Archive System**: Keep completed tickets organized
- **Timestamp-based Naming**: Automatic chronological ordering with YYYYMMDDHHMM prefixes

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/vide-ticket.git
cd vide-ticket

# Build with cargo
cargo build --release

# Install to PATH
cargo install --path .
```

### Prerequisites

- Rust 1.70 or higher
- Git (for branch management features)

## Quick Start

```bash
# Initialize a new project
vide-ticket init

# Create a new ticket
vide-ticket new "fix-login-bug" --title "Fix login authentication issue" --priority high

# List all tickets
vide-ticket list

# Start working on a ticket (creates Git branch)
vide-ticket start fix-login-bug

# Check current status
vide-ticket check

# Complete a ticket
vide-ticket close fix-login-bug --message "Fixed authentication logic"
```

## Command Reference

### Project Management

#### `init`
Initialize a new vide-ticket project in the current directory.

```bash
vide-ticket init [OPTIONS]

Options:
  -n, --name <NAME>              Project name
  -d, --description <DESC>       Project description
  -f, --force                    Force initialization even if directory is not empty
```

### Ticket Operations

#### `new`
Create a new ticket with automatic timestamp prefix.

```bash
vide-ticket new <SLUG> [OPTIONS]

Arguments:
  <SLUG>                         Ticket identifier (will be prefixed with timestamp)

Options:
  -t, --title <TITLE>           Ticket title
  -d, --description <DESC>      Detailed description
  -p, --priority <PRIORITY>     Priority level [low, medium, high, critical]
  --tags <TAGS>                 Comma-separated tags
  -s, --start                   Start working immediately
```

Example:
```bash
vide-ticket new "user-auth" --title "Implement user authentication" --priority high --tags "backend,security"
# Creates: 202507201345-user-auth
```

#### `list`
List tickets with various filtering options.

```bash
vide-ticket list [OPTIONS]

Options:
  -s, --status <STATUS>         Filter by status [todo, progress, review, done]
  -p, --priority <PRIORITY>     Filter by priority
  -a, --assignee <ASSIGNEE>     Filter by assignee
  --sort <FIELD>                Sort by field [created, updated, priority, status]
  -r, --reverse                 Reverse sort order
  -l, --limit <N>               Limit number of results
  --archived                    Show archived tickets
  --since <DATE>                Show tickets created since date
  --until <DATE>                Show tickets created until date
```

#### `start`
Start working on a ticket (sets status to "In Progress" and creates Git branch).

```bash
vide-ticket start [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -b, --branch                  Create Git branch
  --branch-name <NAME>          Custom branch name (default: ticket slug)
```

#### `close`
Complete a ticket and optionally archive it.

```bash
vide-ticket close [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -m, --message <MESSAGE>       Closing message
  -a, --archive                 Archive the ticket
  --pr                          Create pull request (requires gh CLI)
```

#### `edit`
Edit ticket properties.

```bash
vide-ticket edit [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -t, --title <TITLE>           New title
  -d, --description <DESC>      New description
  -p, --priority <PRIORITY>     New priority
  -s, --status <STATUS>         New status
  --add-tags <TAGS>             Add tags (comma-separated)
  --remove-tags <TAGS>          Remove tags (comma-separated)
  -e, --editor                  Open in text editor
```

#### `show`
Display detailed information about a ticket.

```bash
vide-ticket show [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -t, --tasks                   Show task details
  -h, --history                 Show status history
  -m, --markdown                Output as markdown
```

### Task Management

#### `task add`
Add a task to a ticket.

```bash
vide-ticket task add <TITLE> [OPTIONS]

Arguments:
  <TITLE>                       Task title

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
```

#### `task complete`
Mark a task as completed.

```bash
vide-ticket task complete <TASK> [OPTIONS]

Arguments:
  <TASK>                        Task index (1-based)

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
```

#### `task list`
List tasks for a ticket.

```bash
vide-ticket task list [OPTIONS]

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
  -c, --completed               Show only completed tasks
  -i, --incomplete              Show only incomplete tasks
```

### Search and Filter

#### `search`
Search tickets by content.

```bash
vide-ticket search <QUERY> [OPTIONS]

Arguments:
  <QUERY>                       Search query

Options:
  -t, --title                   Search in titles only
  -d, --description             Search in descriptions only
  --tags                        Search in tags only
  -r, --regex                   Use regex matching
```

### Data Management

#### `archive`
Archive or unarchive tickets.

```bash
vide-ticket archive <TICKET> [OPTIONS]

Arguments:
  <TICKET>                      Ticket ID or slug

Options:
  -u, --unarchive              Unarchive the ticket
```

#### `export`
Export tickets to various formats.

```bash
vide-ticket export <FORMAT> [OPTIONS]

Arguments:
  <FORMAT>                      Export format [json, yaml, csv, markdown]

Options:
  -o, --output <FILE>          Output file (defaults to stdout)
  --include-archived           Include archived tickets
```

#### `import`
Import tickets from files.

```bash
vide-ticket import <FILE> [OPTIONS]

Arguments:
  <FILE>                       Import file path

Options:
  -f, --format <FORMAT>        File format (auto-detected if not specified)
  --skip-validation            Skip validation checks
  --dry-run                    Preview without importing
```

### Utility Commands

#### `check`
Check project status and active ticket.

```bash
vide-ticket check [OPTIONS]

Options:
  -d, --detailed               Show detailed information
  -s, --stats                  Show statistics
```

## Global Options

These options can be used with any command:

- `-p, --project <DIR>`: Use specific project directory
- `-j, --json`: Output in JSON format
- `-n, --no-color`: Disable colored output
- `-v, --verbose`: Enable verbose logging

## Data Formats

### Export/Import JSON Format

```json
{
  "tickets": [
    {
      "id": "uuid-here",
      "slug": "202507201345-feature-name",
      "title": "Feature Title",
      "description": "Detailed description",
      "priority": "high",
      "status": "todo",
      "tags": ["backend", "api"],
      "created_at": "2025-07-20T13:45:00Z",
      "started_at": null,
      "closed_at": null,
      "assignee": "username",
      "tasks": [
        {
          "title": "Task description",
          "completed": false
        }
      ],
      "metadata": {}
    }
  ]
}
```

### CSV Format

CSV exports include the following columns:
- ID, Slug, Title, Status, Priority, Assignee, Tags, Created At, Started At, Closed At, Tasks Total, Tasks Completed, Description

## Configuration

Project configuration is stored in `.vide-ticket/config.yaml`:

```yaml
name: "My Project"
description: "Project description"
created_at: "2025-07-20T00:00:00Z"
default_priority: "medium"
auto_archive: true
archive_after_days: 30
```

## File Structure

```
.vide-ticket/
├── config.yaml          # Project configuration
├── active              # Active ticket ID
└── tickets/            # Ticket YAML files
    ├── <ticket-id>.yaml
    └── ...
```

## Tips and Best Practices

1. **Ticket Naming**: Use descriptive slugs that clearly indicate the ticket's purpose
2. **Git Integration**: Always use `--branch` when starting tickets for better Git workflow
3. **Task Breakdown**: Break complex tickets into smaller tasks for better tracking
4. **Regular Archiving**: Archive completed tickets to keep the active list manageable
5. **Tag Consistently**: Use consistent tag names for better organization
6. **Export Regularly**: Export tickets periodically for backup

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run with verbose output
RUST_LOG=debug cargo run -- list
```

### Architecture

- **Core**: Data models and business logic (`src/core/`)
- **Storage**: File-based storage implementation (`src/storage/`)
- **CLI**: Command-line interface and handlers (`src/cli/`)
- **Error Handling**: Comprehensive error types with user-friendly messages

## License

[Your License Here]

## Contributing

[Your Contributing Guidelines Here]