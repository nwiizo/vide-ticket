# MCP (Model Context Protocol) Guide for vibe-ticket

This guide provides comprehensive instructions for using vibe-ticket through MCP, enabling AI assistants to interact with your ticket management system programmatically.

## Table of Contents

- [Overview](#overview)
- [Setup](#setup)
- [Basic Operations](#basic-operations)
- [Advanced Workflows](#advanced-workflows)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

MCP (Model Context Protocol) is a standardized interface that allows AI assistants like Claude to interact with external tools. vibe-ticket's MCP integration provides full access to ticket management capabilities through a structured API.

### Key Benefits

- **Automated Ticket Management**: AI can create, update, and close tickets based on context
- **Task Tracking**: Automatic task creation and completion tracking
- **Intelligent Search**: AI can search and analyze ticket patterns
- **Workflow Automation**: Complex multi-step operations can be automated

## Setup

### Building with MCP Support

```bash
# Install from source with MCP features
git clone https://github.com/nwiizo/vibe-ticket
cd vibe-ticket
cargo install --path . --features mcp
```

### Adding to Claude Desktop

```bash
# If installed globally
claude mcp add vibe-ticket vibe-ticket --scope local -- mcp serve

# If using cargo run
claude mcp add vibe-ticket cargo --scope local -- run --features mcp -- mcp serve
```

### Verification

After setup, verify MCP is working:

```bash
# In Claude Desktop, ask:
"Show me the current vibe-ticket status"
```

## Basic Operations

### Ticket Management

#### Creating Tickets

```typescript
// MCP tool call example
mcp__vibe-ticket__vibe-ticket_new({
  slug: "fix-login-bug",
  title: "Fix authentication issue in login flow",
  description: "Users report being unable to login with valid credentials",
  priority: "high",
  tags: ["bug", "authentication", "urgent"]
})
```

**Example Claude Interaction:**
```
User: Create a ticket for the login bug we discussed
Claude: I'll create a ticket for the login authentication issue.

[Creates ticket with appropriate details based on context]

Created ticket "202507312345-fix-login-bug" with high priority.
```

#### Updating Ticket Status

```typescript
// Update ticket status
mcp__vibe-ticket__vibe-ticket_edit({
  ticket: "fix-login-bug",
  status: "doing"
})

// Add assignee
mcp__vibe-ticket__vibe-ticket_edit({
  ticket: "fix-login-bug",
  assignee: "john.doe"
})
```

#### Closing Tickets

```typescript
// Close with completion message
mcp__vibe-ticket__vibe-ticket_close({
  ticket: "fix-login-bug",
  message: "Fixed null check in authentication flow. All tests passing."
})
```

### Task Management

#### Adding Tasks

```typescript
// Add task to active ticket
mcp__vibe-ticket__vibe-ticket_task_add({
  title: "Add null check for user credentials"
})

// Add task to specific ticket
mcp__vibe-ticket__vibe-ticket_task_add({
  ticket: "fix-login-bug",
  title: "Write unit tests for auth flow"
})
```

#### Completing Tasks

```typescript
// Complete task by ID
mcp__vibe-ticket__vibe-ticket_task_complete({
  task_id: "1",
  ticket: "fix-login-bug"
})
```

#### Listing Tasks

```typescript
// List all tasks for a ticket
mcp__vibe-ticket__vibe-ticket_task_list({
  ticket: "fix-login-bug"
})

// List only incomplete tasks
mcp__vibe-ticket__vibe-ticket_task_list({
  ticket: "fix-login-bug",
  incomplete_only: true
})
```

### Search and Discovery

#### Finding Tickets

```typescript
// Search by keyword
mcp__vibe-ticket__vibe-ticket_search({
  query: "authentication",
  in_title: true
})

// List tickets by status
mcp__vibe-ticket__vibe-ticket_list({
  status: "doing",
  priority: "high"
})

// Show open tickets only
mcp__vibe-ticket__vibe-ticket_list({
  open: true,
  assignee: "john.doe"
})
```

## Advanced Workflows

### Automated Ticket Lifecycle

**Scenario**: Automatically manage ticket lifecycle based on code changes

```typescript
// 1. Create feature ticket
const ticket = await mcp__vibe-ticket__vibe-ticket_new({
  slug: "add-user-profile",
  title: "Implement user profile page",
  priority: "medium"
})

// 2. Start work (creates worktree)
await mcp__vibe-ticket__vibe-ticket_start({
  ticket: ticket.id
})

// 3. Add implementation tasks
const tasks = [
  "Design profile layout",
  "Create profile component",
  "Add API endpoints",
  "Write tests"
]

for (const task of tasks) {
  await mcp__vibe-ticket__vibe-ticket_task_add({
    ticket: ticket.id,
    title: task
  })
}

// 4. Complete tasks as work progresses
await mcp__vibe-ticket__vibe-ticket_task_complete({
  ticket: ticket.id,
  task_id: "1"
})

// 5. Close ticket when done
await mcp__vibe-ticket__vibe-ticket_close({
  ticket: ticket.id,
  message: "User profile feature completed with tests"
})
```

### Bulk Operations

**Scenario**: Clean up completed tickets

```typescript
// Find all tickets with completed tasks
const tickets = await mcp__vibe-ticket__vibe-ticket_list({
  status: "doing"
})

for (const ticket of tickets.tickets) {
  const tasks = await mcp__vibe-ticket__vibe-ticket_task_list({
    ticket: ticket.id
  })
  
  // Check if all tasks completed
  const allCompleted = tasks.tasks.every(t => t.completed)
  
  if (allCompleted && tasks.tasks.length > 0) {
    await mcp__vibe-ticket__vibe-ticket_close({
      ticket: ticket.id,
      message: "All tasks completed - auto-closing"
    })
  }
}
```

### Specification-Driven Development

```typescript
// Create specification
await mcp__vibe-ticket__vibe-ticket_spec_add({
  ticket: "implement-oauth",
  spec_type: "requirements",
  content: {
    title: "OAuth2 Implementation",
    requirements: [
      "Support Google OAuth",
      "Support GitHub OAuth",
      "Implement token refresh"
    ]
  }
})

// Add design specification
await mcp__vibe-ticket__vibe-ticket_spec_add({
  ticket: "implement-oauth",
  spec_type: "design",
  content: {
    architecture: "JWT-based token storage",
    endpoints: ["/auth/google", "/auth/github", "/auth/refresh"]
  }
})

// Check specification status
const status = await mcp__vibe-ticket__vibe-ticket_spec_check({
  ticket: "implement-oauth"
})
```

## Best Practices

### 1. Consistent Naming

Use descriptive, consistent ticket slugs:
- ✅ `fix-login-null-pointer`
- ✅ `add-user-profile-page`
- ❌ `bug1`
- ❌ `feature`

### 2. Task Granularity

Break work into atomic, testable tasks:
- ✅ "Add null check for user.email"
- ✅ "Write unit test for login validation"
- ❌ "Fix everything"
- ❌ "Do the implementation"

### 3. Status Management

Keep ticket status current:
```typescript
// When starting work
await mcp__vibe-ticket__vibe-ticket_start({ ticket: id })

// When blocked
await mcp__vibe-ticket__vibe-ticket_edit({ 
  ticket: id, 
  status: "blocked" 
})

// When ready for review
await mcp__vibe-ticket__vibe-ticket_edit({ 
  ticket: id, 
  status: "review" 
})
```

### 4. Meaningful Descriptions

Provide context in ticket descriptions:
```typescript
await mcp__vibe-ticket__vibe-ticket_new({
  slug: "optimize-search",
  title: "Optimize search performance",
  description: `
    Current search takes 2-3 seconds for large datasets.
    Users report timeouts when searching > 10k records.
    
    Goal: Reduce search time to < 500ms
    Approach: Add database indexes and implement caching
  `,
  priority: "high"
})
```

### 5. Tag Usage

Use tags for organization and filtering:
- `bug`, `feature`, `enhancement`
- `frontend`, `backend`, `database`
- `urgent`, `blocked`, `needs-review`

## Troubleshooting

### Common Issues

#### MCP Connection Failed

```bash
# Check if MCP server is running
ps aux | grep "vibe-ticket mcp"

# Restart MCP server
claude mcp restart vibe-ticket
```

#### Permission Denied

```bash
# Ensure vibe-ticket has correct permissions
chmod +x $(which vibe-ticket)

# Check project directory permissions
ls -la .vibe-ticket/
```

#### Ticket Not Found

```typescript
// Use full ID or unique slug prefix
mcp__vibe-ticket__vibe-ticket_show({
  ticket: "202507312345-fix-login"  // Full slug
})

// Or use ID
mcp__vibe-ticket__vibe-ticket_show({
  ticket: "84c3d1ed-f87c-435a-aa33-92e9a2e74a64"
})
```

### Debug Mode

Enable verbose logging for troubleshooting:

```bash
# Set environment variable
export VIBE_TICKET_LOG=debug

# Run with debug output
vibe-ticket mcp serve --verbose
```

## Integration Examples

### GitHub Actions Integration

```yaml
name: Ticket Management
on:
  pull_request:
    types: [opened, closed]

jobs:
  manage-ticket:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Update ticket status
        run: |
          if [ "${{ github.event.action }}" = "opened" ]; then
            vibe-ticket edit ${{ github.event.pull_request.head.ref }} --status review
          elif [ "${{ github.event.action }}" = "closed" ]; then
            vibe-ticket close ${{ github.event.pull_request.head.ref }} \
              --message "PR merged: ${{ github.event.pull_request.title }}"
          fi
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Check if there's an active ticket
active_ticket=$(vibe-ticket check --json | jq -r '.active_ticket.slug')

if [ -z "$active_ticket" ]; then
  echo "Error: No active ticket. Please start a ticket before committing."
  echo "Run: vibe-ticket start <ticket-id>"
  exit 1
fi

# Add ticket ID to commit message
echo "\n\nTicket: $active_ticket" >> $1
```

## Using MCP for Workflow Improvements

### Suggesting MCP Tools to AI Assistants

When you notice opportunities for automation or improvement, actively suggest that your AI assistant use vibe-ticket MCP tools:

**Common scenarios for MCP tool suggestions:**

1. **Repetitive ticket creation**
   ```
   "I need to create similar tickets for each microservice. Can you use MCP to automate this?"
   ```

2. **Bulk task management**
   ```
   "Use MCP to add these test tasks to all open bug tickets"
   ```

3. **Regular status reports**
   ```
   "Generate a weekly report of completed tickets using MCP search and export"
   ```

4. **Workflow automation**
   ```
   "When I mention a bug in our conversation, automatically create a ticket for it"
   ```

### Best Practices for AI Collaboration

1. **Be explicit about MCP usage** - Tell your AI when you want it to use MCP tools
2. **Describe patterns** - Help AI identify when to proactively use vibe-ticket
3. **Request automation** - Ask AI to create scripts or workflows using MCP tools
4. **Provide feedback** - Let AI know which automations are helpful

## MCP Tool Reference

### Available Tools

- `vibe-ticket_new` - Create new ticket
- `vibe-ticket_list` - List tickets with filters
- `vibe-ticket_show` - Show ticket details
- `vibe-ticket_edit` - Edit ticket properties
- `vibe-ticket_close` - Close ticket
- `vibe-ticket_start` - Start working on ticket
- `vibe-ticket_check` - Check current status
- `vibe-ticket_task_add` - Add task to ticket
- `vibe-ticket_task_complete` - Complete task
- `vibe-ticket_task_list` - List tasks
- `vibe-ticket_task_remove` - Remove task
- `vibe-ticket_search` - Search tickets
- `vibe-ticket_export` - Export tickets
- `vibe-ticket_import` - Import tickets
- `vibe-ticket_config_show` - Show configuration
- `vibe-ticket_config_set` - Set configuration
- `vibe-ticket_spec_add` - Add specification
- `vibe-ticket_spec_update` - Update specification
- `vibe-ticket_spec_check` - Check specification status
- `vibe-ticket_worktree_list` - List worktrees
- `vibe-ticket_worktree_remove` - Remove worktree
- `vibe-ticket_worktree_prune` - Clean up worktrees

For detailed parameter information, use the tool discovery feature in your MCP client.

---

## Next Steps

1. Review the [Command Reference](./commands.md) for CLI usage
2. Configure your project with [Configuration Guide](./configuration.md)
3. Learn about [Git Worktree Integration](./git-worktree.md)
4. Explore [Spec-Driven Development](./spec-driven-development.md)

For additional help, create a ticket:
```
User: Create a ticket for improving MCP documentation
Claude: I'll create that documentation improvement ticket for you...
```