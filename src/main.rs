pub(crate) mod colors;
pub(crate) mod commands;
pub(crate) mod components;
pub(crate) mod kasl_node;
pub(crate) mod load_write;
pub(crate) mod metadata;
pub(crate) mod ui_state;

mod app;
mod fonts;
mod ui;

use app::KnodiqApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Knodiq",
        options,
        Box::new(|cc| Ok(Box::new(KnodiqApp::new(cc)))),
    )
}
