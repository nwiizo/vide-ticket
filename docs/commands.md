# vibe-ticket Command Reference

## Project Management

### `init`
Initialize a new vibe-ticket project in the current directory.

```bash
vibe-ticket init [OPTIONS]

Options:
  -n, --name <NAME>              Project name
  -d, --description <DESC>       Project description
  -f, --force                    Force initialization even if directory is not empty
```

## Ticket Operations

### `new`
Create a new ticket with automatic timestamp prefix.

```bash
vibe-ticket new <SLUG> [OPTIONS]

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
vibe-ticket new "user-auth" --title "Implement user authentication" --priority high --tags "backend,security"
# Creates: 202507201345-user-auth
```

### `list`
List tickets with various filtering options.

```bash
vibe-ticket list [OPTIONS]

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

### `start`
Start working on a ticket (sets status to "In Progress" and creates Git worktree by default).

```bash
vibe-ticket start [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -b, --branch                  Create Git branch (default: true)
  --branch-name <NAME>          Custom branch name (default: ticket slug)
  --worktree                    Create Git worktree (default: true)
  --no-worktree                 Disable worktree creation (only create branch)
```

### `close`
Complete a ticket and optionally archive it.

```bash
vibe-ticket close [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -m, --message <MESSAGE>       Closing message
  -a, --archive                 Archive the ticket
  --pr                          Create pull request (requires gh CLI)
```

### `edit`
Edit ticket properties.

```bash
vibe-ticket edit [TICKET] [OPTIONS]

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

### `open`
Show all open tickets (alias for `list --open`). This is a quick way to see tickets that need attention.

```bash
vibe-ticket open [OPTIONS]

Options:
  --sort <FIELD>                Sort by field [created, updated, priority, status, slug]
  -r, --reverse                 Reverse sort order
  -l, --limit <N>               Limit number of results
```

Example:
```bash
# Show all open tickets sorted by update time
vibe-ticket open

# Show high priority open tickets
vibe-ticket open --sort priority -r
```

### `show`
Display detailed information about a ticket.

```bash
vibe-ticket show [TICKET] [OPTIONS]

Arguments:
  [TICKET]                      Ticket ID or slug (defaults to active ticket)

Options:
  -t, --tasks                   Show task details
  -h, --history                 Show status history
  -m, --markdown                Output as markdown
```

## Task Management

### `task add`
Add a task to a ticket.

```bash
vibe-ticket task add <TITLE> [OPTIONS]

Arguments:
  <TITLE>                       Task title

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
```

### `task complete`
Mark a task as completed.

```bash
vibe-ticket task complete <TASK> [OPTIONS]

Arguments:
  <TASK>                        Task index (1-based)

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
```

### `task list`
List tasks for a ticket.

```bash
vibe-ticket task list [OPTIONS]

Options:
  -t, --ticket <TICKET>         Target ticket (defaults to active)
  -c, --completed               Show only completed tasks
  -i, --incomplete              Show only incomplete tasks
```

## Search and Filter

### `search`
Search tickets by content.

```bash
vibe-ticket search <QUERY> [OPTIONS]

Arguments:
  <QUERY>                       Search query

Options:
  -t, --title                   Search in titles only
  -d, --description             Search in descriptions only
  --tags                        Search in tags only
  -r, --regex                   Use regex matching
```

## Data Management

### `archive`
Archive or unarchive tickets.

```bash
vibe-ticket archive <TICKET> [OPTIONS]

Arguments:
  <TICKET>                      Ticket ID or slug

Options:
  -u, --unarchive              Unarchive the ticket
```

### `export`
Export tickets to various formats.

```bash
vibe-ticket export <FORMAT> [OPTIONS]

Arguments:
  <FORMAT>                      Export format [json, yaml, csv, markdown]

Options:
  -o, --output <FILE>          Output file (defaults to stdout)
  --include-archived           Include archived tickets
```

### `import`
Import tickets from files.

```bash
vibe-ticket import <FILE> [OPTIONS]

Arguments:
  <FILE>                       Import file path

Options:
  -f, --format <FORMAT>        File format (auto-detected if not specified)
  --skip-validation            Skip validation checks
  --dry-run                    Preview without importing
```

## Configuration Management

### `config`
Manage project configuration settings.

```bash
vibe-ticket config <SUBCOMMAND>

Subcommands:
  show                         Display current configuration
  set <KEY> <VALUE>           Set configuration value
  get <KEY>                   Get specific configuration value
  reset <KEY>                 Reset to default value
  claude [OPTIONS]            Generate or update CLAUDE.md

Examples:
  vibe-ticket config show
  vibe-ticket config set git.auto_branch true
  vibe-ticket config get project.default_priority
  vibe-ticket config claude --template advanced
```

## Git Worktree Commands

### `worktree`
Manage Git worktrees for tickets.

```bash
vibe-ticket worktree <SUBCOMMAND>

Subcommands:
  list                         List all ticket worktrees
  remove <worktree>           Remove a specific worktree
  prune                       Clean up stale worktrees

# List worktrees
vibe-ticket worktree list [OPTIONS]
Options:
  -a, --all                   Show all worktrees (not just ticket ones)
  -s, --status <STATUS>       Filter by status (active, stale, orphaned)
  -v, --verbose               Show detailed information

# Remove worktree
vibe-ticket worktree remove <WORKTREE> [OPTIONS]
Arguments:
  <WORKTREE>                  Worktree path or ticket ID/slug
Options:
  -f, --force                 Force removal even with uncommitted changes
  --keep-branch               Keep the associated branch

# Prune worktrees
vibe-ticket worktree prune [OPTIONS]
Options:
  -f, --force                 Remove without confirmation
  -d, --dry-run               Show what would be removed
  --remove-branches           Also remove associated branches
```

## Utility Commands

### `check`
Check project status and active ticket.

```bash
vibe-ticket check [OPTIONS]

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