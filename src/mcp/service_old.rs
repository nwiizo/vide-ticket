//! MCP service implementation for vibe-ticket

use crate::mcp::{
    config::McpConfig,
};
use crate::storage::{FileStorage, TicketRepository};
use crate::core::{Priority, Ticket, Status};
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    schemars, tool, tool_router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::future::Future;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use chrono::Utc;

/// Request for listing tickets
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListTicketsRequest {
    /// Optional status filter
    #[schemars(description = "Filter by ticket status (todo, doing, done, etc.)")]
    pub status: Option<String>,
    /// Optional priority filter
    #[schemars(description = "Filter by priority (low, medium, high, critical)")]
    pub priority: Option<String>,
}

/// Request for getting a specific ticket
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetTicketRequest {
    #[schemars(description = "Ticket ID or slug")]
    pub ticket_id: String,
}

/// Request for creating a new ticket
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateTicketRequest {
    #[schemars(description = "Unique ticket slug")]
    pub slug: String,
    #[schemars(description = "Ticket title")]
    pub title: String,
    #[schemars(description = "Ticket description")]
    pub description: Option<String>,
    #[schemars(description = "Priority level (low, medium, high, critical)")]
    pub priority: Option<String>,
    #[schemars(description = "Tags for the ticket")]
    pub tags: Option<Vec<String>>,
}

/// MCP service implementation
#[derive(Clone)]
pub struct VibeTicketService {
    config: Arc<RwLock<McpConfig>>,
    storage: Arc<FileStorage>,
    initialized: Arc<RwLock<bool>>,
    tool_router: ToolRouter<Self>,
}

impl VibeTicketService {
    /// Create a new service instance
    pub fn new(config: McpConfig, storage: FileStorage) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            storage: Arc::new(storage),
            initialized: Arc::new(RwLock::new(false)),
            tool_router: Self::tool_router(),
        }
    }
    
    /// Get the tool router
    pub fn tool_router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }
}

// Implement ServerHandler trait for MCP protocol
impl ServerHandler for VibeTicketService {
    fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "vibe-ticket".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Default::default()
        }
    }
    
    fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            tools: Some(serde_json::json!({
                "supported": true
            })),
            ..Default::default()
        }
    }
}

// Tool router implementation
#[tool_router]
impl VibeTicketService {
    /// List all tickets
    #[tool(description = "List all tickets in the system")]
    async fn list_tickets(&self, Parameters(req): Parameters<ListTicketsRequest>) -> Result<CallToolResult, rmcp::ServiceError> {
        info!("Listing all tickets");
        
        let tickets = self.storage.load_all()
            .map_err(|e| rmcp::ServiceError::other(e.to_string()))?;
        
        // Apply filters
        let filtered_tickets: Vec<_> = tickets
            .into_iter()
            .filter(|ticket| {
                if let Some(ref status) = req.status {
                    if ticket.status.to_string().to_lowercase() != status.to_lowercase() {
                        return false;
                    }
                }
                if let Some(ref priority) = req.priority {
                    if ticket.priority.to_string().to_lowercase() != priority.to_lowercase() {
                        return false;
                    }
                }
                true
            })
            .collect();
        
        // Convert tickets to JSON
        let result: Vec<Value> = filtered_tickets
            .iter()
            .map(|ticket| {
                json!({
                    "id": ticket.id.to_string(),
                    "slug": ticket.slug,
                    "title": ticket.title,
                    "status": ticket.status.to_string(),
                    "priority": ticket.priority.to_string(),
                    "tags": ticket.tags,
                    "created_at": ticket.created_at.to_rfc3339(),
                    "updated_at": ticket.updated_at.to_rfc3339(),
                })
            })
            .collect();

        let content = json!({
            "tickets": result,
            "total": result.len()
        });

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string_pretty(&content)
                .unwrap_or_else(|_| content.to_string()))
        ]))
    }

    /// Get ticket by ID or slug
    #[tool(description = "Get a specific ticket by ID or slug")]
    async fn get_ticket(&self, Parameters(req): Parameters<GetTicketRequest>) -> Result<CallToolResult, rmcp::ServiceError> {
        info!("Getting ticket: {}", req.ticket_id);
        
        let ticket = self.storage.load(&req.ticket_id)
            .map_err(|e| rmcp::ServiceError::other(e.to_string()))?;
        
        let content = json!({
            "id": ticket.id.to_string(),
            "slug": ticket.slug,
            "title": ticket.title,
            "description": ticket.description,
            "status": ticket.status.to_string(),
            "priority": ticket.priority.to_string(),
            "tags": ticket.tags,
            "parent_id": ticket.parent_id.map(|id| id.to_string()),
            "tasks": ticket.tasks,
            "assignees": ticket.assignees,
            "created_at": ticket.created_at.to_rfc3339(),
            "updated_at": ticket.updated_at.to_rfc3339(),
            "closed_at": ticket.closed_at.map(|dt| dt.to_rfc3339()),
            "completion_message": ticket.completion_message,
        });

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string_pretty(&content)
                .unwrap_or_else(|_| content.to_string()))
        ]))
    }

    /// Create a new ticket
    #[tool(description = "Create a new ticket")]
    async fn create_ticket(&self, Parameters(req): Parameters<CreateTicketRequest>) -> Result<CallToolResult, rmcp::ServiceError> {
        info!("Creating new ticket: {}", req.slug);
        
        let priority = req.priority
            .and_then(|p| p.parse::<Priority>().ok())
            .unwrap_or(Priority::Medium);
            
        let ticket = Ticket {
            id: Uuid::new_v4(),
            slug: req.slug.clone(),
            title: req.title,
            description: req.description.unwrap_or_default(),
            status: Status::Todo,
            priority,
            tags: req.tags.unwrap_or_default(),
            parent_id: None,
            tasks: Vec::new(),
            assignees: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            completion_message: None,
        };
        
        self.storage.save(&ticket)
            .map_err(|e| rmcp::ServiceError::other(e.to_string()))?;
        
        let content = json!({
            "message": format!("Ticket '{}' created successfully", req.slug),
            "ticket_id": ticket.id.to_string(),
        });

        Ok(CallToolResult::success(vec![
            Content::text(serde_json::to_string_pretty(&content)
                .unwrap_or_else(|_| content.to_string()))
        ]))
    }
}