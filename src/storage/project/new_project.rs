use crate::{
    consts::PROJECT_FILE_EXTENSION,
    core::metadata::ProjectMeta,
    storage::project::save_project,
    ui::workspaces::{EditorTransition, EditorUi},
};
use kadent_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
};
use std::{io, path::PathBuf};

pub(crate) fn create_new_project(
    project_name: &str,
    parent_path: PathBuf,
) -> io::Result<EditorTransition> {
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
        channels: 2,
        sample_rate: 44100,
        buffer_size: 512,
        max_voices: 32,
    };
    let project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
    let project_meta = ProjectMeta {
        kasl_search_paths: EditorUi::system_kasl_search_paths(),
        ..Default::default()
    };
    save_project(&project_path, &project, &project_meta)?;

    Ok(EditorTransition {
        project_path,
        audio_ctx,
        project,
        project_meta,
    })
}
