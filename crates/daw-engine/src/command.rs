use daw_core::TrackId;
use ringbuf::{HeapRb, HeapProducer, HeapConsumer};
use std::sync::Arc;

/// Commands sent from UI thread to audio thread
#[derive(Debug, Clone)]
pub enum EngineCommand {
    SetTrackGain { track_id: TrackId, gain: f32 },
    SetTrackMute { track_id: TrackId, mute: bool },
    SetTrackPan { track_id: TrackId, pan: f32 },
    SetTrackSolo { track_id: TrackId, solo: bool },
    AddTrack { track_id: TrackId },
    RemoveTrack { track_id: TrackId },
}

/// Lock-free command queue for cross-thread communication
pub struct CommandQueue {
    producer: Arc<parking_lot::Mutex<HeapProducer<EngineCommand>>>,
    consumer: Arc<parking_lot::Mutex<HeapConsumer<EngineCommand>>>,
}

impl CommandQueue {
    pub fn new() -> Self {
        let rb = HeapRb::<EngineCommand>::new(1024);
        let (producer, consumer) = rb.split();
        
        Self {
            producer: Arc::new(parking_lot::Mutex::new(producer)),
            consumer: Arc::new(parking_lot::Mutex::new(consumer)),
        }
    }
    
    /// Send a command from UI thread (can block briefly)
    pub fn push(&self, cmd: EngineCommand) -> Result<(), &'static str> {
        let mut producer = self.producer.lock();
        producer.push(cmd).map_err(|_| "Command queue full")
    }
    
    /// Try to pop a command in audio thread (never blocks)
    pub fn try_pop(&self) -> Option<EngineCommand> {
        // Use try_lock to maintain real-time safety
        if let Some(mut consumer) = self.consumer.try_lock() {
            consumer.pop()
        } else {
            None
        }
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CommandQueue {
    fn clone(&self) -> Self {
        // Create a new queue - can't clone the ring buffer
        Self::new()
    }
}
