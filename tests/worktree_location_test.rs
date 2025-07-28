//! Test that worktree configuration uses current directory by default

#[test]
fn test_default_worktree_prefix_is_current_directory() {
    // The default config should use ./{project}-vibeticket- prefix
    let config_content = include_str!("../src/config/mod.rs");
    assert!(
        config_content.contains(r#"worktree_prefix: "./{project}-vibeticket-".to_string()"#),
        "Default worktree prefix should be in current directory"
    );
}

#[test]
fn test_gitignore_includes_worktree_pattern() {
    // Check that .gitignore pattern is correct
    let gitignore_content = include_str!("../.gitignore");
    assert!(
        gitignore_content.contains("*-vibeticket-*/"),
        ".gitignore should include worktree pattern"
    );
}

#[test]
fn test_init_adds_gitignore_pattern() {
    // Check that init handler adds the pattern
    let init_content = include_str!("../src/cli/handlers/init.rs");
    assert!(
        init_content.contains("*-vibeticket-*/"),
        "Init handler should add worktree pattern to .gitignore"
    );
}
