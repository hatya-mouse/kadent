mod add_track;

pub use add_track::{AddTrackState, TrackType};

#[derive(Default)]
pub enum DialogState {
    #[default]
    None,
    AddTrack(AddTrackState),
}
