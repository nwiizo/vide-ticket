//! MCP service implementation for vibe-ticket

use crate::cli::find_project_root;
use crate::core::{Priority, Status, Ticket};
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};
use rmcp::{
    model::{ServerCapabilities, ServerInfo, Tool},
    ServerHandler,
};
use serde_json::{json, Value};
use std::borrow::Cow;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

/// MCP service implementation
#[derive(Clone)]
pub struct VibeTicketService {
    storage: Arc<FileStorage>,
    project_root: PathBuf,
}

impl VibeTicketService {
    /// Create a new service instance
    pub fn new(storage: FileStorage, project_root: PathBuf) -> Self {
        Self {
            storage: Arc::new(storage),
            project_root,
        }
    }

    /// Get all available tools
    pub fn get_tools() -> Vec<Tool> {
        use crate::mcp::handlers;
        let mut tools = Vec::new();
        
        // Ticket operations
        tools.extend(handlers::tickets::register_tools());
        tools.extend(handlers::tasks::register_tools());
        tools.extend(handlers::worktree::register_tools());
        tools.extend(handlers::search::register_tools());
        tools.extend(handlers::config::register_tools());
        tools.extend(handlers::spec::register_tools());
        
        tools
    }
}

// Implement ServerHandler trait for MCP protocol
impl ServerHandler for VibeTicketService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: Some("vibe-ticket".into()),
            version: Some(env!("CARGO_PKG_VERSION").into()),
            instructions: Some(
                "vibe-ticket MCP server provides comprehensive ticket management capabilities. \
                 Use the available tools to create, manage, and track tickets, tasks, and worktrees."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }

    fn list_tools(&self) -> Vec<Tool> {
        Self::get_tools()
    }

    fn call_tool(
        &self,
        name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send + 'static>> {
        let service = self.clone();
        let name = name.to_string();
        let arguments = arguments.clone();

        Box::pin(async move {
            match name.as_str() {
                // Ticket operations
                "vibe-ticket.new" => crate::mcp::handlers::tickets::handle_new(&service, arguments).await,
                "vibe-ticket.list" => crate::mcp::handlers::tickets::handle_list(&service, arguments).await,
                "vibe-ticket.show" => crate::mcp::handlers::tickets::handle_show(&service, arguments).await,
                "vibe-ticket.edit" => crate::mcp::handlers::tickets::handle_edit(&service, arguments).await,
                "vibe-ticket.close" => crate::mcp::handlers::tickets::handle_close(&service, arguments).await,
                "vibe-ticket.start" => crate::mcp::handlers::tickets::handle_start(&service, arguments).await,
                "vibe-ticket.check" => crate::mcp::handlers::tickets::handle_check(&service, arguments).await,
                
                // Task operations
                "vibe-ticket.task.add" => crate::mcp::handlers::tasks::handle_add(&service, arguments).await,
                "vibe-ticket.task.complete" => crate::mcp::handlers::tasks::handle_complete(&service, arguments).await,
                "vibe-ticket.task.list" => crate::mcp::handlers::tasks::handle_list(&service, arguments).await,
                "vibe-ticket.task.remove" => crate::mcp::handlers::tasks::handle_remove(&service, arguments).await,
                
                // Worktree operations
                "vibe-ticket.worktree.list" => crate::mcp::handlers::worktree::handle_list(&service, arguments).await,
                "vibe-ticket.worktree.remove" => crate::mcp::handlers::worktree::handle_remove(&service, arguments).await,
                "vibe-ticket.worktree.prune" => crate::mcp::handlers::worktree::handle_prune(&service, arguments).await,
                
                // Search and export
                "vibe-ticket.search" => crate::mcp::handlers::search::handle_search(&service, arguments).await,
                "vibe-ticket.export" => crate::mcp::handlers::search::handle_export(&service, arguments).await,
                "vibe-ticket.import" => crate::mcp::handlers::search::handle_import(&service, arguments).await,
                
                // Config operations
                "vibe-ticket.config.show" => crate::mcp::handlers::config::handle_show(&service, arguments).await,
                "vibe-ticket.config.set" => crate::mcp::handlers::config::handle_set(&service, arguments).await,
                
                // Spec operations
                "vibe-ticket.spec.add" => crate::mcp::handlers::spec::handle_add(&service, arguments).await,
                "vibe-ticket.spec.update" => crate::mcp::handlers::spec::handle_update(&service, arguments).await,
                "vibe-ticket.spec.check" => crate::mcp::handlers::spec::handle_check(&service, arguments).await,
                
                _ => Err(format!("Unknown tool: {}", name)),
            }
        })
    }
}
