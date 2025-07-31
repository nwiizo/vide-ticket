//! Event system for CLI-MCP synchronization

use crate::core::{Status, Task, Ticket, TicketId};
use crate::error::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Events that can be emitted by CLI operations
#[derive(Debug, Clone)]
pub enum TicketEvent {
    /// A new ticket was created
    Created(Ticket),
    /// An existing ticket was updated
    Updated(Ticket),
    /// A ticket was closed
    Closed(TicketId, String), // id, completion message
    /// A task was added to a ticket
    TaskAdded(TicketId, Task),
    /// A task was completed
    TaskCompleted(TicketId, String), // ticket id, task id
    /// A task was removed
    TaskRemoved(TicketId, String), // ticket id, task id
    /// Ticket status changed
    StatusChanged(TicketId, Status, Status), // id, old status, new status
}

/// Trait for handling ticket events
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle a ticket event
    async fn handle_event(&self, event: TicketEvent) -> Result<()>;
}

/// Event bus for distributing events to handlers
pub struct EventBus {
    handlers: Arc<RwLock<Vec<Arc<dyn EventHandler>>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register an event handler
    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.push(handler);
    }

    /// Emit an event to all registered handlers
    pub async fn emit(&self, event: TicketEvent) -> Result<()> {
        let handlers = self.handlers.read().await;

        // Process events asynchronously but wait for all to complete
        let mut tasks = Vec::new();
        for handler in handlers.iter() {
            let handler = Arc::clone(handler);
            let event = event.clone();

            let task = tokio::spawn(async move {
                if let Err(e) = handler.handle_event(event).await {
                    eprintln!("Event handler error: {}", e);
                }
            });

            tasks.push(task);
        }

        // Wait for all handlers to complete
        for task in tasks {
            let _ = task.await;
        }

        Ok(())
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Global event bus instance
static EVENT_BUS: once_cell::sync::Lazy<EventBus> = once_cell::sync::Lazy::new(EventBus::new);

/// Get the global event bus
pub fn event_bus() -> &'static EventBus {
    &EVENT_BUS
}

/// Emit an event to the global event bus
pub async fn emit_event(event: TicketEvent) -> Result<()> {
    event_bus().emit(event).await
}
