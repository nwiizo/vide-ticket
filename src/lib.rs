// vide-ticket library root
// This file serves as the entry point for the library

pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod plugins;
pub mod storage;

#[cfg(feature = "api")]
pub mod api;

// Re-export commonly used types
pub use error::{Result, VideTicketError};
