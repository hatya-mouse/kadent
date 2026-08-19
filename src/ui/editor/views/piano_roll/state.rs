use kadent_engine::data_types::Ticks;
use std::time::Instant;

#[derive(Default)]
pub(crate) struct PianoRollState {
    /// MIDI note numbers and Instants for currently playing preview notes.
    pub preview_notes: Vec<(u8, Instant)>,
    /// Length of the last edited note.
    pub last_edited_note_length: Option<Ticks>,
}
