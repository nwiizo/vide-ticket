//! Search functionality for vibe-ticket
//!
//! This module provides various search capabilities including:
//! - Exact text search
//! - Regular expression search
//! - Fuzzy search for better discovery

pub mod fuzzy;

pub use fuzzy::{FuzzySearcher, FuzzySearchConfig, FuzzyMatch, highlight_matches};