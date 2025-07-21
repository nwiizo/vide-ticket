use colored::*;
use serde::Serialize;
use std::io::Write;

use crate::core::{Priority, Status, Ticket};
use crate::error::Result;

/// Output formatter for CLI commands
pub struct OutputFormatter {
    json: bool,
    #[allow(dead_code)]
    no_color: bool,
}

impl OutputFormatter {
    /// Creates a new output formatter
    pub fn new(json: bool, no_color: bool) -> Self {
        if no_color {
            colored::control::set_override(false);
        }
        Self { json, no_color }
    }

    /// Check if JSON output is enabled
    pub fn is_json(&self) -> bool {
        self.json
    }

    /// Print JSON output
    pub fn json<T: Serialize>(&self, data: &T) -> Result<()> {
        if self.json {
            self.print_json(data)?;
        }
        Ok(())
    }

    /// Create a progress bar
    pub fn progress_bar(&self, message: &str) -> indicatif::ProgressBar {
        if self.json {
            // In JSON mode, return a hidden progress bar
            let pb = indicatif::ProgressBar::hidden();
            pb.set_message(message.to_string());
            pb
        } else {
            let pb = indicatif::ProgressBar::new_spinner();
            pb.set_style(
                indicatif::ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            pb.set_message(message.to_string());
            pb.enable_steady_tick(std::time::Duration::from_millis(100));
            pb
        }
    }

    /// Prints a success message
    pub fn success(&self, message: &str) {
        if !self.json {
            println!("{} {}", "✓".green(), message);
        }
    }

    /// Prints an error message
    pub fn error(&self, message: &str) {
        if !self.json {
            eprintln!("{} {}", "✗".red(), message);
        }
    }

    /// Prints a warning message
    pub fn warning(&self, message: &str) {
        if !self.json {
            eprintln!("{} {}", "⚠".yellow(), message);
        }
    }

    /// Prints an info message
    pub fn info(&self, message: &str) {
        if !self.json {
            println!("{} {}", "ℹ".blue(), message);
        }
    }

    /// Prints a ticket
    pub fn print_ticket(&self, ticket: &Ticket) -> Result<()> {
        if self.json {
            self.print_json(ticket)?;
        } else {
            self.print_ticket_formatted(ticket);
        }
        Ok(())
    }

    /// Prints a list of tickets
    pub fn print_tickets(&self, tickets: &[Ticket]) -> Result<()> {
        if self.json {
            self.print_json(tickets)?;
        } else {
            self.print_tickets_table(tickets);
        }
        Ok(())
    }

    /// Prints data as JSON
    pub fn print_json<T: Serialize + ?Sized>(&self, data: &T) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        println!("{}", json);
        Ok(())
    }

    /// Prints a formatted ticket
    fn print_ticket_formatted(&self, ticket: &Ticket) {
        println!("{}", "─".repeat(80).bright_black());
        println!(
            "{} {} {}",
            ticket.status.emoji(),
            ticket.title.bold(),
            format!("({})", ticket.slug).bright_black()
        );
        println!("{}", "─".repeat(80).bright_black());

        println!("{:<12} {}", "ID:".bright_black(), ticket.id.short());
        println!(
            "{:<12} {}",
            "Status:".bright_black(),
            self.format_status(&ticket.status)
        );
        println!(
            "{:<12} {}",
            "Priority:".bright_black(),
            self.format_priority(&ticket.priority)
        );

        if let Some(assignee) = &ticket.assignee {
            println!("{:<12} {}", "Assignee:".bright_black(), assignee);
        }

        if !ticket.tags.is_empty() {
            println!(
                "{:<12} {}",
                "Tags:".bright_black(),
                ticket.tags.join(", ").cyan()
            );
        }

        println!(
            "{:<12} {}",
            "Created:".bright_black(),
            ticket.created_at.format("%Y-%m-%d %H:%M")
        );

        if let Some(started) = ticket.started_at {
            println!(
                "{:<12} {}",
                "Started:".bright_black(),
                started.format("%Y-%m-%d %H:%M")
            );
        }

        if !ticket.description.is_empty() {
            println!("\n{}", "Description:".bright_black());
            println!("{}", ticket.description);
        }

        if !ticket.tasks.is_empty() {
            println!("\n{}", "Tasks:".bright_black());
            for task in &ticket.tasks {
                let checkbox = if task.completed {
                    "✓".green()
                } else {
                    "☐".white()
                };
                println!("  {} {}", checkbox, task.title);
            }

            let completed = ticket.completed_tasks_count();
            let total = ticket.total_tasks_count();
            let percentage = ticket.completion_percentage();

            println!(
                "\n{} {}/{} ({:.0}%)",
                "Progress:".bright_black(),
                completed,
                total,
                percentage
            );
        }

        println!("{}", "─".repeat(80).bright_black());
    }

    /// Prints tickets in a table format
    fn print_tickets_table(&self, tickets: &[Ticket]) {
        if tickets.is_empty() {
            println!("No tickets found.");
            return;
        }

        // Header
        println!(
            "{:<8} {:<10} {:<10} {:<40} {}",
            "ID".bold(),
            "Status".bold(),
            "Priority".bold(),
            "Title".bold(),
            "Tasks".bold()
        );
        println!("{}", "─".repeat(90).bright_black());

        // Rows
        for ticket in tickets {
            let tasks = format!(
                "{}/{}",
                ticket.completed_tasks_count(),
                ticket.total_tasks_count()
            );

            println!(
                "{:<8} {:<10} {:<10} {:<40} {}",
                ticket.id.short(),
                self.format_status(&ticket.status),
                self.format_priority(&ticket.priority),
                truncate(&ticket.title, 40),
                tasks
            );
        }

        println!("{}", "─".repeat(90).bright_black());
        println!("Total: {} tickets", tickets.len());
    }

    /// Formats status with color
    fn format_status(&self, status: &Status) -> ColoredString {
        match status {
            Status::Todo => "Todo".blue(),
            Status::Doing => "Doing".yellow(),
            Status::Done => "Done".green(),
            Status::Blocked => "Blocked".red(),
            Status::Review => "Review".cyan(),
        }
    }

    /// Formats priority with color
    fn format_priority(&self, priority: &Priority) -> ColoredString {
        match priority {
            Priority::Low => "Low".green(),
            Priority::Medium => "Medium".yellow(),
            Priority::High => "High".magenta(),
            Priority::Critical => "Critical".red(),
        }
    }
}

/// Truncates a string to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Progress bar for long-running operations
pub struct ProgressBar {
    message: String,
    total: usize,
    current: usize,
}

impl ProgressBar {
    /// Creates a new progress bar
    pub fn new(message: &str, total: usize) -> Self {
        Self {
            message: message.to_string(),
            total,
            current: 0,
        }
    }

    /// Updates the progress
    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.draw();
    }

    /// Increments the progress by 1
    pub fn increment(&mut self) {
        self.current += 1;
        self.draw();
    }

    /// Completes the progress bar
    pub fn finish(&self) {
        println!();
    }

    /// Draws the progress bar
    fn draw(&self) {
        let percentage = (self.current as f32 / self.total as f32 * 100.0) as u32;
        let filled = (percentage as usize * 30) / 100;
        let empty = 30 - filled;

        print!(
            "\r{}: [{}{}] {}% ({}/{})",
            self.message,
            "█".repeat(filled).green(),
            "░".repeat(empty).bright_black(),
            percentage,
            self.current,
            self.total
        );

        std::io::stdout().flush().unwrap();
    }
}
