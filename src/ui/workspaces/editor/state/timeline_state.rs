use eframe::egui;
use std::path::PathBuf;

pub(crate) struct TimelineState {
    /// The height of each track in the timeline.
    pub track_height: f32,
    /// The width of the track list.
    pub track_list_width: f32,
    /// Pixels per beat in the timeline.
    pub pixels_per_beat: f32,

    // --- AUDIO FILE IMPORT ---
    /// The last audio file dropped into the editor, along with the position where it was dropped.
    pub last_audio_drop: Option<(PathBuf, egui::Pos2)>,
    /// Currently being dragged file and its position.
    pub dragging_audio_file: Option<(PathBuf, egui::Pos2)>,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            track_height: 50.0,
            track_list_width: 200.0,
            pixels_per_beat: 80.0,
            last_audio_drop: None,
            dragging_audio_file: None,
        }
    }
}
