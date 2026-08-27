mod state;

pub(crate) use state::StatusBarView;

use crate::{
    background_thread::BackgroundTaskStatus,
    ui::{EditorState, theme},
};
use eframe::egui;

impl StatusBarView {
    pub(super) fn ui(&mut self, ui: &mut egui::Ui, state: &EditorState) {
        ui.horizontal_centered(|ui| {
            if let Some(track_id) = state.selection.track_id()
                && let Some(track_meta) = state.project.meta.get_track(&track_id)
            {
                status_text(ui, &format!("Selection: {}", track_meta.name));

                if let Some(region_id) = state.selection.region_id()
                    && let Some(region_meta) = track_meta.get_region(&region_id)
                {
                    status_text(ui, "—");
                    status_text(ui, &region_meta.name);
                }

                if let Some(node_id) = state.selection.node_id()
                    && let Some(node_meta) = track_meta.graph.get_node_meta(&node_id)
                {
                    status_text(ui, "—");
                    status_text(ui, &node_meta.display_name);
                }
            }

            if let Some(task) = &self.current_task {
                status_text(
                    ui,
                    match task {
                        BackgroundTaskStatus::Save => "Saving Project...",
                        BackgroundTaskStatus::Open => "Opening Project...",
                        BackgroundTaskStatus::Export => "Exporting Project...",
                        BackgroundTaskStatus::Import => "Importing File...",
                        BackgroundTaskStatus::GenerateWaveform => "Generating Waveform...",
                    },
                );
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.notification_text(ui);
            });
        });
    }
}

fn status_text(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
}
