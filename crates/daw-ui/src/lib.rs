//! GUI implementation using egui
//! 
//! This module runs in the main thread and communicates with
//! the audio engine via the command queue.

pub mod app;
pub mod widgets;
pub mod theme;

pub use app::DawApp;
