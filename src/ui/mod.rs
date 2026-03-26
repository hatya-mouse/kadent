pub(crate) mod piano_roll;
pub(crate) mod timeline;
pub(crate) mod toolbar;

use crate::{app::KnodiqApp, colors};
use eframe::{App, egui};

impl App for KnodiqApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(colors::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show_inside(ui, |ui| {
                self.toolbar(ui);
            });

        if self.ui_state.selected_region.is_some() {
            egui::Panel::bottom("piano_roll")
                .frame(
                    egui::Frame::new()
                        .fill(colors::primary_bg(ui.visuals().dark_mode))
                        .inner_margin(0),
                )
                .min_size(300.0)
                .show_inside(ui, |ui| {
                    self.piano_roll(ui);
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(colors::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(0),
            )
            .show_inside(ui, |ui| {
                self.timeline(ui);
            });

        // Show dialogs
        self.track_dialog(ui);

        // Check for project updating
        self.update_project();
    }
}
