#[cfg(test)]
mod tests {
    use crate::core::{Ticket, TicketId};
    use crate::storage::FileStorage;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use tempfile::TempDir;

    /// Test concurrent ticket creation
    #[test]
    fn test_concurrent_ticket_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(FileStorage::new(temp_dir.path()));
        
        // Ensure directories exist
        storage.ensure_directories().unwrap();
        
        let num_threads = 10;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for i in 0..num_threads {
            let storage = Arc::clone(&storage);
            let barrier = Arc::clone(&barrier);
            
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier.wait();
                
                // Create a ticket
                let ticket = Ticket::new(
                    &format!("ticket-{}", i),
                    &format!("Ticket {}", i)
                );
                
                // Save the ticket
                storage.save_ticket(&ticket).unwrap();
                
                ticket.id
            });
            
            handles.push(handle);
        }

        // Collect all ticket IDs
        let ticket_ids: Vec<TicketId> = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();

        // Verify all tickets were created
        assert_eq!(ticket_ids.len(), num_threads);
        
        // Verify all tickets can be loaded
        for id in &ticket_ids {
            let ticket = storage.load_ticket(id).unwrap();
            assert_eq!(ticket.id, *id);
        }
    }

    /// Test concurrent modification of the same ticket
    #[test]
    fn test_concurrent_ticket_modification() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(FileStorage::new(temp_dir.path()));
        
        // Ensure directories exist
        storage.ensure_directories().unwrap();
        
        // Create a ticket
        let ticket = Ticket::new("test-ticket", "Test Ticket");
        storage.save_ticket(&ticket).unwrap();
        let ticket_id = ticket.id.clone();

        let num_threads = 5;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for i in 0..num_threads {
            let storage = Arc::clone(&storage);
            let barrier = Arc::clone(&barrier);
            let ticket_id = ticket_id.clone();
            
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier.wait();
                
                // Load, modify, and save the ticket
                for _ in 0..10 {
                    let mut ticket = storage.load_ticket(&ticket_id).unwrap();
                    ticket.description = format!("Modified by thread {}", i);
                    storage.save_ticket(&ticket).unwrap();
                }
            });
            
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify the ticket still exists and is valid
        let final_ticket = storage.load_ticket(&ticket_id).unwrap();
        assert_eq!(final_ticket.id, ticket_id);
        assert!(final_ticket.description.starts_with("Modified by thread"));
    }

    /// Test concurrent active ticket changes
    #[test]
    fn test_concurrent_active_ticket_changes() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(FileStorage::new(temp_dir.path()));
        
        // Ensure directories exist
        storage.ensure_directories().unwrap();
        
        // Create multiple tickets
        let mut ticket_ids = vec![];
        for i in 0..5 {
            let ticket = Ticket::new(&format!("ticket-{}", i), &format!("Ticket {}", i));
            storage.save_ticket(&ticket).unwrap();
            ticket_ids.push(ticket.id);
        }

        let num_threads = 10;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];

        for i in 0..num_threads {
            let storage = Arc::clone(&storage);
            let barrier = Arc::clone(&barrier);
            let ticket_id = ticket_ids[i % ticket_ids.len()].clone();
            
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier.wait();
                
                // Repeatedly set and get active ticket
                for _ in 0..10 {
                    storage.set_active_ticket(&ticket_id).unwrap();
                    let active = storage.get_active_ticket().unwrap();
                    assert!(active.is_some());
                }
            });
            
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify we still have a valid active ticket
        let final_active = storage.get_active_ticket().unwrap();
        assert!(final_active.is_some());
        assert!(ticket_ids.contains(&final_active.unwrap()));
    }

    /// Test concurrent delete operations
    #[test]
    fn test_concurrent_delete_safety() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Arc::new(FileStorage::new(temp_dir.path()));
        
        // Ensure directories exist
        storage.ensure_directories().unwrap();
        
        // Create a ticket
        let ticket = Ticket::new("delete-test", "Delete Test");
        storage.save_ticket(&ticket).unwrap();
        let ticket_id = ticket.id.clone();

        let num_threads = 3;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = vec![];
        let delete_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        for _ in 0..num_threads {
            let storage = Arc::clone(&storage);
            let barrier = Arc::clone(&barrier);
            let delete_count = Arc::clone(&delete_count);
            let ticket_id = ticket_id.clone();
            
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier.wait();
                
                // Try to delete the ticket
                match storage.delete_ticket(&ticket_id) {
                    Ok(()) => {
                        delete_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                    Err(_) => {
                        // Expected - ticket already deleted by another thread
                    }
                }
            });
            
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify only one thread successfully deleted the ticket
        assert_eq!(delete_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        
        // Verify the ticket no longer exists
        assert!(storage.load_ticket(&ticket_id).is_err());
    }
}