//! Specification manager for handling spec lifecycle
//!
//! This module provides the core functionality for managing specifications,
//! including creation, loading, saving, and version control.

use super::{SpecDocumentType, SpecMetadata, SpecPhase, Specification};
use crate::error::{ErrorContext, Result, VideTicketError};
use std::fs;
use std::path::{Path, PathBuf};

/// Manages specifications in a project
pub struct SpecManager {
    /// Root directory for specs (.vide-ticket/specs)
    specs_dir: PathBuf,
}

impl SpecManager {
    /// Create a new spec manager
    pub fn new(specs_dir: PathBuf) -> Self {
        Self { specs_dir }
    }
    
    /// Initialize the specs directory structure
    pub fn initialize(&self) -> Result<()> {
        fs::create_dir_all(&self.specs_dir)
            .with_context(|| format!("Failed to create specs directory: {:?}", self.specs_dir))?;
        
        Ok(())
    }
    
    /// Create a new specification
    pub fn create_spec(&self, title: String, description: String) -> Result<SpecMetadata> {
        self.initialize()?;
        
        let metadata = SpecMetadata::new(title, description);
        let spec_dir = self.get_spec_dir(&metadata.id);
        
        // Create spec directory
        fs::create_dir_all(&spec_dir)
            .with_context(|| format!("Failed to create spec directory: {:?}", spec_dir))?;
        
        // Save initial metadata
        self.save_metadata(&metadata)?;
        
        Ok(metadata)
    }
    
    /// Load a specification by ID
    pub fn load_spec(&self, spec_id: &str) -> Result<Specification> {
        let metadata = self.load_metadata(spec_id)?;
        let spec_dir = self.get_spec_dir(spec_id);
        
        // Load document contents
        let requirements = self.load_document(&spec_dir, SpecDocumentType::Requirements)?;
        let design = self.load_document(&spec_dir, SpecDocumentType::Design)?;
        let tasks = self.load_document(&spec_dir, SpecDocumentType::Tasks)?;
        
        Ok(Specification {
            metadata,
            requirements,
            design,
            tasks,
        })
    }
    
    /// Save a document for a specification
    pub fn save_document(
        &self,
        spec_id: &str,
        doc_type: SpecDocumentType,
        content: &str,
    ) -> Result<()> {
        let spec_dir = self.get_spec_dir(spec_id);
        let doc_path = spec_dir.join(doc_type.file_name());
        
        // Ensure directory exists
        fs::create_dir_all(&spec_dir)
            .with_context(|| format!("Failed to create spec directory: {:?}", spec_dir))?;
        
        // Save document
        fs::write(&doc_path, content)
            .with_context(|| format!("Failed to write document: {:?}", doc_path))?;
        
        // Update metadata
        let mut metadata = self.load_metadata(spec_id)?;
        match doc_type {
            SpecDocumentType::Requirements => {
                metadata.progress.requirements_completed = true;
                metadata.version.bump_patch();
            }
            SpecDocumentType::Design => {
                metadata.progress.design_completed = true;
                metadata.version.bump_patch();
            }
            SpecDocumentType::Tasks => {
                metadata.progress.tasks_completed = true;
                metadata.version.bump_patch();
            }
        }
        metadata.update_phase();
        self.save_metadata(&metadata)?;
        
        Ok(())
    }
    
    /// List all specifications
    pub fn list_specs(&self) -> Result<Vec<SpecMetadata>> {
        if !self.specs_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut specs = Vec::new();
        
        let entries = fs::read_dir(&self.specs_dir)
            .with_context(|| format!("Failed to read specs directory: {:?}", self.specs_dir))?;
        
        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_dir() {
                let spec_json_path = path.join("spec.json");
                if spec_json_path.exists() {
                    match self.load_metadata_from_path(&spec_json_path) {
                        Ok(metadata) => specs.push(metadata),
                        Err(e) => {
                            eprintln!("Warning: Failed to load spec from {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
        
        // Sort by creation date (newest first)
        specs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(specs)
    }
    
    /// Approve a document phase
    pub fn approve_phase(&self, spec_id: &str, phase: SpecPhase) -> Result<()> {
        let mut metadata = self.load_metadata(spec_id)?;
        
        match phase {
            SpecPhase::Requirements => {
                if !metadata.progress.requirements_completed {
                    return Err(VideTicketError::custom(
                        "Cannot approve requirements: document not completed",
                    ));
                }
                metadata.progress.requirements_approved = true;
            }
            SpecPhase::Design => {
                if !metadata.progress.design_completed {
                    return Err(VideTicketError::custom(
                        "Cannot approve design: document not completed",
                    ));
                }
                metadata.progress.design_approved = true;
            }
            SpecPhase::Implementation => {
                if !metadata.progress.tasks_completed {
                    return Err(VideTicketError::custom(
                        "Cannot approve tasks: document not completed",
                    ));
                }
                metadata.progress.tasks_approved = true;
            }
            _ => {
                return Err(VideTicketError::custom(
                    "Invalid phase for approval",
                ));
            }
        }
        
        metadata.update_phase();
        self.save_metadata(&metadata)?;
        
        Ok(())
    }
    
    /// Get the directory path for a spec
    fn get_spec_dir(&self, spec_id: &str) -> PathBuf {
        self.specs_dir.join(spec_id)
    }
    
    /// Load metadata for a spec
    fn load_metadata(&self, spec_id: &str) -> Result<SpecMetadata> {
        let spec_json_path = self.get_spec_dir(spec_id).join("spec.json");
        self.load_metadata_from_path(&spec_json_path)
    }
    
    /// Load metadata from a specific path
    fn load_metadata_from_path(&self, path: &Path) -> Result<SpecMetadata> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read spec metadata: {:?}", path))?;
        
        let metadata: SpecMetadata = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse spec metadata: {:?}", path))?;
        
        Ok(metadata)
    }
    
    /// Save metadata for a spec
    fn save_metadata(&self, metadata: &SpecMetadata) -> Result<()> {
        let spec_dir = self.get_spec_dir(&metadata.id);
        
        // Ensure spec directory exists
        fs::create_dir_all(&spec_dir)
            .with_context(|| format!("Failed to create spec directory: {:?}", spec_dir))?;
        
        let spec_json_path = spec_dir.join("spec.json");
        let content = serde_json::to_string_pretty(metadata)
            .context("Failed to serialize spec metadata")?;
        
        fs::write(&spec_json_path, content)
            .with_context(|| format!("Failed to write spec metadata: {:?}", spec_json_path))?;
        
        Ok(())
    }
    
    /// Load a document from a spec directory
    fn load_document(
        &self,
        spec_dir: &Path,
        doc_type: SpecDocumentType,
    ) -> Result<Option<String>> {
        let doc_path = spec_dir.join(doc_type.file_name());
        
        if !doc_path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&doc_path)
            .with_context(|| format!("Failed to read document: {:?}", doc_path))?;
        
        Ok(Some(content))
    }
    
    /// Find spec by title (partial match)
    pub fn find_spec_by_title(&self, query: &str) -> Result<Option<SpecMetadata>> {
        let specs = self.list_specs()?;
        let query_lower = query.to_lowercase();
        
        Ok(specs.into_iter().find(|spec| {
            spec.title.to_lowercase().contains(&query_lower)
        }))
    }
    
    /// Delete a specification
    pub fn delete_spec(&self, spec_id: &str) -> Result<()> {
        let spec_dir = self.get_spec_dir(spec_id);
        
        if !spec_dir.exists() {
            return Err(VideTicketError::custom(format!(
                "Spec not found: {}",
                spec_id
            )));
        }
        
        fs::remove_dir_all(&spec_dir)
            .with_context(|| format!("Failed to delete spec directory: {:?}", spec_dir))?;
        
        Ok(())
    }

    // Convenience methods for the handlers
    
    /// Save a specification
    pub fn save(&self, spec: &Specification) -> Result<()> {
        // Save metadata
        self.save_metadata(&spec.metadata)?;
        
        // Save documents if they exist
        if let Some(ref content) = spec.requirements {
            self.save_document(&spec.metadata.id, SpecDocumentType::Requirements, content)?;
        }
        if let Some(ref content) = spec.design {
            self.save_document(&spec.metadata.id, SpecDocumentType::Design, content)?;
        }
        if let Some(ref content) = spec.tasks {
            self.save_document(&spec.metadata.id, SpecDocumentType::Tasks, content)?;
        }
        
        Ok(())
    }
    
    /// Load a specification
    pub fn load(&self, spec_id: &str) -> Result<Specification> {
        self.load_spec(spec_id)
    }
    
    /// List all specifications
    pub fn list(&self) -> Result<Vec<Specification>> {
        let metadata_list = self.list_specs()?;
        let mut specs = Vec::new();
        
        for metadata in metadata_list {
            match self.load_spec(&metadata.id) {
                Ok(spec) => specs.push(spec),
                Err(e) => {
                    eprintln!("Warning: Failed to load spec {}: {}", metadata.id, e);
                }
            }
        }
        
        Ok(specs)
    }
    
    /// Delete a specification
    pub fn delete(&self, spec_id: &str) -> Result<()> {
        self.delete_spec(spec_id)
    }
    
    /// Get the path for a document
    pub fn get_document_path(&self, spec_id: &str, doc_type: SpecDocumentType) -> PathBuf {
        self.get_spec_dir(spec_id).join(doc_type.file_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_spec_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".vide-ticket/specs");
        let manager = SpecManager::new(specs_dir.clone());
        
        assert_eq!(
            manager.specs_dir,
            specs_dir
        );
    }
    
    #[test]
    fn test_create_and_load_spec() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".vide-ticket/specs");
        let manager = SpecManager::new(specs_dir);
        
        // Create spec
        let metadata = manager
            .create_spec("Test Spec".to_string(), "Test description".to_string())
            .unwrap();
        
        assert_eq!(metadata.title, "Test Spec");
        assert_eq!(metadata.description, "Test description");
        assert_eq!(metadata.progress.current_phase, SpecPhase::Initial);
        
        // Load spec
        let spec = manager.load_spec(&metadata.id).unwrap();
        assert_eq!(spec.metadata.title, "Test Spec");
        assert!(spec.requirements.is_none());
        assert!(spec.design.is_none());
        assert!(spec.tasks.is_none());
    }
    
    #[test]
    fn test_save_and_load_documents() {
        let temp_dir = TempDir::new().unwrap();
        let specs_dir = temp_dir.path().join(".vide-ticket/specs");
        let manager = SpecManager::new(specs_dir);
        
        // Create spec
        let metadata = manager
            .create_spec("Test Spec".to_string(), "Test description".to_string())
            .unwrap();
        
        // Save requirements
        manager
            .save_document(
                &metadata.id,
                SpecDocumentType::Requirements,
                "# Requirements\nTest requirements",
            )
            .unwrap();
        
        // Load spec and verify
        let spec = manager.load_spec(&metadata.id).unwrap();
        assert!(spec.requirements.is_some());
        assert_eq!(
            spec.requirements.unwrap(),
            "# Requirements\nTest requirements"
        );
        assert!(spec.metadata.progress.requirements_completed);
        assert_eq!(spec.metadata.progress.current_phase, SpecPhase::Design);
    }
}