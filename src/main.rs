mod app;
mod background_thread;
mod consts;
mod core;
mod fonts;
mod storage;
mod ui;
mod utils;

use crate::{app::KadentApp, consts::APP_NAME};
use eframe::egui::ViewportBuilder;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let initial_project = check_project_open();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(KadentApp::new(cc, initial_project)))),
    )
}

/// Checks the command line arguments for handling project file opening.
fn check_project_open() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().collect();

    // The first element is the path of the executable,
    // so check if there is a second argument which may be the project file path.
    if args.len() > 1 {
        let path = std::path::PathBuf::from(&args[1]);
        if path.extension().is_some_and(|ext| ext == "kdp") {
            return Some(path);
        }
    }

    None
}
