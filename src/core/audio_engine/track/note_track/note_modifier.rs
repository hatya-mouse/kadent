use crate::core::audio_engine::track::note_track::Note;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct NoteModifierID(Uuid);

pub(crate) trait NoteModifier: Send + Debug {
    /// Clones the modifier.
    fn clone_box(&self) -> Box<dyn NoteModifier>;

    /// Processes the Note using the modifier.
    fn process(&mut self, input_notes: Vec<Note>) -> Vec<Note>;
}

impl Clone for Box<dyn NoteModifier> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
