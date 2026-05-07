use daw_core::TrackId;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Real-time safe track state
pub struct RtTrack {
    pub id: TrackId,
    gain: AtomicU32,  // f32 as u32 bits
    pan: AtomicU32,
    mute: AtomicBool,
    solo: AtomicBool,
    position: AtomicU32,
}

impl RtTrack {
    pub fn new(id: TrackId) -> Self {
        Self {
            id,
            gain: AtomicU32::new(1.0_f32.to_bits()),
            pan: AtomicU32::new(0.0_f32.to_bits()),
            mute: AtomicBool::new(false),
            solo: AtomicBool::new(false),
            position: AtomicU32::new(0),
        }
    }
    
    pub fn get_gain(&self) -> f32 {
        f32::from_bits(self.gain.load(Ordering::Acquire))
    }
    
    pub fn set_gain(&self, value: f32) {
        self.gain.store(value.to_bits(), Ordering::Release);
    }
    
    pub fn get_pan(&self) -> f32 {
        f32::from_bits(self.pan.load(Ordering::Acquire))
    }
    
    pub fn set_pan(&self, value: f32) {
        self.pan.store(value.to_bits(), Ordering::Release);
    }
    
    pub fn is_muted(&self) -> bool {
        self.mute.load(Ordering::Acquire)
    }
    
    pub fn set_mute(&self, value: bool) {
        self.mute.store(value, Ordering::Release);
    }
}

/// Audio mixer - sums all tracks
pub struct Mixer {
    sample_rate: u32,
    tracks: Vec<RtTrack>,
    master_gain: AtomicU32,
}

impl Mixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            tracks: Vec::new(),
            master_gain: AtomicU32::new(1.0_f32.to_bits()),
        }
    }
    
    pub fn add_track(&mut self, id: TrackId) {
        self.tracks.push(RtTrack::new(id));
    }
    
    pub fn set_track_gain(&mut self, id: TrackId, gain: f32) {
        if let Some(track) = self.tracks.iter().find(|t| t.id == id) {
            track.set_gain(gain);
        }
    }
    
    pub fn set_track_mute(&mut self, id: TrackId, mute: bool) {
        if let Some(track) = self.tracks.iter().find(|t| t.id == id) {
            track.set_mute(mute);
        }
    }
    
    /// Process audio - real-time safe
    pub fn process(&mut self, output: &mut [f32]) {
        // Clear output buffer
        output.fill(0.0);
        
        // Sum all tracks
        for track in &self.tracks {
            if track.is_muted() {
                continue;
            }
            
            let gain = track.get_gain();
            let pan = track.get_pan();
            
            // TODO: Get actual audio from track's sample buffer
            // For now, just demonstrate the mixing structure
            
            // Apply gain (simplified mono for now)
            for sample in output.iter_mut() {
                *sample += 0.0 * gain;  // Would add track's audio here
            }
        }
        
        // Apply master gain
        let master = f32::from_bits(self.master_gain.load(Ordering::Acquire));
        for sample in output.iter_mut() {
            *sample *= master;
        }
    }
}
