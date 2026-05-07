use anyhow::Result;
use daw_engine::{AudioEngine, AudioDevice};
use daw_ui::DawApp;
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    log::info!("Starting DAW...");
    
    // Initialize audio device
    let mut audio_device = AudioDevice::new()?;
    let sample_rate = audio_device.sample_rate();
    
    log::info!("Audio device initialized: {} Hz, {} channels", 
               sample_rate, audio_device.channels());
    
    // Create audio engine
    let engine = Arc::new(Mutex::new(AudioEngine::new(sample_rate)));
    let engine_clone = Arc::clone(&engine);
    
    // Get command queue for UI communication
    let command_queue = {
        let engine_lock = engine.lock().unwrap();
        Arc::new(engine_lock.command_queue().clone())
    };
    
    // Start audio stream
    audio_device.start(move |output: &mut [f32]| {
        // This runs in the real-time audio thread
        let mut engine = engine_clone.lock().unwrap();
        engine.process(output);
    })?;
    
    log::info!("Audio stream started");
    
    // Start GUI
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("My DAW"),
        ..Default::default()
    };
    
    eframe::run_native(
        "My DAW",
        native_options,
        Box::new(|cc| Box::new(DawApp::new(cc, command_queue))),
    )?;
    
    Ok(())
}
