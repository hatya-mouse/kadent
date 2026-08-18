use crate::ui::editor::TimelineCoord;
use kadent_engine::data_types::Ticks;
use std::{collections::HashMap, time::Instant};
use uuid::Uuid;

#[derive(Default)]
pub(crate) struct PianoRollState {
    /// MIDI note numbers and Instants for currently playing preview notes.
    pub preview_notes: Vec<(u8, Instant)>,
    /// Length of the last edited note.
    pub last_edited_note_length: Option<Ticks>,
    /// Timeline coordinates of the panel.
    pub timeline_coords: HashMap<Uuid, TimelineCoord>,
}

impl PianoRollState {
    pub(crate) fn remove_panel_state(&mut self, panel_id: &Uuid) {
        self.timeline_coords.remove(panel_id);
    }
}
