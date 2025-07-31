//! Configuration for MCP server

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Server configuration
    pub server: ServerConfig,

    /// Authentication configuration
    pub auth: AuthConfig,

    /// Ticket storage path
    pub storage_path: PathBuf,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            auth: AuthConfig::default(),
            storage_path: PathBuf::from(".vibe-ticket"),
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,

    /// Port to listen on
    pub port: u16,

    /// Transport type (stdio, tcp, websocket)
    pub transport: TransportType,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3033,
            transport: TransportType::Stdio,
        }
    }
}

/// Transport type for MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    /// Standard input/output
    Stdio,

    /// TCP socket
    Tcp,

    /// WebSocket
    WebSocket,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,

    /// API key for authentication
    pub api_key: Option<String>,
}
