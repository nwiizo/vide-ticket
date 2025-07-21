# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the Alias Test project.

## Project Overview

A vibe-ticket managed project

Generated during project initialization on: 2025-07-21

## vibe-ticket Commands

### Getting Started

```bash
# Check current project status
vibe-ticket check

# List all tickets
vibe-ticket list

# Create a new ticket
vibe-ticket new <slug> --title "Title" --description "Description"

# Start working on a ticket (creates Git branch if enabled)
vibe-ticket start <ticket-id>

# Close a ticket when done
vibe-ticket close <ticket-id>
```

### Searching and Filtering

```bash
# Search tickets by keyword
vibe-ticket search "keyword"

# List tickets by status
vibe-ticket list --status doing

# List high priority tickets
vibe-ticket list --priority high
```

### Configuration

```bash
# View current configuration
vibe-ticket config show

# Set your name as default assignee
vibe-ticket config set project.default_assignee "your-name"

# Enable Git integration
vibe-ticket config set git.enabled true
```

## Project Settings

- Project Name: Alias Test
- Default Priority: medium
- Git Integration: Enabled
- Branch Prefix: ticket/

## Workflow Guidelines

1. **Create descriptive tickets**: Use meaningful slugs like `fix-login-bug` or `add-user-auth`
2. **Start work properly**: Always use `vibe-ticket start` to track active work
3. **Update regularly**: Keep ticket status current as you progress
4. **Close when done**: Use `vibe-ticket close` to mark completion

## Best Practices

- Use tags to categorize related tickets
- Add tasks to break down complex tickets
- Archive old tickets to keep the list manageable
- Export tickets regularly for backup

## Tips for Claude Code

When working with this project:
- Always check current ticket status before making changes
- Create new tickets for significant features or bugs
- Update ticket status to reflect actual progress
- Use the built-in search to find related tickets
