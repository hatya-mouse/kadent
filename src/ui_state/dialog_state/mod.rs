mod add_track;

pub use add_track::{AddTrackState, TrackType};

pub enum DialogState {
    None,
    AddTrack(AddTrackState),
}
