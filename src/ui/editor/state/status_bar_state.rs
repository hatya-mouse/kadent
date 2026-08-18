use crate::background_thread::BackgroundTaskStatus;
use eframe::egui;
use kadent_engine::data_types::Ticks;
use std::time::{Duration, Instant};

const TEMP_STATUS_DURATION: u64 = 5;
/// Duration for fade-in effect in seconds.
const TEMP_STATUS_IN: f32 = 0.2;
/// Duration for fade-out effect in seconds.
const TEMP_STATUS_OUT: f32 = 0.5;

#[derive(Default)]
pub(crate) struct StatusBarState {
    /// Temporary status notification shown in the right side of the status bar.
    pub(crate) temp_status: Option<TempStatusNotification>,
    /// Currently processing task to show in the status bar.
    pub(crate) current_task: Option<BackgroundTaskStatus>,
    /// The last modified value in purpose of showing the value in the status bar.
    pub status_hint: StatusHint,
}

pub(crate) struct TempStatusNotification {
    pub(crate) text: String,
    pub(crate) color: egui::Color32,
    pub(crate) started_at: Instant,
    pub(crate) expires_at: Instant,
}

#[derive(Default)]
pub(crate) enum StatusHint {
    #[default]
    None,
    ProjectRange(Ticks, Ticks),
    RegionRange(Ticks, Ticks),
    /// (start, end, pitch)
    NotePosition(Ticks, Ticks, f32),
}

impl StatusHint {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, StatusHint::None)
    }
}

impl StatusBarState {
    pub(crate) fn show_temp_status(&mut self, text: &str, color: egui::Color32) {
        if let Some(ref notif) = self.temp_status
            && notif.text == text
        {
            // If the same notification is already being displayed, make the notification stay longer
            self.temp_status.as_mut().unwrap().expires_at =
                Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION);
            return;
        }

        self.temp_status = Some(TempStatusNotification {
            text: text.to_string(),
            color,
            started_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(TEMP_STATUS_DURATION),
        });
    }

    pub(crate) fn notification_text(&mut self, ui: &mut egui::Ui) {
        if let Some(ref notif) = self.temp_status {
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
                self.temp_status = None;
            }
        }
    }

    pub(crate) fn set_status_hint(&mut self, hint: StatusHint) {
        self.status_hint = hint;
    }
}
