//! Storage layer for vibe-ticket
//!
//! This module provides implementations for persisting and retrieving ticket data.
//! It supports multiple storage backends through a common trait interface.
//!
//! # Storage Backends
//!
//! The storage module supports various backends:
//! - File-based (YAML files for simple use cases)
//! - `SQLite` (for local storage) - feature gated
//! - `PostgreSQL` (for production deployments) - feature gated
//! - In-memory (for testing)
//!
//! # Architecture
//!
//! The storage layer implements the repository pattern:
//! - Repository traits define the interface
//! - Concrete implementations for each storage backend
//! - Migration support for schema management
//! - Connection pooling and transaction management
//!
//! # Example
//!
//! ```ignore
//! use vibe_ticket::storage::{FileStorage, Repository};
//!
//! // Initialize storage backend
//! let storage = FileStorage::new(".vibe-ticket");
//!
//! // Use storage through repository traits
//! let tickets = storage.get_all()?;
//! ```
//!
//! # Error Handling
//!
//! All storage operations return `Result<T, VibeTicketError>` to handle:
//! - I/O errors
//! - Serialization/deserialization errors
//! - Not found errors
//! - Permission errors

mod file;
mod lock;
mod repository;

pub use file::{FileStorage, ProjectState};
pub use lock::{FileLock, LockGuard};
pub use repository::{ActiveTicketRepository, Repository, TicketRepository};
