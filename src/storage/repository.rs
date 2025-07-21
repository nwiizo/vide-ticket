use crate::core::{Ticket, TicketId};
use crate::error::Result;

/// Repository trait for ticket storage operations
///
/// This trait defines the interface for storing and retrieving tickets,
/// allowing for different storage implementations.
pub trait TicketRepository: Send + Sync {
    /// Saves a ticket to the repository
    fn save(&self, ticket: &Ticket) -> Result<()>;

    /// Loads a ticket by ID
    fn load(&self, id: &TicketId) -> Result<Ticket>;

    /// Loads all tickets
    fn load_all(&self) -> Result<Vec<Ticket>>;

    /// Deletes a ticket by ID
    fn delete(&self, id: &TicketId) -> Result<()>;

    /// Checks if a ticket exists by ID
    fn exists(&self, id: &TicketId) -> Result<bool>;

    /// Finds tickets matching a predicate
    fn find<F>(&self, predicate: F) -> Result<Vec<Ticket>>
    where
        F: Fn(&Ticket) -> bool;

    /// Counts tickets matching a predicate
    fn count<F>(&self, predicate: F) -> Result<usize>
    where
        F: Fn(&Ticket) -> bool;
}

/// Repository trait for managing the active ticket
pub trait ActiveTicketRepository: Send + Sync {
    /// Sets the active ticket ID
    fn set_active(&self, id: &TicketId) -> Result<()>;

    /// Gets the active ticket ID
    fn get_active(&self) -> Result<Option<TicketId>>;

    /// Clears the active ticket
    fn clear_active(&self) -> Result<()>;
}

/// Combined repository trait
pub trait Repository: TicketRepository + ActiveTicketRepository {}

/// Implementation of Repository for types that implement both traits
impl<T> Repository for T where T: TicketRepository + ActiveTicketRepository {}

use super::file::FileStorage;

impl TicketRepository for FileStorage {
    fn save(&self, ticket: &Ticket) -> Result<()> {
        self.save_ticket(ticket)
    }

    fn load(&self, id: &TicketId) -> Result<Ticket> {
        self.load_ticket(id)
    }

    fn load_all(&self) -> Result<Vec<Ticket>> {
        self.load_all_tickets()
    }

    fn delete(&self, id: &TicketId) -> Result<()> {
        self.delete_ticket(id)
    }

    fn exists(&self, id: &TicketId) -> Result<bool> {
        match self.load_ticket(id) {
            Ok(_) => Ok(true),
            Err(crate::error::VibeTicketError::TicketNotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn find<F>(&self, predicate: F) -> Result<Vec<Ticket>>
    where
        F: Fn(&Ticket) -> bool,
    {
        let tickets = self.load_all_tickets()?;
        Ok(tickets.into_iter().filter(predicate).collect())
    }

    fn count<F>(&self, predicate: F) -> Result<usize>
    where
        F: Fn(&Ticket) -> bool,
    {
        let tickets = self.load_all_tickets()?;
        Ok(tickets.iter().filter(|t| predicate(t)).count())
    }
}

impl ActiveTicketRepository for FileStorage {
    fn set_active(&self, id: &TicketId) -> Result<()> {
        self.set_active_ticket(id)
    }

    fn get_active(&self) -> Result<Option<TicketId>> {
        self.get_active_ticket()
    }

    fn clear_active(&self) -> Result<()> {
        self.clear_active_ticket()
    }
}
