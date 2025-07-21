//! Handler for the `config` command
//!
//! This module implements the logic for managing project configuration.

use crate::cli::{OutputFormatter, find_project_root, ConfigCommands};
use crate::config::Config;
use crate::error::{Result, VideTicketError};

/// Handler for the `config` subcommands
///
/// Manages project configuration including:
/// - Viewing configuration values
/// - Setting configuration values
/// - Getting specific configuration values
/// - Resetting configuration to defaults
///
/// # Arguments
///
/// * `command` - The config subcommand to execute
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_config_command(
    command: ConfigCommands,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let config_path = project_root.join(".vide-ticket/config.yaml");
    
    match command {
        ConfigCommands::Show { key } => handle_show(key, &config_path, output),
        ConfigCommands::Set { key, value } => handle_set(&key, &value, &config_path, output),
        ConfigCommands::Get { key } => handle_get(&key, &config_path, output),
        ConfigCommands::Reset { force } => handle_reset(force, &config_path, output),
    }
}

/// Show configuration values
fn handle_show(
    key: Option<String>,
    config_path: &std::path::Path,
    output: &OutputFormatter,
) -> Result<()> {
    let config = Config::load_from_path(config_path)?;
    
    if let Some(key) = key {
        // Show specific key
        let value = get_config_value(&config, &key)?;
        if output.is_json() {
            output.print_json(&serde_json::json!({
                "key": key,
                "value": value,
            }))?;
        } else {
            output.info(&format!("{}: {}", key, format_value(&value)));
        }
    } else {
        // Show all configuration
        if output.is_json() {
            output.print_json(&serde_json::to_value(&config)?)?;
        } else {
            output.success("Project Configuration:");
            output.info("");
            
            // Project section
            output.info("[project]");
            output.info(&format!("  name: {}", config.project.name));
            if let Some(desc) = &config.project.description {
                output.info(&format!("  description: {}", desc));
            }
            if let Some(assignee) = &config.project.default_assignee {
                output.info(&format!("  default_assignee: {}", assignee));
            }
            output.info(&format!("  default_priority: {}", config.project.default_priority));
            output.info("");
            
            // UI section
            output.info("[ui]");
            output.info(&format!("  theme: {}", config.ui.theme));
            output.info(&format!("  emoji: {}", config.ui.emoji));
            output.info(&format!("  page_size: {}", config.ui.page_size));
            output.info(&format!("  date_format: {}", config.ui.date_format));
            output.info("");
            
            // Git section
            output.info("[git]");
            output.info(&format!("  enabled: {}", config.git.enabled));
            output.info(&format!("  branch_prefix: {}", config.git.branch_prefix));
            output.info(&format!("  auto_branch: {}", config.git.auto_branch));
            if let Some(template) = &config.git.commit_template {
                output.info(&format!("  commit_template: {}", template));
            }
            output.info("");
            
            // Plugins section
            output.info("[plugins]");
            output.info(&format!("  enabled: {:?}", config.plugins.enabled));
            output.info(&format!("  directory: {}", config.plugins.directory));
        }
    }
    
    Ok(())
}

/// Set a configuration value
fn handle_set(
    key: &str,
    value: &str,
    config_path: &std::path::Path,
    output: &OutputFormatter,
) -> Result<()> {
    let mut config = Config::load_from_path(config_path)?;
    
    // Parse and set the value
    set_config_value(&mut config, key, value)?;
    
    // Save the configuration
    config.save_to_path(config_path)?;
    
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "key": key,
            "value": value,
        }))?;
    } else {
        output.success(&format!("Set {} = {}", key, value));
    }
    
    Ok(())
}

/// Get a specific configuration value
fn handle_get(
    key: &str,
    config_path: &std::path::Path,
    output: &OutputFormatter,
) -> Result<()> {
    let config = Config::load_from_path(config_path)?;
    let value = get_config_value(&config, key)?;
    
    if output.is_json() {
        output.print_json(&value)?;
    } else {
        output.info(&format_value(&value));
    }
    
    Ok(())
}

/// Reset configuration to defaults
fn handle_reset(
    force: bool,
    config_path: &std::path::Path,
    output: &OutputFormatter,
) -> Result<()> {
    if !force {
        return Err(VideTicketError::custom(
            "Configuration reset requires --force flag to confirm"
        ));
    }
    
    // Create default configuration
    let config = Config::default();
    
    // Save it
    config.save_to_path(config_path)?;
    
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "message": "Configuration reset to defaults",
        }))?;
    } else {
        output.success("Configuration reset to defaults");
    }
    
    Ok(())
}

/// Get a configuration value by key path
fn get_config_value(config: &Config, key: &str) -> Result<serde_json::Value> {
    // Convert config to JSON for easy path access
    let json = serde_json::to_value(config)?;
    
    // Split the key path
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = &json;
    
    for part in parts {
        match current.get(part) {
            Some(value) => current = value,
            None => return Err(VideTicketError::custom(format!(
                "Configuration key '{}' not found", key
            ))),
        }
    }
    
    Ok(current.clone())
}

/// Set a configuration value by key path
fn set_config_value(config: &mut Config, key: &str, value: &str) -> Result<()> {
    match key {
        "project.name" => config.project.name = value.to_string(),
        "project.description" => config.project.description = Some(value.to_string()),
        "project.default_assignee" => config.project.default_assignee = Some(value.to_string()),
        "project.default_priority" => {
            // Validate priority
            if !["low", "medium", "high", "critical"].contains(&value) {
                return Err(VideTicketError::custom(
                    "Invalid priority. Must be one of: low, medium, high, critical"
                ));
            }
            config.project.default_priority = value.to_string();
        }
        "ui.theme" => {
            // Validate theme
            if !["light", "dark", "auto"].contains(&value) {
                return Err(VideTicketError::custom(
                    "Invalid theme. Must be one of: light, dark, auto"
                ));
            }
            config.ui.theme = value.to_string();
        }
        "ui.emoji" => {
            config.ui.emoji = value.parse::<bool>()
                .map_err(|_| VideTicketError::custom("Value must be true or false"))?;
        }
        "ui.page_size" => {
            config.ui.page_size = value.parse::<usize>()
                .map_err(|_| VideTicketError::custom("Value must be a positive number"))?;
        }
        "ui.date_format" => config.ui.date_format = value.to_string(),
        "git.enabled" => {
            config.git.enabled = value.parse::<bool>()
                .map_err(|_| VideTicketError::custom("Value must be true or false"))?;
        }
        "git.branch_prefix" => config.git.branch_prefix = value.to_string(),
        "git.auto_branch" => {
            config.git.auto_branch = value.parse::<bool>()
                .map_err(|_| VideTicketError::custom("Value must be true or false"))?;
        }
        "git.commit_template" => config.git.commit_template = Some(value.to_string()),
        "plugins.directory" => config.plugins.directory = value.to_string(),
        _ => return Err(VideTicketError::custom(format!(
            "Configuration key '{}' cannot be set or doesn't exist", key
        ))),
    }
    
    Ok(())
}

/// Format a JSON value for display
fn format_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter()
                .map(|v| format_value(v))
                .collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_get_config_value() {
        let config = Config::default();
        
        // Test valid keys
        assert!(get_config_value(&config, "project.name").is_ok());
        assert!(get_config_value(&config, "ui.emoji").is_ok());
        
        // Test invalid key
        assert!(get_config_value(&config, "invalid.key").is_err());
    }
    
    #[test]
    fn test_set_config_value() {
        let mut config = Config::default();
        
        // Test setting valid values
        assert!(set_config_value(&mut config, "project.name", "Test Project").is_ok());
        assert_eq!(config.project.name, "Test Project");
        
        assert!(set_config_value(&mut config, "ui.emoji", "false").is_ok());
        assert!(!config.ui.emoji);
        
        // Test invalid values
        assert!(set_config_value(&mut config, "project.default_priority", "invalid").is_err());
        assert!(set_config_value(&mut config, "ui.emoji", "not_a_bool").is_err());
    }
}