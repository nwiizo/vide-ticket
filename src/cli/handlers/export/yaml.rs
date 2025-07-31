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
