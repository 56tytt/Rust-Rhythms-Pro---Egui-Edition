// src/main.rs

use eframe::egui;

// Declare our project modules
mod audio;
mod bottom_panel;
mod equalizer;
mod playlist;
mod ui;

pub fn main() -> eframe::Result {
    // 1. Initialize GStreamer before doing anything else
    gstreamer::init().expect("Failed to initialize GStreamer");

    // 2. Configure the native window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    // 3. Run the application using our UI module
    eframe::run_native(
        "Rust Rhythms - Egui Edition",
        options,
        Box::new(|cc| Ok(Box::new(ui::MusicPlayerApp::new(&cc.egui_ctx)))),
    )
}
