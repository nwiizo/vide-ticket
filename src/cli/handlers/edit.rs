//! Handler for the `edit` command
//!
//! This module implements the logic for editing ticket properties,
//! including title, description, priority, status, and tags.

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::{Priority, Status, Ticket, TicketId};
use crate::error::{Result, VibeTicketError};
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};

/// Parameters for the edit command
pub struct EditParams<'a> {
    /// Optional ticket ID or slug (defaults to active ticket)
    pub ticket_ref: Option<String>,
    /// New title for the ticket
    pub title: Option<String>,
    /// New description for the ticket
    pub description: Option<String>,
    /// New priority for the ticket
    pub priority: Option<String>,
    /// New status for the ticket
    pub status: Option<String>,
    /// Tags to add (comma-separated)
    pub add_tags: Option<String>,
    /// Tags to remove (comma-separated)
    pub remove_tags: Option<String>,
    /// Whether to open in the default editor
    pub editor: bool,
    /// Optional project directory path
    pub project_dir: Option<&'a str>,
}

/// Handler for the `edit` command
///
/// This function allows editing various properties of a ticket:
/// 1. Title
/// 2. Description
/// 3. Priority
/// 4. Status
/// 5. Tags (add/remove)
/// 6. Opens in editor if requested
///
/// # Arguments
///
/// * `params` - Edit command parameters
/// * `output` - Output formatter for displaying results
///
/// # Errors
///
/// Returns an error if:
/// - The project is not initialized
/// - No ticket is specified and there's no active ticket
/// - The ticket is not found
/// - Invalid priority or status values are provided
pub fn handle_edit_command(params: EditParams<'_>, output: &OutputFormatter) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(params.project_dir)?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref ref_str) = params.ticket_ref {
        resolve_ticket_ref(&storage, ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Track what was changed
    let mut changes = Vec::new();

    // Open in editor if requested
    if params.editor {
        edit_in_editor(&mut ticket, &storage, output)?;
        return Ok(());
    }

    // Apply updates
    update_ticket_fields(&mut ticket, &params, &mut changes)?;

    // Check if any changes were made
    if changes.is_empty() {
        output.warning("No changes specified");
        return Ok(());
    }

    // Save the updated ticket
    storage.save(&ticket)?;

    // Output results
    output_edit_results(output, &ticket, &changes)?;

    Ok(())
}

/// Update ticket fields based on parameters
fn update_ticket_fields(
    ticket: &mut Ticket,
    params: &EditParams<'_>,
    changes: &mut Vec<String>,
) -> Result<()> {
    // Update title
    if let Some(new_title) = &params.title {
        changes.push(format!("title: {} → {}", ticket.title, new_title));
        ticket.title = new_title.clone();
    }

    // Update description
    if let Some(new_desc) = &params.description {
        let old_desc = if ticket.description.is_empty() {
            "(empty)"
        } else if ticket.description.len() > 50 {
            &format!("{}...", &ticket.description[..50])
        } else {
            &ticket.description
        };
        changes.push(format!("description: {} → {}", old_desc, new_desc));
        ticket.description = new_desc.clone();
    }

    // Update priority
    if let Some(priority_str) = &params.priority {
        let new_priority = Priority::try_from(priority_str.as_str())
            .map_err(|_| VibeTicketError::InvalidPriority {
                priority: priority_str.clone(),
            })?;
        if new_priority != ticket.priority {
            changes.push(format!(
                "priority: {} → {}",
                ticket.priority, new_priority
            ));
            ticket.priority = new_priority;
        }
    }

    // Update status
    if let Some(status_str) = &params.status {
        let new_status = Status::try_from(status_str.as_str())
            .map_err(|_| VibeTicketError::InvalidStatus {
                status: status_str.clone(),
            })?;
        if new_status != ticket.status {
            changes.push(format!("status: {} → {}", ticket.status, new_status));
            ticket.status = new_status;
        }
    }

    // Add tags
    if let Some(add_tags_str) = &params.add_tags {
        let tags_to_add = super::parse_tags(Some(add_tags_str.clone()));
        for tag in tags_to_add {
            if !ticket.tags.contains(&tag) {
                ticket.tags.push(tag.clone());
                changes.push(format!("added tag: {}", tag));
            }
        }
    }

    // Remove tags
    if let Some(remove_tags_str) = &params.remove_tags {
        let tags_to_remove = super::parse_tags(Some(remove_tags_str.clone()));
        for tag in &tags_to_remove {
            if let Some(pos) = ticket.tags.iter().position(|t| t == tag) {
                ticket.tags.remove(pos);
                changes.push(format!("removed tag: {}", tag));
            }
        }
    }

    Ok(())
}

/// Output edit results in the appropriate format
fn output_edit_results(
    output: &OutputFormatter,
    ticket: &Ticket,
    changes: &[String],
) -> Result<()> {
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "success": true,
            "message": "Updated ticket",
            "ticket": ticket,
            "changes": changes,
        }))?;
    } else {
        output.success(&format!("Updated ticket: {}", ticket.slug));
        output.info("");
        for change in changes {
            output.info(&format!("  • {}", change));
        }
    }
    Ok(())
}

/// Resolve a ticket reference (ID or slug) to a ticket ID
fn resolve_ticket_ref(storage: &FileStorage, ticket_ref: &str) -> Result<TicketId> {
    // First try to parse as ticket ID
    if let Ok(ticket_id) = TicketId::parse_str(ticket_ref) {
        // Verify the ticket exists
        if storage.load(&ticket_id).is_ok() {
            return Ok(ticket_id);
        }
    }

    // Try to find by slug
    let all_tickets = storage.load_all()?;
    for ticket in all_tickets {
        if ticket.slug == ticket_ref {
            return Ok(ticket.id);
        }
    }

    Err(VibeTicketError::TicketNotFound {
        id: ticket_ref.to_string(),
    })
}

/// Edit ticket in the default editor
fn edit_in_editor(
    ticket: &mut crate::core::Ticket,
    storage: &FileStorage,
    output: &OutputFormatter,
) -> Result<()> {
    use std::io::Write as IoWrite;
    use std::process::Command;

    // Create a temporary file with the ticket content
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("vibe-ticket-{}.yaml", ticket.id));

    // Serialize ticket to YAML
    let yaml_content = serde_yaml::to_string(&ticket)
        .map_err(|e| VibeTicketError::custom(format!("Failed to serialize ticket: {e}")))?;

    // Write to temporary file
    let mut file = std::fs::File::create(&temp_file)
        .map_err(|e| VibeTicketError::custom(format!("Failed to create temp file: {e}")))?;
    file.write_all(yaml_content.as_bytes())
        .map_err(|e| VibeTicketError::custom(format!("Failed to write temp file: {e}")))?;

    // Get editor from environment
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    // Open editor
    let status = Command::new(&editor)
        .arg(&temp_file)
        .status()
        .map_err(|e| VibeTicketError::custom(format!("Failed to launch editor: {e}")))?;

    if !status.success() {
        return Err(VibeTicketError::custom("Editor exited with error"));
    }

    // Read the edited content
    let edited_content = std::fs::read_to_string(&temp_file)
        .map_err(|e| VibeTicketError::custom(format!("Failed to read edited file: {e}")))?;

    // Parse the edited ticket
    let edited_ticket: crate::core::Ticket = serde_yaml::from_str(&edited_content)
        .map_err(|e| VibeTicketError::custom(format!("Failed to parse edited ticket: {e}")))?;

    // Update the original ticket
    *ticket = edited_ticket;

    // Save the updated ticket
    storage.save(ticket)?;

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);

    output.success(&format!("Updated ticket: {}", ticket.slug));

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_tag_parsing() {
        let tags_str = "bug, ui, urgent";
        let tags: Vec<String> = tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        assert_eq!(tags, vec!["bug", "ui", "urgent"]);
    }
}
