use kadent_engine::data_types::Ticks;
use std::time::Instant;

pub(crate) struct PianoRollState {
    /// Pixels per beat in the piano roll.
    pub pixels_per_beat: f32,
    /// The height of the each note in the piano roll.
    pub note_height: f32,
    /// MIDI note numbers and Instants for currently playing preview notes.
    pub preview_notes: Vec<(u8, Instant)>,
    /// Length of the last edited note.
    pub last_edited_note_length: Option<Ticks>,
}

impl Default for PianoRollState {
    fn default() -> Self {
        Self {
            pixels_per_beat: 80.0,
            note_height: 10.0,
            preview_notes: Vec::new(),
            last_edited_note_length: None,
        }
    }
}
