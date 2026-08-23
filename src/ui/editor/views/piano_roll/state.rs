use crate::core::audio_engine::data_types::Ticks;
use std::time::Instant;

#[derive(Default)]
pub(crate) struct PianoRollState {
    /// MIDI note numbers and Instants for currently playing preview notes.
    pub(crate) preview_notes: Vec<(u8, Instant)>,
    /// Length of the last edited note.
    pub(crate) last_edited_note_length: Option<Ticks>,
}
