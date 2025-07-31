//! Authentication and authorization for MCP server

use crate::mcp::{config::AuthConfig, error::McpError};

/// Simple authentication check
pub fn validate_token(config: &AuthConfig, token: &str) -> Result<(), McpError> {
    if !config.enabled {
        return Ok(());
    }

    match &config.token {
        Some(expected_token) => {
            if token == expected_token {
                Ok(())
            } else {
                Err(McpError::AuthenticationFailed(
                    "Invalid authentication token".to_string(),
                ))
            }
        }
        None => {
            Err(McpError::AuthenticationFailed(
                "Authentication enabled but no token configured".to_string(),
            ))
        }
    }
}