//! Handler for the `export` command
//!
//! This module implements the logic for exporting tickets
//! to various formats (JSON, YAML, CSV, Markdown).

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::Ticket;
use crate::error::{Result, VideTicketError};
use crate::storage::{FileStorage, TicketRepository};

/// Handler for the `export` command
///
/// Exports tickets to various formats:
/// 1. JSON - Full structured data
/// 2. YAML - Human-readable structured data
/// 3. CSV - Spreadsheet-compatible format
/// 4. Markdown - Documentation format
///
/// # Arguments
///
/// * `format` - Export format (json, yaml, csv, markdown)
/// * `output_path` - Optional output file path (defaults to stdout)
/// * `include_archived` - Whether to include archived tickets
/// * `project_dir` - Optional project directory path
/// * `output` - Output formatter for displaying results
pub fn handle_export_command(
    format: String,
    output_path: Option<String>,
    include_archived: bool,
    project_dir: Option<String>,
    output: &OutputFormatter,
) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(project_dir.as_deref())?;
    let vide_ticket_dir = project_root.join(".vide-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vide_ticket_dir);

    // Load all tickets
    let mut tickets = storage.load_all()?;

    // Filter out archived tickets if not included
    if !include_archived {
        tickets.retain(|t| {
            !t.metadata
                .get("archived")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        });
    }

    // Sort tickets by creation date
    tickets.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    // Generate export content based on format
    let content = match format.to_lowercase().as_str() {
        "json" => export_json(&tickets)?,
        "yaml" => export_yaml(&tickets)?,
        "csv" => export_csv(&tickets)?,
        "markdown" | "md" => export_markdown(&tickets)?,
        _ => {
            return Err(VideTicketError::custom(format!(
                "Unsupported export format: {}. Supported formats: json, yaml, csv, markdown",
                format
            )))
        },
    };

    // Write to output
    if let Some(path) = output_path {
        std::fs::write(&path, content)
            .map_err(|e| VideTicketError::custom(format!("Failed to write to {}: {}", path, e)))?;

        output.success(&format!("Exported {} tickets to {}", tickets.len(), path));
        output.info(&format!("Format: {}", format));
        if !include_archived {
            output.info(
                "Note: Archived tickets were excluded. Use --include-archived to include them.",
            );
        }
    } else {
        // Output to stdout
        println!("{}", content);
    }

    Ok(())
}

/// Export tickets as JSON
fn export_json(tickets: &[Ticket]) -> Result<String> {
    let json = serde_json::json!({
        "tickets": tickets,
        "exported_at": chrono::Utc::now(),
        "total": tickets.len(),
    });

    serde_json::to_string_pretty(&json)
        .map_err(|e| VideTicketError::custom(format!("Failed to serialize to JSON: {}", e)))
}

/// Export tickets as YAML
fn export_yaml(tickets: &[Ticket]) -> Result<String> {
    #[derive(serde::Serialize)]
    struct Export {
        tickets: Vec<Ticket>,
        exported_at: chrono::DateTime<chrono::Utc>,
        total: usize,
    }

    let export = Export {
        tickets: tickets.to_vec(),
        exported_at: chrono::Utc::now(),
        total: tickets.len(),
    };

    serde_yaml::to_string(&export)
        .map_err(|e| VideTicketError::custom(format!("Failed to serialize to YAML: {}", e)))
}

/// Export tickets as CSV
fn export_csv(tickets: &[Ticket]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    // Write header
    wtr.write_record(&[
        "ID",
        "Slug",
        "Title",
        "Status",
        "Priority",
        "Assignee",
        "Tags",
        "Created At",
        "Started At",
        "Closed At",
        "Tasks Total",
        "Tasks Completed",
        "Description",
    ])
    .map_err(|e| VideTicketError::custom(format!("Failed to write CSV header: {}", e)))?;

    // Write ticket records
    for ticket in tickets {
        let tasks_total = ticket.tasks.len();
        let tasks_completed = ticket.tasks.iter().filter(|t| t.completed).count();

        wtr.write_record(&[
            ticket.id.to_string(),
            ticket.slug.clone(),
            ticket.title.clone(),
            ticket.status.to_string(),
            ticket.priority.to_string(),
            ticket.assignee.clone().unwrap_or_else(|| "".to_string()),
            ticket.tags.join(", "),
            ticket.created_at.to_rfc3339(),
            ticket
                .started_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "".to_string()),
            ticket
                .closed_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "".to_string()),
            tasks_total.to_string(),
            tasks_completed.to_string(),
            ticket.description.replace('\n', " "),
        ])
        .map_err(|e| VideTicketError::custom(format!("Failed to write CSV record: {}", e)))?;
    }

    let data = wtr
        .into_inner()
        .map_err(|e| VideTicketError::custom(format!("Failed to finalize CSV: {}", e)))?;

    String::from_utf8(data)
        .map_err(|e| VideTicketError::custom(format!("Failed to convert CSV to string: {}", e)))
}

/// Export tickets as Markdown
fn export_markdown(tickets: &[Ticket]) -> Result<String> {
    let mut output = String::new();

    // Title and metadata
    output.push_str("# Ticket Export\n\n");
    output.push_str(&format!(
        "**Exported at**: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    ));
    output.push_str(&format!("**Total tickets**: {}\n\n", tickets.len()));

    // Summary table
    output.push_str("## Summary\n\n");
    output.push_str("| Status | Count |\n");
    output.push_str("|--------|-------|\n");

    let mut status_counts = std::collections::HashMap::new();
    for ticket in tickets {
        *status_counts.entry(ticket.status.to_string()).or_insert(0) += 1;
    }

    for (status, count) in &status_counts {
        output.push_str(&format!("| {} | {} |\n", status, count));
    }

    output.push_str("\n## Tickets\n\n");

    // Group tickets by status
    let mut todo_tickets = Vec::new();
    let mut doing_tickets = Vec::new();
    let mut review_tickets = Vec::new();
    let mut blocked_tickets = Vec::new();
    let mut done_tickets = Vec::new();

    for ticket in tickets {
        match ticket.status {
            crate::core::Status::Todo => todo_tickets.push(ticket),
            crate::core::Status::Doing => doing_tickets.push(ticket),
            crate::core::Status::Review => review_tickets.push(ticket),
            crate::core::Status::Blocked => blocked_tickets.push(ticket),
            crate::core::Status::Done => done_tickets.push(ticket),
        }
    }

    // Output tickets by status
    if !todo_tickets.is_empty() {
        output.push_str("### 📋 Todo\n\n");
        for ticket in todo_tickets {
            output_ticket_markdown(&mut output, ticket);
        }
    }

    if !doing_tickets.is_empty() {
        output.push_str("### 🔄 In Progress\n\n");
        for ticket in doing_tickets {
            output_ticket_markdown(&mut output, ticket);
        }
    }

    if !review_tickets.is_empty() {
        output.push_str("### 👀 In Review\n\n");
        for ticket in review_tickets {
            output_ticket_markdown(&mut output, ticket);
        }
    }

    if !blocked_tickets.is_empty() {
        output.push_str("### 🚫 Blocked\n\n");
        for ticket in blocked_tickets {
            output_ticket_markdown(&mut output, ticket);
        }
    }

    if !done_tickets.is_empty() {
        output.push_str("### ✅ Done\n\n");
        for ticket in done_tickets {
            output_ticket_markdown(&mut output, ticket);
        }
    }

    Ok(output)
}

/// Output a single ticket in Markdown format
fn output_ticket_markdown(output: &mut String, ticket: &Ticket) {
    output.push_str(&format!("#### {} - {}\n\n", ticket.slug, ticket.title));
    output.push_str(&format!("- **Priority**: {}\n", ticket.priority));

    if let Some(assignee) = &ticket.assignee {
        output.push_str(&format!("- **Assignee**: {}\n", assignee));
    }

    if !ticket.tags.is_empty() {
        output.push_str(&format!("- **Tags**: {}\n", ticket.tags.join(", ")));
    }

    if !ticket.tasks.is_empty() {
        let completed = ticket.tasks.iter().filter(|t| t.completed).count();
        output.push_str(&format!(
            "- **Tasks**: {}/{}\n",
            completed,
            ticket.tasks.len()
        ));
    }

    output.push_str(&format!(
        "- **Created**: {}\n",
        ticket.created_at.format("%Y-%m-%d")
    ));

    if !ticket.description.trim().is_empty() {
        output.push_str("\n");
        output.push_str(&ticket.description);
        output.push_str("\n");
    }

    output.push_str("\n---\n\n");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_csv_escaping() {
        let description = "This has\nnewlines and, commas";
        let escaped = description.replace('\n', " ");
        assert!(!escaped.contains('\n'));
    }
}
