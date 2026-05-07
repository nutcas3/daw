use serde::{Deserialize, Serialize};

/// MIDI note with timing
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MidiNote {
    /// MIDI note number (0-127)
    pub pitch: u8,
    
    /// Velocity (0-127)
    pub velocity: u8,
    
    /// Start position in samples
    pub start: u64,
    
    /// Duration in samples
    pub duration: u64,
}

/// MIDI event (Note On/Off, CC, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MidiEvent {
    NoteOn {
        position: u64,
        pitch: u8,
        velocity: u8,
    },
    NoteOff {
        position: u64,
        pitch: u8,
    },
    ControlChange {
        position: u64,
        controller: u8,
        value: u8,
    },
}

impl MidiEvent {
    pub fn position(&self) -> u64 {
        match self {
            MidiEvent::NoteOn { position, .. } => *position,
            MidiEvent::NoteOff { position, .. } => *position,
            MidiEvent::ControlChange { position, .. } => *position,
        }
    }
}
