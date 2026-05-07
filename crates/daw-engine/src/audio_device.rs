use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use std::sync::{Arc, Mutex};

pub struct AudioDevice {
    device: Device,
    config: StreamConfig,
    stream: Option<Stream>,
}

impl AudioDevice {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or("No output device available")?;
        
        let config = device.default_output_config()
            .map_err(|e| format!("Failed to get default config: {}", e))?
            .into();
        
        Ok(Self {
            device,
            config,
            stream: None,
        })
    }
    
    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }
    
    pub fn channels(&self) -> u16 {
        self.config.channels
    }
    
    /// Start audio stream with callback
    pub fn start<F>(&mut self, mut callback: F) -> Result<(), String>
    where
        F: FnMut(&mut [f32]) + Send + 'static,
    {
        let err_callback = |err| {
            log::error!("Audio stream error: {}", err);
        };
        
        let stream = self.device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    callback(data);
                },
                err_callback,
                None,
            )
            .map_err(|e| format!("Failed to build stream: {}", e))?;
        
        stream.play()
            .map_err(|e| format!("Failed to play stream: {}", e))?;
        
        self.stream = Some(stream);
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.stream = None;
    }
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new().expect("Failed to create audio device")
    }
}
