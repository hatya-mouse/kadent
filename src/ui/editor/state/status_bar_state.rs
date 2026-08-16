use crate::background_thread::BackgroundTaskStatus;
use eframe::egui;
use std::time::Instant;

#[derive(Default)]
pub(crate) struct StatusBarState {
    /// Temporary status notification shown in the right side of the status bar.
    pub(crate) temp_status: Option<TempStatusNotification>,
    /// Currently processing task to show in the status bar.
    pub(crate) current_task: Option<BackgroundTaskStatus>,
}

pub(crate) struct TempStatusNotification {
    pub(crate) text: String,
    pub(crate) color: egui::Color32,
    pub(crate) started_at: Instant,
    pub(crate) expires_at: Instant,
}
