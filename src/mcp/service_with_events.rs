//! MCP service implementation with event support

use crate::events::{event_bus, EventHandler};
use crate::mcp::handlers::events::McpEventHandler;
use crate::storage::FileStorage;
use rmcp::{
    model::{ServerCapabilities, ServerInfo, Tool},
    service::RequestContext,
    ErrorData, RoleServer, ServerHandler,
};
use serde_json::Value;
use std::borrow::Cow;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

/// MCP service implementation with event support
#[derive(Clone)]
pub struct VibeTicketService {
    pub storage: Arc<FileStorage>,
    pub project_root: PathBuf,
}

impl VibeTicketService {
    /// Create a new service instance with event handler registration
    pub async fn new(storage: FileStorage, project_root: PathBuf) -> Self {
        let service = Self {
            storage: Arc::new(storage),
            project_root,
        };

        // Create and register MCP event handler
        let mcp_handler = McpEventHandler::new(Arc::new(service.clone()));
        event_bus()
            .register_handler(Arc::new(mcp_handler))
            .await;

        tracing::info!("MCP event handler registered");

        service
    }

    /// Get available tools
    fn get_tools() -> Vec<Tool> {
        let mut tools = Vec::new();
        
        // Register tools from each handler module
        tools.extend(crate::mcp::handlers::tickets::register_tools());
        tools.extend(crate::mcp::handlers::tasks::register_tools());
        tools.extend(crate::mcp::handlers::search::register_tools());
        tools.extend(crate::mcp::handlers::config::register_tools());
        tools.extend(crate::mcp::handlers::worktree::register_tools());
        
        tools
    }
}

impl ServerHandler for VibeTicketService {
    fn server_info(&self, _ctx: RequestContext<RoleServer>) -> impl Future<Output = ServerInfo> + Send + '_ {
        async move {
            ServerInfo {
                name: Cow::Borrowed("vibe-ticket"),
                version: Cow::Borrowed(env!("CARGO_PKG_VERSION")),
                protocol_version: Cow::Borrowed("0.1.0"),
                vendor: Cow::Borrowed("vibe-ticket"),
                description: Some(
                    Cow::Borrowed(
                        "vibe-ticket MCP server for ticket management.
                         Use the available tools to create, manage, and track tickets, tasks, and worktrees."
                    ),
                ),
                capabilities: ServerCapabilities::builder().enable_tools().build(),
                ..Default::default()
            }
        }
    }

    fn list_tools(
        &self,
        _pagination: Option<rmcp::model::PaginatedRequestParam>,
        _ctx: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListToolsResult, ErrorData>> + Send + '_ {
        async move {
            Ok(rmcp::model::ListToolsResult {
                tools: Self::get_tools(),
                next_cursor: None,
            })
        }
    }

    fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Pin<Box<dyn Future<Output = Result<rmcp::model::CallToolResult, ErrorData>> + Send + 'static>> {
        let service = self.clone();
        let name = request.name.clone();
        let arguments = request.arguments.unwrap_or_default();

        Box::pin(async move {
            let result = match name.as_str() {
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
                
                // Search operations
                "vibe-ticket.search" => crate::mcp::handlers::search::handle_search(&service, arguments).await,
                
                // Worktree operations
                "vibe-ticket.worktree.list" => crate::mcp::handlers::worktree::handle_list(&service, arguments).await,
                "vibe-ticket.worktree.create" => crate::mcp::handlers::worktree::handle_create(&service, arguments).await,
                "vibe-ticket.worktree.remove" => crate::mcp::handlers::worktree::handle_remove(&service, arguments).await,
                
                // Config operations
                "vibe-ticket.config.get" => crate::mcp::handlers::config::handle_get(&service, arguments).await,
                "vibe-ticket.config.set" => crate::mcp::handlers::config::handle_set(&service, arguments).await,
                
                _ => Err(format!("Unknown tool: {}", name)),
            };
            
            match result {
                Ok(content) => Ok(rmcp::model::CallToolResult {
                    content: vec![rmcp::model::content::Content::Text {
                        text: serde_json::to_string_pretty(&content)
                            .unwrap_or_else(|_| content.to_string()),
                    }],
                    is_error: None,
                }),
                Err(e) => Err(ErrorData {
                    error_code: rmcp::model::ErrorCode::InternalError,
                    message: e,
                }),
            }
        })
    }
}