use crate::ui_state::dialog_state::DialogState;

pub mod dialog_state;

pub struct KnodiqUIState {
    /// The current dialog state.
    pub dialog_state: DialogState,

    /// The height of each track in the timeline.
    pub track_height: f32,

    /// The width of the track list.
    pub track_list_width: f32,

    /// Scroll amount of the timeline.
    pub timeline_scroll_y: f32,
}

impl Default for KnodiqUIState {
    fn default() -> Self {
        Self {
            dialog_state: DialogState::None,
            track_height: 50.0,
            track_list_width: 250.0,
            timeline_scroll_y: 0.0,
        }
    }
}
