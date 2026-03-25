pub(crate) mod toolbar;
pub(crate) mod track_list;

use crate::app::KnodiqApp;
use eframe::{egui, App};

impl App for KnodiqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(44.0)
            .show(ctx, |ui| {
                self.toolbar(ui);
            });

        egui::SidePanel::left("track_list")
            .min_width(150.0)
            .max_width(300.0)
            .default_width(200.0)
            .show(ctx, |ui| {
                self.track_list(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Knodiq");
        });

        // Add track dialog
        if self.ui_state.show_add_track_dialog {
            self.add_track_dialog(ctx);
        }
    }
}
