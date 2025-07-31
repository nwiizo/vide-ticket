//! Simplified MCP service implementation for vibe-ticket

use crate::storage::FileStorage;
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    ServerHandler,
};
use std::sync::Arc;

/// MCP service implementation
#[derive(Clone)]
pub struct VibeTicketService {
    #[allow(dead_code)]
    storage: Arc<FileStorage>,
}

impl VibeTicketService {
    /// Create a new service instance
    pub fn new(storage: FileStorage) -> Self {
        Self {
            storage: Arc::new(storage),
        }
    }
}

// Implement ServerHandler trait for MCP protocol
impl ServerHandler for VibeTicketService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("vibe-ticket MCP server for ticket management".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
