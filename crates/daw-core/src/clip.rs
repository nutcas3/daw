use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type ClipId = u32;

/// A region of audio or MIDI on a track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,
    
    /// Start position on timeline (in samples)
    pub start_position: u64,
    
    /// Length in samples
    pub length_samples: u64,
    
    /// Offset into source file (for audio clips)
    pub source_offset: u64,
    
    /// Reference to source audio file
    pub audio_file: Option<PathBuf>,
    
    /// MIDI events (for MIDI clips)
    pub midi_events: Vec<crate::midi::MidiEvent>,
    
    /// Clip gain
    pub gain: f32,
    
    /// Fade in/out in samples
    pub fade_in: u64,
    pub fade_out: u64,
}

impl Clip {
    pub fn new_audio(id: ClipId, name: String, audio_file: PathBuf) -> Self {
        Self {
            id,
            name,
            start_position: 0,
            length_samples: 0,
            source_offset: 0,
            audio_file: Some(audio_file),
            midi_events: Vec::new(),
            gain: 1.0,
            fade_in: 0,
            fade_out: 0,
        }
    }
    
    pub fn new_midi(id: ClipId, name: String) -> Self {
        Self {
            id,
            name,
            start_position: 0,
            length_samples: 0,
            source_offset: 0,
            audio_file: None,
            midi_events: Vec::new(),
            gain: 1.0,
            fade_in: 0,
            fade_out: 0,
        }
    }
}
