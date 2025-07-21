# vide-ticket

A high-performance ticket management system for Vide Coding environment, built with Rust for maximum speed and reliability.

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
- **Open Tickets View**: Quickly view all active tickets (todo/doing status)
- **Spec-Driven Development**: Built-in support for specifications and requirements tracking
- **AI Integration**: Claude Code integration with automatic CLAUDE.md generation

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

### From crates.io (Coming Soon)

```bash
cargo install vide-ticket
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
  -P, --priority <PRIORITY>     Priority level [low, medium, high, critical]
  --tags <TAGS>                 Comma-separated tags
  -s, --start                   Start working immediately

Note: Use -P or --priority for priority (not -p, which is for project path)
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
  -s, --status <STATUS>         Filter by status [todo, doing, done, blocked, review]
  --priority <PRIORITY>         Filter by priority
  -a, --assignee <ASSIGNEE>     Filter by assignee
  --sort <FIELD>                Sort by field [created, updated, priority, status, slug]
  -r, --reverse                 Reverse sort order
  -l, --limit <N>               Limit number of results
  --archived                    Show archived tickets
  --open                        Show only open tickets (todo, doing)
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

#### `open`
Show all open tickets (alias for `list --open`). This is a quick way to see tickets that need attention.

```bash
vide-ticket open [OPTIONS]

Options:
  --sort <FIELD>                Sort by field [created, updated, priority, status, slug]
  -r, --reverse                 Reverse sort order
  -l, --limit <N>               Limit number of results
```

Example:
```bash
# Show all open tickets sorted by update time
vide-ticket open

# Show high priority open tickets
vide-ticket open --sort priority -r
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

### Configuration Management

#### `config`
Manage project configuration settings.

```bash
vide-ticket config <SUBCOMMAND>

Subcommands:
  show                         Display current configuration
  set <KEY> <VALUE>           Set configuration value
  get <KEY>                   Get specific configuration value
  reset <KEY>                 Reset to default value
  claude [OPTIONS]            Generate or update CLAUDE.md

Examples:
  vide-ticket config show
  vide-ticket config set git.auto_branch true
  vide-ticket config get project.default_priority
  vide-ticket config claude --template advanced
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
project:
  name: "My Project"
  description: "Project description"
  default_assignee: null
  default_priority: "medium"

git:
  enabled: true
  auto_branch: true
  branch_prefix: "ticket/"
  remote: "origin"

ui:
  theme: "auto"
  emoji: true
  page_size: 20
  date_format: "%Y-%m-%d %H:%M"

archive:
  auto_archive: false
  archive_after_days: 30

export:
  default_format: "json"
  include_archived: false
```

### Configuration Keys

- `project.name`: Project name
- `project.description`: Project description
- `project.default_assignee`: Default assignee for new tickets
- `project.default_priority`: Default priority (low, medium, high, critical)
- `git.enabled`: Enable Git integration
- `git.auto_branch`: Automatically create branches when starting tickets
- `git.branch_prefix`: Prefix for Git branches
- `ui.emoji`: Enable emoji in output
- `ui.page_size`: Number of items per page in lists
- `archive.auto_archive`: Automatically archive completed tickets
- `archive.archive_after_days`: Days before auto-archiving

## File Structure

```
.vide-ticket/
├── config.yaml          # Project configuration
├── state.yaml          # Project state and metadata
├── active_ticket       # Currently active ticket ID
├── tickets/            # Ticket YAML files
│   ├── <ticket-id>.yaml
│   └── ...
├── specs/              # Specification files
│   ├── <spec-id>.yaml
│   └── ...
├── templates/          # Custom ticket templates
│   ├── bug.yaml
│   └── feature.yaml
├── plugins/            # Plugin extensions
└── backups/            # Backup files
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

## Claude Code Integration

vide-ticket includes built-in support for [Claude Code](https://claude.ai/code) through automatic CLAUDE.md generation. This feature enhances AI-assisted development by providing project context to Claude.

### Overview

CLAUDE.md files help Claude Code understand your project structure, commands, and workflows. vide-ticket can automatically generate and maintain these files with project-specific information.

### Generating CLAUDE.md

#### During Project Initialization

The simplest way to get started is during project initialization:

```bash
# Initialize project with CLAUDE.md
vide-ticket init --claude-md

# Alternative syntax
vide-ticket init --claude
```

This creates a comprehensive CLAUDE.md file that includes:
- Project name and description
- Common vide-ticket commands with examples
- Current project configuration
- Workflow guidelines
- Best practices for ticket management
- The initialization command itself (for reproducibility)

#### For Existing Projects

Add CLAUDE.md to existing projects using the config command:

```bash
# Generate basic CLAUDE.md
vide-ticket config claude

# Generate with advanced template
vide-ticket config claude --template advanced

# Append to existing CLAUDE.md (preserves custom content)
vide-ticket config claude --append

# Custom output location
vide-ticket config claude --output ./docs/CLAUDE.md

# Combine options
vide-ticket config claude --template advanced --append --output ./CLAUDE.md
```

### Template Options

#### Basic Template (default)

The basic template includes:
- **Project Overview**: Name, description, and purpose
- **Essential Commands**: Common vide-ticket operations with examples
- **Configuration**: Current project settings (Git integration, default priority, etc.)
- **Statistics**: Real-time ticket counts (total, active, completed)
- **Workflow Guidelines**: Standard ticket management practices
- **Best Practices**: Naming conventions and organizational tips

#### Advanced Template

The advanced template includes everything from basic plus:
- **Git Worktree Support**: Examples for parallel development
- **Advanced Search**: Complex filtering and regex patterns
- **Export/Import**: Data migration and backup commands
- **Environment Variables**: `VIDE_TICKET_PROJECT`, `VIDE_TICKET_NO_COLOR`, etc.
- **Git Hooks Integration**: Pre-commit and post-checkout examples
- **Troubleshooting**: Common issues and solutions
- **Performance Tips**: Optimization strategies

### Dynamic Content

Generated CLAUDE.md files include dynamically populated information:
- Current date of generation
- Active project configuration values
- Real-time ticket statistics
- Git integration status
- Project-specific settings

### Workflow Examples

#### New Project with AI Assistance

```bash
# Create and initialize project
mkdir my-project && cd my-project
vide-ticket init --claude-md

# Open with Claude Code
claude .

# Claude now understands your project structure
```

#### Adding to Existing Project

```bash
# Generate initial CLAUDE.md
vide-ticket config claude

# Later, after configuration changes
vide-ticket config set git.auto_branch true
vide-ticket config claude --append  # Updates CLAUDE.md
```

#### Team Onboarding

```bash
# Generate comprehensive documentation
vide-ticket config claude --template advanced

# Add team-specific instructions
echo "## Team Conventions" >> CLAUDE.md
echo "- Use 'feat/' prefix for feature branches" >> CLAUDE.md
```

### Claude Code Benefits

With a properly configured CLAUDE.md, Claude Code can:

1. **Understand Project Context**
   - Knows available commands and their usage
   - Understands project structure and conventions
   - Aware of current configuration and settings

2. **Provide Better Assistance**
   - Suggests appropriate vide-ticket commands
   - Follows established workflows
   - Respects project-specific conventions

3. **Automate Common Tasks**
   - Generate tickets from error logs
   - Create task breakdowns
   - Suggest implementation approaches

4. **Maintain Consistency**
   - Uses correct naming conventions
   - Follows team practices
   - Applies project standards

### Best Practices

1. **Initial Setup**: Always use `--claude-md` during initialization for new projects
2. **Regular Updates**: Run `config claude --append` after major configuration changes
3. **Customization**: Add project-specific sections after generation
4. **Version Control**: Commit CLAUDE.md to track project evolution
5. **Team Alignment**: Review and update CLAUDE.md during team meetings

### Advanced Usage

#### Custom Sections

After generation, add custom sections for your team:

```markdown
## Custom Commands
- `make deploy`: Deploy to production
- `npm run e2e`: Run end-to-end tests

## Architecture Decisions
- We use feature flags for gradual rollouts
- All API endpoints require authentication
```

#### Integration with CI/CD

```yaml
# .github/workflows/update-claude.yml
on:
  push:
    paths:
      - '.vide-ticket/config.yaml'
jobs:
  update-claude:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: vide-ticket config claude --append
      - uses: EndBug/add-and-commit@v9
        with:
          message: 'chore: update CLAUDE.md'
```

## License

[Your License Here]

## Contributing

[Your Contributing Guidelines Here]
