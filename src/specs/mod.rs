//! Spec-Driven Development (仕様駆動開発) module
//!
//! This module implements a specification-driven development workflow inspired by Kiro.
//! It manages three types of documents:
//! - Requirements Definition (要件定義書)
//! - Technical Design (技術設計書)
//! - Implementation Plan (実装計画書)
//!
//! # Architecture
//!
//! The specs are stored as Markdown files in `.vide-ticket/specs/` directory
//! with a `spec.json` file tracking the progress and metadata.
//!
//! # Workflow
//!
//! 1. Initialize a new spec
//! 2. Create requirements definition
//! 3. Create technical design based on requirements
//! 4. Create implementation plan based on design
//! 5. Track progress through spec.json

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod manager;
pub mod storage;
pub mod templates;

pub use manager::{SpecManager, save, load, list, delete, get_document_path};
pub use templates::{SpecTemplate, TemplateEngine};

/// Specification metadata and progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecMetadata {
    /// Unique spec ID
    pub id: String,
    
    /// Spec title
    pub title: String,
    
    /// Brief description
    pub description: String,
    
    /// Associated ticket ID (if any)
    pub ticket_id: Option<String>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Progress tracking
    pub progress: SpecProgress,
    
    /// Version information
    pub version: SpecVersion,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Progress tracking for spec documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecProgress {
    /// Requirements definition completed
    pub requirements_completed: bool,
    
    /// Technical design completed
    pub design_completed: bool,
    
    /// Implementation plan completed
    pub tasks_completed: bool,
    
    /// Requirements approval status
    pub requirements_approved: bool,
    
    /// Design approval status
    pub design_approved: bool,
    
    /// Tasks approval status
    pub tasks_approved: bool,
    
    /// Current phase
    pub current_phase: SpecPhase,
    
    /// Approval status with additional metadata
    pub approval_status: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Current phase of the specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpecPhase {
    /// Initial phase - no documents created yet
    Initial,
    
    /// Requirements definition phase
    Requirements,
    
    /// Technical design phase
    Design,
    
    /// Implementation planning phase
    Implementation,
    
    /// Tasks phase (alias for Implementation)
    Tasks,
    
    /// All phases completed
    Completed,
}

/// Version information for spec documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecVersion {
    /// Major version (breaking changes)
    pub major: u32,
    
    /// Minor version (new features)
    pub minor: u32,
    
    /// Patch version (bug fixes)
    pub patch: u32,
}

/// Specification document type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecDocumentType {
    /// Requirements definition document
    Requirements,
    
    /// Technical design document
    Design,
    
    /// Implementation plan/tasks document
    Tasks,
}


/// A complete specification with all documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    /// Metadata
    pub metadata: SpecMetadata,
    
    /// Requirements definition content
    pub requirements: Option<String>,
    
    /// Technical design content
    pub design: Option<String>,
    
    /// Implementation plan content
    pub tasks: Option<String>,
}

impl Specification {
    /// Create a new specification
    pub fn new(
        title: String,
        description: String,
        ticket_id: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        let mut metadata = SpecMetadata::new(title, description);
        metadata.ticket_id = ticket_id;
        metadata.tags = tags;
        
        Self {
            metadata,
            requirements: None,
            design: None,
            tasks: None,
        }
    }
}

impl SpecMetadata {
    /// Create new spec metadata
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            ticket_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            progress: SpecProgress::default(),
            version: SpecVersion::default(),
            tags: Vec::new(),
        }
    }
    
    /// Update the current phase based on progress
    pub fn update_phase(&mut self) {
        self.progress.current_phase = match (
            self.progress.requirements_completed,
            self.progress.design_completed,
            self.progress.tasks_completed,
        ) {
            (false, _, _) => SpecPhase::Requirements,
            (true, false, _) => SpecPhase::Design,
            (true, true, false) => SpecPhase::Implementation,
            (true, true, true) => SpecPhase::Completed,
        };
        self.updated_at = Utc::now();
    }
}

impl SpecProgress {
    /// Get the current phase
    pub fn current_phase(&self) -> SpecPhase {
        self.current_phase
    }
}

impl Default for SpecProgress {
    fn default() -> Self {
        Self {
            requirements_completed: false,
            design_completed: false,
            tasks_completed: false,
            requirements_approved: false,
            design_approved: false,
            tasks_approved: false,
            current_phase: SpecPhase::Initial,
            approval_status: None,
        }
    }
}

impl Default for SpecVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}

impl SpecVersion {
    /// Get version string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    
    /// Increment patch version
    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }
    
    /// Increment minor version (resets patch)
    pub fn bump_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }
    
    /// Increment major version (resets minor and patch)
    pub fn bump_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }
}

impl SpecDocumentType {
    /// Get file name for this document type
    pub fn file_name(&self) -> &'static str {
        match self {
            Self::Requirements => "requirements.md",
            Self::Design => "design.md",
            Self::Tasks => "tasks.md",
        }
    }
    
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Requirements => "Requirements Definition",
            Self::Design => "Technical Design",
            Self::Tasks => "Implementation Plan",
        }
    }
}

impl std::fmt::Display for SpecPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initial => write!(f, "Initial"),
            Self::Requirements => write!(f, "Requirements Definition"),
            Self::Design => write!(f, "Technical Design"),
            Self::Implementation => write!(f, "Implementation Planning"),
            Self::Tasks => write!(f, "Tasks"),
            Self::Completed => write!(f, "Completed"),
        }
    }
}