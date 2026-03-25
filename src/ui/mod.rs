pub(crate) mod timeline;
pub(crate) mod toolbar;

use crate::app::KnodiqApp;
use eframe::{App, egui};

impl App for KnodiqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(44.0)
            .show(ctx, |ui| {
                self.toolbar(ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(0))
            .show(ctx, |ui| {
                self.timeline(ui);
            });

        // Show dialogs
        self.track_dialog(ctx);
    }
}
