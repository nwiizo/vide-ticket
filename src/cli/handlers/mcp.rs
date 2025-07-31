//! MCP server CLI handlers

#[cfg(feature = "mcp")]
use crate::cli::{find_project_root, OutputFormatter};
#[cfg(feature = "mcp")]
use crate::error::Result;
#[cfg(feature = "mcp")]
use crate::mcp::{McpConfig, McpServer};
#[cfg(feature = "mcp")]
use crate::storage::FileStorage;

/// Handle the MCP serve command
#[cfg(feature = "mcp")]
pub fn handle_mcp_serve(
    config_path: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    daemon: bool,
    project_dir: Option<&str>,
    output: &OutputFormatter,
) -> Result<()> {
    // Find project root
    let project_root = find_project_root(project_dir)?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");
    
    // Load or create MCP configuration
    let mut config = if let Some(path) = config_path {
        let config_file = std::path::PathBuf::from(&path);
        if config_file.exists() {
            let content = std::fs::read_to_string(&config_file)?;
            serde_yaml::from_str::<McpConfig>(&content)?
        } else {
            output.warning(&format!("Config file not found: {}, using defaults", path));
            McpConfig::default()
        }
    } else {
        McpConfig::default()
    };
    
    // Override with command line options
    if let Some(h) = host {
        config.server.host = h;
    }
    if let Some(p) = port {
        config.server.port = p;
    }
    
    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);
    
    // Create and start server
    let server = McpServer::new(config.clone(), storage);
    
    if daemon {
        output.info("Starting MCP server in daemon mode...");
        // TODO: Implement daemon mode
        output.error("Daemon mode not yet implemented");
        return Ok(());
    }
    
    output.info(&format!(
        "Starting MCP server on {}:{}",
        config.server.host, config.server.port
    ));
    
    // Create async runtime and run server
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        if let Err(e) = server.start().await {
            output.error(&format!("MCP server error: {}", e));
        }
    });
    
    Ok(())
}