use crate::{
    core::{
        metadata::ProjectMeta,
        project_ctx::{EditorContext, ProjectContext},
    },
    storage::{
        app_state::add_and_store_recent_projects,
        project::{get_project_dir, init_kasl_nodes, load_project},
    },
};
use std::path::PathBuf;

/// Opens the project file at the given path, returning the transition data if successful.
pub(crate) fn open_project_to_ctx(project_path: PathBuf) -> Option<EditorContext> {
    // Store the project to recent projects
    add_and_store_recent_projects(&project_path);

    // Load the project and pass the data to the editor UI
    match load_project(&project_path) {
        Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
            Ok(project_meta) => {
                let project_dir = get_project_dir(&project_path);
                init_kasl_nodes(
                    &mut proj_res.project,
                    &project_meta.kasl_search_paths,
                    &project_dir,
                );

                let audio_ctx = proj_res.project.audio_ctx.clone();
                println!("Project loaded successfully: {:?}", audio_ctx);
                Some(EditorContext::new(
                    ProjectContext::new(project_path, proj_res.project, project_meta),
                    audio_ctx,
                ))
            }
            Err(e) => {
                eprintln!("Failed to extract project metadata: {:?}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("Failed to load project: {:?}", e);
            None
        }
    }
}
