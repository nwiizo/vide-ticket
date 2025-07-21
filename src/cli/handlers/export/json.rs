//! JSON export implementation

use super::Exporter;
use crate::core::Ticket;
use crate::error::{Result, VideTicketError};
use serde_json::json;

/// JSON exporter implementation
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let json = json!({
            "tickets": tickets,
            "exported_at": chrono::Utc::now(),
            "total": tickets.len(),
        });

        serde_json::to_string_pretty(&json)
            .map_err(|e| VideTicketError::custom(format!("Failed to serialize to JSON: {}", e)))
    }
    
    fn format_name(&self) -> &'static str {
        "JSON"
    }
}