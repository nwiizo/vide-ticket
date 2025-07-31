use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::error::{Result, VibeTicketError};

/// Lock information stored in lock files
#[derive(Debug, Serialize, Deserialize)]
struct LockInfo {
    /// Process ID that holds the lock
    pid: u32,
    /// Timestamp when lock was acquired
    timestamp: u64,
    /// Operation being performed (for debugging)
    operation: Option<String>,
}

/// A file lock that automatically releases when dropped
pub struct FileLock {
    path: PathBuf,
    #[allow(dead_code)]
    holder_id: String,
}

impl FileLock {
    /// Acquire a lock on the given path
    pub fn acquire(path: &Path, operation: Option<String>) -> Result<Self> {
        let lock_path = Self::lock_path(path);
        let lock_dir = lock_path.parent()
            .ok_or_else(|| VibeTicketError::custom("Invalid lock path"))?;
        
        // Ensure lock directory exists
        fs::create_dir_all(lock_dir)
            .map_err(|e| VibeTicketError::custom(format!("Failed to create lock directory: {}", e)))?;
        
        let pid = std::process::id();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let lock_info = LockInfo {
            pid,
            timestamp,
            operation,
        };
        
        // Try to acquire lock with retries
        const MAX_RETRIES: u32 = 10;
        const RETRY_DELAY: Duration = Duration::from_millis(100);
        
        for attempt in 0..MAX_RETRIES {
            // Check for stale locks
            if lock_path.exists() {
                if let Ok(existing_lock) = Self::read_lock_info(&lock_path) {
                    // Check if lock is stale (older than 30 seconds)
                    let current_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    if current_time - existing_lock.timestamp > 30 {
                        // Remove stale lock
                        let _ = fs::remove_file(&lock_path);
                    } else if attempt < MAX_RETRIES - 1 {
                        // Lock is held by another process, wait and retry
                        std::thread::sleep(RETRY_DELAY);
                        continue;
                    } else {
                        // Final attempt failed
                        return Err(VibeTicketError::custom(format!(
                            "Failed to acquire lock after {} attempts. Lock held by PID {} since {}",
                            MAX_RETRIES, existing_lock.pid, existing_lock.timestamp
                        )));
                    }
                }
            }
            
            // Try to create lock file exclusively
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lock_path)
            {
                Ok(mut file) => {
                    // Write lock info
                    let lock_json = serde_json::to_string(&lock_info)
                        .map_err(|e| VibeTicketError::custom(format!("Failed to serialize lock info: {}", e)))?;
                    
                    file.write_all(lock_json.as_bytes())
                        .map_err(|e| VibeTicketError::custom(format!("Failed to write lock file: {}", e)))?;
                    
                    return Ok(FileLock {
                        path: lock_path,
                        holder_id: format!("{}-{}", pid, timestamp),
                    });
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    // Lock exists, retry
                    if attempt < MAX_RETRIES - 1 {
                        std::thread::sleep(RETRY_DELAY);
                        continue;
                    }
                }
                Err(e) => {
                    return Err(VibeTicketError::custom(format!("Failed to create lock file: {}", e)));
                }
            }
        }
        
        Err(VibeTicketError::custom("Failed to acquire lock after maximum retries"))
    }
    
    /// Get the lock file path for a given file
    fn lock_path(path: &Path) -> PathBuf {
        let parent = path.parent().unwrap_or(Path::new("."));
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        parent.join(format!(".{}.lock", filename))
    }
    
    /// Read lock information from a lock file
    fn read_lock_info(path: &Path) -> Result<LockInfo> {
        let content = fs::read_to_string(path)
            .map_err(|e| VibeTicketError::custom(format!("Failed to read lock file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| VibeTicketError::custom(format!("Failed to parse lock file: {}", e)))
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        // Release the lock by removing the lock file
        let _ = fs::remove_file(&self.path);
    }
}

/// A guard that holds a lock and releases it when dropped
pub struct LockGuard<'a> {
    _lock: FileLock,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> LockGuard<'a> {
    /// Create a new lock guard
    pub fn new(lock: FileLock) -> Self {
        LockGuard {
            _lock: lock,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_lock_acquire_and_release() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.yaml");
        
        // Acquire lock
        let lock = FileLock::acquire(&file_path, Some("test".to_string())).unwrap();
        let lock_path = FileLock::lock_path(&file_path);
        
        // Lock file should exist
        assert!(lock_path.exists());
        
        // Drop lock
        drop(lock);
        
        // Lock file should be removed
        assert!(!lock_path.exists());
    }
    
    #[test]
    fn test_concurrent_lock_attempts() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.yaml");
        
        // First lock should succeed
        let _lock1 = FileLock::acquire(&file_path, Some("first".to_string())).unwrap();
        
        // Second lock should fail (with shorter retry for test)
        std::env::set_var("VIBE_TICKET_TEST_MODE", "1");
        let result = FileLock::acquire(&file_path, Some("second".to_string()));
        assert!(result.is_err());
    }
}