use crate::cli::{find_project_root, validate_slug, OutputFormatter};
use crate::core::{Priority, Ticket};
use crate::error::{Result, VideTicketError};
use crate::storage::{ActiveTicketRepository, FileStorage, TicketRepository};

use super::parse_tags;

/// Handler for the `new` command
pub fn handle_new_command(
    slug: String,
    title: Option<String>,
    description: Option<String>,
    priority: String,
    tags: Option<String>,
    start: bool,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vide_ticket_dir = project_root.join(".vide-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vide_ticket_dir);

    // Generate timestamp prefix for the slug
    let now = chrono::Local::now();
    let timestamp_prefix = now.format("%Y%m%d%H%M").to_string();

    // Validate and normalize the slug
    let base_slug = slug.trim();
    validate_slug(base_slug)?;

    // Combine timestamp and slug
    let slug = format!("{}-{}", timestamp_prefix, base_slug);

    // Check if a ticket with this slug already exists
    if storage.ticket_exists_with_slug(&slug)? {
        return Err(VideTicketError::DuplicateTicket { slug: slug.clone() });
    }

    // Parse priority
    let priority = Priority::try_from(priority.as_str())
        .map_err(|_| VideTicketError::InvalidPriority { priority })?;

    // Parse tags
    let tags = tags.map(|t| parse_tags(Some(t))).unwrap_or_default();

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

    // If --start flag is provided, start working on the ticket immediately
    if start {
        ticket.start();
        storage.save(&ticket)?;
        storage.set_active(&ticket.id)?;

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
        output.info(&format!("  vide-ticket start {}", ticket.slug));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_ticket_from_slug() {
        let temp_dir = TempDir::new().unwrap();
        let vide_ticket_dir = temp_dir.path().join(".vide-ticket");
        std::fs::create_dir_all(&vide_ticket_dir).unwrap();

        // Initialize project state
        let state = crate::storage::ProjectState {
            name: "Test Project".to_string(),
            description: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            ticket_count: 0,
        };

        let storage = FileStorage::new(&vide_ticket_dir);
        storage.save_state(&state).unwrap();
        storage.ensure_directories().unwrap();

        // Create output formatter
        let output = OutputFormatter::new(false, false);

        // Test creating a ticket
        let result = handle_new_command(
            "fix-login-bug".to_string(),
            None,
            Some("Users cannot login".to_string()),
            "high".to_string(),
            Some("bug,auth".to_string()),
            false,
            Some(temp_dir.path().to_str().unwrap().to_string()),
            &output,
        );

        assert!(result.is_ok());

        // Verify ticket was created
        let tickets = storage.load_all().unwrap();
        assert_eq!(tickets.len(), 1);

        let ticket = &tickets[0];
        // Check that slug has timestamp prefix and base slug
        assert!(
            ticket.slug.ends_with("-fix-login-bug"),
            "Expected slug to end with '-fix-login-bug', got: {}",
            ticket.slug
        );
        assert_eq!(ticket.slug.len(), 26); // YYYYMMDDHHMM (12) + "-" (1) + "fix-login-bug" (13) = 26
        assert_eq!(ticket.title, "Fix Login Bug");
        assert_eq!(ticket.description, "Users cannot login");
        assert_eq!(ticket.priority, Priority::High);
        assert_eq!(ticket.tags, vec!["bug", "auth"]);
    }
}
