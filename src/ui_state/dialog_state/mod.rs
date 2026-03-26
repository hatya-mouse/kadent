mod add_track;

pub use add_track::{AddTrackState, TrackType};

pub enum DialogState {
    None,
    AddTrack(AddTrackState),
}

impl Default for DialogState {
    fn default() -> Self {
        Self::None
    }
}
