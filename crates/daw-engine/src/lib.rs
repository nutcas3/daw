//! Real-time audio engine
//! 
//! **CRITICAL:** This module runs in a high-priority real-time thread.
//! 
//! Rules for real-time safety:
//! - NO memory allocation (Vec::new, Box, String, etc.)
//! - NO locks that can block (use try_lock only)
//! - NO syscalls (file I/O, network)
//! - Use lock-free data structures (atomics, lock-free queues)

pub mod command;
pub mod mixer;
pub mod node;
pub mod scheduler;
pub mod audio_device;

pub use command::{EngineCommand, CommandQueue};
pub use mixer::Mixer;
pub use node::{AudioNode, GainNode, PanNode};
pub use scheduler::MidiScheduler;
pub use audio_device::AudioDevice;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Main audio engine state
pub struct AudioEngine {
    sample_rate: u32,
    is_playing: Arc<AtomicBool>,
    mixer: Mixer,
    command_queue: CommandQueue,
}

impl AudioEngine {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            is_playing: Arc::new(AtomicBool::new(false)),
            mixer: Mixer::new(sample_rate),
            command_queue: CommandQueue::new(),
        }
    }
    
    pub fn play(&self) {
        self.is_playing.store(true, Ordering::Release);
    }
    
    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Release);
    }
    
    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Acquire)
    }
    
    pub fn command_queue(&self) -> &CommandQueue {
        &self.command_queue
    }
    
    /// Real-time audio callback
    /// 
    /// **This runs in the audio thread - must be real-time safe!**
    pub fn process(&mut self, output: &mut [f32]) {
        // Process commands at block boundaries only
        self.process_commands();
        
        // Generate audio
        if self.is_playing() {
            self.mixer.process(output);
        } else {
            // Silence
            output.fill(0.0);
        }
    }
    
    fn process_commands(&mut self) {
        // Process all pending commands
        while let Some(cmd) = self.command_queue.try_pop() {
            self.execute_command(cmd);
        }
    }
    
    fn execute_command(&mut self, cmd: EngineCommand) {
        match cmd {
            EngineCommand::SetTrackGain { track_id, gain } => {
                self.mixer.set_track_gain(track_id, gain);
            }
            EngineCommand::SetTrackMute { track_id, mute } => {
                self.mixer.set_track_mute(track_id, mute);
            }
            // Add more command handlers as needed
        }
    }
}
