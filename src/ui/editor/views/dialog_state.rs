use crate::core::metadata::TrackType;

#[derive(Default)]
pub(crate) enum DialogState {
    #[default]
    None,
    AddTrack(AddTrackState),
}

pub(crate) struct AddTrackState {
    pub(crate) selected_track_type: TrackType,
    pub(crate) name: String,
}
