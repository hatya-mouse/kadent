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
                self.notification_text(ui);
            });
        });
    }

    pub(crate) fn show_temp_status(&mut self, text: &str, color: egui::Color32) {
        self.ui_state.status_bar_state.temp_status = Some(TempStatusNotification {
            text: text.to_string(),
            color,
            started_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION),
        });
    }

    fn notification_text(&mut self, ui: &mut egui::Ui) {
        if let Some(ref notif) = self.ui_state.status_bar_state.temp_status {
            let now = Instant::now();

            if now < notif.expires_at {
                let elapsed = now.duration_since(notif.started_at).as_secs_f32();
                let remaining = notif.expires_at.duration_since(now).as_secs_f32();

                // Fade in and fade out calculations
                let fade_in = (elapsed / 0.2).min(1.0);
                let fade_out = (remaining / 0.5).min(1.0);
                let alpha_multiplier = fade_in * fade_out;

                let animated_color = notif.color.linear_multiply(alpha_multiplier);

                // Draw the notification text with the animated color
                ui.label(egui::RichText::new(&notif.text).color(animated_color));
            } else {
                self.ui_state.status_bar_state.temp_status = None;
            }
        }
    }

    fn status_text(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
    }
}
