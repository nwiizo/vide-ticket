//! Handler for the `search` command
//!
//! This module implements the logic for searching tickets
//! by title, description, tags, or using regex patterns.

use crate::cli::{find_project_root, OutputFormatter};
use crate::core::Ticket;
use crate::error::Result;
use crate::search::{FuzzySearcher, FuzzySearchConfig};
use crate::storage::{FileStorage, TicketRepository};
use regex::Regex;

/// Parameters for the search command
pub struct SearchParams<'a> {
    /// Search query or regex pattern
    pub query: &'a str,
    /// Search only in titles
    pub title_only: bool,
    /// Search only in descriptions
    pub description_only: bool,
    /// Search only in tags
    pub tags_only: bool,
    /// Treat query as a regex pattern
    pub use_regex: bool,
    /// Use fuzzy matching for better discovery
    pub use_fuzzy: bool,
    /// Optional project directory path
    pub project_dir: Option<&'a str>,
}

/// Handler for the `search` command
///
/// Searches tickets based on various criteria:
/// 1. Full text search across all fields
/// 2. Title-only search
/// 3. Description-only search
/// 4. Tags-only search
/// 5. Regex pattern matching
///
/// # Arguments
///
/// * `params` - Search command parameters
/// * `output` - Output formatter for displaying results
pub fn handle_search_command(params: &SearchParams<'_>, output: &OutputFormatter) -> Result<()> {
    // Ensure project is initialized
    let project_root = find_project_root(params.project_dir)?;
    let vibe_ticket_dir = project_root.join(".vibe-ticket");

    // Initialize storage
    let storage = FileStorage::new(&vibe_ticket_dir);

    // Load all tickets
    let tickets = storage.load_all()?;

    // Compile regex if needed
    let regex = if params.use_regex {
        Some(Regex::new(params.query).map_err(|e| {
            crate::error::VibeTicketError::custom(format!("Invalid regex pattern: {e}"))
        })?)
    } else {
        None
    };

    // Search tickets
    let matches = if params.use_fuzzy {
        search_tickets_fuzzy(tickets, params)?
    } else {
        search_tickets(tickets, params, regex.as_ref())?
    };

    // Output results
    output_search_results(output, params, &matches, regex.as_ref())?;

    Ok(())
}

/// Search tickets based on parameters and return matches
fn search_tickets(
    tickets: Vec<Ticket>,
    params: &SearchParams<'_>,
    regex: Option<&Regex>,
) -> Result<Vec<(Ticket, Vec<String>)>> {
    let mut matches: Vec<(Ticket, Vec<String>)> = Vec::new();

    for ticket in tickets {
        let match_locations = if params.use_regex {
            search_ticket_regex(&ticket, params, regex.unwrap())
        } else {
            search_ticket_substring(&ticket, params)
        };

        if !match_locations.is_empty() {
            matches.push((ticket, match_locations));
        }
    }

    // Sort matches by creation date (newest first)
    matches.sort_by(|a, b| b.0.created_at.cmp(&a.0.created_at));

    Ok(matches)
}

/// Search tickets using fuzzy matching
fn search_tickets_fuzzy(
    tickets: Vec<Ticket>,
    params: &SearchParams<'_>,
) -> Result<Vec<(Ticket, Vec<String>)>> {
    // Configure fuzzy search based on parameters
    let mut config = FuzzySearchConfig::default();
    config.search_description = !params.title_only && !params.tags_only;
    config.search_tags = !params.title_only && !params.description_only;
    
    let searcher = FuzzySearcher::with_config(config);
    let fuzzy_matches = searcher.search(params.query, &tickets);
    
    // Convert fuzzy matches to the expected format
    let matches: Vec<(Ticket, Vec<String>)> = fuzzy_matches
        .into_iter()
        .map(|fm| {
            let locations: Vec<String> = fm.matched_fields
                .into_iter()
                .map(|mf| format!("{} (score: {})", mf.field, mf.score))
                .collect();
            (fm.ticket, locations)
        })
        .collect();
    
    Ok(matches)
}

/// Search a ticket using regex
fn search_ticket_regex(ticket: &Ticket, params: &SearchParams<'_>, regex: &Regex) -> Vec<String> {
    let mut match_locations = Vec::new();

    if !params.title_only && !params.description_only && !params.tags_only {
        // Search all fields
        if regex.is_match(&ticket.title) {
            match_locations.push("title".to_string());
        }
        if regex.is_match(&ticket.description) {
            match_locations.push("description".to_string());
        }
        if ticket.tags.iter().any(|tag| regex.is_match(tag)) {
            match_locations.push("tags".to_string());
        }
    } else {
        // Search specific fields
        if params.title_only && regex.is_match(&ticket.title) {
            match_locations.push("title".to_string());
        }
        if params.description_only && regex.is_match(&ticket.description) {
            match_locations.push("description".to_string());
        }
        if params.tags_only && ticket.tags.iter().any(|tag| regex.is_match(tag)) {
            match_locations.push("tags".to_string());
        }
    }

    match_locations
}

/// Search a ticket using substring matching
fn search_ticket_substring(ticket: &Ticket, params: &SearchParams<'_>) -> Vec<String> {
    let mut match_locations = Vec::new();
    let query_lower = params.query.to_lowercase();

    if !params.title_only && !params.description_only && !params.tags_only {
        // Search all fields
        if ticket.title.to_lowercase().contains(&query_lower) {
            match_locations.push("title".to_string());
        }
        if ticket.description.to_lowercase().contains(&query_lower) {
            match_locations.push("description".to_string());
        }
        if ticket
            .tags
            .iter()
            .any(|tag| tag.to_lowercase().contains(&query_lower))
        {
            match_locations.push("tags".to_string());
        }
    } else {
        // Search specific fields
        if params.title_only && ticket.title.to_lowercase().contains(&query_lower) {
            match_locations.push("title".to_string());
        }
        if params.description_only && ticket.description.to_lowercase().contains(&query_lower) {
            match_locations.push("description".to_string());
        }
        if params.tags_only
            && ticket
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&query_lower))
        {
            match_locations.push("tags".to_string());
        }
    }

    match_locations
}

/// Output search results in the appropriate format
fn output_search_results(
    output: &OutputFormatter,
    params: &SearchParams<'_>,
    matches: &[(Ticket, Vec<String>)],
    regex: Option<&Regex>,
) -> Result<()> {
    if output.is_json() {
        output_json_results(output, params, matches)?;
    } else {
        output_text_results(output, params, matches, regex);
    }
    Ok(())
}

/// Output search results in JSON format
fn output_json_results(
    output: &OutputFormatter,
    params: &SearchParams<'_>,
    matches: &[(Ticket, Vec<String>)],
) -> Result<()> {
    output.print_json(&serde_json::json!({
        "query": params.query,
        "regex": params.use_regex,
        "search_fields": {
            "title": params.title_only || (!params.description_only && !params.tags_only),
            "description": params.description_only || (!params.title_only && !params.tags_only),
            "tags": params.tags_only || (!params.title_only && !params.description_only),
        },
        "results": matches.iter().map(|(ticket, locations)| serde_json::json!({
            "id": ticket.id.to_string(),
            "slug": ticket.slug,
            "title": ticket.title,
            "status": ticket.status.to_string(),
            "priority": ticket.priority.to_string(),
            "matched_in": locations,
        })).collect::<Vec<_>>(),
        "total": matches.len(),
    }))
}

/// Output search results in text format
fn output_text_results(
    output: &OutputFormatter,
    params: &SearchParams<'_>,
    matches: &[(Ticket, Vec<String>)],
    regex: Option<&Regex>,
) {
    if matches.is_empty() {
        output.info(&format!("No tickets found matching '{}'", params.query));
        return;
    }

    output.success(&format!(
        "Found {} ticket{} matching '{}'",
        matches.len(),
        if matches.len() == 1 { "" } else { "s" },
        params.query
    ));
    output.info("");

    for (ticket, locations) in matches {
        let status_emoji = match ticket.status {
            crate::core::Status::Todo => "📋",
            crate::core::Status::Doing => "🔄",
            crate::core::Status::Review => "👀",
            crate::core::Status::Blocked => "🚫",
            crate::core::Status::Done => "✅",
        };

        output.info(&format!(
            "{} {} - {}",
            status_emoji, ticket.slug, ticket.title
        ));
        output.info(&format!(
            "   Priority: {} | Status: {} | Matched in: {}",
            ticket.priority,
            ticket.status,
            locations.join(", ")
        ));

        // Show matching context for description
        if locations.contains(&"description".to_string()) && !params.description_only {
            let excerpt =
                get_match_excerpt(&ticket.description, params.query, params.use_regex, regex);
            if let Some(excerpt) = excerpt {
                output.info(&format!("   Description: ...{excerpt}..."));
            }
        }

        // Show matching tags
        if locations.contains(&"tags".to_string()) && !params.tags_only && !ticket.tags.is_empty() {
            output.info(&format!("   Tags: {}", ticket.tags.join(", ")));
        }

        output.info("");
    }
}

/// Extract a short excerpt around the match
fn get_match_excerpt(
    text: &str,
    query: &str,
    use_regex: bool,
    regex: Option<&Regex>,
) -> Option<String> {
    const CONTEXT_CHARS: usize = 30;

    let match_pos = if use_regex {
        regex?.find(text).map(|m| m.start())
    } else {
        text.to_lowercase().find(&query.to_lowercase())
    };

    if let Some(pos) = match_pos {
        let start = pos.saturating_sub(CONTEXT_CHARS);
        let end = (pos + query.len() + CONTEXT_CHARS).min(text.len());

        let excerpt = &text[start..end];
        let excerpt = excerpt.replace('\n', " ").trim().to_string();

        Some(excerpt)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_excerpt() {
        let text = "This is a long text with some interesting content in the middle of it.";
        let query = "interesting";
        let excerpt = get_match_excerpt(text, query, false, None);
        assert!(excerpt.is_some());
        assert!(excerpt.unwrap().contains("interesting"));
    }

    #[test]
    fn test_regex_validation() {
        assert!(Regex::new("test.*pattern").is_ok());
        assert!(Regex::new("[invalid").is_err());
    }
}
