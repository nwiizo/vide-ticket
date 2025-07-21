//! YAML export implementation

use super::Exporter;
use crate::core::Ticket;
use crate::error::{Result, VibeTicketError};
use serde::Serialize;

/// YAML exporter implementation
pub struct YamlExporter;

#[derive(Serialize)]
struct YamlExport {
    tickets: Vec<Ticket>,
    exported_at: chrono::DateTime<chrono::Utc>,
    total: usize,
}

impl Exporter for YamlExporter {
    fn export(&self, tickets: &[Ticket]) -> Result<String> {
        let export = YamlExport {
            tickets: tickets.to_vec(),
            exported_at: chrono::Utc::now(),
            total: tickets.len(),
        };

        serde_yaml::to_string(&export)
            .map_err(|e| VibeTicketError::custom(format!("Failed to serialize to YAML: {e}")))
    }

    fn format_name(&self) -> &'static str {
        "YAML"
    }
}
