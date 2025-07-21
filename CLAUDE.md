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

# Start working on a ticket
vibe-ticket start fix-bug

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

### Configuration
```bash
# View configuration
vibe-ticket config show

# Set configuration values
vibe-ticket config set project.default_priority medium
vibe-ticket config set git.auto_branch true

# Generate this file
vibe-ticket config claude
```

## Project Configuration

The project has been initialized with default settings. You can customize them using the config commands above.

## Workflow Guidelines

1. Create a ticket before starting any work
2. Use descriptive ticket slugs (e.g., fix-login-bug, add-search-feature)
3. Break down complex work into tasks within tickets
4. Keep ticket status updated as work progresses
5. Close tickets with meaningful completion messages

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
