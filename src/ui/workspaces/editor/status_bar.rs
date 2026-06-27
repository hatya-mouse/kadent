use crate::ui::{theme, workspaces::EditorUi};
use eframe::egui;

impl EditorUi {
    pub(super) fn status_bar(&self, ui: &mut egui::Ui) {
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
        });
    }

    fn status_text(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
    }
}
