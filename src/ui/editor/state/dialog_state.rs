use crate::core::metadata::TrackType;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Default)]
pub(crate) enum DialogState {
    #[default]
    None,
    AddTrack {
        selected_track_type: TrackType,
        name: String,
    },
    ChangeCodeBuffer {
        panel_id: Uuid,
        path: PathBuf,
    },
}

pub(crate) enum DialogType {
    AddTrack,
    ChangeCodeBuffer { panel_id: Uuid, path: PathBuf },
}
