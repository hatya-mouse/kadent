use crate::core::audio_engine::{data_types::Ticks, track::note_track::NoteModifierID};
use std::collections::HashSet;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct NoteID(pub(crate) u64);

impl From<NoteID> for u64 {
    fn from(value: NoteID) -> u64 {
        value.0
    }
}

impl From<u64> for NoteID {
    fn from(value: u64) -> Self {
        NoteID(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Note {
    /// Relative start position in the region in ticks.
    pub(crate) start: Ticks,
    /// Duration of the note in ticks.
    pub(crate) duration: Ticks,
    /// Frequency of the note.
    pub(crate) pitch: f32,
    /// Velocity of the note.
    pub(crate) velocity: f32,
    /// IDs of the applied modifiers for the note.
    /// Modifiers will be applied in the order set in the `NoteTrack`.
    pub(crate) modifiers: HashSet<NoteModifierID>,
}

impl Note {
    pub(crate) fn new(start: Ticks, duration: Ticks, pitch: f32, velocity: f32) -> Self {
        Self {
            start,
            duration,
            pitch,
            velocity,
            modifiers: HashSet::new(),
        }
    }
}
