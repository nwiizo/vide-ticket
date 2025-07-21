use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for a ticket
///
/// Uses UUID v4 internally for globally unique identification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TicketId(Uuid);

impl TicketId {
    /// Creates a new random TicketId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Creates a TicketId from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    
    /// Parses a TicketId from a string
    pub fn parse_str(input: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(input)?))
    }
    
    /// Returns a shortened version of the ID for display
    pub fn short(&self) -> String {
        self.0.to_string()[..8].to_string()
    }
}

impl Default for TicketId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TicketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TicketId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl AsRef<Uuid> for TicketId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

/// Unique identifier for a task
///
/// Uses UUID v4 internally for globally unique identification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(Uuid);

impl TaskId {
    /// Creates a new random TaskId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Creates a TaskId from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Returns the inner UUID
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    
    /// Parses a TaskId from a string
    pub fn parse_str(input: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(input)?))
    }
    
    /// Returns a shortened version of the ID for display
    pub fn short(&self) -> String {
        self.0.to_string()[..8].to_string()
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TaskId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl AsRef<Uuid> for TaskId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ticket_id_creation() {
        let id1 = TicketId::new();
        let id2 = TicketId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_ticket_id_parsing() {
        let id = TicketId::new();
        let id_str = id.to_string();
        let parsed = TicketId::parse_str(&id_str).unwrap();
        assert_eq!(id, parsed);
    }
    
    #[test]
    fn test_ticket_id_short() {
        let id = TicketId::new();
        let short = id.short();
        assert_eq!(short.len(), 8);
        assert!(id.to_string().starts_with(&short));
    }
    
    #[test]
    fn test_task_id_creation() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_task_id_parsing() {
        let id = TaskId::new();
        let id_str = id.to_string();
        let parsed = TaskId::parse_str(&id_str).unwrap();
        assert_eq!(id, parsed);
    }
}