# CLI-MCP Integration Guide

## Overview

This document describes the integration between vibe-ticket CLI commands and the Model Context Protocol (MCP) service. The integration ensures that operations performed via CLI are automatically synchronized with MCP.

## Architecture

```
┌─────────────┐         ┌──────────────┐         ┌──────────────┐
│     CLI     │ ──1──> │  Integration │ ──2──> │     MCP      │
│  Commands   │         │   Service    │         │   Service    │
└─────────────┘         └──────────────┘         └──────────────┘
       │                                                  │
       └──────────────────3───────────────────────────────┘
                     Shared Storage Layer
```

1. CLI commands execute and notify Integration Service
2. Integration Service broadcasts events to MCP
3. Both CLI and MCP use the same storage layer

## Integration Points

### 1. Ticket Creation (`vibe-ticket new`)
- CLI creates ticket in storage
- Emits `TicketCreated` event
- MCP receives notification and updates internal state

### 2. Ticket Updates (`vibe-ticket edit`)
- CLI updates ticket in storage
- Emits `TicketUpdated` event
- MCP synchronizes changes

### 3. Status Changes (`vibe-ticket start/close`)
- CLI changes ticket status
- Emits `StatusChanged` event
- MCP tracks workflow transitions

### 4. Task Operations
- CLI adds/completes/removes tasks
- Emits corresponding task events
- MCP maintains task state

## Implementation

### Integration Module

The `integration` module provides a lightweight event system:

```rust
use vibe_ticket::integration::{
    notify_ticket_created,
    notify_ticket_updated,
    notify_status_changed,
    notify_ticket_closed,
};

// In CLI handler
pub fn handle_new_command(...) {
    // Create ticket
    storage.save(&ticket)?;
    
    // Notify MCP
    notify_ticket_created(&ticket);
}
```

### MCP Event Listener

The MCP service can subscribe to integration events:

```rust
// In MCP initialization
let mut receiver = integration.subscribe();

tokio::spawn(async move {
    while let Ok(event) = receiver.recv().await {
        match event {
            IntegrationEvent::TicketCreated { ticket } => {
                // Handle ticket creation
            }
            // ... other events
        }
    }
});
```

## Benefits

1. **Real-time Synchronization**: MCP always has current state
2. **No Duplicate Logic**: Single source of truth for business logic
3. **Backward Compatible**: Existing CLI usage unchanged
4. **Performance**: Asynchronous notifications don't block CLI
5. **Extensible**: Easy to add new event types

## Configuration

Enable integration in your config:

```yaml
integration:
  enabled: true
  mcp_sync: true
  event_buffer_size: 100
```

## Testing

Run the integration example:

```bash
cargo run --example cli_mcp_integration
```

This demonstrates:
- Creating a ticket via CLI-like interface
- Automatic MCP notification
- Status change tracking

## Future Enhancements

1. **Bidirectional Sync**: MCP operations notify CLI
2. **Conflict Resolution**: Handle concurrent modifications
3. **Event Persistence**: Store events for replay
4. **WebSocket Support**: Real-time updates to connected clients
5. **Metrics & Monitoring**: Track sync performance

## Migration

To add integration to existing CLI commands:

1. Import integration functions
2. Add notification calls after storage operations
3. No changes needed to command signatures
4. MCP automatically receives updates

Example patch:
```diff
+ use crate::integration::notify_ticket_created;

  // Save the ticket
  storage.save(&ticket)?;
  
+ // Notify MCP
+ notify_ticket_created(&ticket);
```