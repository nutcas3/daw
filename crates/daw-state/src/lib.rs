//! Project save/load and serialization
//! 
//! Handles:
//! - Saving projects to JSON
//! - Loading projects from disk
//! - Autosave
//! - Crash recovery

use daw_core::Project;
use std::path::Path;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct ProjectState;

impl ProjectState {
    /// Save project to JSON file
    pub fn save(project: &Project, path: impl AsRef<Path>) -> Result<(), StateError> {
        let json = serde_json::to_string_pretty(project)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// Load project from JSON file
    pub fn load(path: impl AsRef<Path>) -> Result<Project, StateError> {
        let json = fs::read_to_string(path)?;
        let project = serde_json::from_str(&json)?;
        Ok(project)
    }
    
    // TODO: Implement autosave
    // TODO: Implement crash recovery
}
