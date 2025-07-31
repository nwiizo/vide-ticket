# MCP (Model Context Protocol) Integration

vibe-ticket supports the Model Context Protocol, allowing AI assistants and other MCP-compatible tools to interact with your ticket system.

## Installation and Setup

### Prerequisites

Ensure vibe-ticket is built with MCP support:
```bash
# Install with MCP feature
cargo install vibe-ticket --features mcp

# Or build from source
cargo build --release --features mcp
```

### Setting up with Claude Code

#### Option 1: Global Installation (Recommended)

After installing vibe-ticket via `cargo install`:

```bash
# The binary is located at ~/.cargo/bin/vibe-ticket
claude mcp add vibe-ticket ~/.cargo/bin/vibe-ticket --scope global -- mcp serve
```

**Installation paths by platform:**
- **macOS/Linux**: `~/.cargo/bin/vibe-ticket`
- **Windows**: `%USERPROFILE%\.cargo\bin\vibe-ticket.exe`

#### Option 2: Project-Specific Setup

For a specific project only:
```bash
cd /path/to/your/project
claude mcp add vibe-ticket cargo --scope project -- run --features mcp -- mcp serve
```

#### Option 3: From Source Code

If developing or customizing vibe-ticket:
```bash
git clone https://github.com/nwiizo/vibe-ticket.git
cd vibe-ticket
claude mcp add vibe-ticket cargo --scope project -- run --release --features mcp -- mcp serve
```

#### Option 4: Custom Configuration

With environment variables:
```bash
claude mcp add vibe-ticket ~/.cargo/bin/vibe-ticket --scope project \
  --env VIBE_TICKET_PROJECT=/path/to/target/project -- mcp serve
```

### Managing MCP Servers

```bash
# List all configured MCP servers
claude mcp list

# Verify vibe-ticket is configured
claude mcp list | grep vibe-ticket

# Remove vibe-ticket server
claude mcp remove vibe-ticket

# Test server manually
vibe-ticket mcp serve

# Run with custom options
vibe-ticket mcp serve --host 127.0.0.1 --port 8080 --daemon
```

## Available MCP Tools

### Ticket Operations

| Tool | Description | Required Arguments |
|------|-------------|-------------------|
| `vibe-ticket_new` | Create a new ticket | `slug`, `title` |
| `vibe-ticket_list` | List tickets with filters | - |
| `vibe-ticket_show` | Show ticket details | `ticket` |
| `vibe-ticket_edit` | Edit ticket properties | `ticket` |
| `vibe-ticket_close` | Close a ticket | `ticket` |
| `vibe-ticket_start` | Start working on a ticket | `ticket` |
| `vibe-ticket_check` | Check current status | - |

### Task Management

| Tool | Description | Required Arguments |
|------|-------------|-------------------|
| `vibe-ticket_task_add` | Add a task to a ticket | `title` |
| `vibe-ticket_task_complete` | Complete a task | `task` |
| `vibe-ticket_task_list` | List tasks for a ticket | - |
| `vibe-ticket_task_remove` | Remove a task | `task` |

### Advanced Features

| Tool | Description | Required Arguments |
|------|-------------|-------------------|
| `vibe-ticket_worktree_list` | List Git worktrees | - |
| `vibe-ticket_worktree_remove` | Remove a worktree | `worktree` |
| `vibe-ticket_worktree_prune` | Prune stale worktrees | - |
| `vibe-ticket_search` | Search tickets | `query` |
| `vibe-ticket_export` | Export tickets | `format` |
| `vibe-ticket_import` | Import tickets | `file` |
| `vibe-ticket_config_show` | Show configuration | - |
| `vibe-ticket_config_set` | Set configuration | `key`, `value` |

## Usage Examples

### Creating a Ticket

```json
{
  "tool": "vibe-ticket_new",
  "arguments": {
    "slug": "fix-login-bug",
    "title": "Fix login authentication issue",
    "description": "Users cannot log in with valid credentials",
    "priority": "high",
    "tags": ["bug", "auth", "urgent"]
  }
}
```

### Listing Active Tickets

```json
{
  "tool": "vibe-ticket_list",
  "arguments": {
    "status": "doing",
    "open": true
  }
}
```

### Working with Tasks

```json
// Add a task to the current ticket
{
  "tool": "vibe-ticket_task_add",
  "arguments": {
    "title": "Write unit tests for auth module"
  }
}

// Complete a task
{
  "tool": "vibe-ticket_task_complete",
  "arguments": {
    "task": "1"  // Task ID or index
  }
}
```

### Managing Worktrees

```json
// List all worktrees
{
  "tool": "vibe-ticket_worktree_list",
  "arguments": {
    "all": true
  }
}

// Start work with worktree creation
{
  "tool": "vibe-ticket_start",
  "arguments": {
    "ticket": "fix-login-bug",
    "no_worktree": false  // Create worktree (default)
  }
}
```

## Integration Architecture

### Storage Synchronization

MCP and CLI operations share the same storage layer:
- Changes made via CLI are immediately visible to MCP
- Changes made via MCP are immediately visible to CLI
- No synchronization or polling required

### File Structure

```
.vibe-ticket/
├── config.yaml         # Shared configuration
├── tickets/           # Ticket YAML files
│   ├── <uuid>.yaml
│   └── ...
├── specs/             # Specification files
└── active_ticket      # Currently active ticket
```

### Error Handling

MCP tools return structured errors:
```json
{
  "error": "Ticket not found: fix-login-bug"
}
```

Common error scenarios:
- Invalid ticket ID or slug
- Missing required fields
- Permission issues
- Git worktree conflicts

## Best Practices

### For AI Assistants

1. **Ticket References**: Use either ticket ID (UUID) or slug
2. **Status Values**: `todo`, `doing`, `done`, `blocked`, `review`
3. **Priority Values**: `low`, `medium`, `high`, `critical`
4. **Tags**: Arrays of strings for categorization
5. **Dates**: ISO 8601 format for all timestamps

### For Developers

1. **Consistent Naming**: Use descriptive slugs (e.g., `fix-login-bug`)
2. **Task Breakdown**: Create granular tasks for better tracking
3. **Regular Updates**: Keep ticket status current
4. **Worktree Cleanup**: Remove worktrees after closing tickets

## Troubleshooting

### Common Issues

**MCP server not starting:**
```bash
# Check if binary has MCP support
vibe-ticket --version

# Verify MCP feature is enabled
cargo build --features mcp
```

**Permission denied:**
```bash
# Ensure binary is executable
chmod +x ~/.cargo/bin/vibe-ticket
```

**Port already in use:**
```bash
# Use a different port
vibe-ticket mcp serve --port 8081
```

### Debug Mode

Enable detailed logging:
```bash
RUST_LOG=debug vibe-ticket mcp serve
```

## Advanced Configuration

### Custom MCP Settings

Create `.vibe-ticket/mcp-config.yaml`:
```yaml
mcp:
  host: "127.0.0.1"
  port: 8080
  timeout: 30
  max_connections: 10
```

### Environment Variables

- `VIBE_TICKET_PROJECT`: Override project directory
- `VIBE_TICKET_CONFIG`: Custom config file path
- `RUST_LOG`: Set logging level

## Future Enhancements

- WebSocket support for real-time updates
- Batch operations for multiple tickets
- Custom tool extensions
- Integration with CI/CD pipelines