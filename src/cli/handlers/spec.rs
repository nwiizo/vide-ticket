//! Handlers for spec-driven development commands
//!
//! This module implements all handlers for specification management commands,
//! supporting the three-phase spec-driven development workflow.

use crate::cli::output::OutputFormatter;
use crate::error::{ErrorContext, Result, VibeTicketError};
use crate::specs::{
    SpecDocumentType, SpecManager, SpecPhase, SpecTemplate, Specification, TemplateEngine,
};
use chrono::Utc;
use std::env;
use std::fs;
use std::path::Path;

/// Handle spec init command
pub fn handle_spec_init(
    title: String,
    description: Option<String>,
    ticket: Option<String>,
    tags: Option<String>,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    // Parse tags
    let tag_list: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Create new specification
    let spec = Specification::new(
        title.clone(),
        description.clone().unwrap_or_default(),
        ticket,
        tag_list,
    );

    // Save specification
    spec_manager.save(&spec)?;

    formatter.success(&format!(
        "Created new specification '{}' with ID: {}",
        title, spec.metadata.id
    ));

    if formatter.is_json() {
        formatter.json(&serde_json::json!({
            "status": "success",
            "spec_id": spec.metadata.id,
            "title": title,
            "description": description,
            "ticket_id": spec.metadata.ticket_id,
            "tags": spec.metadata.tags,
        }))?;
    } else {
        formatter.info(&format!("Specification ID: {}", spec.metadata.id));
        if let Some(desc) = description {
            formatter.info(&format!("Description: {}", desc));
        }
        if let Some(ticket_id) = &spec.metadata.ticket_id {
            formatter.info(&format!("Associated ticket: {}", ticket_id));
        }
        formatter.info("\nNext steps:");
        formatter.info("  1. Define requirements: vibe-ticket spec requirements");
        formatter.info("  2. Create design: vibe-ticket spec design");
        formatter.info("  3. Plan tasks: vibe-ticket spec tasks");
    }

    Ok(())
}

/// Handle spec requirements command
pub fn handle_spec_requirements(
    spec: Option<String>,
    editor: bool,
    complete: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    // Get spec ID (from parameter or active spec)
    let spec_id = match spec {
        Some(id) => id,
        None => get_active_spec(&project_dir)?,
    };

    // Load specification
    let mut specification = spec_manager.load(&spec_id)?;

    if complete {
        // Mark requirements phase as complete
        specification.metadata.progress.requirements_completed = true;
        specification.metadata.updated_at = Utc::now();
        spec_manager.save(&specification)?;

        formatter.success(&format!(
            "Marked requirements phase as complete for spec '{}'",
            specification.metadata.title
        ));
        return Ok(());
    }

    // Get or create requirements document
    let doc_path = spec_manager.get_document_path(&spec_id, SpecDocumentType::Requirements);

    if !doc_path.exists() {
        // Create from template
        let mut engine = TemplateEngine::new();
        engine.set_variable("spec_id".to_string(), spec_id.clone());

        let template = SpecTemplate::for_document_type(
            SpecDocumentType::Requirements,
            specification.metadata.title.clone(),
            Some(specification.metadata.description.clone()),
        );

        let content = engine.generate(&template);
        fs::write(&doc_path, content).context("Failed to create requirements document")?;

        formatter.info(&format!(
            "Created requirements document: {}",
            doc_path.display()
        ));
    }

    if editor {
        // Open in editor
        open_in_editor(&doc_path)?;
        formatter.success("Requirements document saved");
    } else {
        // Display content
        let content =
            fs::read_to_string(&doc_path).context("Failed to read requirements document")?;
        formatter.info(&content);
    }

    Ok(())
}

/// Handle spec design command
pub fn handle_spec_design(
    spec: Option<String>,
    editor: bool,
    complete: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    // Get spec ID (from parameter or active spec)
    let spec_id = match spec {
        Some(id) => id,
        None => get_active_spec(&project_dir)?,
    };

    // Load specification
    let mut specification = spec_manager.load(&spec_id)?;

    // Check if requirements are complete
    if !specification.metadata.progress.requirements_completed {
        formatter.warning("Requirements phase is not complete. Consider completing it first.");
    }

    if complete {
        // Mark design phase as complete
        specification.metadata.progress.design_completed = true;
        specification.metadata.updated_at = Utc::now();
        spec_manager.save(&specification)?;

        formatter.success(&format!(
            "Marked design phase as complete for spec '{}'",
            specification.metadata.title
        ));
        return Ok(());
    }

    // Get or create design document
    let doc_path = spec_manager.get_document_path(&spec_id, SpecDocumentType::Design);

    if !doc_path.exists() {
        // Create from template with requirements summary
        let requirements_path =
            spec_manager.get_document_path(&spec_id, SpecDocumentType::Requirements);
        let requirements_summary = if requirements_path.exists() {
            // Extract summary from requirements doc
            "See requirements document for details."
        } else {
            "Requirements not yet defined."
        };

        let mut engine = TemplateEngine::new();
        engine.set_variable("spec_id".to_string(), spec_id.clone());

        let template = SpecTemplate::for_document_type(
            SpecDocumentType::Design,
            specification.metadata.title.clone(),
            Some(requirements_summary.to_string()),
        );

        let content = engine.generate(&template);
        fs::write(&doc_path, content).context("Failed to create design document")?;

        formatter.info(&format!("Created design document: {}", doc_path.display()));
    }

    if editor {
        // Open in editor
        open_in_editor(&doc_path)?;
        formatter.success("Design document saved");
    } else {
        // Display content
        let content = fs::read_to_string(&doc_path).context("Failed to read design document")?;
        formatter.info(&content);
    }

    Ok(())
}

/// Handle spec tasks command
pub fn handle_spec_tasks(
    spec: Option<String>,
    editor: bool,
    complete: bool,
    export_tickets: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    // Get spec ID (from parameter or active spec)
    let spec_id = match spec {
        Some(id) => id,
        None => get_active_spec(&project_dir)?,
    };

    // Load specification
    let mut specification = spec_manager.load(&spec_id)?;

    // Check if design is complete
    if !specification.metadata.progress.design_completed {
        formatter.warning("Design phase is not complete. Consider completing it first.");
    }

    if complete {
        // Mark tasks phase as complete
        specification.metadata.progress.tasks_completed = true;
        specification.metadata.updated_at = Utc::now();
        spec_manager.save(&specification)?;

        formatter.success(&format!(
            "Marked tasks phase as complete for spec '{}'",
            specification.metadata.title
        ));
        return Ok(());
    }

    // Get or create tasks document
    let doc_path = spec_manager.get_document_path(&spec_id, SpecDocumentType::Tasks);

    if !doc_path.exists() {
        // Create from template with design summary
        let design_path = spec_manager.get_document_path(&spec_id, SpecDocumentType::Design);
        let design_summary = if design_path.exists() {
            "See design document for technical details."
        } else {
            "Design not yet defined."
        };

        let mut engine = TemplateEngine::new();
        engine.set_variable("spec_id".to_string(), spec_id.clone());

        let template = SpecTemplate::for_document_type(
            SpecDocumentType::Tasks,
            specification.metadata.title.clone(),
            Some(design_summary.to_string()),
        );

        let content = engine.generate(&template);
        fs::write(&doc_path, content).context("Failed to create tasks document")?;

        formatter.info(&format!("Created tasks document: {}", doc_path.display()));
    }

    if export_tickets {
        // TODO: Implement task export to tickets
        formatter.warning("Task export to tickets is not yet implemented");
    }

    if editor {
        // Open in editor
        open_in_editor(&doc_path)?;
        formatter.success("Tasks document saved");
    } else {
        // Display content
        let content = fs::read_to_string(&doc_path).context("Failed to read tasks document")?;
        formatter.info(&content);
    }

    Ok(())
}

/// Handle spec status command
pub fn handle_spec_status(
    spec: Option<String>,
    detailed: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    // Get spec ID (from parameter or active spec)
    let spec_id = match spec {
        Some(id) => id,
        None => get_active_spec(&project_dir)?,
    };

    // Load specification
    let specification = spec_manager.load(&spec_id)?;

    if formatter.is_json() {
        formatter.json(&serde_json::json!({
            "spec_id": specification.metadata.id,
            "title": specification.metadata.title,
            "status": format!("{:?}", specification.metadata.progress.current_phase()),
            "progress": {
                "requirements": specification.metadata.progress.requirements_completed,
                "design": specification.metadata.progress.design_completed,
                "tasks": specification.metadata.progress.tasks_completed,
            },
            "approval": specification.metadata.progress.approval_status,
        }))?;
    } else {
        formatter.info(&format!(
            "Specification: {} ({})",
            specification.metadata.title, specification.metadata.id
        ));
        formatter.info(&format!(
            "Current Phase: {:?}",
            specification.metadata.progress.current_phase()
        ));

        formatter.info("\nProgress:");
        formatter.info(&format!(
            "  Requirements: {}",
            if specification.metadata.progress.requirements_completed {
                "✓ Complete"
            } else {
                "○ In Progress"
            }
        ));
        formatter.info(&format!(
            "  Design: {}",
            if specification.metadata.progress.design_completed {
                "✓ Complete"
            } else {
                "○ Pending"
            }
        ));
        formatter.info(&format!(
            "  Tasks: {}",
            if specification.metadata.progress.tasks_completed {
                "✓ Complete"
            } else {
                "○ Pending"
            }
        ));

        if detailed {
            formatter.info(&format!("\nCreated: {}", specification.metadata.created_at));
            formatter.info(&format!("Updated: {}", specification.metadata.updated_at));
            if let Some(ticket_id) = &specification.metadata.ticket_id {
                formatter.info(&format!("Ticket: {}", ticket_id));
            }
            if !specification.metadata.tags.is_empty() {
                formatter.info(&format!("Tags: {}", specification.metadata.tags.join(", ")));
            }
        }
    }

    Ok(())
}

/// Handle spec list command
pub fn handle_spec_list(
    status: Option<String>,
    phase: Option<String>,
    _archived: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));
    let specs = spec_manager.list()?;

    // Filter specs
    let filtered_specs: Vec<_> = specs
        .into_iter()
        .filter(|spec| {
            // Filter by status if provided
            if let Some(ref status_filter) = status {
                let current_status = format!("{:?}", spec.progress.current_phase()).to_lowercase();
                if !current_status.contains(&status_filter.to_lowercase()) {
                    return false;
                }
            }

            // Filter by phase if provided
            if let Some(ref phase_filter) = phase {
                match phase_filter.to_lowercase().as_str() {
                    "requirements" => {
                        if spec.progress.requirements_completed {
                            return false;
                        }
                    },
                    "design" => {
                        if !spec.progress.requirements_completed || spec.progress.design_completed {
                            return false;
                        }
                    },
                    "tasks" => {
                        if !spec.progress.design_completed || spec.progress.tasks_completed {
                            return false;
                        }
                    },
                    _ => {},
                }
            }

            true
        })
        .collect();

    if formatter.is_json() {
        let specs_json: Vec<_> = filtered_specs
            .iter()
            .map(|spec| {
                serde_json::json!({
                    "id": spec.id,
                    "title": spec.title,
                    "description": spec.description,
                    "phase": format!("{:?}", spec.progress.current_phase()),
                    "created_at": spec.created_at,
                    "updated_at": spec.updated_at,
                })
            })
            .collect();
        formatter.json(&serde_json::json!(specs_json))?;
    } else if filtered_specs.is_empty() {
        formatter.info("No specifications found");
    } else {
        formatter.info(&format!(
            "Found {} specification(s):\n",
            filtered_specs.len()
        ));

        for spec in &filtered_specs {
            formatter.info(&format!(
                "{} - {} ({:?})",
                spec.id,
                spec.title,
                spec.progress.current_phase()
            ));
        }
    }

    Ok(())
}

/// Handle spec show command
pub fn handle_spec_show(
    spec: String,
    all: bool,
    markdown: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));
    let specification = spec_manager.load(&spec)?;

    if formatter.is_json() {
        formatter.json(&serde_json::json!(specification))?;
    } else {
        formatter.info(&format!(
            "# Specification: {}",
            specification.metadata.title
        ));
        formatter.info(&format!("ID: {}", specification.metadata.id));
        formatter.info(&format!(
            "Description: {}",
            specification.metadata.description
        ));
        formatter.info(&format!(
            "Phase: {:?}",
            specification.metadata.progress.current_phase()
        ));

        if all || markdown {
            // Show all documents
            let doc_types = [
                SpecDocumentType::Requirements,
                SpecDocumentType::Design,
                SpecDocumentType::Tasks,
            ];

            for doc_type in &doc_types {
                let doc_path = spec_manager.get_document_path(&spec, *doc_type);
                if doc_path.exists() {
                    formatter.info(&format!("\n## {:?} Document\n", doc_type));
                    let content =
                        fs::read_to_string(&doc_path).context("Failed to read document")?;
                    formatter.info(&content);
                }
            }
        }
    }

    Ok(())
}

/// Handle spec delete command
pub fn handle_spec_delete(
    spec: String,
    force: bool,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));

    if !force {
        // Confirm deletion
        formatter.warning(&format!(
            "Are you sure you want to delete specification '{}'?",
            spec
        ));
        formatter.warning("This will delete all associated documents and cannot be undone.");
        formatter.info("Use --force to skip this confirmation.");
        return Ok(());
    }

    spec_manager.delete(&spec)?;
    formatter.success(&format!("Deleted specification '{}'", spec));

    Ok(())
}

/// Handle spec approve command
pub fn handle_spec_approve(
    spec: String,
    phase: String,
    message: Option<String>,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    let spec_manager = SpecManager::new(project_dir.join("specs"));
    let mut specification = spec_manager.load(&spec)?;

    // Parse phase
    let phase_enum = match phase.to_lowercase().as_str() {
        "requirements" => SpecPhase::Requirements,
        "design" => SpecPhase::Design,
        "tasks" => SpecPhase::Tasks,
        _ => {
            return Err(VibeTicketError::InvalidInput(
                "Invalid phase. Must be one of: requirements, design, tasks".to_string(),
            ));
        },
    };

    // Update approval status
    if specification.metadata.progress.approval_status.is_none() {
        specification.metadata.progress.approval_status = Some(std::collections::HashMap::new());
    }

    if let Some(ref mut approvals) = specification.metadata.progress.approval_status {
        approvals.insert(
            format!("{:?}", phase_enum),
            serde_json::json!({
                "approved": true,
                "approved_at": Utc::now(),
                "message": message,
            }),
        );
    }

    specification.metadata.updated_at = Utc::now();
    spec_manager.save(&specification)?;

    formatter.success(&format!(
        "Approved {} phase for specification '{}'",
        phase, specification.metadata.title
    ));

    Ok(())
}

/// Handle spec activate command
pub fn handle_spec_activate(
    spec: String,
    project: Option<String>,
    formatter: &OutputFormatter,
) -> Result<()> {
    // Change to project directory if specified
    if let Some(project_path) = project {
        std::env::set_current_dir(&project_path)
            .with_context(|| format!("Failed to change to project directory: {}", project_path))?;
    }

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project_dir = current_dir.join(".vibe-ticket");

    if !project_dir.exists() {
        return Err(VibeTicketError::ProjectNotInitialized);
    }

    // Verify spec exists
    let spec_manager = SpecManager::new(project_dir.join("specs"));
    let specification = spec_manager.load(&spec)?;

    // Save active spec
    let active_spec_path = project_dir.join(".active_spec");
    fs::write(&active_spec_path, &spec).context("Failed to set active specification")?;

    formatter.success(&format!(
        "Set active specification to '{}' ({})",
        specification.metadata.title, spec
    ));

    Ok(())
}

/// Get the active specification ID
fn get_active_spec(project_dir: &Path) -> Result<String> {
    let active_spec_path = project_dir.join(".active_spec");

    if !active_spec_path.exists() {
        return Err(VibeTicketError::NoActiveSpec);
    }

    fs::read_to_string(&active_spec_path)
        .context("Failed to read active specification")
        .map(|s| s.trim().to_string())
}

/// Open a file in the default editor
fn open_in_editor(path: &Path) -> Result<()> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    std::process::Command::new(&editor)
        .arg(path)
        .status()
        .with_context(|| format!("Failed to open editor: {}", editor))?;

    Ok(())
}
