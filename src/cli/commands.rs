use clap::{Parser, Subcommand};

/// vide-ticket: A high-performance ticket management system for Vide Coding
#[derive(Parser, Debug)]
#[command(name = "vide-ticket")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Use JSON output format
    #[arg(short, long, global = true)]
    pub json: bool,
    
    /// Disable color output
    #[arg(long, global = true)]
    pub no_color: bool,
    
    /// Path to the project directory
    #[arg(short, long, global = true)]
    pub project: Option<String>,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new vide-ticket project
    Init {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        
        /// Project description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Force initialization even if already initialized
        #[arg(short, long)]
        force: bool,
    },
    
    /// Create a new ticket
    New {
        /// Ticket slug (e.g., fix-login-bug)
        slug: String,
        
        /// Ticket title
        #[arg(short, long)]
        title: Option<String>,
        
        /// Ticket description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Priority (low, medium, high, critical)
        #[arg(long, default_value = "medium")]
        priority: String,
        
        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
        
        /// Start working on the ticket immediately
        #[arg(short, long)]
        start: bool,
    },
    
    /// List all tickets
    List {
        /// Filter by status (todo, doing, done, blocked, review)
        #[arg(short, long)]
        status: Option<String>,
        
        /// Filter by priority (low, medium, high, critical)
        #[arg(long)]
        priority: Option<String>,
        
        /// Filter by assignee
        #[arg(short, long)]
        assignee: Option<String>,
        
        /// Sort by field (created, updated, priority, status, slug)
        #[arg(long, default_value = "slug")]
        sort: String,
        
        /// Reverse sort order
        #[arg(short, long)]
        reverse: bool,
        
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
        
        /// Show archived tickets
        #[arg(long)]
        archived: bool,
        
        /// Filter tickets created since (e.g., "yesterday", "2 days ago", "2025-07-18")
        #[arg(long)]
        since: Option<String>,
        
        /// Filter tickets created until (e.g., "today", "1 week ago", "2025-07-20")
        #[arg(long)]
        until: Option<String>,
    },
    
    /// Start working on a ticket
    Start {
        /// Ticket ID or slug
        ticket: String,
        
        /// Create a new Git branch
        #[arg(short, long, default_value = "true")]
        branch: bool,
        
        /// Custom branch name (default: ticket-<slug>)
        #[arg(long)]
        branch_name: Option<String>,
    },
    
    /// Close the current ticket
    Close {
        /// Ticket ID or slug (defaults to active ticket)
        ticket: Option<String>,
        
        /// Close message
        #[arg(short, long)]
        message: Option<String>,
        
        /// Archive the ticket
        #[arg(short, long)]
        archive: bool,
        
        /// Create a merge/pull request
        #[arg(long)]
        pr: bool,
    },
    
    /// Check the current status
    Check {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
        
        /// Include statistics
        #[arg(short, long)]
        stats: bool,
    },
    
    /// Edit a ticket
    Edit {
        /// Ticket ID or slug (defaults to active ticket)
        ticket: Option<String>,
        
        /// New title
        #[arg(long)]
        title: Option<String>,
        
        /// New description
        #[arg(long)]
        description: Option<String>,
        
        /// New priority
        #[arg(long)]
        priority: Option<String>,
        
        /// New status
        #[arg(long)]
        status: Option<String>,
        
        /// Add tags (comma-separated)
        #[arg(long)]
        add_tags: Option<String>,
        
        /// Remove tags (comma-separated)
        #[arg(long)]
        remove_tags: Option<String>,
        
        /// Open in editor
        #[arg(short, long)]
        editor: bool,
    },
    
    /// Show ticket details
    Show {
        /// Ticket ID or slug
        ticket: String,
        
        /// Show tasks
        #[arg(short, long)]
        tasks: bool,
        
        /// Show history
        #[arg(long)]
        history: bool,
        
        /// Show in markdown format
        #[arg(short, long)]
        markdown: bool,
    },
    
    /// Manage tasks within a ticket
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    
    /// Archive or unarchive tickets
    Archive {
        /// Ticket ID or slug
        ticket: String,
        
        /// Unarchive instead of archive
        #[arg(short, long)]
        unarchive: bool,
    },
    
    /// Search tickets
    Search {
        /// Search query
        query: String,
        
        /// Search in title only
        #[arg(long)]
        title: bool,
        
        /// Search in description only
        #[arg(long)]
        description: bool,
        
        /// Search in tags only
        #[arg(long)]
        tags: bool,
        
        /// Use regex
        #[arg(short, long)]
        regex: bool,
    },
    
    /// Export tickets
    Export {
        /// Output format (json, yaml, csv, markdown)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
        
        /// Include archived tickets
        #[arg(long)]
        include_archived: bool,
    },
    
    /// Import tickets
    Import {
        /// Input file
        file: String,
        
        /// Input format (json, yaml, csv)
        #[arg(short, long)]
        format: Option<String>,
        
        /// Skip validation
        #[arg(long)]
        skip_validation: bool,
        
        /// Dry run (don't actually import)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Manage project configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Show specific key
        key: Option<String>,
    },
    
    /// Set configuration value
    Set {
        /// Configuration key (e.g., project.name, ui.emoji)
        key: String,
        
        /// Value to set
        value: String,
    },
    
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    
    /// Reset configuration to defaults
    Reset {
        /// Confirm reset
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// Add a new task to a ticket
    Add {
        /// Task title
        title: String,
        
        /// Ticket ID or slug (defaults to active ticket)
        #[arg(short, long)]
        ticket: Option<String>,
    },
    
    /// Complete a task
    Complete {
        /// Task ID
        task: String,
        
        /// Ticket ID or slug (defaults to active ticket)
        #[arg(short, long)]
        ticket: Option<String>,
    },
    
    /// Uncomplete a task
    Uncomplete {
        /// Task ID
        task: String,
        
        /// Ticket ID or slug (defaults to active ticket)
        #[arg(short, long)]
        ticket: Option<String>,
    },
    
    /// List tasks in a ticket
    List {
        /// Ticket ID or slug (defaults to active ticket)
        #[arg(short, long)]
        ticket: Option<String>,
        
        /// Show completed tasks only
        #[arg(long)]
        completed: bool,
        
        /// Show incomplete tasks only
        #[arg(long)]
        incomplete: bool,
    },
    
    /// Remove a task
    Remove {
        /// Task ID
        task: String,
        
        /// Ticket ID or slug (defaults to active ticket)
        #[arg(short, long)]
        ticket: Option<String>,
        
        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },
}