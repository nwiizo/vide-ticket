//! Handler for Git worktree management commands
//!
//! This module provides functionality to manage Git worktrees associated with tickets,
//! enabling parallel development workflows.

use crate::cli::{find_project_root, OutputFormatter};
use crate::config::Config;
use crate::error::{Result, VibeTicketError};
use crate::storage::{FileStorage, TicketRepository};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Handle the worktree list command
pub fn handle_worktree_list(
    all: bool,
    status_filter: Option<String>,
    verbose: bool,
    output: &OutputFormatter,
) -> Result<()> {
    let project_root = find_project_root(None)?;
    let config = Config::load_or_default()?;
    
    // Get all Git worktrees
    let worktrees = list_git_worktrees(&project_root)?;
    
    // Load ticket information
    let storage = FileStorage::new(project_root.join(".vibe-ticket"));
    let tickets = storage.load_all()?;
    
    // Create a map of ticket slugs to tickets
    let ticket_map: HashMap<String, _> = tickets
        .into_iter()
        .map(|t| (t.slug.clone(), t))
        .collect();
    
    // Filter and display worktrees
    let mut displayed_count = 0;
    let mut filtered_worktrees = Vec::new();
    
    for worktree in worktrees {
        // Extract ticket slug from worktree path
        let ticket_slug = extract_ticket_slug(&worktree.path, &config)?;
        
        // Check if this is a vibe-ticket worktree
        if !all && ticket_slug.is_none() {
            continue;
        }
        
        // Apply status filter if provided
        if let Some(ref filter) = status_filter {
            let status = determine_worktree_status(&worktree);
            if !status.eq_ignore_ascii_case(filter) {
                continue;
            }
        }
        
        // Display worktree information
        if output.is_json() {
            filtered_worktrees.push(worktree);
        } else {
            display_worktree(&worktree, ticket_slug.as_deref(), &ticket_map, verbose, output);
        }
        
        displayed_count += 1;
    }
    
    if output.is_json() {
        output.json(&serde_json::json!({
            "worktrees": filtered_worktrees,
            "count": displayed_count,
        }))?;
    } else {
        output.info(&format!("\nTotal worktrees: {}", displayed_count));
    }
    
    Ok(())
}

/// Handle the worktree remove command
pub fn handle_worktree_remove(
    worktree_ref: &str,
    force: bool,
    keep_branch: bool,
    output: &OutputFormatter,
) -> Result<()> {
    let project_root = find_project_root(None)?;
    let config = Config::load_or_default()?;
    
    // Resolve worktree path
    let worktree_path = resolve_worktree_path(worktree_ref, &project_root, &config)?;
    
    // Check for uncommitted changes
    if !force {
        check_uncommitted_changes(&worktree_path)?;
    }
    
    // Get branch name before removal
    let branch_name = get_worktree_branch(&worktree_path)?;
    
    // Remove the worktree
    remove_git_worktree(&project_root, &worktree_path, force)?;
    
    output.success(&format!(
        "Removed worktree: {}",
        worktree_path.display()
    ));
    
    // Remove branch if requested
    if !keep_branch && branch_name.is_some() {
        let branch = branch_name.unwrap();
        remove_git_branch(&project_root, &branch)?;
        output.info(&format!("Removed branch: {}", branch));
    }
    
    Ok(())
}

/// Handle the worktree prune command
pub fn handle_worktree_prune(
    force: bool,
    dry_run: bool,
    remove_branches: bool,
    output: &OutputFormatter,
) -> Result<()> {
    let project_root = find_project_root(None)?;
    
    // Run git worktree prune
    let mut cmd = Command::new("git");
    cmd.arg("worktree")
        .arg("prune")
        .current_dir(&project_root);
    
    if dry_run {
        cmd.arg("--dry-run");
    }
    
    if !force && !dry_run {
        output.warning("This will remove stale worktree information. Use --force to confirm.");
        return Ok(());
    }
    
    let result = cmd.output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to run git worktree prune: {}", e)))?;
    
    if !result.status.success() {
        let error_msg = String::from_utf8_lossy(&result.stderr);
        return Err(VibeTicketError::custom(format!(
            "Failed to prune worktrees: {}",
            error_msg
        )));
    }
    
    let output_text = String::from_utf8_lossy(&result.stdout);
    if output_text.is_empty() {
        output.info("No stale worktrees found");
    } else {
        output.success(&format!("Pruned worktrees:\n{}", output_text));
        
        // TODO: Implement branch removal for pruned worktrees
        if remove_branches && !dry_run {
            output.warning("Branch removal not yet implemented");
        }
    }
    
    Ok(())
}

/// Worktree information
#[derive(Debug, serde::Serialize)]
struct WorktreeInfo {
    path: PathBuf,
    branch: Option<String>,
    commit: String,
    status: String,
}

/// List all Git worktrees
fn list_git_worktrees(project_root: &Path) -> Result<Vec<WorktreeInfo>> {
    let output = Command::new("git")
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .current_dir(project_root)
        .output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to list worktrees: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(VibeTicketError::custom(format!(
            "Failed to list worktrees: {}",
            error_msg
        )));
    }
    
    let output_text = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current_worktree: Option<WorktreeInfo> = None;
    
    for line in output_text.lines() {
        if line.starts_with("worktree ") {
            if let Some(wt) = current_worktree.take() {
                worktrees.push(wt);
            }
            let path = PathBuf::from(line.strip_prefix("worktree ").unwrap());
            current_worktree = Some(WorktreeInfo {
                path,
                branch: None,
                commit: String::new(),
                status: "active".to_string(),
            });
        } else if line.starts_with("HEAD ") {
            if let Some(ref mut wt) = current_worktree {
                wt.commit = line.strip_prefix("HEAD ").unwrap().to_string();
            }
        } else if line.starts_with("branch ") {
            if let Some(ref mut wt) = current_worktree {
                wt.branch = Some(line.strip_prefix("branch ").unwrap().to_string());
            }
        } else if line.starts_with("detached") {
            if let Some(ref mut wt) = current_worktree {
                wt.status = "detached".to_string();
            }
        }
    }
    
    if let Some(wt) = current_worktree {
        worktrees.push(wt);
    }
    
    Ok(worktrees)
}

/// Extract ticket slug from worktree path
fn extract_ticket_slug(path: &Path, config: &Config) -> Result<Option<String>> {
    let _path_str = path.to_string_lossy();
    let project_name = &config.project.name;
    let prefix = config.git.worktree_prefix.replace("{project}", project_name);
    
    // Check if this follows our worktree naming pattern
    if let Some(file_name) = path.file_name() {
        let name = file_name.to_string_lossy();
        let prefix_cleaned = prefix.trim_start_matches("../").trim_end_matches('-');
        
        if name.starts_with(prefix_cleaned) {
            let slug = name.strip_prefix(prefix_cleaned)
                .unwrap()
                .trim_start_matches('-');
            return Ok(Some(slug.to_string()));
        }
    }
    
    Ok(None)
}

/// Determine worktree status
fn determine_worktree_status(worktree: &WorktreeInfo) -> String {
    if !worktree.path.exists() {
        "orphaned".to_string()
    } else if worktree.status == "detached" {
        "detached".to_string()
    } else {
        "active".to_string()
    }
}

/// Display worktree information
fn display_worktree(
    worktree: &WorktreeInfo,
    ticket_slug: Option<&str>,
    ticket_map: &HashMap<String, crate::core::Ticket>,
    verbose: bool,
    output: &OutputFormatter,
) {
    let status = determine_worktree_status(worktree);
    let path_display = worktree.path.display();
    
    if let Some(slug) = ticket_slug {
        if let Some(ticket) = ticket_map.get(slug) {
            output.info(&format!(
                "{} [{}] - {} ({})",
                path_display,
                status,
                ticket.title,
                ticket.status
            ));
        } else {
            output.info(&format!(
                "{} [{}] - {} (no ticket found)",
                path_display,
                status,
                slug
            ));
        }
    } else {
        output.info(&format!("{} [{}]", path_display, status));
    }
    
    if verbose {
        if let Some(branch) = &worktree.branch {
            output.info(&format!("  Branch: {}", branch));
        }
        output.info(&format!("  Commit: {}", &worktree.commit[..8]));
    }
}

/// Resolve worktree path from reference
fn resolve_worktree_path(
    worktree_ref: &str,
    project_root: &Path,
    config: &Config,
) -> Result<PathBuf> {
    // Check if it's a direct path
    let path = Path::new(worktree_ref);
    if path.is_absolute() || path.exists() {
        return Ok(path.to_path_buf());
    }
    
    // Try to resolve as ticket slug
    let project_name = &config.project.name;
    let prefix = config.git.worktree_prefix.replace("{project}", project_name);
    let worktree_name = format!("{}{}", prefix.trim_end_matches('-'), worktree_ref);
    
    let parent_dir = project_root.parent()
        .ok_or_else(|| VibeTicketError::custom("Cannot find parent directory"))?;
    
    let worktree_path = parent_dir.join(&worktree_name);
    if worktree_path.exists() {
        return Ok(worktree_path);
    }
    
    Err(VibeTicketError::custom(format!(
        "Worktree not found: {}",
        worktree_ref
    )))
}

/// Check for uncommitted changes in worktree
fn check_uncommitted_changes(worktree_path: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(worktree_path)
        .output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to check git status: {}", e)))?;
    
    if !output.status.success() {
        // Might not be a git repository, which is fine
        return Ok(());
    }
    
    let output_text = String::from_utf8_lossy(&output.stdout);
    if !output_text.trim().is_empty() {
        return Err(VibeTicketError::custom(
            "Worktree has uncommitted changes. Use --force to remove anyway"
        ));
    }
    
    Ok(())
}

/// Get branch name for worktree
fn get_worktree_branch(worktree_path: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(worktree_path)
        .output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to get branch name: {}", e)))?;
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch == "HEAD" {
        Ok(None)
    } else {
        Ok(Some(branch))
    }
}

/// Remove a Git worktree
fn remove_git_worktree(project_root: &Path, worktree_path: &Path, force: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("worktree")
        .arg("remove")
        .arg(worktree_path)
        .current_dir(project_root);
    
    if force {
        cmd.arg("--force");
    }
    
    let output = cmd.output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to remove worktree: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(VibeTicketError::custom(format!(
            "Failed to remove worktree: {}",
            error_msg
        )));
    }
    
    Ok(())
}

/// Remove a Git branch
fn remove_git_branch(project_root: &Path, branch: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .current_dir(project_root)
        .output()
        .map_err(|e| VibeTicketError::custom(format!("Failed to remove branch: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        // Try force delete if regular delete fails
        let force_output = Command::new("git")
            .arg("branch")
            .arg("-D")
            .arg(branch)
            .current_dir(project_root)
            .output()
            .map_err(|e| VibeTicketError::custom(format!("Failed to force remove branch: {}", e)))?;
        
        if !force_output.status.success() {
            return Err(VibeTicketError::custom(format!(
                "Failed to remove branch: {}",
                error_msg
            )));
        }
    }
    
    Ok(())
}