use std::time::{Duration, Instant};

use crate::ui::{
    theme,
    workspaces::{
        EditorUi,
        editor::state::{Modification, TempStatusNotification},
    },
};
use eframe::egui;

const TEMP_STATUS_DURATION: u64 = 5;
/// Duration for fade-in effect in seconds.
const TEMP_STATUS_IN: f32 = 0.2;
/// Duration for fade-out effect in seconds.
const TEMP_STATUS_OUT: f32 = 0.5;

impl EditorUi {
    pub(super) fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            let audio_ctx = &self.ui_state.audio_ctx;
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

            self.modification_text(ui);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.notification_text(ui);
            });
        });
    }

    pub(crate) fn show_temp_status(&mut self, text: &str, color: egui::Color32) {
        if let Some(ref notif) = self.ui_state.status_bar_state.temp_status
            && notif.text == text
        {
            // If the same notification is already being displayed, make the notification stay longer
            self.ui_state
                .status_bar_state
                .temp_status
                .as_mut()
                .unwrap()
                .expires_at = Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION);
            return;
        }

        self.ui_state.status_bar_state.temp_status = Some(TempStatusNotification {
            text: text.to_string(),
            color,
            started_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION),
        });
    }

    fn modification_text(&mut self, ui: &mut egui::Ui) {
        if self.ui_state.modification.is_none() {
            return;
        }

        let resolution = self.ui_state.audio_ctx.resolution as f32;
        let modification_string = match self.ui_state.modification {
            Modification::None => unreachable!(),
            Modification::ProjectRange(start_ticks, duration_ticks) => {
                let start_beats = start_ticks.0 as f32 / resolution;
                let duration_beats = duration_ticks.0 as f32 / resolution;
                format!(
                    "Project Range: {:.3} – {:.3} Beats",
                    start_beats,
                    start_beats + duration_beats
                )
            }
            Modification::RegionRange(start_ticks, duration_ticks) => {
                let start_beats = start_ticks.0 as f32 / resolution;
                let duration_beats = duration_ticks.0 as f32 / resolution;
                format!(
                    "Region Range: {:.3} – {:.3} Beats",
                    start_beats,
                    start_beats + duration_beats
                )
            }
            Modification::NotePosition(start_ticks, duration_ticks, pitch) => {
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

        self.status_text(ui, &modification_string);
    }

    fn notification_text(&mut self, ui: &mut egui::Ui) {
        if let Some(ref notif) = self.ui_state.status_bar_state.temp_status {
            let now = Instant::now();

            if now < notif.expires_at {
                let elapsed = now.duration_since(notif.started_at).as_secs_f32();
                let remaining = notif.expires_at.duration_since(now).as_secs_f32();

                // Fade in and fade out calculations
                let fade_in = (elapsed / TEMP_STATUS_IN).min(1.0);
                let fade_out = (remaining / TEMP_STATUS_OUT).min(1.0);
                let opacity = fade_in * fade_out;
                let text_color = egui::Color32::WHITE.linear_multiply(opacity);

                // Draw the notification with the animated opacity
                egui::Frame::new()
                    .fill(notif.color)
                    .multiply_with_opacity(opacity)
                    .corner_radius(4)
                    .inner_margin(egui::Margin::symmetric(6, 2))
                    .outer_margin(egui::Margin::symmetric(0, 4))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&notif.text).color(text_color));
                    });

                // Request a repaint to keep the notification animated
                ui.ctx().request_repaint();
            } else {
                self.ui_state.status_bar_state.temp_status = None;
            }
        }
    }

    fn status_text(&self, ui: &mut egui::Ui, text: &str) {
        ui.label(egui::RichText::new(text).color(theme::secondary_fg(ui.visuals().dark_mode)));
    }
}
