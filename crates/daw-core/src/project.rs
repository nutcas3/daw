use serde::{Deserialize, Serialize};
use crate::{Track, TimeSignature, TempoMap};

/// Top-level project structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project name
    pub name: String,
    
    /// Sample rate (44100, 48000, etc.)
    pub sample_rate: u32,
    
    /// All tracks in the project
    pub tracks: Vec<Track>,
    
    /// Tempo map (BPM changes over time)
    pub tempo_map: TempoMap,
    
    /// Time signature
    pub time_signature: TimeSignature,
    
    /// Project length in samples
    pub length_samples: u64,
}

impl Project {
    pub fn new(name: String, sample_rate: u32) -> Self {
        Self {
            name,
            sample_rate,
            tracks: Vec::new(),
            tempo_map: TempoMap::constant(120.0),
            time_signature: TimeSignature::default(),
            length_samples: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("Test Project".to_string(), 48000);
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.sample_rate, 48000);
        assert_eq!(project.tracks.len(), 0);
    }

    #[test]
    fn test_add_track() {
        let mut project = Project::new("Test".to_string(), 48000);
        let track = Track::new(1, "Audio Track".to_string(), TrackType::Audio);
        project.tracks.push(track);
        
        assert_eq!(project.tracks.len(), 1);
        assert_eq!(project.tracks[0].name, "Audio Track");
    }

    #[test]
    fn test_serialization() {
        let project = Project::new("Test".to_string(), 48000);
        let json = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&json).unwrap();
        
        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.sample_rate, deserialized.sample_rate);
    }
}
