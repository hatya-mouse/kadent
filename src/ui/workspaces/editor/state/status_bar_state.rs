use std::time::Instant;

#[derive(Default)]
pub(crate) struct StatusBarState {
    /// Temporary status notification shown in the right side of the status bar.
    pub(crate) temp_status: Option<TempStatusNotification>,
}

pub(crate) struct TempStatusNotification {
    pub(crate) text: String,
    /// An Instant to manage the duration of the temporary status text.
    pub(crate) expires_at: Instant,
}
