use crate::{
    consts::PROJECT_FILE_EXTENSION,
    core::{
        metadata::ProjectMeta,
        project_ctx::{EditorContext, ProjectContext},
    },
    storage::project::save_project,
    ui::workspaces::EditorUi,
};
use kadent_engine::{
    data_types::{AudioContext, Ticks},
    mixer::Project,
};
use std::{io, path::PathBuf};

pub(crate) fn create_new_project(
    project_name: &str,
    parent_path: PathBuf,
) -> io::Result<EditorContext> {
    // 1. Generate paths for each subdirectories
    let root_path = parent_path.join(project_name);
    let src_dir = root_path.join("src");
    let assets_dir = root_path.join("assets");
    let project_path = root_path
        .join(project_name)
        .with_added_extension(PROJECT_FILE_EXTENSION);

    // 2. Create folders and files
    std::fs::create_dir_all(&src_dir)?;
    std::fs::create_dir_all(&assets_dir)?;

    // 3. Create an empty project file
    let audio_ctx = AudioContext {
        resolution: 480,
        channels: 2,
        sample_rate: 48000,
        buffer_size: 512,
        max_voices: 32,
    };
    let project = Project::new(audio_ctx.clone(), 120.0, Ticks(3840), Ticks(3840));
    let project_meta = ProjectMeta {
        kasl_search_paths: EditorUi::system_kasl_search_paths(),
        ..Default::default()
    };
    save_project(&project_path, &project, &project_meta)?;

    Ok(EditorContext::new(
        ProjectContext::new(project_path, project, project_meta),
        audio_ctx,
    ))
}
