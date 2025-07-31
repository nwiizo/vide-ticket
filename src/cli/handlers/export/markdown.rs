//! Markdown export implementation

use super::Exporter;
use crate::core::{Status, Ticket};
use crate::error::Result;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

/// Markdown exporter implementation
pub struct MarkdownExporter;

impl Exporter for MarkdownExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let mut output = String::new();

        // Write header
        write_header(&mut output, tickets.len());

        // Write summary
        write_summary(&mut output, tickets);

        // Write tickets grouped by status
        write_tickets_by_status(&mut output, tickets);

        Ok(output)
    }

    fn format_name(&self) -> &'static str {
        "Markdown"
    }
}

/// Write the document header
fn write_header(output: &mut String, ticket_count: usize) {
    writeln!(output, "# Ticket Export\n").unwrap();
    writeln!(
        output,
        "**Exported at**: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    )
    .unwrap();
    writeln!(output, "**Total tickets**: {ticket_count}\n").unwrap();
}

/// Write the summary table
fn write_summary(output: &mut String, tickets: &[Ticket]) {
    writeln!(output, "## Summary\n").unwrap();
    writeln!(output, "| Status | Count |").unwrap();
    writeln!(output, "|--------|-------|").unwrap();

    let status_counts = count_by_status(tickets);
    for (status, count) in &status_counts {
        writeln!(output, "| {status} | {count} |").unwrap();
    }

    writeln!(output, "\n## Tickets\n").unwrap();
}

/// Count tickets by status
fn count_by_status(tickets: &[Ticket]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for ticket in tickets {
        *counts.entry(ticket.status.to_string()).or_insert(0) += 1;
    }
    counts
}

/// Write tickets grouped by status
fn write_tickets_by_status(output: &mut String, tickets: &[Ticket]) {
    let groups = group_by_status(tickets);

    if let Some(tickets) = groups.get(&Status::Todo) {
        write_status_section(output, "📋 Todo", tickets);
    }

    if let Some(tickets) = groups.get(&Status::Doing) {
        write_status_section(output, "🔄 In Progress", tickets);
    }

    if let Some(tickets) = groups.get(&Status::Review) {
        write_status_section(output, "👀 In Review", tickets);
    }

    if let Some(tickets) = groups.get(&Status::Blocked) {
        write_status_section(output, "🚫 Blocked", tickets);
    }

    if let Some(tickets) = groups.get(&Status::Done) {
        write_status_section(output, "✅ Done", tickets);
    }
}

/// Group tickets by status
fn group_by_status(tickets: &[Ticket]) -> HashMap<Status, Vec<&Ticket>> {
    let mut groups: HashMap<Status, Vec<&Ticket>> = HashMap::new();

    for ticket in tickets {
        groups.entry(ticket.status).or_default().push(ticket);
    }

    groups
}

/// Write a section for a specific status
fn write_status_section(output: &mut String, title: &str, tickets: &[&Ticket]) {
    writeln!(output, "### {title}\n").unwrap();
    for ticket in tickets {
        write_ticket(output, ticket);
    }
}

/// Write a single ticket in Markdown format
fn write_ticket(output: &mut String, ticket: &Ticket) {
    writeln!(output, "#### {} - {}\n", ticket.slug, ticket.title).unwrap();
    writeln!(output, "- **Priority**: {}", ticket.priority).unwrap();

    if let Some(assignee) = &ticket.assignee {
        writeln!(output, "- **Assignee**: {assignee}").unwrap();
    }

    if !ticket.tags.is_empty() {
        writeln!(output, "- **Tags**: {}", ticket.tags.join(", ")).unwrap();
    }

    if !ticket.tasks.is_empty() {
        let completed = ticket.tasks.iter().filter(|t| t.completed).count();
        writeln!(output, "- **Tasks**: {}/{}", completed, ticket.tasks.len()).unwrap();
    }

    writeln!(
        output,
        "- **Created**: {}",
        ticket.created_at.format("%Y-%m-%d")
    )
    .unwrap();

    if !ticket.description.trim().is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "{}", ticket.description).unwrap();
    }

    writeln!(output, "\n---\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Priority, Task};

    #[test]
    fn test_markdown_export() {
        let exporter = MarkdownExporter;
        let tickets = vec![
            Ticket::new("test-1".to_string(), "Test Ticket 1".to_string()),
            Ticket::new("test-2".to_string(), "Test Ticket 2".to_string()),
        ];

        let result = exporter.export(&tickets);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert!(markdown.contains("# Ticket Export"));
        assert!(markdown.contains("**Total tickets**: 2"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("## Tickets"));
    }

    #[test]
    fn test_markdown_export_empty() {
        let exporter = MarkdownExporter;
        let tickets: Vec<Ticket> = vec![];

        let result = exporter.export(&tickets);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert!(markdown.contains("**Total tickets**: 0"));
    }

    #[test]
    fn test_markdown_export_with_different_statuses() {
        let exporter = MarkdownExporter;

        let mut todo_ticket = Ticket::new("todo".to_string(), "Todo Ticket".to_string());
        todo_ticket.status = Status::Todo;

        let mut doing_ticket = Ticket::new("doing".to_string(), "Doing Ticket".to_string());
        doing_ticket.status = Status::Doing;

        let mut done_ticket = Ticket::new("done".to_string(), "Done Ticket".to_string());
        done_ticket.status = Status::Done;

        let tickets = vec![todo_ticket, doing_ticket, done_ticket];
        let result = exporter.export(&tickets);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert!(markdown.contains("### 📋 Todo"));
        assert!(markdown.contains("### 🔄 In Progress"));
        assert!(markdown.contains("### ✅ Done"));
    }

    #[test]
    fn test_markdown_export_with_rich_ticket() {
        let exporter = MarkdownExporter;

        let mut ticket = Ticket::new("rich".to_string(), "Rich Ticket".to_string());
        ticket.description = "This is a detailed description\nwith multiple lines".to_string();
        ticket.priority = Priority::High;
        ticket.assignee = Some("user@example.com".to_string());
        ticket.tags = vec!["feature".to_string(), "urgent".to_string()];
        ticket.tasks = vec![
            Task::new("Task 1".to_string()),
            Task::new("Task 2".to_string()),
        ];
        ticket.tasks[0].completed = true;

        let tickets = vec![ticket];
        let result = exporter.export(&tickets);
        assert!(result.is_ok());

        let markdown = result.unwrap();
        assert!(markdown.contains("- **Priority**: High"));
        assert!(markdown.contains("- **Assignee**: user@example.com"));
        assert!(markdown.contains("- **Tags**: feature, urgent"));
        assert!(markdown.contains("- **Tasks**: 1/2"));
        assert!(markdown.contains("This is a detailed description"));
    }

    #[test]
    fn test_format_name() {
        let exporter = MarkdownExporter;
        assert_eq!(exporter.format_name(), "Markdown");
    }

    #[test]
    fn test_count_by_status() {
        let mut tickets = vec![];
        for _ in 0..3 {
            let mut ticket = Ticket::new("todo".to_string(), "Todo".to_string());
            ticket.status = Status::Todo;
            tickets.push(ticket);
        }
        for _ in 0..2 {
            let mut ticket = Ticket::new("done".to_string(), "Done".to_string());
            ticket.status = Status::Done;
            tickets.push(ticket);
        }

        let counts = count_by_status(&tickets);
        assert_eq!(*counts.get("Todo").unwrap(), 3);
        assert_eq!(*counts.get("Done").unwrap(), 2);
    }
}
