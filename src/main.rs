//! vide-ticket - High-performance ticket management system
//!
//! This is the main entry point for the vide-ticket CLI application.
//! It handles command-line argument parsing and dispatches to the appropriate
//! command handlers.

use clap::Parser;
use std::process;
use vide_ticket::cli::{
    handlers::handle_init, Cli, Commands, OutputFormatter, SpecCommands, TaskCommands,
};
use vide_ticket::error::Result;

/// Main entry point for the vide-ticket CLI
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
        std::env::set_current_dir(project_path)
            .map_err(|e| vide_ticket::error::VideTicketError::Io(e))?;
    }

    // Dispatch to command handler
    match cli.command {
        Commands::Init {
            name,
            description,
            force,
            claude_md,
        } => handle_init(name, description, force, claude_md, formatter),

        Commands::New {
            slug,
            title,
            description,
            priority,
            tags,
            start,
        } => {
            use vide_ticket::cli::handlers::handle_new_command;
            handle_new_command(
                slug,
                title,
                description,
                priority,
                tags,
                start,
                cli.project,
                &formatter,
            )
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
            use vide_ticket::cli::handlers::handle_list_command;
            handle_list_command(
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
                cli.project,
                &formatter,
            )
        },

        Commands::Open {
            sort,
            reverse,
            limit,
        } => {
            use vide_ticket::cli::handlers::handle_list_command;
            // Call list handler with open filter set to true
            handle_list_command(
                None, // status
                None, // priority
                None, // assignee
                sort,
                reverse,
                limit,
                false, // archived
                true,  // open
                None,  // since
                None,  // until
                cli.project,
                &formatter,
            )
        },

        Commands::Start {
            ticket,
            branch,
            branch_name,
        } => {
            use vide_ticket::cli::handlers::handle_start_command;
            handle_start_command(ticket, branch, branch_name, cli.project, &formatter)
        },

        Commands::Close {
            ticket,
            message,
            archive,
            pr,
        } => {
            use vide_ticket::cli::handlers::handle_close_command;
            handle_close_command(ticket, message, archive, pr, cli.project, &formatter)
        },

        Commands::Check { detailed, stats } => {
            use vide_ticket::cli::handlers::handle_check_command;
            handle_check_command(detailed, stats, cli.project, &formatter)
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
            use vide_ticket::cli::handlers::handle_edit_command;
            handle_edit_command(
                ticket,
                title,
                description,
                priority,
                status,
                add_tags,
                remove_tags,
                editor,
                cli.project,
                &formatter,
            )
        },

        Commands::Show {
            ticket,
            tasks,
            history,
            markdown,
        } => {
            use vide_ticket::cli::handlers::handle_show_command;
            handle_show_command(ticket, tasks, history, markdown, cli.project, &formatter)
        },

        Commands::Task { command } => match command {
            TaskCommands::Add { title, ticket } => {
                use vide_ticket::cli::handlers::handle_task_add;
                handle_task_add(title, ticket, cli.project, &formatter)
            },
            TaskCommands::Complete { task, ticket } => {
                use vide_ticket::cli::handlers::handle_task_complete;
                handle_task_complete(task, ticket, cli.project, &formatter)
            },
            TaskCommands::Uncomplete { task, ticket } => {
                use vide_ticket::cli::handlers::handle_task_uncomplete;
                handle_task_uncomplete(task, ticket, cli.project, &formatter)
            },
            TaskCommands::List {
                ticket,
                completed,
                incomplete,
            } => {
                use vide_ticket::cli::handlers::handle_task_list;
                handle_task_list(ticket, completed, incomplete, cli.project, &formatter)
            },
            TaskCommands::Remove {
                task,
                ticket,
                force,
            } => {
                use vide_ticket::cli::handlers::handle_task_remove;
                handle_task_remove(task, ticket, force, cli.project, &formatter)
            },
        },

        Commands::Archive { ticket, unarchive } => {
            use vide_ticket::cli::handlers::handle_archive_command;
            handle_archive_command(ticket, unarchive, cli.project, &formatter)
        },

        Commands::Search {
            query,
            title,
            description,
            tags,
            regex,
        } => {
            use vide_ticket::cli::handlers::handle_search_command;
            handle_search_command(
                query,
                title,
                description,
                tags,
                regex,
                cli.project,
                &formatter,
            )
        },

        Commands::Export {
            format,
            output,
            include_archived,
        } => {
            use vide_ticket::cli::handlers::handle_export_command;
            handle_export_command(format, output, include_archived, cli.project, &formatter)
        },

        Commands::Import {
            file,
            format,
            skip_validation,
            dry_run,
        } => {
            use vide_ticket::cli::handlers::handle_import_command;
            handle_import_command(
                file,
                format,
                skip_validation,
                dry_run,
                cli.project,
                &formatter,
            )
        },

        Commands::Config { command } => {
            use vide_ticket::cli::handlers::handle_config_command;
            handle_config_command(command, cli.project, &formatter)
        },

        Commands::Spec { command } => match command {
            SpecCommands::Init {
                title,
                description,
                ticket,
                tags,
            } => {
                use vide_ticket::cli::handlers::handle_spec_init;
                handle_spec_init(title, description, ticket, tags, cli.project, &formatter)
            },
            SpecCommands::Requirements {
                spec,
                editor,
                complete,
            } => {
                use vide_ticket::cli::handlers::handle_spec_requirements;
                handle_spec_requirements(spec, editor, complete, cli.project, &formatter)
            },
            SpecCommands::Design {
                spec,
                editor,
                complete,
            } => {
                use vide_ticket::cli::handlers::handle_spec_design;
                handle_spec_design(spec, editor, complete, cli.project, &formatter)
            },
            SpecCommands::Tasks {
                spec,
                editor,
                complete,
                export_tickets,
            } => {
                use vide_ticket::cli::handlers::handle_spec_tasks;
                handle_spec_tasks(
                    spec,
                    editor,
                    complete,
                    export_tickets,
                    cli.project,
                    &formatter,
                )
            },
            SpecCommands::Status { spec, detailed } => {
                use vide_ticket::cli::handlers::handle_spec_status;
                handle_spec_status(spec, detailed, cli.project, &formatter)
            },
            SpecCommands::List {
                status,
                phase,
                archived,
            } => {
                use vide_ticket::cli::handlers::handle_spec_list;
                handle_spec_list(status, phase, archived, cli.project, &formatter)
            },
            SpecCommands::Show {
                spec,
                all,
                markdown,
            } => {
                use vide_ticket::cli::handlers::handle_spec_show;
                handle_spec_show(spec, all, markdown, cli.project, &formatter)
            },
            SpecCommands::Delete { spec, force } => {
                use vide_ticket::cli::handlers::handle_spec_delete;
                handle_spec_delete(spec, force, cli.project, &formatter)
            },
            SpecCommands::Approve {
                spec,
                phase,
                message,
            } => {
                use vide_ticket::cli::handlers::handle_spec_approve;
                handle_spec_approve(spec, phase, message, cli.project, &formatter)
            },
            SpecCommands::Activate { spec } => {
                use vide_ticket::cli::handlers::handle_spec_activate;
                handle_spec_activate(spec, cli.project, &formatter)
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
fn handle_error(error: vide_ticket::error::VideTicketError, formatter: &OutputFormatter) {
    // Display the main error message
    formatter.error(&error.user_message());

    // Display suggestions if available
    let suggestions = error.suggestions();
    if !suggestions.is_empty() {
        formatter.info("\nSuggestions:");
        for suggestion in &suggestions {
            formatter.info(&format!("  • {}", suggestion));
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
        eprintln!("{:?}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test that the CLI can be parsed with various commands
        let _cli = Cli::parse_from(&["vide-ticket", "init"]);
        let _cli = Cli::parse_from(&["vide-ticket", "list"]);
        let _cli = Cli::parse_from(&["vide-ticket", "new", "test-ticket"]);
    }
}
