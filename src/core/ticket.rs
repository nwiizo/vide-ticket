use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Priority, Status, Task, TaskId, TicketId};

/// Represents a ticket in the vide-ticket system
///
/// A ticket encapsulates a unit of work with associated metadata,
/// tasks, and lifecycle information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ticket {
    /// Unique identifier for the ticket
    pub id: TicketId,

    /// URL-friendly slug derived from the title
    pub slug: String,

    /// Human-readable title of the ticket
    pub title: String,

    /// Detailed description of the work to be done
    pub description: String,

    /// Priority level of the ticket
    pub priority: Priority,

    /// Current status of the ticket
    pub status: Status,

    /// Tags for categorization and filtering
    #[serde(default)]
    pub tags: Vec<String>,

    /// Timestamp when the ticket was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when work started on the ticket
    pub started_at: Option<DateTime<Utc>>,

    /// Timestamp when the ticket was closed
    pub closed_at: Option<DateTime<Utc>>,

    /// Username of the person assigned to the ticket
    pub assignee: Option<String>,

    /// List of tasks associated with this ticket
    #[serde(default)]
    pub tasks: Vec<Task>,

    /// Additional metadata for extensibility
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Ticket {
    /// Creates a new ticket with the given slug and title
    pub fn new(slug: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: TicketId::new(),
            slug: slug.into(),
            title: title.into(),
            description: String::new(),
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            created_at: Utc::now(),
            started_at: None,
            closed_at: None,
            assignee: None,
            tasks: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Creates a new ticket with a specific ID (useful for deserialization)
    pub fn with_id(id: TicketId, slug: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id,
            slug: slug.into(),
            title: title.into(),
            description: String::new(),
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            created_at: Utc::now(),
            started_at: None,
            closed_at: None,
            assignee: None,
            tasks: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Starts work on the ticket, updating status and timestamp
    pub fn start(&mut self) {
        self.status = Status::Doing;
        self.started_at = Some(Utc::now());
    }

    /// Closes the ticket, updating status and timestamp
    pub fn close(&mut self) {
        self.status = Status::Done;
        self.closed_at = Some(Utc::now());
    }

    /// Adds a task to the ticket
    pub fn add_task(&mut self, title: impl Into<String>) -> TaskId {
        let task = Task::new(title);
        let task_id = task.id.clone();
        self.tasks.push(task);
        task_id
    }

    /// Marks a task as completed
    pub fn complete_task(&mut self, task_id: &TaskId) -> Result<(), String> {
        self.tasks
            .iter_mut()
            .find(|task| &task.id == task_id)
            .ok_or_else(|| format!("Task with ID {} not found", task_id))?
            .complete();
        Ok(())
    }

    /// Returns the number of completed tasks
    pub fn completed_tasks_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.completed).count()
    }

    /// Returns the total number of tasks
    pub fn total_tasks_count(&self) -> usize {
        self.tasks.len()
    }

    /// Calculates the completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.tasks.is_empty() {
            0.0
        } else {
            (self.completed_tasks_count() as f32 / self.total_tasks_count() as f32) * 100.0
        }
    }

    /// Returns the duration the ticket has been open
    pub fn duration(&self) -> chrono::Duration {
        let end_time = self.closed_at.unwrap_or_else(Utc::now);
        end_time - self.created_at
    }

    /// Returns the working duration (from start to close/now)
    pub fn working_duration(&self) -> Option<chrono::Duration> {
        self.started_at.map(|start| {
            let end_time = self.closed_at.unwrap_or_else(Utc::now);
            end_time - start
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ticket() {
        let ticket = Ticket::new("test-ticket", "Test Ticket");
        assert_eq!(ticket.slug, "test-ticket");
        assert_eq!(ticket.title, "Test Ticket");
        assert_eq!(ticket.status, Status::Todo);
        assert!(ticket.started_at.is_none());
        assert!(ticket.closed_at.is_none());
    }

    #[test]
    fn test_start_ticket() {
        let mut ticket = Ticket::new("test", "Test");
        ticket.start();
        assert_eq!(ticket.status, Status::Doing);
        assert!(ticket.started_at.is_some());
    }

    #[test]
    fn test_close_ticket() {
        let mut ticket = Ticket::new("test", "Test");
        ticket.close();
        assert_eq!(ticket.status, Status::Done);
        assert!(ticket.closed_at.is_some());
    }

    #[test]
    fn test_task_management() {
        let mut ticket = Ticket::new("test", "Test");
        let task_id = ticket.add_task("Task 1");

        assert_eq!(ticket.total_tasks_count(), 1);
        assert_eq!(ticket.completed_tasks_count(), 0);

        ticket.complete_task(&task_id).unwrap();
        assert_eq!(ticket.completed_tasks_count(), 1);
        assert_eq!(ticket.completion_percentage(), 100.0);
    }
}
