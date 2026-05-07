use serde::{Deserialize, Serialize};

pub type SamplePosition = u64;

/// Time signature (numerator/denominator)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeSignature {
    pub numerator: u32,
    pub denominator: u32,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            numerator: 4,
            denominator: 4,
        }
    }
}

/// Tempo map for handling tempo changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempoMap {
    changes: Vec<TempoChange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct TempoChange {
    sample_position: SamplePosition,
    bpm: f64,
}

impl TempoMap {
    pub fn constant(bpm: f64) -> Self {
        Self {
            changes: vec![TempoChange {
                sample_position: 0,
                bpm,
            }],
        }
    }
    
    /// Convert beats to samples at a given position
    pub fn beats_to_samples(&self, beats: f64, sample_rate: u32) -> SamplePosition {
        // Simplified: assumes constant tempo
        let bpm = self.changes[0].bpm;
        let samples_per_beat = (sample_rate as f64 * 60.0) / bpm;
        (beats * samples_per_beat) as SamplePosition
    }
    
    /// Convert samples to beats at a given position
    pub fn samples_to_beats(&self, samples: SamplePosition, sample_rate: u32) -> f64 {
        let bpm = self.changes[0].bpm;
        let samples_per_beat = (sample_rate as f64 * 60.0) / bpm;
        samples as f64 / samples_per_beat
    }
}
