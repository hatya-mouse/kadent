mod commands;
mod core;
mod storage;
mod ui;
mod ui_state;

mod app;
mod fonts;

use crate::app::KadentApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Kadent",
        options,
        Box::new(|cc| Ok(Box::new(KadentApp::new(cc)))),
    )
}
