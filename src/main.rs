//! vibe-ticket - High-performance ticket management system
//!
//! This is the main entry point for the vibe-ticket CLI application.
//! It handles command-line argument parsing and dispatches to the appropriate
//! command handlers.

use clap::Parser;
use std::process;
use vibe_ticket::cli::{
    handlers::handle_init, Cli, Commands, OutputFormatter, SpecCommands, TaskCommands,
};
use vibe_ticket::error::Result;

/// Main entry point for the vibe-ticket CLI
///
/// Parses command-line arguments and executes the requested command.
/// Handles errors gracefully and provides helpful error messages to users.
fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Configure output formatter based on flags
    let formatter = OutputFormatter::new(cli.json, cli.no_color);

    // Execute the command and handle errors
    if let Err(e) = run(cli, &formatter) {
        handle_error(e, &formatter);
        process::exit(1);
    }
}

/// Run the CLI application with the parsed arguments
///
/// This function dispatches to the appropriate command handler based on
/// the parsed command. Each handler is responsible for its own business logic.
///
/// # Arguments
///
/// * `cli` - Parsed CLI arguments
/// * `formatter` - Output formatter for displaying results
///
/// # Errors
///
/// Returns any error that occurs during command execution
fn run(cli: Cli, formatter: &OutputFormatter) -> Result<()> {
    // Set up logging if verbose mode is enabled
    if cli.verbose {
        tracing_subscriber::fmt().with_env_filter("debug").init();
    }

    // Change to project directory if specified
    if let Some(project_path) = &cli.project {
        std::env::set_current_dir(project_path).map_err(vibe_ticket::error::VibeTicketError::Io)?;
    }

    // Dispatch to command handler
    match cli.command {
        Commands::Init {
            name,
            description,
            force,
            claude_md,
        } => handle_init(
            name.as_deref(),
            description.as_deref(),
            force,
            claude_md,
            formatter,
        ),

        Commands::New {
            slug,
            title,
            description,
            priority,
            tags,
            start,
        } => {
            use vibe_ticket::cli::handlers::{handle_new_command, NewParams};
            let params = NewParams {
                slug: &slug,
                title,
                description,
                priority: &priority,
                tags,
                start,
                project_dir: cli.project.as_deref(),
            };
            handle_new_command(params, formatter)
        },

        Commands::List {
            status,
            priority,
            assignee,
            sort,
            reverse,
            limit,
            archived,
            open,
            since,
            until,
        } => {
            use vibe_ticket::cli::handlers::{handle_list_command, ListParams};
            let params = ListParams {
                status,
                priority,
                assignee,
                sort: &sort,
                reverse,
                limit,
                archived,
                open,
                since,
                until,
                project_dir: cli.project.as_deref(),
            };
            handle_list_command(params, formatter)
        },

        Commands::Open {
            sort,
            reverse,
            limit,
        } => {
            use vibe_ticket::cli::handlers::{handle_list_command, ListParams};
            // Call list handler with open filter set to true
            let params = ListParams {
                status: None,
                priority: None,
                assignee: None,
                sort: &sort,
                reverse,
                limit,
                archived: false,
                open: true,
                since: None,
                until: None,
                project_dir: cli.project.as_deref(),
            };
            handle_list_command(params, formatter)
        },

        Commands::Start {
            ticket,
            branch,
            branch_name,
            worktree,
        } => {
            use vibe_ticket::cli::handlers::handle_start_command;
            handle_start_command(
                ticket,
                branch,
                branch_name,
                worktree,
                cli.project,
                formatter,
            )
        },

        Commands::Close {
            ticket,
            message,
            archive,
            pr,
        } => {
            use vibe_ticket::cli::handlers::handle_close_command;
            handle_close_command(
                ticket,
                message,
                archive,
                pr,
                cli.project.as_deref(),
                formatter,
            )
        },

        Commands::Check { detailed, stats } => {
            use vibe_ticket::cli::handlers::handle_check_command;
            handle_check_command(detailed, stats, cli.project.as_deref(), formatter)
        },

        Commands::Edit {
            ticket,
            title,
            description,
            priority,
            status,
            add_tags,
            remove_tags,
            editor,
        } => {
            use vibe_ticket::cli::handlers::{handle_edit_command, EditParams};
            let params = EditParams {
                ticket_ref: ticket,
                title,
                description,
                priority,
                status,
                add_tags,
                remove_tags,
                editor,
                project_dir: cli.project.as_deref(),
            };
            handle_edit_command(params, formatter)
        },

        Commands::Show {
            ticket,
            tasks,
            history,
            markdown,
        } => {
            use vibe_ticket::cli::handlers::handle_show_command;
            handle_show_command(
                &ticket,
                tasks,
                history,
                markdown,
                cli.project.as_deref(),
                formatter,
            )
        },

        Commands::Task { command } => match command {
            TaskCommands::Add { title, ticket } => {
                use vibe_ticket::cli::handlers::handle_task_add;
                handle_task_add(title, ticket, cli.project, formatter)
            },
            TaskCommands::Complete { task, ticket } => {
                use vibe_ticket::cli::handlers::handle_task_complete;
                handle_task_complete(task, ticket, cli.project, formatter)
            },
            TaskCommands::Uncomplete { task, ticket } => {
                use vibe_ticket::cli::handlers::handle_task_uncomplete;
                handle_task_uncomplete(task, ticket, cli.project, formatter)
            },
            TaskCommands::List {
                ticket,
                completed,
                incomplete,
            } => {
                use vibe_ticket::cli::handlers::handle_task_list;
                handle_task_list(ticket, completed, incomplete, cli.project, formatter)
            },
            TaskCommands::Remove {
                task,
                ticket,
                force,
            } => {
                use vibe_ticket::cli::handlers::handle_task_remove;
                handle_task_remove(task, ticket, force, cli.project, formatter)
            },
        },

        Commands::Archive { ticket, unarchive } => {
            use vibe_ticket::cli::handlers::handle_archive_command;
            handle_archive_command(&ticket, unarchive, cli.project.as_deref(), formatter)
        },

        Commands::Search {
            query,
            title,
            description,
            tags,
            regex,
            fuzzy,
        } => {
            use vibe_ticket::cli::handlers::{handle_search_command, SearchParams};
            let params = SearchParams {
                query: &query,
                title_only: title,
                description_only: description,
                tags_only: tags,
                use_regex: regex,
                use_fuzzy: fuzzy,
                project_dir: cli.project.as_deref(),
            };
            handle_search_command(&params, formatter)
        },

        Commands::Export {
            format,
            output,
            include_archived,
        } => {
            use vibe_ticket::cli::handlers::handle_export_command;
            handle_export_command(
                &format,
                output,
                include_archived,
                cli.project.as_deref(),
                formatter,
            )
        },

        Commands::Import {
            file,
            format,
            skip_validation,
            dry_run,
        } => {
            use vibe_ticket::cli::handlers::handle_import_command;
            handle_import_command(
                &file,
                format.as_deref(),
                skip_validation,
                dry_run,
                cli.project.as_deref(),
                formatter,
            )
        },

        Commands::Config { command } => {
            use vibe_ticket::cli::handlers::handle_config_command;
            handle_config_command(command, cli.project.as_deref(), formatter)
        },

        Commands::Spec { command } => match command {
            SpecCommands::Init {
                title,
                description,
                ticket,
                tags,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_init;
                handle_spec_init(title, description, ticket, tags, cli.project, formatter)
            },
            SpecCommands::Requirements {
                spec,
                editor,
                complete,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_requirements;
                handle_spec_requirements(spec, editor, complete, cli.project, formatter)
            },
            SpecCommands::Design {
                spec,
                editor,
                complete,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_design;
                handle_spec_design(spec, editor, complete, cli.project, formatter)
            },
            SpecCommands::Tasks {
                spec,
                editor,
                complete,
                export_tickets,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_tasks;
                handle_spec_tasks(
                    spec,
                    editor,
                    complete,
                    export_tickets,
                    cli.project,
                    formatter,
                )
            },
            SpecCommands::Status { spec, detailed } => {
                use vibe_ticket::cli::handlers::handle_spec_status;
                handle_spec_status(spec, detailed, cli.project, formatter)
            },
            SpecCommands::List {
                status,
                phase,
                archived,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_list;
                handle_spec_list(status, phase, archived, cli.project, formatter)
            },
            SpecCommands::Show {
                spec,
                all,
                markdown,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_show;
                handle_spec_show(spec, all, markdown, cli.project, formatter)
            },
            SpecCommands::Delete { spec, force } => {
                use vibe_ticket::cli::handlers::handle_spec_delete;
                handle_spec_delete(spec, force, cli.project, formatter)
            },
            SpecCommands::Approve {
                spec,
                phase,
                message,
            } => {
                use vibe_ticket::cli::handlers::handle_spec_approve;
                handle_spec_approve(spec, phase, message, cli.project, formatter)
            },
            SpecCommands::Activate { spec } => {
                use vibe_ticket::cli::handlers::handle_spec_activate;
                handle_spec_activate(spec, cli.project, formatter)
            },
        },
    }
}

/// Handle errors and display them to the user
///
/// This function formats errors in a user-friendly way, including:
/// - The main error message
/// - Any suggestions for fixing the error
/// - Additional context in verbose mode
///
/// # Arguments
///
/// * `error` - The error to handle
/// * `formatter` - Output formatter for displaying the error
fn handle_error(error: vibe_ticket::error::VibeTicketError, formatter: &OutputFormatter) {
    // Display the main error message
    formatter.error(&error.user_message());

    // Display suggestions if available
    let suggestions = error.suggestions();
    if !suggestions.is_empty() {
        formatter.info("\nSuggestions:");
        for suggestion in &suggestions {
            formatter.info(&format!("  • {suggestion}"));
        }
    }

    // In JSON mode, output error as JSON
    if formatter.is_json() {
        let _ = formatter.json(&serde_json::json!({
            "status": "error",
            "error": error.to_string(),
            "error_type": format!("{:?}", error),
            "suggestions": suggestions,
            "recoverable": error.is_recoverable(),
            "is_config_error": error.is_config_error(),
        }));
    }

    // In verbose mode, show the full error chain
    if tracing::enabled!(tracing::Level::DEBUG) {
        eprintln!("\nDebug information:");
        eprintln!("{error:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test that the CLI can be parsed with various commands
        let _cli = Cli::parse_from(["vibe-ticket", "init"]);
        let _cli = Cli::parse_from(["vibe-ticket", "list"]);
        let _cli = Cli::parse_from(["vibe-ticket", "new", "test-ticket"]);
    }
}
