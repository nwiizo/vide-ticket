use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for vide-ticket
///
/// This enum represents all possible errors that can occur in the application.
/// Using thiserror for automatic Error trait implementation.
#[derive(Error, Debug)]
pub enum VideTicketError {
    /// I/O related errors
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// YAML serialization/deserialization errors
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Git operation errors
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    /// Ticket not found
    #[error("Ticket not found: {id}")]
    TicketNotFound { id: String },

    /// Task not found
    #[error("Task not found: {id}")]
    TaskNotFound { id: String },

    /// Invalid ticket status
    #[error("Invalid ticket status: {status}")]
    InvalidStatus { status: String },

    /// Invalid priority
    #[error("Invalid priority: {priority}")]
    InvalidPriority { priority: String },

    /// Project not initialized
    #[error("Project not initialized. Run 'vide-ticket init' first")]
    ProjectNotInitialized,

    /// Project already initialized
    #[error("Project already initialized at {path:?}")]
    ProjectAlreadyInitialized { path: PathBuf },

    /// No active ticket
    #[error("No active ticket. Use 'vide-ticket start <id>' to start working on a ticket")]
    NoActiveTicket,

    /// Multiple active tickets
    #[error("Multiple active tickets found. This should not happen")]
    MultipleActiveTickets,

    /// Invalid slug format
    #[error("Invalid slug format: {slug}. Slugs must be lowercase alphanumeric with hyphens")]
    InvalidSlug { slug: String },

    /// Duplicate ticket
    #[error("Ticket with slug '{slug}' already exists")]
    DuplicateTicket { slug: String },

    /// File operation error
    #[error("File operation failed for {path:?}: {message}")]
    FileOperation { path: PathBuf, message: String },

    /// Permission denied
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    /// Template error
    #[error("Template error: {0}")]
    Template(#[from] tera::Error),

    /// UUID parsing error
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),
}

/// Result type alias for vide-ticket operations
pub type Result<T> = std::result::Result<T, VideTicketError>;

impl VideTicketError {
    /// Creates a custom error with the given message
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }

    /// Returns true if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::TicketNotFound { .. }
                | Self::TaskNotFound { .. }
                | Self::NoActiveTicket
                | Self::InvalidSlug { .. }
        )
    }

    /// Returns true if this error is a configuration issue
    pub fn is_config_error(&self) -> bool {
        matches!(
            self,
            Self::Config(_) | Self::ProjectNotInitialized | Self::ProjectAlreadyInitialized { .. }
        )
    }

    /// Returns a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::Io(e) if e.kind() == io::ErrorKind::NotFound => {
                "File or directory not found".to_string()
            },
            Self::Io(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                "Permission denied. Check file permissions".to_string()
            },
            Self::Git(e) => format!("Git operation failed: {}", e.message()),
            _ => self.to_string(),
        }
    }

    /// Returns suggested actions for the error
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            Self::ProjectNotInitialized => vec![
                "Run 'vide-ticket init' to initialize the project".to_string(),
                "Make sure you're in the correct directory".to_string(),
            ],
            Self::NoActiveTicket => vec![
                "Run 'vide-ticket list' to see available tickets".to_string(),
                "Run 'vide-ticket start <id>' to start working on a ticket".to_string(),
            ],
            Self::InvalidSlug { .. } => vec![
                "Use lowercase letters, numbers, and hyphens only".to_string(),
                "Example: 'fix-login-bug' or 'feature-123'".to_string(),
            ],
            Self::DuplicateTicket { slug } => vec![
                format!("Use a different slug or check existing ticket '{}'", slug),
                "Run 'vide-ticket list' to see all tickets".to_string(),
            ],
            _ => vec![],
        }
    }
}

/// Error context extension trait
pub trait ErrorContext<T> {
    /// Adds context to the error
    fn context(self, msg: &str) -> Result<T>;

    /// Adds context with a lazy message
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<VideTicketError>,
{
    fn context(self, msg: &str) -> Result<T> {
        self.map_err(|e| {
            let base_error = e.into();
            VideTicketError::Custom(format!("{}: {}", msg, base_error))
        })
    }

    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            VideTicketError::Custom(format!("{}: {}", f(), base_error))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = VideTicketError::TicketNotFound {
            id: "123".to_string(),
        };
        assert_eq!(err.to_string(), "Ticket not found: 123");
    }

    #[test]
    fn test_is_recoverable() {
        assert!(VideTicketError::NoActiveTicket.is_recoverable());
        assert!(!VideTicketError::ProjectNotInitialized.is_recoverable());
    }

    #[test]
    fn test_suggestions() {
        let err = VideTicketError::ProjectNotInitialized;
        let suggestions = err.suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("vide-ticket init"));
    }
}
