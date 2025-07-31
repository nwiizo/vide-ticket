//! MCP server command handler

#[cfg(feature = "mcp")]
use crate::cli::output::OutputFormatter;
#[cfg(feature = "mcp")]
use crate::config::Config;
#[cfg(feature = "mcp")]
use crate::mcp::{McpConfig, McpServer};
#[cfg(feature = "mcp")]
use crate::storage::FileStorage;
#[cfg(feature = "mcp")]
use std::path::PathBuf;

#[cfg(feature = "mcp")]
pub fn handle_mcp_serve(
    config: Config,
    host: Option<String>,
    port: Option<u16>,
    daemon: bool,
    project_path: Option<&str>,
    formatter: &OutputFormatter,
) -> anyhow::Result<()> {
    use tracing::{error, info};
    
    // Create MCP configuration
    let mut mcp_config = McpConfig::default();
    
    if let Some(host) = host {
        mcp_config.server.host = host;
    }
    
    if let Some(port) = port {
        mcp_config.server.port = port;
    }
    
    // Get storage path
    let storage_path = if let Some(path) = project_path {
        PathBuf::from(path).join(".vibe-ticket")
    } else {
        config.storage_path()
    };
    
    mcp_config.storage_path = storage_path.clone();
    
    // Create storage
    let storage = FileStorage::new(storage_path);
    
    // Create and start server
    let server = McpServer::new(mcp_config.clone(), storage);
    
    if daemon {
        formatter.info("Starting MCP server in daemon mode...");
        // TODO: Implement daemon mode
        return Err(anyhow::anyhow!("Daemon mode not yet implemented"));
    }
    
    formatter.info(&format!(
        "Starting MCP server on {}:{}",
        mcp_config.server.host, mcp_config.server.port
    ));
    
    // Run server
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        if let Err(e) = server.start().await {
            error!("MCP server error: {}", e);
            return Err(anyhow::anyhow!("MCP server error: {}", e));
        }
        Ok(())
    })
}

#[cfg(not(feature = "mcp"))]
pub fn handle_mcp_serve(
    _config: crate::config::Config,
    _host: Option<String>,
    _port: Option<u16>,
    _daemon: bool,
    _project_path: Option<&str>,
    formatter: &crate::cli::output::OutputFormatter,
) -> anyhow::Result<()> {
    formatter.error("MCP support is not enabled. Rebuild with --features mcp");
    Err(anyhow::anyhow!("MCP feature not enabled"))
}