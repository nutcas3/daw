use serde::{Deserialize, Serialize};
use crate::Clip;

pub type TrackId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Audio,
    Midi,
    Master,
}

/// A single track in the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub track_type: TrackType,
    
    /// Audio clips on this track
    pub clips: Vec<Clip>,
    
    /// Track settings (will be read by engine via atomics)
    pub gain: f32,
    pub pan: f32,
    pub mute: bool,
    pub solo: bool,
    pub armed: bool,
}

impl Track {
    pub fn new(id: TrackId, name: String, track_type: TrackType) -> Self {
        Self {
            id,
            name,
            track_type,
            clips: Vec::new(),
            gain: 1.0,
            pan: 0.0,
            mute: false,
            solo: false,
            armed: false,
        }
    }
}
