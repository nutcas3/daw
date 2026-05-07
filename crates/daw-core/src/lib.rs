//! Core data types for the DAW
//! 
//! This crate defines the fundamental data structures used throughout
//! the DAW: projects, tracks, clips, MIDI notes, etc.
//! 
//! **Important:** This crate should remain pure data models with no
//! dependencies on audio I/O, GUI, or plugin hosting.

pub mod project;
pub mod track;
pub mod clip;
pub mod time;
pub mod midi;

pub use project::Project;
pub use track::{Track, TrackId, TrackType};
pub use clip::{Clip, ClipId};
pub use time::{TimeSignature, TempoMap, SamplePosition};
pub use midi::{MidiNote, MidiEvent};
