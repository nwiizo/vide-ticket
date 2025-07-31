// vibe-ticket library root
// This file serves as the entry point for the library

pub mod cache;
pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod plugins;
pub mod specs;
pub mod storage;

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "mcp")]
pub mod mcp;

#[cfg(test)]
pub mod test_utils;

// Re-export commonly used types
pub use error::{Result, VibeTicketError};
