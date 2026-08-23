use crate::core::audio_engine::data_types::MidiEvent;

#[derive(Debug, Clone)]
pub(super) enum VoiceEventKind {
    NoteOn { pitch: f32, velocity: f32 },
    NoteOff { pitch: f32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum VoiceSource {
    Sequenced,
    Midi,
}

#[derive(Debug, Clone)]
pub(super) struct VoiceEvent {
    /// The time when the event is to be emitted in samples.
    pub(crate) sample_time: usize,
    /// The kind of the voice.
    pub(crate) kind: VoiceEventKind,
    /// Source of the voice event.
    pub(crate) source: VoiceSource,
}

impl VoiceEvent {
    pub(crate) fn new(sample_time: usize, kind: VoiceEventKind, source: VoiceSource) -> Self {
        Self {
            sample_time,
            kind,
            source,
        }
    }

    pub(crate) fn from_midi_event(sample_time: usize, midi_event: MidiEvent) -> Self {
        match midi_event {
            MidiEvent::NoteOn { pitch, velocity } => Self {
                sample_time,
                kind: VoiceEventKind::NoteOn {
                    pitch: pitch as f32,
                    velocity: velocity as f32 / 256f32,
                },
                source: VoiceSource::Midi,
            },
            MidiEvent::NoteOff { pitch } => Self {
                sample_time,
                kind: VoiceEventKind::NoteOff {
                    pitch: pitch as f32,
                },
                source: VoiceSource::Midi,
            },
        }
    }
}

// Implement Eq and Ord to use VoiceEvent in a BinaryHeap

impl PartialEq for VoiceEvent {
    fn eq(&self, other: &Self) -> bool {
        self.sample_time == other.sample_time
    }
}

impl Eq for VoiceEvent {}

impl PartialOrd for VoiceEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VoiceEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.sample_time < other.sample_time {
            std::cmp::Ordering::Less
        } else if self.sample_time == other.sample_time {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
