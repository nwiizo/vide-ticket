# Configuration

Project configuration is stored in `.vibe-ticket/config.yaml`:

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
  worktree_enabled: true
  worktree_default: true
  worktree_prefix: "./{project}-vibeticket-"
  worktree_cleanup_on_close: false

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

## Configuration Keys

- `project.name`: Project name
- `project.description`: Project description
- `project.default_assignee`: Default assignee for new tickets
- `project.default_priority`: Default priority (low, medium, high, critical)
- `git.enabled`: Enable Git integration
- `git.auto_branch`: Automatically create branches when starting tickets
- `git.branch_prefix`: Prefix for Git branches
- `git.worktree_enabled`: Enable Git worktree integration
- `git.worktree_default`: Use worktree by default when starting tickets
- `git.worktree_prefix`: Worktree directory naming pattern (use {project} placeholder)
- `git.worktree_cleanup_on_close`: Automatically remove worktree when closing ticket
- `ui.emoji`: Enable emoji in output
- `ui.page_size`: Number of items per page in lists
- `archive.auto_archive`: Automatically archive completed tickets
- `archive.archive_after_days`: Days before auto-archiving