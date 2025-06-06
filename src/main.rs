mod core;
mod model;
mod ui;

use core::GoalApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OSRS Goals",
        options,
        Box::new(|_cc| Box::new(GoalApp::load_from_file())),
    )
}