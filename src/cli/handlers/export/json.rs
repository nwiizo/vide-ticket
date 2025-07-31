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
