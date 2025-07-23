//! Generic storage traits and implementations for specifications
//!
//! This module provides abstractions for persisting specification data,
//! reducing code duplication in file I/O operations.

use crate::error::{ErrorContext, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Trait for document storage operations
pub trait DocumentStore {
    /// Save a serializable document
    fn save_json<T: Serialize>(&self, path: &Path, data: &T) -> Result<()> {
        let content = serde_json::to_string_pretty(data).context("Failed to serialize document")?;
        self.save_text(path, &content)
    }

    /// Load a deserializable document
    fn load_json<T: DeserializeOwned>(&self, path: &Path) -> Result<T> {
        let content = self.load_text(path)?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from: {}", path.display()))
    }

    /// Save text content
    fn save_text(&self, path: &Path, content: &str) -> Result<()>;

    /// Load text content
    fn load_text(&self, path: &Path) -> Result<String>;

    /// Check if a path exists
    fn exists(&self, path: &Path) -> bool;

    /// Ensure directory exists
    fn ensure_dir(&self, dir: &Path) -> Result<()>;

    /// List directory entries
    fn list_dirs(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

/// File system based document store
pub struct FileSystemStore;

impl DocumentStore for FileSystemStore {
    fn save_text(&self, path: &Path, content: &str) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)?;
        }

        fs::write(path, content).with_context(|| format!("Failed to write file: {}", path.display()))
    }

    fn load_text(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn ensure_dir(&self, dir: &Path) -> Result<()> {
        fs::create_dir_all(dir).with_context(|| format!("Failed to create directory: {}", dir.display()))
    }

    fn list_dirs(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut dirs = Vec::new();
        let entries =
            fs::read_dir(path).with_context(|| format!("Failed to read directory: {}", path.display()))?;

        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }

        Ok(dirs)
    }
}

/// Generic document operations helper
pub struct DocumentOperations<S: DocumentStore> {
    store: S,
    base_dir: PathBuf,
}

impl<S: DocumentStore> DocumentOperations<S> {
    /// Create new document operations helper
    pub const fn new(store: S, base_dir: PathBuf) -> Self {
        Self { store, base_dir }
    }

    /// Get subdirectory path
    pub fn get_subdir(&self, id: &str) -> PathBuf {
        self.base_dir.join(id)
    }

    /// Initialize base directory
    pub fn initialize(&self) -> Result<()> {
        self.store.ensure_dir(&self.base_dir)
    }

    /// Save a document in a subdirectory
    pub fn save_in_subdir<T: Serialize>(&self, id: &str, filename: &str, data: &T) -> Result<()> {
        let subdir = self.get_subdir(id);
        let path = subdir.join(filename);
        self.store.save_json(&path, data)
    }

    /// Load a document from a subdirectory
    pub fn load_from_subdir<T: DeserializeOwned>(&self, id: &str, filename: &str) -> Result<T> {
        let subdir = self.get_subdir(id);
        let path = subdir.join(filename);
        self.store.load_json(&path)
    }

    /// Save text in a subdirectory
    pub fn save_text_in_subdir(&self, id: &str, filename: &str, content: &str) -> Result<()> {
        let subdir = self.get_subdir(id);
        let path = subdir.join(filename);
        self.store.save_text(&path, content)
    }

    /// Load text from a subdirectory
    pub fn load_text_from_subdir(&self, id: &str, filename: &str) -> Result<Option<String>> {
        let subdir = self.get_subdir(id);
        let path = subdir.join(filename);

        if !self.store.exists(&path) {
            return Ok(None);
        }

        Ok(Some(self.store.load_text(&path)?))
    }

    /// List all subdirectories
    pub fn list_subdirs(&self) -> Result<Vec<PathBuf>> {
        self.store.list_dirs(&self.base_dir)
    }

    /// Get the base directory
    pub const fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Mock document store for testing
    struct MockStore {
        files: std::cell::RefCell<HashMap<PathBuf, String>>,
    }

    impl MockStore {
        fn new() -> Self {
            Self {
                files: std::cell::RefCell::new(HashMap::new()),
            }
        }
    }

    impl DocumentStore for MockStore {
        fn save_text(&self, path: &Path, content: &str) -> Result<()> {
            self.files
                .borrow_mut()
                .insert(path.to_path_buf(), content.to_string());
            Ok(())
        }

        fn load_text(&self, path: &Path) -> Result<String> {
            self.files
                .borrow()
                .get(path)
                .cloned()
                .ok_or_else(|| crate::error::VibeTicketError::custom("File not found"))
        }

        fn exists(&self, path: &Path) -> bool {
            self.files.borrow().contains_key(path)
        }

        fn ensure_dir(&self, _dir: &Path) -> Result<()> {
            Ok(())
        }

        fn list_dirs(&self, _path: &Path) -> Result<Vec<PathBuf>> {
            Ok(vec![])
        }
    }

    #[test]
    fn test_document_operations() {
        let store = MockStore::new();
        let ops = DocumentOperations::new(store, PathBuf::from("/test"));

        // Test saving and loading JSON
        #[derive(Serialize, serde::Deserialize, PartialEq, Debug)]
        struct TestDoc {
            name: String,
            value: i32,
        }

        let doc = TestDoc {
            name: "test".to_string(),
            value: 42,
        };

        ops.save_in_subdir("id1", "test.json", &doc).unwrap();
        let loaded: TestDoc = ops.load_from_subdir("id1", "test.json").unwrap();
        assert_eq!(doc, loaded);

        // Test saving and loading text
        ops.save_text_in_subdir("id2", "test.txt", "Hello, World!")
            .unwrap();
        let text = ops.load_text_from_subdir("id2", "test.txt").unwrap();
        assert_eq!(text, Some("Hello, World!".to_string()));

        // Test non-existent file
        let missing = ops.load_text_from_subdir("id3", "missing.txt").unwrap();
        assert_eq!(missing, None);
    }
}
