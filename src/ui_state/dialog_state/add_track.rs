use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum TrackType {
    AudioTrack,
    NoteTrack,
}

impl Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AudioTrack => write!(f, "Audio Track"),
            Self::NoteTrack => write!(f, "Note Track"),
        }
    }
}

pub struct AddTrackState {
    pub selected_track_type: TrackType,
    pub name: String,
}
