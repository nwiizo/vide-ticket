//! MCP server implementation

use crate::mcp::{
    config::McpConfig,
    error::McpResult,
};
use crate::storage::FileStorage;
use std::sync::Arc;
use tracing::info;

/// MCP server for vibe-ticket
pub struct McpServer {
    /// Server configuration
    config: McpConfig,
    
    /// Storage backend
    storage: Arc<FileStorage>,
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(config: McpConfig, storage: FileStorage) -> Self {
        Self {
            config,
            storage: Arc::new(storage),
        }
    }

    /// Start the MCP server
    pub async fn start(&self) -> McpResult<()> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        
        info!("Starting MCP server on {}", addr);
        
        // TODO: Implement actual MCP server using rmcp
        
        Ok(())
    }
}