//! Example demonstrating CLI-MCP integration

use vibe_ticket::core::{Priority, Ticket};
use vibe_ticket::error::{Result, VibeTicketError};
use vibe_ticket::integration::{notify_ticket_created, notify_status_changed};
use vibe_ticket::storage::{FileStorage, TicketRepository, ActiveTicketRepository};
use std::convert::TryFrom;

/// Example of creating a ticket with integration notifications
pub fn create_ticket_with_notification(
    slug: &str,
    title: Option<String>,
    description: Option<String>,
    priority: &str,
    tags: Option<String>,
    start: bool,
) -> Result<()> {
    println!("Creating ticket with CLI-MCP integration...");

    // Initialize storage
    let storage = FileStorage::new(".vibe-ticket");

    // Generate timestamp prefix
    let now = chrono::Local::now();
    let timestamp_prefix = now.format("%Y%m%d%H%M").to_string();

    // Validate slug
    let base_slug = slug.trim();
    if base_slug.is_empty() || !base_slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(VibeTicketError::InvalidSlug {
            slug: base_slug.to_string(),
        });
    }

    // Combine timestamp and slug
    let slug = format!("{timestamp_prefix}-{base_slug}");

    // Check for duplicates
    if storage.ticket_exists_with_slug(&slug)? {
        return Err(VibeTicketError::DuplicateTicket { slug });
    }

    // Parse priority
    let priority = Priority::try_from(priority)
        .map_err(|_| VibeTicketError::InvalidPriority {
            priority: priority.to_string(),
        })?;

    // Parse tags
    let tags = tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default();

    // Create title from slug if not provided
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

    // NOTIFY MCP ABOUT TICKET CREATION
    notify_ticket_created(&ticket);
    println!("✓ Notified MCP about ticket creation: {}", ticket.slug);

    // If starting the ticket
    if start {
        let old_status = ticket.status.clone();
        ticket.start();
        storage.save(&ticket)?;
        storage.set_active(&ticket.id)?;

        // NOTIFY MCP ABOUT STATUS CHANGE
        notify_status_changed(&ticket.id, old_status, ticket.status.clone());
        println!("✓ Notified MCP about status change: Todo → Doing");
    }

    println!("✓ Ticket created successfully: {} (ID: {})", ticket.slug, ticket.id.short());

    Ok(())
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Initialize integration (in a real app, this would be done once at startup)
    let storage = std::sync::Arc::new(FileStorage::new(".vibe-ticket"));
    vibe_ticket::integration::init_integration(storage);

    // Create a sample ticket
    create_ticket_with_notification(
        "test-integration",
        Some("Test CLI-MCP Integration".to_string()),
        Some("This ticket demonstrates how CLI operations notify MCP".to_string()),
        "high",
        Some("integration,mcp".to_string()),
        true, // Start immediately
    )?;

    println!("\n✅ Integration example completed successfully!");
    println!("In a real implementation, MCP would receive these notifications");

    Ok(())
}