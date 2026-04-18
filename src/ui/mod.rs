mod editor;
mod splash;

use crate::{app::KnodiqApp, colors, ui_state::AppScreen};
use eframe::{App, egui};

impl App for KnodiqApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.ui_state.app_screen {
            AppScreen::Splash => {
                egui::CentralPanel::default()
                    .frame(
                        egui::Frame::new()
                            .fill(colors::primary_bg(ui.visuals().dark_mode))
                            .inner_margin(0),
                    )
                    .show_inside(ui, |ui| {
                        self.splash_screen(ui);
                    });
            }
            AppScreen::Editor => {
                self.calculate_playhead();

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

                if self.ui_state.editor_state.selected_region.is_some() {
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

                // Drain and log errors from the audio thread
                while let Ok(err) = self.thread_handle.error_rx.try_recv() {
                    eprintln!("Audio thread error occurred");
                    self.errors.push(err);
                }
            }
        }
    }
}
