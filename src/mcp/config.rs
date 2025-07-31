//! MCP server configuration

use serde::{Deserialize, Serialize};

/// Main MCP configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Authentication configuration
    pub auth: AuthConfig,
    
    /// Feature configuration
    pub features: FeatureConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Port to listen on
    #[serde(default = "default_port")]
    pub port: u16,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Whether authentication is enabled
    #[serde(default)]
    pub enabled: bool,
    
    /// Authentication token
    pub token: Option<String>,
}

/// Feature configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    /// Enable ticket management tools
    #[serde(default = "default_true")]
    pub tickets: bool,
    
    /// Enable task management tools
    #[serde(default = "default_true")]
    pub tasks: bool,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            auth: AuthConfig::default(),
            features: FeatureConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            token: None,
        }
    }
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            tickets: true,
            tasks: true,
        }
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3333
}

fn default_true() -> bool {
    true
}