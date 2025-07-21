//! CSV export implementation

use super::Exporter;
use crate::core::Ticket;
use crate::error::{Result, VibeTicketError};
use csv::Writer;

/// CSV exporter implementation
pub struct CsvExporter;

impl Exporter for CsvExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let mut wtr = Writer::from_writer(vec![]);

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
        .map_err(|e| VibeTicketError::custom(format!("Failed to write CSV header: {}", e)))?;

        // Write ticket records
        for ticket in tickets {
            write_ticket_record(&mut wtr, ticket)?;
        }

        // Convert to string
        let data = wtr
            .into_inner()
            .map_err(|e| VibeTicketError::custom(format!("Failed to finalize CSV: {}", e)))?;

        String::from_utf8(data)
            .map_err(|e| VibeTicketError::custom(format!("Failed to convert CSV to string: {}", e)))
    }
    
    fn format_name(&self) -> &'static str {
        "CSV"
    }
}

/// Write a single ticket record to CSV
fn write_ticket_record<W: std::io::Write>(wtr: &mut Writer<W>, ticket: &Ticket) -> Result<()> {
    let tasks_total = ticket.tasks.len();
    let tasks_completed = ticket.tasks.iter().filter(|t| t.completed).count();

    wtr.write_record(&[
        ticket.id.to_string(),
        ticket.slug.clone(),
        ticket.title.clone(),
        ticket.status.to_string(),
        ticket.priority.to_string(),
        ticket.assignee.clone().unwrap_or_default(),
        ticket.tags.join(", "),
        ticket.created_at.to_rfc3339(),
        ticket
            .started_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
        ticket
            .closed_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
        tasks_total.to_string(),
        tasks_completed.to_string(),
        ticket.description.replace('\n', " "),
    ])
    .map_err(|e| VibeTicketError::custom(format!("Failed to write CSV record: {}", e)))
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