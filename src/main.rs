mod app;
mod commands;
mod consts;
mod core;
mod fonts;
mod storage;
mod ui;
mod utils;

use crate::{app::KadentApp, consts::APP_NAME};
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(KadentApp::new(cc)))),
    )
}
