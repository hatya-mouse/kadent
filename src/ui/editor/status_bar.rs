use crate::{
    background_thread::BackgroundTaskStatus,
    ui::{EditorState, editor::state::StatusHint, theme},
};
use eframe::egui;

impl EditorState {
    pub(super) fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            if let Some(track_id) = self.selection.track_id()
                && let Some(track_meta) = self.project.meta.get_track(&track_id)
            {
                status_text(ui, &format!("Selection: {}", track_meta.name));

                if let Some(region_id) = self.selection.region_id()
                    && let Some(region_meta) = track_meta.get_region(&region_id)
                {
                    status_text(ui, "—");
                    status_text(ui, &region_meta.name);
                }

                if let Some(node_id) = self.selection.node_id()
                    && let Some(node_meta) = track_meta.graph.get_node_meta(&node_id)
                {
                    status_text(ui, "—");
                    status_text(ui, &node_meta.display_name);
                }
            }

            if let Some(task) = &self.views.status_bar.current_task {
                status_text(
                    ui,
                    match task {
                        BackgroundTaskStatus::Save => "Saving ProjectData...",
                        BackgroundTaskStatus::Open => "Opening ProjectData...",
                        BackgroundTaskStatus::Export => "Exporting ProjectData...",
                        BackgroundTaskStatus::Import => "Importing File...",
                        BackgroundTaskStatus::GenerateWaveform => "Generating Waveform...",
                    },
                );
            }

            self.modification_text(ui);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.views.status_bar.notification_text(ui);
            });
        });
    }

    fn modification_text(&self, ui: &mut egui::Ui) {
        if self.views.status_bar.status_hint.is_none() {
            return;
        }

        let resolution = self.project.data.audio_ctx.resolution as f32;
        let hint_string = match self.views.status_bar.status_hint {
            StatusHint::None => unreachable!(),
            StatusHint::ProjectRange(start_ticks, duration_ticks) => {
                let start_beats = start_ticks.0 as f32 / resolution;
                let duration_beats = duration_ticks.0 as f32 / resolution;
                format!(
                    "ProjectData Range: {:.3} – {:.3} Beats",
                    start_beats,
                    start_beats + duration_beats
                )
            }
            StatusHint::RegionRange(start_ticks, duration_ticks) => {
                let start_beats = start_ticks.0 as f32 / resolution;
                let duration_beats = duration_ticks.0 as f32 / resolution;
                format!(
                    "Region Range: {:.3} – {:.3} Beats",
                    start_beats,
                    start_beats + duration_beats
                )
            }
            StatusHint::NotePosition(start_ticks, duration_ticks, pitch) => {
                let start_beats = start_ticks.0 as f32 / resolution;
                let duration_beats = duration_ticks.0 as f32 / resolution;
                format!(
                    "Note: {:.3} – {:.3} Beats, Pitch {:.3}",
                    start_beats,
                    start_beats + duration_beats,
                    pitch
                )
            }
        };

        status_text(ui, &hint_string);
    }
}

fn status_text(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
}
