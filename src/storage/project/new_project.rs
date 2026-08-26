use crate::{
    consts::{DEFAULT_BUFFER_SIZE, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE, PROJECT_FILE_EXTENSION},
    core::{metadata::ProjectMeta, project_ctx::ProjectContext},
    storage::project::save_project,
};
use crate::{
    core::audio_engine::{
        data_types::{AudioContext, PlaybackContext, Ticks},
        mixer::ProjectData,
        timing::TimeBounds,
    },
    storage::project::error::SaveError,
};
use std::path::PathBuf;

pub(crate) fn create_new_project(
    project_name: &str,
    root_path: PathBuf,
) -> Result<ProjectContext, SaveError> {
    // 1. Generate paths for each subdirectories
    let src_dir = root_path.join("src");
    let assets_dir = root_path.join("assets");
    let project_path = root_path
        .join(project_name)
        .with_added_extension(PROJECT_FILE_EXTENSION);

    // 2. Create folders and files
    std::fs::create_dir_all(&src_dir).map_err(SaveError::IoError)?;
    std::fs::create_dir_all(&assets_dir).map_err(SaveError::IoError)?;

    // 3. Create an empty project file
    let audio_ctx = AudioContext { resolution: 480 };
    let export_ctx = PlaybackContext {
        channels: DEFAULT_CHANNELS,
        sample_rate: DEFAULT_SAMPLE_RATE,
        buffer_size: DEFAULT_BUFFER_SIZE,
    };

    let project = ProjectData::new(
        audio_ctx.clone(),
        120.0,
        TimeBounds::Musical {
            start: Ticks::ZERO,
            duration: Ticks(3840),
        },
    );
    let project_meta = ProjectMeta {
        export_ctx,
        ..Default::default()
    };
    save_project(&project_path, &project, &project_meta)?;

    Ok(ProjectContext::new(project_path, project, project_meta))
}
