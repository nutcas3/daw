use eframe::egui;
use daw_core::Project;
use daw_engine::{AudioEngine, EngineCommand};
use std::sync::Arc;

pub struct DawApp {
    project: Project,
    engine_commands: Arc<daw_engine::CommandQueue>,
}

impl DawApp {
    pub fn new(cc: &eframe::CreationContext, engine_commands: Arc<daw_engine::CommandQueue>) -> Self {
        Self {
            project: Project::new("Untitled".to_string(), 48000),
            engine_commands,
        }
    }
    
    fn render_transport(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("▶ Play").clicked() {
                log::info!("Play clicked");
            }
            if ui.button("⏸ Pause").clicked() {
                log::info!("Pause clicked");
            }
            if ui.button("⏹ Stop").clicked() {
                log::info!("Stop clicked");
            }
            if ui.button("⏺ Record").clicked() {
                log::info!("Record clicked");
            }
        });
    }
    
    fn render_mixer(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mixer");
        
        for track in &mut self.project.tracks {
            ui.horizontal(|ui| {
                ui.label(&track.name);
                
                // Volume slider
                let mut gain = track.gain;
                if ui.add(egui::Slider::new(&mut gain, 0.0..=2.0).text("Vol")).changed() {
                    track.gain = gain;
                    let _ = self.engine_commands.push(EngineCommand::SetTrackGain {
                        track_id: track.id,
                        gain,
                    });
                }
                
                // Mute button
                let mut mute = track.mute;
                if ui.checkbox(&mut mute, "M").changed() {
                    track.mute = mute;
                    let _ = self.engine_commands.push(EngineCommand::SetTrackMute {
                        track_id: track.id,
                        mute,
                    });
                }
            });
        }
    }
}

impl eframe::App for DawApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("transport").show(ctx, |ui| {
            self.render_transport(ui);
        });
        
        egui::SidePanel::right("mixer").show(ctx, |ui| {
            self.render_mixer(ui);
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timeline / Piano Roll");
            ui.label("Timeline view will go here");
        });
    }
}
