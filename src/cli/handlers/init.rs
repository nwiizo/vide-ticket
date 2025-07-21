//! Handler for the `init` command
//!
//! This module implements the initialization of a new vide-ticket project,
//! creating the necessary directory structure and configuration files.

use crate::cli::output::OutputFormatter;
use crate::config::Config;
use crate::error::{ErrorContext, Result, VideTicketError};
use crate::storage::{FileStorage, ProjectState};
use std::env;
use std::fs;
use std::path::Path;

/// Handle the init command
///
/// Initializes a new vide-ticket project in the current directory by:
/// 1. Creating the `.vide-ticket` directory structure
/// 2. Generating default configuration
/// 3. Setting up the storage repository
/// 4. Creating necessary subdirectories
///
/// # Arguments
///
/// * `name` - Optional project name (defaults to current directory name)
/// * `description` - Optional project description
/// * `force` - Force initialization even if already initialized
/// * `formatter` - Output formatter for displaying results
///
/// # Errors
///
/// Returns an error if:
/// - The project is already initialized (unless `force` is true)
/// - File system operations fail
/// - Configuration cannot be saved
///
/// # Example
///
/// ```no_run
/// use vide_ticket::cli::handlers::init::handle_init;
/// use vide_ticket::cli::output::OutputFormatter;
///
/// let formatter = OutputFormatter::new(false, false);
/// handle_init(Some("my-project".to_string()), None, false, &formatter)?;
/// ```
pub fn handle_init(
    name: Option<String>,
    description: Option<String>,
    force: bool,
    formatter: &OutputFormatter,
) -> Result<()> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vide-ticket");
    
    // Check if already initialized
    if project_dir.exists() && !force {
        return Err(VideTicketError::ProjectAlreadyInitialized {
            path: project_dir,
        });
    }
    
    // Determine project name
    let project_name = name.unwrap_or_else(|| {
        current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("vide-ticket-project")
            .to_string()
    });
    
    // Start progress
    let progress = formatter.progress_bar("Initializing project");
    progress.set_message("Creating directory structure");
    
    // Create directory structure
    create_directory_structure(&project_dir)?;
    
    progress.set_message("Creating configuration");
    
    // Create default configuration
    let mut config = Config::default();
    config.project.name = project_name.clone();
    config.project.description = description.clone();
    
    // Save configuration
    let config_path = project_dir.join("config.yaml");
    let config_content = serde_yaml::to_string(&config)
        .context("Failed to serialize configuration")?;
    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write config to {:?}", config_path))?;
    
    progress.set_message("Initializing repository");
    
    // Initialize storage with project state
    let storage = FileStorage::new(&project_dir);
    let project_state = ProjectState {
        name: project_name.clone(),
        description: description.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        ticket_count: 0,
    };
    storage.save_state(&project_state)?;
    
    progress.set_message("Creating default templates");
    
    // Create default templates
    create_default_templates(&project_dir)?;
    
    // Create .gitignore if it doesn't exist
    create_gitignore(&current_dir)?;
    
    progress.finish_with_message("Project initialized successfully");
    
    // Display success message
    formatter.success(&format!(
        "Initialized vide-ticket project '{}'",
        project_name
    ));
    
    if formatter.is_json() {
        formatter.json(&serde_json::json!({
            "status": "success",
            "project_name": project_name,
            "project_path": current_dir,
            "config_path": config_path,
            "description": description,
        }))?;
    } else {
        formatter.info(&format!("Project directory: {}", current_dir.display()));
        if let Some(desc) = description {
            formatter.info(&format!("Description: {}", desc));
        }
        formatter.info("\nNext steps:");
        formatter.info("  1. Create your first ticket: vide-ticket new <slug>");
        formatter.info("  2. List tickets: vide-ticket list");
        formatter.info("  3. Start working: vide-ticket start <ticket>");
    }
    
    Ok(())
}

/// Create the vide-ticket directory structure
///
/// Creates all necessary subdirectories for the project:
/// - `.vide-ticket/` - Main project directory
/// - `.vide-ticket/tickets/` - Ticket storage
/// - `.vide-ticket/templates/` - Custom templates
/// - `.vide-ticket/plugins/` - Plugin directory
/// - `.vide-ticket/backups/` - Backup directory
fn create_directory_structure(project_dir: &Path) -> Result<()> {
    let directories = [
        project_dir,
        &project_dir.join("tickets"),
        &project_dir.join("templates"),
        &project_dir.join("plugins"),
        &project_dir.join("backups"),
    ];
    
    for dir in directories {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory {:?}", dir))?;
    }
    
    Ok(())
}

/// Create default templates
///
/// Creates starter templates for tickets and other documents
fn create_default_templates(project_dir: &Path) -> Result<()> {
    let templates_dir = project_dir.join("templates");
    
    // Default ticket template
    let ticket_template = r#"# {{ title }}

## Description
{{ description }}

## Acceptance Criteria
- [ ] 

## Technical Notes


## Related Tickets
- 

---
Created: {{ created_at }}
Status: {{ status }}
Priority: {{ priority }}
"#;
    
    fs::write(
        templates_dir.join("ticket.md"),
        ticket_template,
    ).context("Failed to create ticket template")?;
    
    // Default PR template
    let pr_template = r#"## Summary
{{ summary }}

## Related Ticket
Closes #{{ ticket_id }} - {{ ticket_title }}

## Changes
- 

## Testing
- [ ] Unit tests pass
- [ ] Manual testing completed
- [ ] Documentation updated

## Screenshots (if applicable)

"#;
    
    fs::write(
        templates_dir.join("pull_request.md"),
        pr_template,
    ).context("Failed to create PR template")?;
    
    Ok(())
}

/// Create or update .gitignore file
///
/// Adds vide-ticket specific entries to .gitignore
fn create_gitignore(project_dir: &Path) -> Result<()> {
    let gitignore_path = project_dir.join(".gitignore");
    let vide_entries = vec![
        "# vide-ticket",
        ".vide-ticket/backups/",
        ".vide-ticket/tmp/",
        ".vide-ticket/*.log",
        "",
    ];
    
    if gitignore_path.exists() {
        // Read existing content
        let mut content = fs::read_to_string(&gitignore_path)
            .context("Failed to read .gitignore")?;
        
        // Check if vide-ticket entries already exist
        if !content.contains("# vide-ticket") {
            // Append our entries
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&vide_entries.join("\n"));
            
            fs::write(&gitignore_path, content)
                .context("Failed to update .gitignore")?;
        }
    } else {
        // Create new .gitignore
        fs::write(&gitignore_path, vide_entries.join("\n"))
            .context("Failed to create .gitignore")?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_directory_structure() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join(".vide-ticket");
        
        create_directory_structure(&project_dir).unwrap();
        
        assert!(project_dir.exists());
        assert!(project_dir.join("tickets").exists());
        assert!(project_dir.join("templates").exists());
        assert!(project_dir.join("plugins").exists());
        assert!(project_dir.join("backups").exists());
    }
    
    #[test]
    fn test_create_default_templates() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join(".vide-ticket");
        create_directory_structure(&project_dir).unwrap();
        
        create_default_templates(&project_dir).unwrap();
        
        assert!(project_dir.join("templates/ticket.md").exists());
        assert!(project_dir.join("templates/pull_request.md").exists());
    }
    
    #[test]
    fn test_create_gitignore() {
        let temp_dir = TempDir::new().unwrap();
        
        create_gitignore(temp_dir.path()).unwrap();
        
        let gitignore_path = temp_dir.path().join(".gitignore");
        assert!(gitignore_path.exists());
        
        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(content.contains("# vide-ticket"));
        assert!(content.contains(".vide-ticket/backups/"));
    }
}