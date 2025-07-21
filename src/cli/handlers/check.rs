//! Handler for the `check` command
//!
//! This module implements the logic for checking the current project status,
//! including active ticket information and project statistics.

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::{Status, Ticket};
use crate::error::Result;
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};
use chrono::{DateTime, Local, Utc};

/// Handler for the `check` command
///
/// This function displays:
/// 1. Project information
/// 2. Active ticket details (if any)
/// 3. Current Git branch
/// 4. Project statistics (optional)
/// 5. Recent tickets (in detailed mode)
///
/// # Arguments
///
/// * `detailed` - Whether to show detailed information
/// * `stats` - Whether to include statistics
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
///
/// # Errors
///
/// Returns an error if:
/// - The project is not initialized
/// - File I/O operations fail
pub fn handle_check_command(
    detailed: bool,
    stats: bool,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Load project state
    let project_state = storage.load_state()?;

    // Get active ticket
    let active_ticket_id = storage.get_active()?;
    let active_ticket = if let Some(id) = &active_ticket_id {
        Some(storage.load(id)?)
    } else {
        None
    };

    // Get current Git branch
    let current_branch = get_current_git_branch(&project_root);

    // Get statistics if requested
    let statistics = if stats || detailed {
        Some(calculate_statistics(&storage)?)
    } else {
        None
    };

    // Get recent tickets if detailed
    let recent_tickets = if detailed {
        get_recent_tickets(&storage, 5)?
    } else {
        vec![]
    };

    // Output results
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "project": {
                "name": project_state.name,
                "description": project_state.description,
                "created_at": project_state.created_at,
                "path": project_root,
            },
            "active_ticket": active_ticket.as_ref().map(|t| serde_json::json!({
                "id": t.id.to_string(),
                "slug": t.slug,
                "title": t.title,
                "status": t.status.to_string(),
                "priority": t.priority.to_string(),
                "started_at": t.started_at,
            })),
            "git_branch": current_branch,
            "statistics": statistics,
            "recent_tickets": recent_tickets.iter().map(|t| serde_json::json!({
                "id": t.id.to_string(),
                "slug": t.slug,
                "title": t.title,
                "status": t.status.to_string(),
            })).collect::<Vec<_>>(),
        }))?;
    } else {
        // Display project information
        output.info(&format!("Project: {}", project_state.name));
        if let Some(desc) = &project_state.description {
            output.info(&format!("Description: {desc}"));
        }
        output.info(&format!("Path: {}", project_root.display()));
        output.info(&format!(
            "Created: {}",
            format_datetime(project_state.created_at)
        ));

        // Display Git branch
        if let Some(branch) = &current_branch {
            output.info(&format!("Git branch: {branch}"));
        }

        output.info("");

        // Display active ticket
        if let Some(ticket) = &active_ticket {
            output.success("Active Ticket:");
            output.info(&format!("  ID: {}", ticket.id));
            output.info(&format!("  Slug: {}", ticket.slug));
            output.info(&format!("  Title: {}", ticket.title));
            output.info(&format!("  Status: {}", ticket.status));
            output.info(&format!("  Priority: {}", ticket.priority));

            if let Some(started_at) = ticket.started_at {
                let duration = Utc::now() - started_at;
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                output.info(&format!("  Time spent: {hours}h {minutes}m"));
            }

            if !ticket.tasks.is_empty() {
                let completed = ticket.tasks.iter().filter(|t| t.completed).count();
                output.info(&format!("  Tasks: {}/{}", completed, ticket.tasks.len()));
            }
        } else {
            output.info("No active ticket");
        }

        // Display statistics
        if let Some(stats) = &statistics {
            output.info("");
            output.info("Statistics:");
            output.info(&format!("  Total tickets: {}", stats.total));
            output.info(&format!("  Todo: {}", stats.todo));
            output.info(&format!("  In progress: {}", stats.doing));
            output.info(&format!("  In review: {}", stats.review));
            output.info(&format!("  Blocked: {}", stats.blocked));
            output.info(&format!("  Done: {}", stats.done));

            if detailed {
                output.info("");
                output.info("Priority breakdown:");
                output.info(&format!("  Critical: {}", stats.critical));
                output.info(&format!("  High: {}", stats.high));
                output.info(&format!("  Medium: {}", stats.medium));
                output.info(&format!("  Low: {}", stats.low));
            }
        }

        // Display recent tickets in detailed mode
        if detailed && !recent_tickets.is_empty() {
            output.info("");
            output.info("Recent tickets:");
            for ticket in &recent_tickets {
                let status_emoji = match ticket.status {
                    Status::Todo => "📋",
                    Status::Doing => "🔄",
                    Status::Review => "👀",
                    Status::Blocked => "🚫",
                    Status::Done => "✅",
                };
                output.info(&format!(
                    "  {} {} - {} ({})",
                    status_emoji, ticket.slug, ticket.title, ticket.priority
                ));
            }
        }
    }

    Ok(())
}

/// Project statistics
#[derive(Debug, serde::Serialize)]
struct Statistics {
    total: usize,
    todo: usize,
    doing: usize,
    review: usize,
    blocked: usize,
    done: usize,
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
}

/// Calculate project statistics
fn calculate_statistics(storage: &FileStorage) -> Result<Statistics> {
    let tickets = storage.load_all()?;

    let mut stats = Statistics {
        total: tickets.len(),
        todo: 0,
        doing: 0,
        review: 0,
        blocked: 0,
        done: 0,
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
    };

    for ticket in &tickets {
        // Count by status
        match ticket.status {
            Status::Todo => stats.todo += 1,
            Status::Doing => stats.doing += 1,
            Status::Review => stats.review += 1,
            Status::Blocked => stats.blocked += 1,
            Status::Done => stats.done += 1,
        }

        // Count by priority
        match ticket.priority {
            crate::core::Priority::Critical => stats.critical += 1,
            crate::core::Priority::High => stats.high += 1,
            crate::core::Priority::Medium => stats.medium += 1,
            crate::core::Priority::Low => stats.low += 1,
        }
    }

    Ok(stats)
}

/// Get recent tickets sorted by creation date
fn get_recent_tickets(storage: &FileStorage, limit: usize) -> Result<Vec<Ticket>> {
    let mut tickets = storage.load_all()?;

    // Sort by creation date (descending)
    tickets.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Take the specified limit
    tickets.truncate(limit);

    Ok(tickets)
}

/// Get current Git branch name
fn get_current_git_branch(project_root: &std::path::Path) -> Option<String> {
    use std::process::Command;

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(project_root)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// Format datetime for display
fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&Local)
        .format("%Y-%m-%d %H:%M")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime() {
        let dt = Utc::now();
        let formatted = format_datetime(dt);
        assert!(!formatted.is_empty());
    }
}
