//! MCP server implementation

use crate::mcp::{config::McpConfig, error::McpResult, service::VibeTicketService};
use crate::storage::FileStorage;
use rmcp::ServiceExt;
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

        // For now, we'll use stdio transport
        // TODO: Implement TCP transport
        self.start_stdio().await
    }

    /// Start server with stdio transport
    pub async fn start_stdio(&self) -> McpResult<()> {
        info!("Starting MCP server with stdio transport");

        // Get project root from storage path (parent of .vibe-ticket)
        let project_root = self
            .config
            .storage_path
            .parent()
            .unwrap_or(&self.config.storage_path)
            .to_path_buf();

        // Create service
        let service = VibeTicketService::new((*self.storage).clone(), project_root);

        // Create stdio transport
        let transport = (tokio::io::stdin(), tokio::io::stdout());

        // Serve the service
        let server = service.serve(transport).await?;

        info!("MCP server started successfully");

        // Wait for the server to complete
        server.waiting().await?;
        info!("MCP server shut down");

        Ok(())
    }
}
