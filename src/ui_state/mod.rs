pub mod dialog_state;

use crate::ui_state::dialog_state::DialogState;
use knodiq_engine::{mixer::TrackID, track::RegionID};

pub struct KnodiqUIState {
    /// The current dialog state.
    pub dialog_state: DialogState,

    /// The latest playhead samples received from the audio thread.
    pub last_playhead: usize,

    /// The last playhead x position.
    pub last_playhead_x: f32,

    /// The x scroll amount of the timeline.
    pub timeline_scroll_x: f32,

    /// An ID of the currently selected region.
    pub selected_region: Option<(TrackID, RegionID)>,

    /// The height of each track in the timeline.
    pub track_height: f32,

    /// The width of the track list.
    pub track_list_width: f32,

    /// Scroll amount of the timeline.
    pub timeline_scroll_y: f32,

    /// Pixels per beat.
    pub pixels_per_beat: f32,
}

impl Default for KnodiqUIState {
    fn default() -> Self {
        Self {
            last_playhead: 0,
            last_playhead_x: 0.0,
            timeline_scroll_x: 0.0,
            dialog_state: DialogState::None,
            selected_region: None,
            track_height: 50.0,
            track_list_width: 250.0,
            timeline_scroll_y: 0.0,
            pixels_per_beat: 80.0,
        }
    }
}
