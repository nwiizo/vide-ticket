//! JSON export implementation

use super::{Exporter, ExportMetadata};
use crate::core::Ticket;
use crate::error::{Result, VibeTicketError};

/// JSON exporter implementation
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let metadata = ExportMetadata::new(tickets.to_vec());

        serde_json::to_string_pretty(&metadata)
            .map_err(|e| VibeTicketError::serialization_error("JSON", e))
    }

    fn format_name(&self) -> &'static str {
        "JSON"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Priority, Status, Ticket};

    #[test]
    fn test_json_export() {
        let exporter = JsonExporter;
        let tickets = vec![
            Ticket::new("test-1".to_string(), "Test Ticket 1".to_string()),
            Ticket::new("test-2".to_string(), "Test Ticket 2".to_string()),
        ];
        
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let json_str = result.unwrap();
        assert!(json_str.contains("\"tickets\""));
        assert!(json_str.contains("\"exported_at\""));
        assert!(json_str.contains("\"total\": 2"));
    }

    #[test]
    fn test_json_export_empty() {
        let exporter = JsonExporter;
        let tickets: Vec<Ticket> = vec![];
        
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let json_str = result.unwrap();
        assert!(json_str.contains("\"total\": 0"));
    }

    #[test]
    fn test_json_export_complex_ticket() {
        let exporter = JsonExporter;
        let mut ticket = Ticket::new("complex".to_string(), "Complex Ticket".to_string());
        ticket.description = "Multi-line\ndescription\nwith special chars: \"quotes\"".to_string();
        ticket.priority = Priority::High;
        ticket.status = Status::Doing;
        ticket.tags = vec!["tag1".to_string(), "tag2".to_string()];
        ticket.assignee = Some("user@example.com".to_string());
        
        let tickets = vec![ticket];
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let json_str = result.unwrap();
        assert!(json_str.contains("Complex Ticket"));
        assert!(json_str.contains("\"priority\": \"high\""));
        assert!(json_str.contains("\"status\": \"doing\""));
    }

    #[test]
    fn test_format_name() {
        let exporter = JsonExporter;
        assert_eq!(exporter.format_name(), "JSON");
    }
}
