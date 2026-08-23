use crate::core::audio_engine::data_types::Ticks;

/// Processed note generated from sequenced note data.
/// Should only be used for processing the note data in the `NoteTrack`.
#[derive(Clone, Debug)]
pub(super) struct ProcessedNote {
    /// Absolute start position in ticks.
    /// This is used to sort the notes in the `NoteTrack`.
    pub(crate) start: Ticks,
    /// Duration of the note in ticks.
    pub(crate) duration: Ticks,
    /// Frequency of the note.
    pub(crate) pitch: f32,
    /// Velocity of the note.
    pub(crate) velocity: f32,
}
