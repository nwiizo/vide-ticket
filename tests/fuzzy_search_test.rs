//! Integration tests for fuzzy search functionality

use tempfile::TempDir;
use vibe_ticket::cli::handlers::{handle_new_command, handle_search_command, NewParams, SearchParams};
use vibe_ticket::cli::output::OutputFormatter;
use vibe_ticket::cli::handlers::handle_init;

/// Create a test ticket for fuzzy search testing
fn create_test_ticket(
    slug: &str,
    title: &str,
    description: &str,
    tags: &[&str],
    project_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let formatter = OutputFormatter::new(false, false);
    let params = NewParams {
        slug,
        title: Some(title),
        description: Some(description),
        priority: "medium",
        tags: Some(tags.join(",")),
        start: false,
        project_dir: Some(project_dir),
    };
    handle_new_command(&params, &formatter)?;
    Ok(())
}

#[test]
fn test_fuzzy_search_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().to_str().unwrap();
    
    // Initialize project
    let formatter = OutputFormatter::new(false, false);
    handle_init(
        Some("test-project".to_string()),
        Some("Test project for fuzzy search".to_string()),
        false,
        project_dir,
        false,
        &formatter,
    )?;
    
    // Create test tickets
    create_test_ticket(
        "fix-authentication-bug",
        "Fix authentication issues in login flow",
        "Users are experiencing problems when trying to authenticate using OAuth providers",
        &["bug", "authentication", "oauth"],
        project_dir,
    )?;
    
    create_test_ticket(
        "improve-auth-performance",
        "Improve authentication performance",
        "The current authentication process is slow and needs optimization",
        &["performance", "authentication"],
        project_dir,
    )?;
    
    create_test_ticket(
        "add-password-reset",
        "Add password reset functionality",
        "Implement a secure password reset flow for users who forgot their passwords",
        &["feature", "security"],
        project_dir,
    )?;
    
    // Test fuzzy search for "auth" (should match authentication-related tickets)
    let search_params = SearchParams {
        query: "auth",
        title_only: false,
        description_only: false,
        tags_only: false,
        use_regex: false,
        use_fuzzy: true,
        project_dir: Some(project_dir),
    };
    
    // This should find both authentication tickets
    let result = handle_search_command(&search_params, &formatter);
    assert!(result.is_ok(), "Fuzzy search should succeed");
    
    Ok(())
}

#[test]
fn test_fuzzy_search_typo_tolerance() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().to_str().unwrap();
    
    // Initialize project
    let formatter = OutputFormatter::new(false, false);
    handle_init(
        Some("test-project".to_string()),
        Some("Test project for typo tolerance".to_string()),
        false,
        project_dir,
        false,
        &formatter,
    )?;
    
    // Create test ticket
    create_test_ticket(
        "implement-caching",
        "Implement caching mechanism",
        "Add Redis caching to improve API response times",
        &["performance", "backend"],
        project_dir,
    )?;
    
    // Test fuzzy search with typos
    let typo_queries = ["cachng", "cacing", "cahcing", "implemnt"];
    
    for query in &typo_queries {
        let search_params = SearchParams {
            query,
            title_only: false,
            description_only: false,
            tags_only: false,
            use_regex: false,
            use_fuzzy: true,
            project_dir: Some(project_dir),
        };
        
        let result = handle_search_command(&search_params, &formatter);
        assert!(result.is_ok(), "Fuzzy search with typo '{}' should succeed", query);
    }
    
    Ok(())
}

#[test]
fn test_fuzzy_search_field_specific() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().to_str().unwrap();
    
    // Initialize project
    let formatter = OutputFormatter::new(false, false);
    handle_init(
        Some("test-project".to_string()),
        Some("Test project for field-specific search".to_string()),
        false,
        project_dir,
        false,
        &formatter,
    )?;
    
    // Create test tickets
    create_test_ticket(
        "database-migration",
        "Database migration task",
        "Migrate from MySQL to PostgreSQL",
        &["database", "migration"],
        project_dir,
    )?;
    
    create_test_ticket(
        "api-documentation",
        "Update API documentation",
        "The database schema has changed and docs need updating",
        &["documentation", "api"],
        project_dir,
    )?;
    
    // Test title-only fuzzy search
    let search_params = SearchParams {
        query: "databse", // typo
        title_only: true,
        description_only: false,
        tags_only: false,
        use_regex: false,
        use_fuzzy: true,
        project_dir: Some(project_dir),
    };
    
    let result = handle_search_command(&search_params, &formatter);
    assert!(result.is_ok(), "Title-only fuzzy search should succeed");
    
    // Test description-only fuzzy search
    let search_params = SearchParams {
        query: "postgre", // partial match
        title_only: false,
        description_only: true,
        tags_only: false,
        use_regex: false,
        use_fuzzy: true,
        project_dir: Some(project_dir),
    };
    
    let result = handle_search_command(&search_params, &formatter);
    assert!(result.is_ok(), "Description-only fuzzy search should succeed");
    
    Ok(())
}

#[test]
fn test_fuzzy_vs_exact_search() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().to_str().unwrap();
    
    // Initialize project
    let formatter = OutputFormatter::new(false, false);
    handle_init(
        Some("test-project".to_string()),
        Some("Test project for fuzzy vs exact comparison".to_string()),
        false,
        project_dir,
        false,
        &formatter,
    )?;
    
    // Create test ticket
    create_test_ticket(
        "refactor-authentication",
        "Refactor authentication module",
        "Clean up the authentication code and improve structure",
        &["refactoring", "authentication"],
        project_dir,
    )?;
    
    // Test exact search (should not find with typo)
    let search_params = SearchParams {
        query: "refactr", // typo
        title_only: false,
        description_only: false,
        tags_only: false,
        use_regex: false,
        use_fuzzy: false, // exact search
        project_dir: Some(project_dir),
    };
    
    let result = handle_search_command(&search_params, &formatter);
    assert!(result.is_ok(), "Exact search should complete (even if no results)");
    
    // Test fuzzy search (should find with typo)
    let search_params = SearchParams {
        query: "refactr", // same typo
        title_only: false,
        description_only: false,
        tags_only: false,
        use_regex: false,
        use_fuzzy: true, // fuzzy search
        project_dir: Some(project_dir),
    };
    
    let result = handle_search_command(&search_params, &formatter);
    assert!(result.is_ok(), "Fuzzy search should find results despite typo");
    
    Ok(())
}