# vibe-ticket Project: vibe-ticket

A vibe-ticket managed project

## Overview

This project uses vibe-ticket for ticket management. This document provides guidance for Claude Code when working with this codebase.

## Common vibe-ticket Commands

### Getting Started
```bash
# Create your first ticket
vibe-ticket new fix-bug --title "Fix login issue" --priority high

# List all tickets
vibe-ticket list

# Start working on a ticket (creates worktree by default)
vibe-ticket start fix-bug
# This creates: ../vibe-ticket-ticket-fix-bug/

# Start without worktree (branch only)
vibe-ticket start fix-bug --no-worktree

# Show current status
vibe-ticket check
```

### Working with Tickets
```bash
# Show ticket details
vibe-ticket show <ticket>

# Update ticket
vibe-ticket edit <ticket> --status review

# Add tasks to ticket
vibe-ticket task add "Write unit tests"
vibe-ticket task add "Update documentation"

# Complete tasks
vibe-ticket task complete 1

# Close ticket
vibe-ticket close <ticket> --message "Fixed the login issue"
```

### Search and Filter
```bash
# Search tickets
vibe-ticket search "login"

# Filter by status
vibe-ticket list --status doing

# Filter by priority
vibe-ticket list --priority high
```

### Git Worktree Management
```bash
# List all worktrees for tickets
vibe-ticket worktree list

# List all worktrees (including non-ticket ones)
vibe-ticket worktree list --all

# Remove a worktree
vibe-ticket worktree remove fix-bug

# Prune stale worktrees
vibe-ticket worktree prune
```

### Configuration
```bash
# View configuration
vibe-ticket config show

# Set configuration values
vibe-ticket config set project.default_priority medium
vibe-ticket config set git.auto_branch true
vibe-ticket config set git.worktree_default false  # Disable default worktree creation

# Generate this file
vibe-ticket config claude
```

## Project Configuration

The project has been initialized with default settings. You can customize them using the config commands above.

### Git Worktree Configuration
```yaml
git:
  worktree_enabled: true              # Enable worktree support
  worktree_default: true              # Create worktree by default when starting tickets
  worktree_prefix: "../{project}-ticket-"  # Directory naming pattern
  worktree_cleanup_on_close: false   # Auto-remove worktree when closing ticket
```

## Workflow Guidelines

1. Create a ticket before starting any work
2. Use descriptive ticket slugs (e.g., fix-login-bug, add-search-feature)
3. When starting a ticket, a Git worktree is created automatically
   - Work in the worktree directory: `../vibe-ticket-ticket-<slug>/`
   - Each ticket has its own isolated working directory
4. Break down complex work into tasks within tickets
5. Keep ticket status updated as work progresses
6. Close tickets with meaningful completion messages
7. Remove worktrees when done: `vibe-ticket worktree remove <ticket>`

## Best Practices for This Project

- Follow the established ticket naming conventions
- Use appropriate priority levels (low, medium, high, critical)
- Tag tickets for better organization
- Document decisions in ticket descriptions
- Link related tickets when applicable

## Tips for Claude Code

When helping with this project:
1. Always check for active tickets before suggesting new work
2. Reference ticket IDs in commit messages
3. Update ticket status as implementation progresses
4. Use `vibe-ticket check` to understand current context
5. Generate new tickets for bugs or features discovered during development
6. **IMPORTANT**: After completing each major task or work session, provide a retrospective that includes:
   - What was accomplished
   - What challenges were encountered
   - What could be improved for next time
   - Any vibe-tickets that should be created for follow-up work
   - Lessons learned that could benefit future development

## Work Retrospectives

### Why Retrospectives Matter
Retrospectives help improve the development process by:
- Identifying recurring issues before they become major problems
- Documenting solutions for future reference
- Creating actionable tickets for improvements
- Building institutional knowledge

### When to Conduct Retrospectives
- After completing any release preparation
- When finishing a complex feature implementation
- After resolving critical bugs
- At the end of each work session involving multiple tasks
- When encountering unexpected challenges

### Retrospective Template
```markdown
## Retrospective: [Task/Feature Name] - [Date]

### Summary
Brief overview of what was worked on.

### What Went Well
- List successes and smooth processes
- Note effective tools or techniques used

### Challenges Encountered
- Document specific problems faced
- Include error messages or unexpected behaviors

### Improvements for Next Time
- Concrete suggestions for process improvements
- Tools or automation that could help

### Follow-up Tickets Created
- List any vibe-tickets created as a result
- Include ticket IDs and brief descriptions

### Lessons Learned
- Key insights that will help future development
- Patterns to watch for or avoid
```

---
Generated on: 2025-07-22


## Project Initialization

This project was initialized with:
```bash
vibe-ticket init --claude-md
```

To regenerate or update this file:
```bash
# Regenerate with basic template
vibe-ticket config claude

# Append with advanced features
vibe-ticket config claude --template advanced --append
```
