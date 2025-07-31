// Test file to verify doc tests compile and run correctly

// Test the parse_tags function doc test
#[test]
fn test_parse_tags_doc_example() {
    use vibe_ticket::cli::handlers::parse_tags;
    
    let tags = parse_tags(Some("bug, ui, urgent".to_string()));
    assert_eq!(tags, vec!["bug", "ui", "urgent"]);
}

// The doc test should handle empty strings and whitespace correctly
#[test]
fn test_parse_tags_edge_cases() {
    use vibe_ticket::cli::handlers::parse_tags;
    
    // Empty input
    let tags = parse_tags(None);
    assert_eq!(tags, Vec::<String>::new());
    
    // Empty string
    let tags = parse_tags(Some("".to_string()));
    assert_eq!(tags, Vec::<String>::new());
    
    // Only whitespace
    let tags = parse_tags(Some("  ,  ,  ".to_string()));
    assert_eq!(tags, Vec::<String>::new());
    
    // Tags with extra whitespace
    let tags = parse_tags(Some(" bug , ui , urgent ".to_string()));
    assert_eq!(tags, vec!["bug", "ui", "urgent"]);
}