# vibe-ticket Command Reference

## Concurrent Access Safety

All vibe-ticket commands are safe to use concurrently. Multiple users or processes can safely:
- Create, edit, and close tickets simultaneously
- Work on different tickets without conflicts
- Access the same ticket with automatic retry and locking

The system uses file-based locking with automatic cleanup of stale locks (after 30 seconds).

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

## Specification Management

### `spec`
Manage specifications for spec-driven development workflow.

```bash
vibe-ticket spec <SUBCOMMAND>

Subcommands:
  init                         Initialize a new specification
  requirements                 Create or update requirements document
  design                      Create or update design document
  tasks                       Create or update implementation tasks
  status                      Show specification status
  list                        List all specifications
  show                        Show specification details
  delete                      Delete a specification
  approve                     Approve a specification phase
  activate                    Set active specification
```

#### `spec init`
Create a new specification with metadata tracking.

```bash
vibe-ticket spec init <TITLE> [OPTIONS]

Arguments:
  <TITLE>                       Specification title

Options:
  -d, --description <DESC>      Brief description
  -t, --ticket <TICKET>         Associated ticket ID/slug
  --tags <TAGS>                 Comma-separated tags
```

#### `spec requirements`
Create or edit requirements definition document.

```bash
vibe-ticket spec requirements [OPTIONS]

Options:
  -s, --spec <SPEC_ID>          Specification ID (defaults to active)
  -e, --editor                  Open in editor
  -c, --complete                Mark phase as complete
```

#### `spec design`
Create or edit technical design document.

```bash
vibe-ticket spec design [OPTIONS]

Options:
  -s, --spec <SPEC_ID>          Specification ID (defaults to active)
  -e, --editor                  Open in editor
  -c, --complete                Mark phase as complete
```

#### `spec tasks`
Create or edit implementation plan/tasks document.

```bash
vibe-ticket spec tasks [OPTIONS]

Options:
  -s, --spec <SPEC_ID>          Specification ID (defaults to active)
  -e, --editor                  Open in editor
  -c, --complete                Mark phase as complete
  --export-tickets              Export tasks as vibe-tickets
```

#### `spec status`
Show current specification progress and phase.

```bash
vibe-ticket spec status [OPTIONS]

Options:
  -s, --spec <SPEC_ID>          Specification ID (defaults to active)
  -d, --detailed                Show detailed information
```

#### `spec list`
List all specifications with filtering options.

```bash
vibe-ticket spec list [OPTIONS]

Options:
  -s, --status <STATUS>         Filter by status
  -p, --phase <PHASE>           Filter by phase (initial, requirements, design, tasks, completed)
  --archived                    Show archived specifications
```

#### `spec show`
Display specification details and documents.

```bash
vibe-ticket spec show <SPEC_ID> [OPTIONS]

Arguments:
  <SPEC_ID>                     Specification ID

Options:
  -a, --all                     Show all documents
  -m, --markdown                Output in markdown format
```

#### `spec delete`
Delete a specification and all associated documents.

```bash
vibe-ticket spec delete <SPEC_ID> [OPTIONS]

Arguments:
  <SPEC_ID>                     Specification ID

Options:
  -f, --force                   Skip confirmation prompt
```

#### `spec approve`
Approve a specification phase for progression.

```bash
vibe-ticket spec approve <SPEC_ID> <PHASE> [OPTIONS]

Arguments:
  <SPEC_ID>                     Specification ID
  <PHASE>                       Phase to approve (requirements, design, tasks)

Options:
  -m, --message <MSG>           Approval message
```

#### `spec activate`
Set the active specification for default operations.

```bash
vibe-ticket spec activate <SPEC_ID>

Arguments:
  <SPEC_ID>                     Specification ID to activate
```

### Example Workflow

```bash
# 1. Initialize a new specification
vibe-ticket spec init "User Authentication System" \
  --description "Implement OAuth2-based authentication" \
  --tags "security,backend"

# 2. Activate the specification
vibe-ticket spec activate 9cc43ac7-d9ff-4628-8509-b329b5c61784

# 3. Create requirements document
vibe-ticket spec requirements --editor

# 4. Mark requirements as complete
vibe-ticket spec requirements --complete

# 5. Create technical design
vibe-ticket spec design --editor

# 6. Create implementation tasks
vibe-ticket spec tasks --editor

# 7. Check overall status
vibe-ticket spec status --detailed

# 8. Approve phases
vibe-ticket spec approve 9cc43ac7-d9ff-4628-8509-b329b5c61784 requirements \
  --message "Requirements reviewed and approved"
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