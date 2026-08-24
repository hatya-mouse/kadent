use crate::core::metadata::TrackType;

#[derive(Default)]
pub(crate) enum DialogState {
    #[default]
    None,
    AddTrack {
        selected_track_type: TrackType,
        name: String,
    },
}

pub(crate) enum DialogType {
    AddTrack,
}
