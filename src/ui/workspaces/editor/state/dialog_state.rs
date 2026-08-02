use crate::core::metadata::TrackType;

#[derive(Default)]
pub(crate) enum DialogState {
    #[default]
    None,
    AddTrack(AddTrackState),
}

pub struct AddTrackState {
    pub selected_track_type: TrackType,
    pub name: String,
}
