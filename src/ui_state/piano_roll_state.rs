use knodiq_engine::track::note_track::NoteID;

pub struct PianoRollState {
    /// Pixels per beat in the piano roll.
    pub pixels_per_beat: f32,

    /// The height of the each note in the piano roll.
    pub note_height: f32,

    /// An ID of the currently selected note.
    pub selected_note: Option<NoteID>,
}

impl Default for PianoRollState {
    fn default() -> Self {
        Self {
            pixels_per_beat: 80.0,
            note_height: 10.0,
            selected_note: None,
        }
    }
}
