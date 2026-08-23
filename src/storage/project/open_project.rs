use crate::{
    core::{metadata::ProjectMeta, project_ctx::ProjectContext},
    storage::{
        app_state::add_and_store_recent_projects,
        project::{get_project_dir, init::init_nodes, load_project},
    },
};
use std::path::PathBuf;

/// Opens the project file at the given path, returning the transition data if successful.
pub(crate) fn open_project_to_ctx(project_path: PathBuf) -> Option<ProjectContext> {
    // Store the project to recent projects
    add_and_store_recent_projects(&project_path);

    // Load the project and pass the data to the editor UI
    match load_project(&project_path) {
        Ok(mut decodable_proj) => {
            let project_meta = ProjectMeta::from_loaded_meta(decodable_proj.meta);
            let project_dir = get_project_dir(&project_path);
            init_nodes(
                &mut decodable_proj.data,
                &project_meta.kasl_search_paths,
                &project_dir,
                &project_meta.export_ctx,
            );

            Some(ProjectContext::new(
                project_path,
                decodable_proj.data,
                project_meta,
            ))
        }
        Err(e) => {
            eprintln!("Failed to load project: {:?}", e);
            None
        }
    }
}
