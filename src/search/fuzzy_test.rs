//! Unit tests for fuzzy search functionality

#[cfg(test)]
mod tests {
    use crate::search::{FuzzySearcher, FuzzySearchConfig};
    use crate::core::{Ticket, Status, Priority, TicketId};
    use chrono::Utc;

    fn create_test_ticket(slug: &str, title: &str, description: &str, tags: Vec<&str>) -> Ticket {
        Ticket {
            id: TicketId::new(),
            slug: slug.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status: Status::Todo,
            priority: Priority::Medium,
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
            assignee: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            archived: false,
            metadata: None,
        }
    }

    #[test]
    fn test_fuzzy_search_partial_match() {
        let tickets = vec![
            create_test_ticket(
                "fix-authentication",
                "Fix authentication bug",
                "Authentication is broken",
                vec!["bug", "auth"],
            ),
            create_test_ticket(
                "add-authorization",
                "Add authorization checks",
                "Need to add auth checks",
                vec!["feature", "security"],
            ),
        ];

        let searcher = FuzzySearcher::new();
        let results = searcher.search("auth", &tickets);

        assert_eq!(results.len(), 2, "Should find both tickets with 'auth'");
        assert!(results.iter().all(|r| r.score > 0), "All results should have positive scores");
    }

    #[test]
    fn test_fuzzy_search_typo_tolerance() {
        let tickets = vec![
            create_test_ticket(
                "database-migration",
                "Database migration script",
                "Migrate from MySQL to PostgreSQL",
                vec!["database", "migration"],
            ),
        ];

        let searcher = FuzzySearcher::new();
        
        // Test various typos
        let typo_queries = vec!["databse", "dtabase", "database", "datbase"];
        
        for query in typo_queries {
            let results = searcher.search(query, &tickets);
            assert_eq!(results.len(), 1, "Should find ticket despite typo: {}", query);
        }
    }

    #[test]
    fn test_fuzzy_search_field_specific() {
        let tickets = vec![
            create_test_ticket(
                "test-ticket",
                "Performance improvements",
                "This is about authentication",
                vec!["testing"],
            ),
        ];

        // Search only in titles
        let mut config = FuzzySearchConfig::default();
        config.search_description = false;
        config.search_tags = false;
        
        let title_searcher = FuzzySearcher::with_config(config);
        let results = title_searcher.search("authentication", &tickets);
        assert_eq!(results.len(), 0, "Should not find in description when searching titles only");

        // Search only in descriptions
        let mut config = FuzzySearchConfig::default();
        config.search_title = false;
        config.search_tags = false;
        
        let desc_searcher = FuzzySearcher::with_config(config);
        let results = desc_searcher.search("authentication", &tickets);
        assert_eq!(results.len(), 1, "Should find in description");
    }

    #[test]
    fn test_fuzzy_search_scoring() {
        let tickets = vec![
            create_test_ticket(
                "exact-match",
                "authentication",
                "Something else",
                vec![],
            ),
            create_test_ticket(
                "partial-match",
                "authentication system",
                "Something else",
                vec![],
            ),
            create_test_ticket(
                "distant-match",
                "new authentication and authorization system",
                "Something else",
                vec![],
            ),
        ];

        let searcher = FuzzySearcher::new();
        let results = searcher.search("authentication", &tickets);

        assert_eq!(results.len(), 3, "Should find all three tickets");
        
        // Exact match should score highest
        assert_eq!(results[0].ticket.slug, "exact-match", "Exact match should rank first");
        assert!(results[0].score > results[1].score, "Exact match should score higher than partial");
    }

    #[test]
    fn test_fuzzy_search_empty_query() {
        let tickets = vec![
            create_test_ticket("test", "Test ticket", "Description", vec![]),
        ];

        let searcher = FuzzySearcher::new();
        let results = searcher.search("", &tickets);
        
        assert_eq!(results.len(), 0, "Empty query should return no results");
    }

    #[test]
    fn test_fuzzy_search_min_score_threshold() {
        let tickets = vec![
            create_test_ticket(
                "unrelated",
                "Completely unrelated ticket",
                "Nothing to do with the search",
                vec!["random"],
            ),
        ];

        let mut config = FuzzySearchConfig::default();
        config.min_score = 80; // High threshold
        
        let searcher = FuzzySearcher::with_config(config);
        let results = searcher.search("authentication", &tickets);
        
        assert_eq!(results.len(), 0, "Should filter out low-scoring matches");
    }

    #[test]
    fn test_highlight_matches() {
        use crate::search::highlight_matches;
        
        let text = "authentication";
        let indices = vec![0, 1, 2, 3]; // First 4 characters
        
        let highlighted = highlight_matches(text, &indices);
        assert!(highlighted.contains("**auth**"), "Should highlight matched characters");
    }
}