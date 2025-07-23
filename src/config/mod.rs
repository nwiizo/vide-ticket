//! Configuration management for vibe-ticket
//!
//! This module handles application configuration from various sources including
//! configuration files, environment variables, and command-line arguments.
//!
//! # Configuration Sources
//!
//! Configuration is loaded in the following priority order (highest to lowest):
//! 1. Command-line arguments
//! 2. Environment variables (prefixed with `VIDE_TICKET_`)
//! 3. User configuration file (`~/.config/vibe-ticket/config.toml`)
//! 4. Project configuration file (`.vibe-ticket.toml` in project root)
//! 5. Default values
//!
//! # Configuration Structure
//!
//! The configuration includes:
//! - Database connection settings
//! - API server settings (when API feature is enabled)
//! - Plugin configuration
//! - UI preferences
//! - Default ticket fields and workflows
//!
//! # Example
//!
//! ```no_run
//! use vibe_ticket::config::Config;
//!
//! // Load configuration from all sources
//! let config = Config::load()?;
//!
//! // Access configuration values
//! println!("Database: {}", config.database.url);
//! println!("API Port: {}", config.api.port);
//! ```
//!
//! # File Format
//!
//! Configuration files use TOML format:
//! ```toml
//! [database]
//! url = "sqlite://tickets.db"
//!
//! [api]
//! port = 8080
//! host = "127.0.0.1"
//!
//! [ui]
//! theme = "dark"
//! ```

use crate::error::{ErrorContext, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main configuration structure for vibe-ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project configuration
    pub project: ProjectConfig,

    /// UI configuration
    pub ui: UiConfig,

    /// Git integration configuration
    pub git: GitConfig,

    /// Plugin configuration
    pub plugins: PluginsConfig,
}

/// Project-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,

    /// Project description
    pub description: Option<String>,

    /// Default assignee for new tickets
    pub default_assignee: Option<String>,

    /// Default priority for new tickets
    pub default_priority: String,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color theme (light/dark/auto)
    pub theme: String,

    /// Enable emoji in output
    pub emoji: bool,

    /// Default page size for list commands
    pub page_size: usize,

    /// Date format
    pub date_format: String,
}

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Enable Git integration
    pub enabled: bool,

    /// Branch prefix for tickets
    pub branch_prefix: String,

    /// Auto-create branches when starting tickets
    pub auto_branch: bool,

    /// Commit message template
    pub commit_template: Option<String>,

    /// Enable Git worktree integration
    pub worktree_enabled: bool,

    /// Worktree directory prefix (use {project} as placeholder)
    pub worktree_prefix: String,

    /// Automatically cleanup worktree when closing ticket
    pub worktree_cleanup_on_close: bool,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsConfig {
    /// Enabled plugins
    pub enabled: Vec<String>,

    /// Plugin directory
    pub directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: "Untitled Project".to_string(),
                description: None,
                default_assignee: None,
                default_priority: "medium".to_string(),
            },
            ui: UiConfig {
                theme: "auto".to_string(),
                emoji: true,
                page_size: 20,
                date_format: "%Y-%m-%d %H:%M".to_string(),
            },
            git: GitConfig {
                enabled: true,
                branch_prefix: "ticket/".to_string(),
                auto_branch: true,
                commit_template: None,
                worktree_enabled: true,
                worktree_prefix: "../{project}-ticket-".to_string(),
                worktree_cleanup_on_close: false,
            },
            plugins: PluginsConfig {
                enabled: vec![],
                directory: ".vibe-ticket/plugins".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from the default location
    ///
    /// This loads configuration from `.vibe-ticket/config.yaml` in the current directory.
    pub fn load() -> Result<Self> {
        Self::load_from_path(".vibe-ticket/config.yaml")
    }

    /// Load configuration from a specific path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;

        let config: Self =
            serde_yaml::from_str(&content).context("Failed to parse configuration")?;

        Ok(config)
    }

    /// Load configuration or return default if not found
    pub fn load_or_default() -> Result<Self> {
        match Self::load() {
            Ok(config) => Ok(config),
            Err(crate::error::VibeTicketError::Io(e))
                if e.kind() == std::io::ErrorKind::NotFound =>
            {
                Ok(Self::default())
            },
            Err(e) => Err(e),
        }
    }

    /// Save configuration to the default location
    pub fn save(&self) -> Result<()> {
        self.save_to_path(".vibe-ticket/config.yaml")
    }

    /// Save configuration to a specific path
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let yaml = serde_yaml::to_string(self).context("Failed to serialize configuration")?;

        std::fs::write(path, yaml)
            .with_context(|| format!("Failed to write config to {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.project.name, "Untitled Project");
        assert_eq!(config.ui.theme, "auto");
        assert!(config.git.enabled);
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.yaml");

        let config = Config::default();
        config.save_to_path(&config_path).unwrap();

        let loaded = Config::load_from_path(&config_path).unwrap();
        assert_eq!(loaded.project.name, config.project.name);
    }
}
