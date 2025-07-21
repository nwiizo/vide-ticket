use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the current status of a ticket
///
/// The status follows a typical workflow progression from
/// Todo → Doing → Done, with additional states for special cases.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Ticket is created but work hasn't started
    Todo,

    /// Work is actively being done on the ticket
    Doing,

    /// Work on the ticket is completed
    Done,

    /// Ticket is blocked by external dependencies
    Blocked,

    /// Ticket is in review/QA phase
    Review,
}

impl Default for Status {
    fn default() -> Self {
        Self::Todo
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo => write!(f, "Todo"),
            Self::Doing => write!(f, "Doing"),
            Self::Done => write!(f, "Done"),
            Self::Blocked => write!(f, "Blocked"),
            Self::Review => write!(f, "Review"),
        }
    }
}

impl Status {
    /// Returns all possible status values
    pub fn all() -> Vec<Self> {
        vec![
            Self::Todo,
            Self::Doing,
            Self::Done,
            Self::Blocked,
            Self::Review,
        ]
    }

    /// Returns whether the status represents active work
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Doing | Self::Review)
    }

    /// Returns whether the status represents completed work
    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Done)
    }

    /// Returns whether the status allows starting work
    pub fn can_start(&self) -> bool {
        matches!(self, Self::Todo | Self::Blocked)
    }

    /// Returns the emoji representation of the status
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Todo => "📋",
            Self::Doing => "🔧",
            Self::Done => "✅",
            Self::Blocked => "🚫",
            Self::Review => "👀",
        }
    }

    /// Returns the color code for terminal output
    pub fn color(&self) -> &'static str {
        match self {
            Self::Todo => "blue",
            Self::Doing => "yellow",
            Self::Done => "green",
            Self::Blocked => "red",
            Self::Review => "cyan",
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::Todo),
            "doing" | "in-progress" | "wip" => Ok(Self::Doing),
            "done" | "completed" | "closed" => Ok(Self::Done),
            "blocked" => Ok(Self::Blocked),
            "review" | "reviewing" => Ok(Self::Review),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_status() {
        assert_eq!(Status::default(), Status::Todo);
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Todo.to_string(), "Todo");
        assert_eq!(Status::Doing.to_string(), "Doing");
        assert_eq!(Status::Done.to_string(), "Done");
    }

    #[test]
    fn test_status_properties() {
        assert!(Status::Doing.is_active());
        assert!(Status::Review.is_active());
        assert!(!Status::Todo.is_active());

        assert!(Status::Done.is_completed());
        assert!(!Status::Doing.is_completed());

        assert!(Status::Todo.can_start());
        assert!(Status::Blocked.can_start());
        assert!(!Status::Doing.can_start());
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::try_from("todo").unwrap(), Status::Todo);
        assert_eq!(Status::try_from("DOING").unwrap(), Status::Doing);
        assert_eq!(Status::try_from("in-progress").unwrap(), Status::Doing);
        assert_eq!(Status::try_from("completed").unwrap(), Status::Done);
        assert!(Status::try_from("invalid").is_err());
    }
}
