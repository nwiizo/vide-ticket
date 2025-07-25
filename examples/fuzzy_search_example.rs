//! Example of using fuzzy search functionality in vibe-ticket
//!
//! This example demonstrates how to use the fuzzy search feature
//! to find tickets even with typos or partial matches.

use vibe_ticket::search::{FuzzySearcher, FuzzySearchConfig};
use vibe_ticket::core::{Ticket, Status, Priority, TicketId};
use chrono::Utc;

fn main() {
    // Create sample tickets
    let tickets = vec![
        Ticket {
            id: TicketId::new(),
            slug: "fix-authentication-bug".to_string(),
            title: "Fix authentication issues in login flow".to_string(),
            description: "Users are experiencing problems when trying to authenticate using OAuth providers".to_string(),
            status: Status::Todo,
            priority: Priority::High,
            tags: vec!["bug".to_string(), "authentication".to_string(), "oauth".to_string()],
            assignee: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            archived: false,
            metadata: None,
        },
        Ticket {
            id: TicketId::new(),
            slug: "improve-auth-performance".to_string(),
            title: "Improve authentication performance".to_string(),
            description: "The current authentication process is slow and needs optimization".to_string(),
            status: Status::Doing,
            priority: Priority::Medium,
            tags: vec!["performance".to_string(), "authentication".to_string()],
            assignee: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            archived: false,
            metadata: None,
        },
        Ticket {
            id: TicketId::new(),
            slug: "add-password-reset".to_string(),
            title: "Add password reset functionality".to_string(),
            description: "Implement a secure password reset flow for users who forgot their passwords".to_string(),
            status: Status::Todo,
            priority: Priority::Medium,
            tags: vec!["feature".to_string(), "security".to_string()],
            assignee: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            archived: false,
            metadata: None,
        },
    ];

    // Create fuzzy searcher with default configuration
    let searcher = FuzzySearcher::new();

    println!("=== Fuzzy Search Examples ===\n");

    // Example 1: Search for "auth" (partial match)
    println!("1. Searching for 'auth' (partial match):");
    let results = searcher.search("auth", &tickets);
    for result in &results {
        println!("   - {} (score: {})", result.ticket.title, result.score);
    }
    println!();

    // Example 2: Search with typo
    println!("2. Searching for 'autentication' (with typo):");
    let results = searcher.search("autentication", &tickets);
    for result in &results {
        println!("   - {} (score: {})", result.ticket.title, result.score);
    }
    println!();

    // Example 3: Search for partial word
    println!("3. Searching for 'passw' (partial word):");
    let results = searcher.search("passw", &tickets);
    for result in &results {
        println!("   - {} (score: {})", result.ticket.title, result.score);
    }
    println!();

    // Example 4: Custom configuration - search only in titles
    println!("4. Searching only in titles for 'performance':");
    let mut config = FuzzySearchConfig::default();
    config.search_description = false;
    config.search_tags = false;
    let title_searcher = FuzzySearcher::with_config(config);
    let results = title_searcher.search("performance", &tickets);
    for result in &results {
        println!("   - {} (score: {})", result.ticket.title, result.score);
    }
    println!();

    // Example 5: Highlight matches
    println!("5. Highlighting matches for 'oauth':");
    let results = searcher.search("oauth", &tickets);
    if let Some(first_result) = results.first() {
        for matched_field in &first_result.matched_fields {
            let highlighted = vibe_ticket::search::highlight_matches(
                &matched_field.content,
                &matched_field.indices,
            );
            println!("   Field '{}': {}", matched_field.field, highlighted);
        }
    }
}