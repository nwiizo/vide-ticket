//! Model Context Protocol (MCP) server implementation for vibe-ticket
//!
//! This module provides MCP server functionality to expose vibe-ticket
//! operations through the Model Context Protocol, enabling AI assistants
//! to interact with the ticket management system.

#[cfg(feature = "mcp")]
pub mod auth;
#[cfg(feature = "mcp")]
pub mod config;
#[cfg(feature = "mcp")]
pub mod error;
#[cfg(feature = "mcp")]
pub mod handlers;
#[cfg(feature = "mcp")]
pub mod server;
#[cfg(feature = "mcp")]
pub mod transport;

#[cfg(feature = "mcp")]
pub use config::McpConfig;
#[cfg(feature = "mcp")]
pub use error::{McpError, McpResult};
#[cfg(feature = "mcp")]
pub use server::McpServer;