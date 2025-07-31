# vibe-ticket Architecture

## Overview

vibe-ticket is designed as a modular, high-performance ticket management system with a focus on developer workflows. The architecture prioritizes safety, extensibility, and integration with existing development tools.

## Core Components

### 1. CLI Interface (`src/cli/`)
- **Command Parser**: Handles command-line arguments and routing
- **Output Formatter**: Supports multiple output formats (plain text, JSON)
- **Command Handlers**: Individual handlers for each command

### 2. Core Domain (`src/core/`)
- **Ticket**: Main entity representing work items
- **Task**: Sub-items within tickets
- **Status**: Workflow states (todo, doing, done, blocked, review)
- **Priority**: Importance levels (low, medium, high, critical)
- **TicketId**: Type-safe UUID wrapper

### 3. Storage Layer (`src/storage/`)
- **Repository Traits**: Abstract interfaces for data operations
- **FileStorage**: YAML-based file storage implementation
- **FileLock**: Concurrent access protection mechanism

### 4. Configuration (`src/config/`)
- Project-level settings
- Git integration preferences
- Default values and behaviors

### 5. Spec Management (`src/specs/`)
- Three-phase development support
- Requirements, design, and task tracking
- Template system for consistency

## Data Flow

```
CLI Command
    ↓
Command Handler
    ↓
Storage Layer (with locking)
    ↓
File System (YAML files)
```

## Concurrent Access Protection

### Problem
Multiple users or processes accessing tickets simultaneously can cause:
- Data corruption from race conditions
- Lost updates when changes overlap
- Inconsistent state during operations

### Solution
vibe-ticket implements a file-based locking mechanism with the following features:

#### Lock Files
- Created as `<filename>.lock` alongside data files
- Contains metadata about the lock holder:
  ```json
  {
    "holder_id": "uuid-v4",
    "pid": 12345,
    "acquired_at": 1234567890,
    "operation": "save_ticket"
  }
  ```

#### Lock Acquisition Process
1. Attempt to create lock file exclusively
2. If fails, check if existing lock is stale (>30 seconds)
3. Remove stale locks automatically
4. Retry with exponential backoff (10 attempts, 100ms delay)
5. Fail gracefully if unable to acquire

#### Automatic Release
- Uses Rust's RAII pattern via `Drop` trait
- Locks released automatically when `FileLock` goes out of scope
- Ensures cleanup even on panic or early return

#### Integration Points
All storage operations are protected:
- `save_ticket()` - Creating/updating tickets
- `load_ticket()` - Reading ticket data
- `delete_ticket()` - Removing tickets
- `set_active()` - Changing active ticket
- State and spec operations

### Example Usage
```rust
// Lock is acquired automatically
let _lock = FileLock::acquire(&path, Some("save_ticket".to_string()))?;

// Perform file operations safely
let content = serde_yaml::to_string(&ticket)?;
fs::write(&path, content)?;

// Lock released automatically when _lock goes out of scope
```

## Error Handling

The system uses a custom error type (`VibeTicketError`) with variants for:
- I/O operations
- YAML parsing
- Ticket not found
- Project not initialized
- Concurrent access failures

Errors bubble up through `Result<T>` with context preservation.

## Git Integration

### Worktree Support
- Each ticket can have its own Git worktree
- Isolated working directories for parallel development
- Automatic creation on `start` command
- Cleanup utilities for stale worktrees

### Branch Management
- Automatic branch creation from ticket slugs
- Configurable branch naming patterns
- Integration with existing Git workflows

## Performance Considerations

### File-Based Storage
- Simple YAML files for human readability
- No database dependencies
- Instant startup time
- Suitable for teams up to ~1000 active tickets

### Locking Overhead
- Minimal performance impact (~1-2ms per operation)
- Retry mechanism prevents user-visible delays
- Stale lock cleanup prevents accumulation

## Future Architecture Considerations

### Planned Enhancements
1. **Plugin System**: Dynamic loading of extensions
2. **Database Backend**: Optional PostgreSQL/SQLite support
3. **Distributed Locking**: For network file systems
4. **Event System**: Webhooks and notifications

### Scalability Path
1. Current: File-based storage with local locking
2. Next: Optional database backend for larger teams
3. Future: Distributed system with proper consensus

## Testing Strategy

### Unit Tests
- Core logic isolation
- Mock storage for handlers
- Property-based testing for IDs

### Integration Tests
- Full command execution
- File system operations
- Git integration

### Concurrent Tests
- Multi-threaded ticket creation
- Simultaneous modifications
- Lock contention scenarios
- Run with `--test-threads=1` for reliability

## Security Considerations

- No sensitive data in lock files
- File permissions follow system defaults
- No network communication by default
- Git integration uses system SSH/HTTPS