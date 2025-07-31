#[cfg(test)]
mod cache_integration_tests {
    use vibe_ticket::storage::FileStorage;
    use vibe_ticket::core::Ticket;
    use tempfile::TempDir;
    use std::time::Instant;

    #[test]
    fn test_cache_performance() {
        // Setup test environment
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join(".vibe-ticket");
        std::fs::create_dir_all(&storage_path.join("tickets")).unwrap();
        let storage = FileStorage::new(&storage_path);

        // Create and save 100 test tickets
        let mut tickets = Vec::new();
        for i in 0..100 {
            let ticket = Ticket::new(
                format!("test-ticket-{}", i),
                format!("Test Ticket {}", i),
            );
            storage.save(&ticket).unwrap();
            tickets.push(ticket);
        }

        // First read - should hit disk
        let start = Instant::now();
        let _ = storage.load_all().unwrap();
        let first_read_duration = start.elapsed();

        // Second read - should hit cache
        let start = Instant::now();
        let _ = storage.load_all().unwrap();
        let cached_read_duration = start.elapsed();

        // Cache should be significantly faster
        println!("First read (disk): {:?}", first_read_duration);
        println!("Cached read: {:?}", cached_read_duration);
        
        // Cached read should be at least 10x faster
        assert!(cached_read_duration < first_read_duration / 10);
    }

    #[test]
    fn test_cache_invalidation_on_save() {
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join(".vibe-ticket");
        std::fs::create_dir_all(&storage_path.join("tickets")).unwrap();
        let storage = FileStorage::new(&storage_path);

        // Create and save a ticket
        let mut ticket = Ticket::new("test-ticket".to_string(), "Original Title".to_string());
        storage.save(&ticket).unwrap();

        // Load it to populate cache
        let loaded = storage.load(&ticket.id).unwrap();
        assert_eq!(loaded.title, "Original Title");

        // Modify and save again
        ticket.title = "Updated Title".to_string();
        storage.save(&ticket).unwrap();

        // Load again - should get updated version, not cached old version
        let reloaded = storage.load(&ticket.id).unwrap();
        assert_eq!(reloaded.title, "Updated Title");
    }
}