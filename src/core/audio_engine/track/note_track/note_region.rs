use crate::core::audio_engine::{
    data_types::Ticks,
    timing::TimeBounds,
    track::note_track::{Note, NoteID},
};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct NoteRegion {
    pub(crate) bounds: TimeBounds,
    pub(crate) notes: HashMap<NoteID, Note>,

    next_note_id: u64,
}

impl NoteRegion {
    // --- NEW ---

    /// Creates a new note region.
    pub(crate) fn new(bounds: TimeBounds) -> Self {
        Self {
            bounds,
            notes: HashMap::new(),
            next_note_id: 0,
        }
    }

    /// Creates a new note region with the given notes.
    pub(crate) fn with_notes(
        bounds: TimeBounds,
        notes: HashMap<NoteID, Note>,
        next_note_id: u64,
    ) -> Self {
        Self {
            bounds,
            notes,
            next_note_id,
        }
    }

    // --- NOTE ID GENERATION ---

    /// Generates a new note ID.
    fn generate_note_id(&mut self) -> NoteID {
        let id = NoteID(self.next_note_id);
        self.next_note_id += 1;
        id
    }

    // --- NOTE MANAGEMENT ---

    /// Adds a given note to the region.
    pub(crate) fn add_note(&mut self, note: Note) {
        let id = self.generate_note_id();
        self.notes.insert(id, note);
    }

    /// Removes the note from the region.
    pub(crate) fn remove_note(&mut self, id: &NoteID) {
        self.notes.remove(id);
    }

    // --- NOTE GETTING ---

    /// Returns a reference to the note.
    pub(crate) fn get_note(&self, id: &NoteID) -> Option<&Note> {
        self.notes.get(id)
    }

    /// Returns a mutable reference to the note.
    pub(crate) fn get_note_mut(&mut self, id: &NoteID) -> Option<&mut Note> {
        self.notes.get_mut(id)
    }

    // --- NOTE MODIFICATION ---

    /// Changes the note's start to the given start.
    pub(crate) fn set_start(&mut self, id: &NoteID, start: Ticks) {
        if let Some(note) = self.get_note_mut(id) {
            note.start = start;
        }
    }

    /// Sets the note's duration to the given duration.
    pub(crate) fn set_duration(&mut self, id: &NoteID, duration: Ticks) {
        if let Some(note) = self.get_note_mut(id) {
            note.duration = duration;
        }
    }

    /// Sets the note's pitch to the given pitch.
    pub(crate) fn set_pitch(&mut self, id: &NoteID, pitch: f32) {
        if let Some(note) = self.get_note_mut(id) {
            note.pitch = pitch;
        }
    }

    // --- NOTE DATA GETTING ---

    /// Returns the start beat of the note.
    pub(crate) fn get_start(&self, id: &NoteID) -> Option<Ticks> {
        self.get_note(id).map(|note| note.start)
    }

    /// Returns the duration of the note.
    pub(crate) fn get_duration(&self, id: &NoteID) -> Option<Ticks> {
        self.get_note(id).map(|note| note.duration)
    }

    /// Returns the pitch of the note.
    pub(crate) fn get_pitch(&self, id: &NoteID) -> Option<f32> {
        self.get_note(id).map(|note| note.pitch)
    }
}
