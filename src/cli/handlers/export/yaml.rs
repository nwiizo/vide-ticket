//! YAML export implementation

use super::{Exporter, ExportMetadata};
use crate::core::Ticket;
use crate::error::{Result, VibeTicketError};

/// YAML exporter implementation
pub struct YamlExporter;

impl Exporter for YamlExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let metadata = ExportMetadata::new(tickets.to_vec());

        serde_yaml::to_string(&metadata)
            .map_err(|e| VibeTicketError::serialization_error("YAML", e))
    }

    fn format_name(&self) -> &'static str {
        "YAML"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Priority, Status, Ticket};

    #[test]
    fn test_yaml_export() {
        let exporter = YamlExporter;
        let tickets = vec![
            Ticket::new("test-1".to_string(), "Test Ticket 1".to_string()),
            Ticket::new("test-2".to_string(), "Test Ticket 2".to_string()),
        ];
        
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("tickets:"));
        assert!(yaml_str.contains("exported_at:"));
        assert!(yaml_str.contains("total: 2"));
    }

    #[test]
    fn test_yaml_export_empty() {
        let exporter = YamlExporter;
        let tickets: Vec<Ticket> = vec![];
        
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("total: 0"));
        assert!(yaml_str.contains("tickets: []"));
    }

    #[test]
    fn test_yaml_export_with_special_chars() {
        let exporter = YamlExporter;
        let mut ticket = Ticket::new("special".to_string(), "Special: Chars & Symbols".to_string());
        ticket.description = "Description with:\n- Bullets\n- Special chars: & < > \"".to_string();
        ticket.priority = Priority::Critical;
        ticket.status = Status::Done;
        
        let tickets = vec![ticket];
        let result = exporter.export(&tickets);
        assert!(result.is_ok());
        
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("Special: Chars & Symbols"));
        assert!(yaml_str.contains("priority: critical"));
        assert!(yaml_str.contains("status: done"));
    }

    #[test]
    fn test_format_name() {
        let exporter = YamlExporter;
        assert_eq!(exporter.format_name(), "YAML");
    }
}
