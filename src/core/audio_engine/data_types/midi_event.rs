#[derive(Clone, Debug)]
pub(crate) enum MidiEvent {
    NoteOn { pitch: u8, velocity: u8 },
    NoteOff { pitch: u8 },
}
