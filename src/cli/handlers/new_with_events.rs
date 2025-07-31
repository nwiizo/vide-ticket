//! New command handler with event support

use crate::cli::output::OutputFormatter;
use crate::cli::utils::find_project_root;
use crate::core::{Priority, Ticket};
use crate::error::{Result, VibeTicketError};
use crate::events::{emit_event, TicketEvent};
use crate::storage::{FileStorage, TicketRepository};
use std::convert::TryFrom;

/// Handler for the `new` command with event emission
#[allow(clippy::too_many_arguments)]
pub async fn handle_new_command_async(
    slug: &str,
    title: Option<String>,
    description: Option<String>,
    priority: &str,
    tags: Option<String>,
    start: bool,
    project_dir: Option<&str>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir)?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Generate timestamp prefix for the slug
    let now = chrono::Local::now();
    let timestamp_prefix = now.format("%Y%m%d%H%M").to_string();

    // Validate and normalize the slug
    let base_slug = slug.trim();
    crate::cli::utils::validate_slug(base_slug)?;

    // Combine timestamp and slug
    let slug = format!("{timestamp_prefix}-{base_slug}");

    // Check if a ticket with this slug already exists
    if storage.ticket_exists_with_slug(&slug)? {
        return Err(VibeTicketError::DuplicateTicket { slug });
    }

    // Parse priority
    let priority = Priority::try_from(priority).map_err(|_| VibeTicketError::InvalidPriority {
        priority: priority.to_string(),
    })?;

    // Parse tags
    let tags = tags.map(|t| crate::cli::utils::parse_tags(Some(t))).unwrap_or_default();

    // Create title from base slug if not provided
    let title = title.unwrap_or_else(|| {
        base_slug
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    });

    // Create the ticket
    let mut ticket = Ticket::new(&slug, &title);
    ticket.description = description.unwrap_or_default();
    ticket.priority = priority;
    ticket.tags = tags;

    // Save the ticket
    storage.save(&ticket)?;

    // Emit the created event
    emit_event(TicketEvent::Created(ticket.clone())).await?;

    // If --start flag is provided, start working on the ticket immediately
    if start {
        let old_status = ticket.status.clone();
        ticket.start();
        storage.save(&ticket)?;
        storage.set_active(&ticket.id)?;

        // Emit status changed event
        emit_event(TicketEvent::StatusChanged(
            ticket.id.clone(),
            old_status,
            ticket.status.clone(),
        ))
        .await?;

        if output.is_json() {
            output.print_json(&serde_json::json!({
                "success": true,
                "message": "Created and started ticket",
                "ticket": ticket,
            }))?;
        } else {
            output.success(&format!(
                "Created ticket '{}' (ID: {})",
                ticket.slug,
                ticket.id.short()
            ));
            output.info(&format!("Started working on ticket '{}'", ticket.slug));

            // TODO: Create Git branch when Git integration is implemented
            output.info("Note: Git branch creation will be available in future version");
        }
    } else if output.is_json() {
        output.print_json(&serde_json::json!({
            "success": true,
            "message": "Created ticket",
            "ticket": ticket,
        }))?;
    } else {
        output.success(&format!(
            "Created ticket '{}' (ID: {})",
            ticket.slug,
            ticket.id.short()
        ));
        output.info(&format!("Title: {}", ticket.title));
        output.info(&format!("Priority: {}", ticket.priority));
        if !ticket.tags.is_empty() {
            output.info(&format!("Tags: {}", ticket.tags.join(", ")));
        }
        output.info("");
        output.info("To start working on this ticket:");
        output.info(&format!("  vibe-ticket start {}", ticket.slug));
    }

    Ok(())
}