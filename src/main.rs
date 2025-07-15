mod core;
mod models;
mod ui;
mod api;
use eframe::egui::{self, IconData};
use image::ImageFormat;
use std::sync::Arc;
use core::GoalApp;

fn main() -> Result<(), eframe::Error> {
    let icon = load_embedded_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([620.0, 620.0])
            .with_min_inner_size([620.0, 620.0])
            .with_resizable(true)
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "OSRS Goals",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(match GoalApp::new(&cc.egui_ctx) {
                Ok(app) => app,
                Err(e) => {
                    eprintln!("Failed to initialize app: {}", e);
                    std::process::exit(1);
                }
            })
        }),
    )
}

// Decode the .png for the taskbar icon
fn load_embedded_icon() -> Arc<IconData> {
    let bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory_with_format(bytes, ImageFormat::Png)
        .expect("Failed to load icon image")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Arc::new(IconData {
        rgba,
        width: width as u32,
        height: height as u32,
    })
}