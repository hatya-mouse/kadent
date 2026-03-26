pub(crate) mod piano_roll;
pub(crate) mod timeline;
pub(crate) mod toolbar;

use crate::{app::KnodiqApp, colors};
use eframe::{App, egui};

impl App for KnodiqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(colors::tertiary_bg(ctx.style().visuals.dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_height(44.0)
            .show(ctx, |ui| {
                self.toolbar(ui);
            });

        if self.ui_state.selected_region.is_some() {
            egui::TopBottomPanel::bottom("piano_roll")
                .frame(
                    egui::Frame::new()
                        .fill(colors::primary_bg(ctx.style().visuals.dark_mode))
                        .inner_margin(0),
                )
                .min_height(300.0)
                .show(ctx, |ui| {
                    self.piano_roll(ui);
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(colors::primary_bg(ctx.style().visuals.dark_mode))
                    .inner_margin(0),
            )
            .show(ctx, |ui| {
                self.timeline(ui);
            });

        // Show dialogs
        self.track_dialog(ctx);

        // Check for project updating
        self.update_project();
    }
}
