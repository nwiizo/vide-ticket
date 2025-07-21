//! Handler for the `task` command and its subcommands
//!
//! This module implements the logic for managing tasks within tickets,
//! including adding, completing, listing, and removing tasks.

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::{Task, TaskId, TicketId};
use crate::error::{Result, VibeTicketError};
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};
use chrono::Utc;

/// Handler for the `task add` subcommand
///
/// Adds a new task to a ticket.
///
/// # Arguments
///
/// * `title` - Title of the task to add
/// * `ticket_ref` - Optional ticket ID or slug (defaults to active ticket)
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_task_add(
    title: String,
    ticket_ref: Option<String>,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref_str) = ticket_ref {
        resolve_ticket_ref(&storage, &ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Create new task
    let task = Task::new(title.clone());
    ticket.tasks.push(task.clone());

    // Save the updated ticket
    storage.save(&ticket)?;

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "task": {
                "id": task.id.to_string(),
                "title": task.title,
                "completed": task.completed,
            },
            "total_tasks": ticket.tasks.len(),
        }))?;
    } else {
        output.success(&format!("Added task to ticket '{}'", ticket.slug));
        output.info(&format!("Task ID: {}", task.id));
        output.info(&format!("Title: {}", task.title));
        output.info(&format!("Total tasks: {}", ticket.tasks.len()));
    }

    Ok(())
}

/// Handler for the `task complete` subcommand
///
/// Marks a task as completed.
///
/// # Arguments
///
/// * `task_id` - ID of the task to complete
/// * `ticket_ref` - Optional ticket ID or slug (defaults to active ticket)
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_task_complete(
    task_id: String,
    ticket_ref: Option<String>,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref_str) = ticket_ref {
        resolve_ticket_ref(&storage, &ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Parse task ID
    let task_id = TaskId::parse_str(&task_id)
        .map_err(|_| VibeTicketError::custom(format!("Invalid task ID: {}", task_id)))?;

    // Find and complete the task
    let mut task_found = false;
    for task in &mut ticket.tasks {
        if task.id == task_id {
            if task.completed {
                return Err(VibeTicketError::custom("Task is already completed"));
            }
            task.completed = true;
            task.completed_at = Some(Utc::now());
            task_found = true;
            break;
        }
    }

    if !task_found {
        return Err(VibeTicketError::custom(format!(
            "Task '{}' not found in ticket",
            task_id
        )));
    }

    // Save the updated ticket
    storage.save(&ticket)?;

    // Calculate completion stats
    let completed_count = ticket.tasks.iter().filter(|t| t.completed).count();
    let total_count = ticket.tasks.len();

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "task_id": task_id.to_string(),
            "progress": {
                "completed": completed_count,
                "total": total_count,
                "percentage": if total_count > 0 { (completed_count * 100) / total_count } else { 0 },
            }
        }))?;
    } else {
        output.success(&format!("Completed task in ticket '{}'", ticket.slug));
        output.info(&format!(
            "Progress: {}/{} tasks completed",
            completed_count, total_count
        ));

        if completed_count == total_count && total_count > 0 {
            output.info("🎉 All tasks completed!");
        }
    }

    Ok(())
}

/// Handler for the `task uncomplete` subcommand
///
/// Marks a completed task as incomplete.
///
/// # Arguments
///
/// * `task_id` - ID of the task to uncomplete
/// * `ticket_ref` - Optional ticket ID or slug (defaults to active ticket)
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_task_uncomplete(
    task_id: String,
    ticket_ref: Option<String>,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref_str) = ticket_ref {
        resolve_ticket_ref(&storage, &ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Parse task ID
    let task_id = TaskId::parse_str(&task_id)
        .map_err(|_| VibeTicketError::custom(format!("Invalid task ID: {}", task_id)))?;

    // Find and uncomplete the task
    let mut task_found = false;
    for task in &mut ticket.tasks {
        if task.id == task_id {
            if !task.completed {
                return Err(VibeTicketError::custom("Task is not completed"));
            }
            task.completed = false;
            task.completed_at = None;
            task_found = true;
            break;
        }
    }

    if !task_found {
        return Err(VibeTicketError::custom(format!(
            "Task '{}' not found in ticket",
            task_id
        )));
    }

    // Save the updated ticket
    storage.save(&ticket)?;

    // Calculate completion stats
    let completed_count = ticket.tasks.iter().filter(|t| t.completed).count();
    let total_count = ticket.tasks.len();

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "task_id": task_id.to_string(),
            "progress": {
                "completed": completed_count,
                "total": total_count,
                "percentage": if total_count > 0 { (completed_count * 100) / total_count } else { 0 },
            }
        }))?;
    } else {
        output.success(&format!(
            "Marked task as incomplete in ticket '{}'",
            ticket.slug
        ));
        output.info(&format!(
            "Progress: {}/{} tasks completed",
            completed_count, total_count
        ));
    }

    Ok(())
}

/// Handler for the `task list` subcommand
///
/// Lists all tasks in a ticket.
///
/// # Arguments
///
/// * `ticket_ref` - Optional ticket ID or slug (defaults to active ticket)
/// * `completed_only` - Show only completed tasks
/// * `incomplete_only` - Show only incomplete tasks
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_task_list(
    ticket_ref: Option<String>,
    completed_only: bool,
    incomplete_only: bool,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref_str) = ticket_ref {
        resolve_ticket_ref(&storage, &ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let ticket = storage.load(&ticket_id)?;

    // Filter tasks based on flags
    let mut tasks: Vec<&Task> = ticket.tasks.iter().collect();
    if completed_only {
        tasks.retain(|t| t.completed);
    } else if incomplete_only {
        tasks.retain(|t| !t.completed);
    }

    // Calculate stats
    let total_count = ticket.tasks.len();
    let completed_count = ticket.tasks.iter().filter(|t| t.completed).count();

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "ticket_title": ticket.title,
            "progress": {
                "completed": completed_count,
                "total": total_count,
                "percentage": if total_count > 0 { (completed_count * 100) / total_count } else { 0 },
            },
            "tasks": tasks.iter().map(|t| serde_json::json!({
                "id": t.id.to_string(),
                "title": t.title,
                "completed": t.completed,
                "created_at": t.created_at,
                "completed_at": t.completed_at,
            })).collect::<Vec<_>>(),
        }))?;
    } else {
        output.info(&format!("Tasks for ticket: {}", ticket.slug));
        output.info(&format!("Title: {}", ticket.title));
        output.info(&format!(
            "Progress: {}/{} completed",
            completed_count, total_count
        ));

        if tasks.is_empty() {
            output.info("\nNo tasks found");
        } else {
            output.info("\nTasks:");
            for task in tasks {
                let checkbox = if task.completed { "✓" } else { "○" };
                let status = if task.completed { "(completed)" } else { "" };
                output.info(&format!(
                    "  {} [{}] {} {}",
                    checkbox,
                    &task.id.to_string()[..8], // Show first 8 chars of ID
                    task.title,
                    status
                ));
            }
        }
    }

    Ok(())
}

/// Handler for the `task remove` subcommand
///
/// Removes a task from a ticket.
///
/// # Arguments
///
/// * `task_id` - ID of the task to remove
/// * `ticket_ref` - Optional ticket ID or slug (defaults to active ticket)
/// * `force` - Skip confirmation
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_task_remove(
    task_id: String,
    ticket_ref: Option<String>,
    force: bool,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Get the active ticket if no ticket specified
    let ticket_id = if let Some(ref_str) = ticket_ref {
        resolve_ticket_ref(&storage, &ref_str)?
    } else {
        // Get active ticket
        storage
            .get_active()?
            .ok_or(VibeTicketError::NoActiveTicket)?
    };

    // Load the ticket
    let mut ticket = storage.load(&ticket_id)?;

    // Parse task ID
    let task_id = TaskId::parse_str(&task_id)
        .map_err(|_| VibeTicketError::custom(format!("Invalid task ID: {}", task_id)))?;

    // Find the task
    let task_index = ticket
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .ok_or_else(|| {
            VibeTicketError::custom(format!("Task '{}' not found in ticket", task_id))
        })?;

    let task = &ticket.tasks[task_index];

    // Confirm removal if not forced
    if !force {
        output.warning(&format!(
            "Are you sure you want to remove task: '{}'?",
            task.title
        ));
        output.info("Use --force to skip this confirmation");
        return Ok(());
    }

    // Remove the task
    let removed_task = ticket.tasks.remove(task_index);

    // Save the updated ticket
    storage.save(&ticket)?;

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "status": "success",
            "ticket_id": ticket.id.to_string(),
            "ticket_slug": ticket.slug,
            "removed_task": {
                "id": removed_task.id.to_string(),
                "title": removed_task.title,
                "was_completed": removed_task.completed,
            },
            "remaining_tasks": ticket.tasks.len(),
        }))?;
    } else {
        output.success(&format!("Removed task from ticket '{}'", ticket.slug));
        output.info(&format!("Removed: {}", removed_task.title));
        output.info(&format!("Remaining tasks: {}", ticket.tasks.len()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test task".to_string());
        assert_eq!(task.title, "Test task");
        assert!(!task.completed);
        assert!(task.completed_at.is_none());
    }
}
