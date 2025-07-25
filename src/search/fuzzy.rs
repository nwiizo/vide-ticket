//! Fuzzy search functionality for ticket discovery
//!
//! This module provides fuzzy matching capabilities to improve ticket search
//! by allowing approximate matches and typo tolerance.

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;

use crate::core::Ticket;

/// Configuration for fuzzy search behavior
#[derive(Debug, Clone)]
pub struct FuzzySearchConfig {
    /// Minimum score threshold for matches (0-100)
    pub min_score: i64,
    /// Whether to search in description
    pub search_description: bool,
    /// Whether to search in tags
    pub search_tags: bool,
    /// Maximum number of results to return
    pub max_results: Option<usize>,
}

impl Default for FuzzySearchConfig {
    fn default() -> Self {
        Self {
            min_score: 30,
            search_description: true,
            search_tags: true,
            max_results: Some(50),
        }
    }
}

/// Result of a fuzzy match
#[derive(Debug, Clone)]
pub struct FuzzyMatch {
    /// The matched ticket
    pub ticket: Ticket,
    /// Overall match score
    pub score: i64,
    /// Which fields matched
    pub matched_fields: Vec<MatchedField>,
}

/// Information about where a match occurred
#[derive(Debug, Clone)]
pub struct MatchedField {
    /// Field name (title, description, tags)
    pub field: String,
    /// Score for this field match
    pub score: i64,
    /// Matched indices for highlighting
    pub indices: Vec<usize>,
}

/// Fuzzy searcher for tickets
pub struct FuzzySearcher {
    matcher: SkimMatcherV2,
    config: FuzzySearchConfig,
}

impl FuzzySearcher {
    /// Create a new fuzzy searcher with default configuration
    pub fn new() -> Self {
        Self::with_config(FuzzySearchConfig::default())
    }

    /// Create a new fuzzy searcher with custom configuration
    pub fn with_config(config: FuzzySearchConfig) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            config,
        }
    }

    /// Search tickets using fuzzy matching
    pub fn search(&self, query: &str, tickets: &[Ticket]) -> Vec<FuzzyMatch> {
        let mut matches: Vec<FuzzyMatch> = tickets
            .iter()
            .filter_map(|ticket| self.match_ticket(query, ticket))
            .collect();

        // Sort by score (highest first)
        matches.sort_by(|a, b| b.score.cmp(&a.score));

        // Apply result limit if configured
        if let Some(limit) = self.config.max_results {
            matches.truncate(limit);
        }

        matches
    }

    /// Match a single ticket against the query
    fn match_ticket(&self, query: &str, ticket: &Ticket) -> Option<FuzzyMatch> {
        let mut matched_fields = Vec::new();
        let mut total_score = 0;

        // Match against title (highest weight)
        if let Some((score, indices)) = self.matcher.fuzzy_indices(&ticket.title, query) {
            let weighted_score = (score * 2).min(100); // Double weight for title matches
            total_score += weighted_score;
            matched_fields.push(MatchedField {
                field: "title".to_string(),
                score: weighted_score,
                indices,
            });
        }

        // Match against description if enabled
        if self.config.search_description && !ticket.description.is_empty() {
            if let Some((score, indices)) = self.matcher.fuzzy_indices(&ticket.description, query) {
                total_score += score;
                matched_fields.push(MatchedField {
                    field: "description".to_string(),
                    score,
                    indices,
                });
            }
        }

        // Match against tags if enabled
        if self.config.search_tags && !ticket.tags.is_empty() {
            let tags_str = ticket.tags.join(" ");
            if let Some((score, indices)) = self.matcher.fuzzy_indices(&tags_str, query) {
                total_score += score;
                matched_fields.push(MatchedField {
                    field: "tags".to_string(),
                    score,
                    indices,
                });
            }
        }

        // Match against slug (for technical searches)
        if let Some((score, indices)) = self.matcher.fuzzy_indices(&ticket.slug, query) {
            total_score += score;
            matched_fields.push(MatchedField {
                field: "slug".to_string(),
                score,
                indices,
            });
        }

        // Return match if score exceeds threshold
        if total_score >= self.config.min_score && !matched_fields.is_empty() {
            Some(FuzzyMatch {
                ticket: ticket.clone(),
                score: total_score,
                matched_fields,
            })
        } else {
            None
        }
    }

    /// Update configuration
    pub fn set_config(&mut self, config: FuzzySearchConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &FuzzySearchConfig {
        &self.config
    }
}

impl Default for FuzzySearcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to highlight matched portions in text
pub fn highlight_matches(text: &str, indices: &[usize], highlight_start: &str, highlight_end: &str) -> String {
    if indices.is_empty() {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len() + indices.len() * (highlight_start.len() + highlight_end.len()));
    let chars: Vec<char> = text.chars().collect();
    let mut in_match = false;

    for (i, ch) in chars.iter().enumerate() {
        let should_highlight = indices.contains(&i);
        
        if should_highlight && !in_match {
            result.push_str(highlight_start);
            in_match = true;
        } else if !should_highlight && in_match {
            result.push_str(highlight_end);
            in_match = false;
        }
        
        result.push(*ch);
    }

    if in_match {
        result.push_str(highlight_end);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Priority, Status};

    fn create_test_ticket(slug: &str, title: &str, description: &str, tags: Vec<&str>) -> Ticket {
        let mut ticket = Ticket::new(slug, title);
        ticket.description = description.to_string();
        ticket.tags = tags.into_iter().map(|s| s.to_string()).collect();
        ticket
    }

    #[test]
    fn test_fuzzy_search_exact_match() {
        let searcher = FuzzySearcher::new();
        let tickets = vec![
            create_test_ticket("fix-bug", "Fix login bug", "Users cannot login", vec!["bug", "auth"]),
            create_test_ticket("add-feature", "Add search feature", "Implement search", vec!["feature"]),
        ];

        let results = searcher.search("Fix login bug", &tickets);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].ticket.slug, "fix-bug");
        assert!(results[0].score > 100); // Exact match should have high score
    }

    #[test]
    fn test_fuzzy_search_with_typo() {
        let searcher = FuzzySearcher::new();
        let tickets = vec![
            create_test_ticket("fix-bug", "Fix login bug", "Users cannot login", vec!["bug", "auth"]),
            create_test_ticket("add-feature", "Add search feature", "Implement search", vec!["feature"]),
        ];

        let results = searcher.search("fix logn bug", &tickets); // Typo: logn instead of login
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].ticket.slug, "fix-bug");
    }

    #[test]
    fn test_fuzzy_search_abbreviation() {
        let searcher = FuzzySearcher::new();
        let tickets = vec![
            create_test_ticket("impl-search", "Implement fuzzy search", "Add fuzzy search capability", vec!["search"]),
        ];

        let results = searcher.search("impl fz src", &tickets);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].ticket.slug, "impl-search");
    }

    #[test]
    fn test_fuzzy_search_multiple_matches() {
        let searcher = FuzzySearcher::new();
        let tickets = vec![
            create_test_ticket("fix-search", "Fix search bug", "Search is broken", vec!["bug", "search"]),
            create_test_ticket("improve-search", "Improve search feature", "Make search faster", vec!["search", "performance"]),
            create_test_ticket("add-filter", "Add filter feature", "Add filtering to list", vec!["feature"]),
        ];

        let results = searcher.search("search", &tickets);
        assert_eq!(results.len(), 2);
        // Both search-related tickets should match
        assert!(results.iter().any(|r| r.ticket.slug == "fix-search"));
        assert!(results.iter().any(|r| r.ticket.slug == "improve-search"));
    }

    #[test]
    fn test_fuzzy_search_min_score() {
        let mut config = FuzzySearchConfig::default();
        config.min_score = 50;
        let searcher = FuzzySearcher::with_config(config);
        
        let tickets = vec![
            create_test_ticket("unrelated", "Completely different topic", "Nothing to do with query", vec!["other"]),
        ];

        let results = searcher.search("search functionality", &tickets);
        assert_eq!(results.len(), 0); // Should not match due to low score
    }

    #[test]
    fn test_highlight_matches() {
        let text = "Fix login bug";
        let indices = vec![0, 1, 2, 4, 5, 6, 7, 8];
        let highlighted = highlight_matches(text, &indices, "[", "]");
        assert_eq!(highlighted, "[Fix] [login] bug");
    }

    #[test]
    fn test_fuzzy_search_config() {
        let mut searcher = FuzzySearcher::new();
        let mut config = FuzzySearchConfig::default();
        config.search_description = false;
        config.search_tags = false;
        searcher.set_config(config);

        let tickets = vec![
            create_test_ticket("test", "Test ticket", "This contains search keyword", vec!["search"]),
        ];

        let results = searcher.search("search", &tickets);
        assert_eq!(results.len(), 0); // Should not match because description/tags search is disabled
    }
}