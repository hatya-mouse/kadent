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
    CloseCodeBuffer {
        panel_id: Uuid,
    },
    RenameFile {
        path: PathBuf,
        new_name: String,
    },
}

pub(crate) enum DialogType {
    AddTrack,
    ChangeCodeBuffer { panel_id: Uuid, path: PathBuf },
    CloseCodeBuffer { panel_id: Uuid },
    RenameFile { path: PathBuf },
}
