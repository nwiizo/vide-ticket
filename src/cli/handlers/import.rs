//! Handler for the `import` command
//!
//! This module implements the logic for importing tickets
//! from various formats (JSON, YAML, CSV).

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::{Priority, Status, Ticket, TicketId};
use crate::error::{Result, VibeTicketError};
use crate::storage::{FileStorage, TicketRepository};
use std::collections::HashMap;

/// Handler for the `import` command
///
/// Imports tickets from various formats:
/// 1. JSON - Full structured data
/// 2. YAML - Human-readable structured data
/// 3. CSV - Spreadsheet format
///
/// # Arguments
///
/// * `file_path` - Path to the import file
/// * `format` - Optional format (auto-detected if not specified)
/// * `skip_validation` - Whether to skip validation
/// * `dry_run` - Whether to perform a dry run (don't actually import)
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_import_command(
    file_path: &str,
    format: Option<&str>,
    skip_validation: bool,
    dry_run: bool,
    project_dir: Option<&str>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir)?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Read file content
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| VibeTicketError::custom(format!("Failed to read file {file_path}: {e}")))?;

    // Detect format if not specified
    let format = if let Some(fmt) = format {
        fmt.to_string()
    } else {
        detect_format(file_path, &content)?
    };

    // Parse tickets based on format
    let tickets = match format.to_lowercase().as_str() {
        "json" => import_json(&content)?,
        "yaml" => import_yaml(&content)?,
        "csv" => import_csv(&content)?,
        _ => {
            return Err(VibeTicketError::custom(format!(
                "Unsupported import format: {format}. Supported formats: json, yaml, csv"
            )))
        },
    };

    // Validate tickets
    if !skip_validation {
        validate_tickets(&tickets, &storage)?;
    }

    // Show what will be imported
    if output.is_json() {
        output.print_json(&serde_json::json!({
            "file": file_path,
            "format": format,
            "tickets_to_import": tickets.len(),
            "dry_run": dry_run,
            "tickets": tickets.iter().map(|t| serde_json::json!({
                "slug": t.slug,
                "title": t.title,
                "status": t.status.to_string(),
                "priority": t.priority.to_string(),
            })).collect::<Vec<_>>(),
        }))?;
    } else {
        output.info(&format!("Import from: {file_path}"));
        output.info(&format!("Format: {format}"));
        output.info(&format!("Tickets to import: {}", tickets.len()));

        if dry_run {
            output.warning("DRY RUN MODE - No changes will be made");
        }

        output.info("\nTickets to import:");
        for ticket in &tickets {
            output.info(&format!(
                "  • {} - {} ({}, {})",
                ticket.slug, ticket.title, ticket.status, ticket.priority
            ));
        }
    }

    // Perform the import if not dry run
    if !dry_run {
        let mut imported = 0;
        let mut skipped = 0;
        let mut errors = Vec::new();

        for ticket in tickets {
            // Check if ticket with same slug already exists
            if storage.find_ticket_by_slug(&ticket.slug)?.is_some() {
                skipped += 1;
                if !output.is_json() {
                    output.warning(&format!(
                        "Skipping '{}': ticket with this slug already exists",
                        ticket.slug
                    ));
                }
                continue;
            }

            // Save the ticket
            match storage.save(&ticket) {
                Ok(()) => imported += 1,
                Err(e) => {
                    errors.push(format!("Failed to import '{}': {}", ticket.slug, e));
                },
            }
        }

        // Report results
        if output.is_json() {
            output.print_json(&serde_json::json!({
                "status": "completed",
                "imported": imported,
                "skipped": skipped,
                "errors": errors,
            }))?;
        } else {
            output.info("");
            output.success(&format!(
                "Import completed: {imported} imported, {skipped} skipped"
            ));

            if !errors.is_empty() {
                output.error("Errors occurred during import:");
                for error in errors {
                    output.error(&format!("  • {error}"));
                }
            }
        }
    }

    Ok(())
}

/// Detect format from file extension or content
fn detect_format(file_path: &str, content: &str) -> Result<String> {
    // Try to detect from file extension
    if let Some(extension) = file_path.split('.').next_back() {
        match extension.to_lowercase().as_str() {
            "json" => return Ok("json".to_string()),
            "yaml" | "yml" => return Ok("yaml".to_string()),
            "csv" => return Ok("csv".to_string()),
            _ => {},
        }
    }

    // Try to detect from content
    let trimmed = content.trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        Ok("json".to_string())
    } else if trimmed.contains("---") || trimmed.contains("tickets:") {
        Ok("yaml".to_string())
    } else if trimmed.lines().any(|line| line.contains(',')) {
        Ok("csv".to_string())
    } else {
        Err(VibeTicketError::custom(
            "Unable to detect file format. Please specify format explicitly with --format",
        ))
    }
}

/// Import tickets from JSON
fn import_json(content: &str) -> Result<Vec<Ticket>> {
    let json: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| VibeTicketError::custom(format!("Failed to parse JSON: {e}")))?;

    // Handle both direct array and object with tickets field
    let tickets_value = if json.is_array() {
        &json
    } else if let Some(tickets) = json.get("tickets") {
        tickets
    } else {
        return Err(VibeTicketError::custom(
            "JSON must be an array of tickets or object with 'tickets' field",
        ));
    };

    let tickets: Vec<Ticket> = serde_json::from_value(tickets_value.clone())
        .map_err(|e| VibeTicketError::custom(format!("Failed to deserialize tickets: {e}")))?;

    Ok(tickets)
}

/// Import tickets from YAML
fn import_yaml(content: &str) -> Result<Vec<Ticket>> {
    #[derive(serde::Deserialize)]
    struct YamlImport {
        tickets: Vec<Ticket>,
    }

    // Try to parse as direct array first
    if let Ok(tickets) = serde_yaml::from_str::<Vec<Ticket>>(content) {
        return Ok(tickets);
    }

    // Try to parse as object with tickets field

    if let Ok(import) = serde_yaml::from_str::<YamlImport>(content) {
        return Ok(import.tickets);
    }

    Err(VibeTicketError::custom(
        "Failed to parse YAML: expected array of tickets or object with 'tickets' field",
    ))
}

/// Import tickets from CSV
fn import_csv(content: &str) -> Result<Vec<Ticket>> {
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut tickets = Vec::new();

    for result in rdr.records() {
        let record = result
            .map_err(|e| VibeTicketError::custom(format!("Failed to read CSV record: {e}")))?;

        // Expected columns: ID, Slug, Title, Status, Priority, Assignee, Tags, Created At, Started At, Closed At, Tasks Total, Tasks Completed, Description
        if record.len() < 13 {
            return Err(VibeTicketError::custom("CSV must have at least 13 columns"));
        }

        let id = TicketId::parse_str(&record[0])
            .map_err(|_| VibeTicketError::custom(format!("Invalid ticket ID: {}", &record[0])))?;

        let status = Status::try_from(&record[3])
            .map_err(|_| VibeTicketError::custom(format!("Invalid status: {}", &record[3])))?;

        let priority = Priority::try_from(&record[4])
            .map_err(|_| VibeTicketError::custom(format!("Invalid priority: {}", &record[4])))?;

        let assignee = if record[5].is_empty() {
            None
        } else {
            Some(record[5].to_string())
        };

        let tags: Vec<String> = if record[6].is_empty() {
            Vec::new()
        } else {
            record[6]
                .split(", ")
                .map(std::string::ToString::to_string)
                .collect()
        };

        let created_at = chrono::DateTime::parse_from_rfc3339(&record[7])
            .map_err(|e| VibeTicketError::custom(format!("Invalid created_at date: {e}")))?
            .with_timezone(&chrono::Utc);

        let started_at = if record[8].is_empty() {
            None
        } else {
            Some(
                chrono::DateTime::parse_from_rfc3339(&record[8])
                    .map_err(|e| VibeTicketError::custom(format!("Invalid started_at date: {e}")))?
                    .with_timezone(&chrono::Utc),
            )
        };

        let closed_at = if record[9].is_empty() {
            None
        } else {
            Some(
                chrono::DateTime::parse_from_rfc3339(&record[9])
                    .map_err(|e| VibeTicketError::custom(format!("Invalid closed_at date: {e}")))?
                    .with_timezone(&chrono::Utc),
            )
        };

        let ticket = Ticket {
            id,
            slug: record[1].to_string(),
            title: record[2].to_string(),
            description: record[12].to_string(),
            priority,
            status,
            tags,
            created_at,
            started_at,
            closed_at,
            assignee,
            tasks: Vec::new(), // CSV doesn't include task details
            metadata: HashMap::new(),
        };

        tickets.push(ticket);
    }

    Ok(tickets)
}

/// Validate tickets before import
fn validate_tickets(tickets: &[Ticket], storage: &FileStorage) -> Result<()> {
    let mut errors = Vec::new();

    // Check for duplicate slugs within import
    let mut seen_slugs = std::collections::HashSet::new();
    for ticket in tickets {
        if !seen_slugs.insert(&ticket.slug) {
            errors.push(format!("Duplicate slug in import: {}", ticket.slug));
        }
    }

    // Check for conflicts with existing tickets
    for ticket in tickets {
        if let Ok(existing) = storage.load(&ticket.id) {
            errors.push(format!(
                "Ticket with ID {} already exists (slug: {})",
                ticket.id, existing.slug
            ));
        }
    }

    if !errors.is_empty() {
        return Err(VibeTicketError::custom(format!(
            "Validation failed:\n{}",
            errors.join("\n")
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(detect_format("data.json", "{}").unwrap(), "json");
        assert_eq!(detect_format("data.yaml", "tickets:").unwrap(), "yaml");
        assert_eq!(detect_format("data.csv", "a,b,c").unwrap(), "csv");

        // Test content-based detection
        assert_eq!(detect_format("unknown", "[{\"test\": 1}]").unwrap(), "json");
        assert_eq!(detect_format("unknown", "---\ntickets:").unwrap(), "yaml");
    }
}
