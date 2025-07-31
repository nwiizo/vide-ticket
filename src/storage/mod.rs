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
//! # Concurrent Access Protection
//!
//! The file storage implementation includes built-in protection against concurrent
//! modifications through a file-based locking mechanism:
//!
//! - **Automatic Locking**: All write operations acquire exclusive locks
//! - **Lock Files**: Created as `<filename>.lock` with metadata
//! - **Retry Logic**: Operations retry up to 10 times with 100ms delays
//! - **Stale Lock Cleanup**: Locks older than 30 seconds are removed automatically
//! - **RAII Pattern**: Locks are released automatically using Rust's Drop trait
//!
//! This ensures data integrity even when multiple users or processes access
//! tickets simultaneously.
//!
//! # Example
//!
//! ```ignore
//! use vibe_ticket::storage::{FileStorage, Repository};
//!
//! // Initialize storage backend
//! let storage = FileStorage::new(".vibe-ticket");
//!
//! // Use storage through repository traits (locking is automatic)
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
//! - Lock acquisition failures

mod file;
mod lock;
mod repository;

pub use file::{FileStorage, ProjectState};
pub use lock::{FileLock, LockGuard};
pub use repository::{ActiveTicketRepository, Repository, TicketRepository};
