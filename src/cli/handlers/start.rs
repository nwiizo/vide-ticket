//! Handler for the `start` command
//!
//! This module implements the logic for starting work on a ticket,
//! including Git branch creation and status updates.

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::{Status, TicketId};
use crate::error::{Result, VideTicketError};
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};
use chrono::Utc;

/// Handler for the `start` command
///
/// This function performs the following operations:
/// 1. Loads the specified ticket
/// 2. Updates the ticket status to "doing"
/// 3. Sets the ticket as active
/// 4. Optionally creates a Git branch for the ticket
///
/// # Arguments
///
/// * `ticket_ref` - Ticket ID or slug to start
/// * `create_branch` - Whether to create a Git branch
/// * `branch_name` - Optional custom branch name
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
///
/// # Errors
///
/// Returns an error if:
/// - The project is not initialized
/// - The ticket is not found
/// - Git operations fail
/// - The ticket is already in progress
pub fn handle_start_command(
    ticket_ref: String,
    create_branch: bool,
    branch_name: Option<String>,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vide_ticket_dir = project_root.join(".vide-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vide_ticket_dir);

    // Resolve ticket ID from reference (ID or slug)
    let ticket_id = resolve_ticket_ref(&storage, &ticket_ref)?;

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Check if ticket is already in progress
    if ticket.status == Status::Doing {
        return Err(VideTicketError::custom(format!(
            "Ticket '{}' is already in progress",
            ticket.slug
        )));
    }

    // Update ticket status and start time
    ticket.status = Status::Doing;
    ticket.started_at = Some(Utc::now());

    // Save the updated ticket
    storage.save(&ticket)?;

    // Set as active ticket
    storage.set_active(&ticket_id)?;

    // Create Git branch if requested
    let branch_name_final = if create_branch {
        let branch_name = branch_name.unwrap_or_else(|| format!("ticket/{}", ticket.slug));

        create_git_branch(&project_root, &branch_name, output)?;
        Some(branch_name)
    } else {
        None
    };

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "ticket": {
                "id": ticket.id.to_string(),
                "slug": ticket.slug,
                "title": ticket.title,
                "status": ticket.status.to_string(),
                "started_at": ticket.started_at,
            },
            "branch_created": create_branch,
            "branch_name": branch_name_final,
        }))?;
    } else {
        output.success(&format!("Started working on ticket: {}", ticket.slug));
        output.info(&format!("Title: {}", ticket.title));
        output.info(&format!("Status: {} → {}", Status::Todo, Status::Doing));

        if let Some(branch) = branch_name_final {
            output.info(&format!("Git branch created: {}", branch));
        }

        output.info(&format!("\nTicket '{}' is now active.", ticket.slug));
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

    Err(VideTicketError::TicketNotFound {
        id: ticket_ref.to_string(),
    })
}

/// Create a Git branch for the ticket
fn create_git_branch(
    project_root: &std::path::Path,
    branch_name: &str,
    output: &OutputFormatter,
) -> Result<()> {
    // Temporarily use git command instead of git2 library due to linking issues
    use std::process::Command;

    // Check if we're in a git repository
    let status = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .current_dir(project_root)
        .output()
        .map_err(|e| VideTicketError::custom(format!("Failed to run git command: {}", e)))?;

    if !status.status.success() {
        return Err(VideTicketError::custom("Not in a Git repository"));
    }

    // Check if branch already exists
    let check_branch = Command::new("git")
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!("refs/heads/{}", branch_name))
        .current_dir(project_root)
        .output()
        .map_err(|e| VideTicketError::custom(format!("Failed to check branch existence: {}", e)))?;

    if check_branch.status.success() {
        return Err(VideTicketError::custom(format!(
            "Branch '{}' already exists",
            branch_name
        )));
    }

    // Create and checkout the new branch
    let create_branch = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .current_dir(project_root)
        .output()
        .map_err(|e| VideTicketError::custom(format!("Failed to create branch: {}", e)))?;

    if !create_branch.status.success() {
        let error_msg = String::from_utf8_lossy(&create_branch.stderr);
        return Err(VideTicketError::custom(format!(
            "Failed to create branch: {}",
            error_msg
        )));
    }

    output.success(&format!("Switched to new branch '{}'", branch_name));

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_branch_name_generation() {
        let default_name = "test-ticket";
        let branch_name = format!("ticket/{}", default_name);
        assert_eq!(branch_name, "ticket/test-ticket");
    }
}
