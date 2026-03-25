mod toolbar;
mod track_list;

use crate::app::KnodiqApp;
use eframe::{App, egui};

impl App for KnodiqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(44.0)
            .show(ctx, |ui| {
                self.toolbar(ui);
            });

        egui::SidePanel::left("track_list")
            .exact_width(200.0)
            .show(ctx, |ui| {
                self.track_list(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Knodiq");
        });
    }
}
