use crate::ui_state::{
    dialog_state::DialogState, piano_roll_state::PianoRollState, timeline_state::TimelineState,
};
use knodiq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};
use std::time::Instant;

#[derive(Default)]
pub struct EditorUIState {
    /// The current dialog state.
    pub dialog_state: DialogState,

    /// The current timeline state.
    pub timeline_state: TimelineState,

    /// The current piano roll state.
    pub piano_roll_state: PianoRollState,

    /// The current playhead position, in beats.
    pub playhead_beats: Beats,

    /// The latest playhead samples received from the audio thread.
    pub last_playhead: usize,

    /// An instant to track the last edited time for project updating.
    pub last_edit_time: Option<Instant>,

    /// An ID of the currently selected region.
    pub selected_region: Option<(TrackID, RegionID)>,
}

impl EditorUIState {
    /// Set the selected region to the given one, deselecting the note.
    pub fn set_selected_region(&mut self, track_id: TrackID, region_id: RegionID) {
        self.selected_region = Some((track_id, region_id));
        // Deselect the note
        self.piano_roll_state.selected_note = None;
    }

    /// Set the selected note to the given one.
    pub fn set_selected_note(&mut self, note_id: NoteID) {
        self.piano_roll_state.selected_note = Some(note_id)
    }
}
