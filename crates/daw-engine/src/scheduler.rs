use daw_core::{MidiEvent, SamplePosition};

/// Schedules MIDI events with sample-accurate timing
pub struct MidiScheduler {
    events: Vec<MidiEvent>,
    current_position: SamplePosition,
    sample_rate: u32,
}

impl MidiScheduler {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            events: Vec::new(),
            current_position: 0,
            sample_rate,
        }
    }
    
    /// Add a MIDI event to the timeline
    pub fn add_event(&mut self, event: MidiEvent) {
        // Insert in sorted order by position
        let pos = self.events.binary_search_by_key(&event.position(), |e| e.position())
            .unwrap_or_else(|e| e);
        self.events.insert(pos, event);
    }
    
    /// Get all events in the current processing block
    pub fn get_events_in_range(&self, start: SamplePosition, end: SamplePosition) -> Vec<&MidiEvent> {
        self.events.iter()
            .filter(|e| {
                let pos = e.position();
                pos >= start && pos < end
            })
            .collect()
    }
    
    /// Advance the playhead
    pub fn advance(&mut self, samples: u32) {
        self.current_position += samples as u64;
    }
    
    pub fn reset(&mut self) {
        self.current_position = 0;
    }
}
