use std::time::{Duration, Instant};

use crate::ui::{
    theme,
    workspaces::{EditorUi, editor::state::TempStatusNotification},
};
use eframe::egui;

const TEMP_STATUS_DURATION: u64 = 5;

impl EditorUi {
    pub(super) fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            let audio_ctx = &self.proj_ctx.project.audio_ctx;
            self.status_text(ui, &format!("Sample Rate {}", audio_ctx.sample_rate));
            self.status_text(ui, &format!("Buffer Size {}", audio_ctx.buffer_size));

            if let Some(track_id) = self.ui_state.selection.track_id()
                && let Some(track_meta) = self.proj_ctx.project_meta.get_track(&track_id)
            {
                self.status_text(ui, &format!("Selection: {}", track_meta.name));

                if let Some(region_id) = self.ui_state.selection.region_id()
                    && let Some(region_meta) = track_meta.get_region(&region_id)
                {
                    self.status_text(ui, "—");
                    self.status_text(ui, &region_meta.name);
                }

                if let Some(node_id) = self.ui_state.selection.node_id()
                    && let Some(node_meta) = track_meta.graph.get_node_meta(&node_id)
                {
                    self.status_text(ui, "—");
                    self.status_text(ui, &node_meta.display_name);
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(notif) = self.ui_state.status_bar_state.temp_status.as_ref() {
                    if Instant::now() < notif.expires_at {
                        self.status_text(ui, &notif.text);
                    } else {
                        self.ui_state.status_bar_state.temp_status = None;
                    }
                }
            });
        });
    }

    pub(crate) fn show_temp_status(&mut self, text: &str) {
        self.ui_state.status_bar_state.temp_status = Some(TempStatusNotification {
            text: text.to_string(),
            expires_at: Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION),
        });
    }

    fn status_text(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
    }
}
