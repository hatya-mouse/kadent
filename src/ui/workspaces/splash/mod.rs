mod project_list;
mod splash_controls;
pub(crate) mod state;

use crate::{
    consts::RECENT_PROJCETS_MAX_NUM,
    core::metadata::ProjectMeta,
    storage::{
        app_state::save_recent_projects,
        project::{init_kasl_nodes, load_project_from_dir},
    },
    ui::workspaces::splash::state::SplashUiState,
};
use eframe::egui;
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::{Path, PathBuf};

/// The splash screen of Kadent.
#[derive(Default)]
pub struct SplashUi {
    /// The current splash UI state.
    splash_state: SplashUiState,
}

/// A struct that contains the data passed to the editor UI.
pub struct EditorTransition {
    pub project_dir: PathBuf,
    pub audio_ctx: AudioContext,
    pub project: Project,
    pub project_meta: ProjectMeta,
}

impl SplashUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let mut transition = None;

        ui.columns(2, |columns| {
            let controls_transition = self.splash_controls(&mut columns[0]);
            let list_transition = self.project_list(&mut columns[1]);

            transition = controls_transition.or(list_transition);
        });

        transition
    }

    /// Opens the project at the given directory, returning the transition data if successful.
    fn open_project(&self, project_dir: PathBuf) -> Option<EditorTransition> {
        // Store the project to recent projects
        self.add_and_store_recent_projects(&project_dir);

        // Load the project and pass the data to the editor UI
        match load_project_from_dir(&project_dir) {
            Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                Ok(project_meta) => {
                    init_kasl_nodes(
                        &mut proj_res.project,
                        &project_meta.kasl_search_paths,
                        &project_dir,
                    );

                    let audio_ctx = proj_res.project.audio_ctx.clone();
                    Some(EditorTransition {
                        project_dir,
                        audio_ctx,
                        project: proj_res.project,
                        project_meta,
                    })
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

    fn add_and_store_recent_projects(&self, project_path: &Path) {
        let mut project_paths: Vec<PathBuf> = self
            .splash_state
            .recent_projects
            .lock()
            .unwrap()
            .iter()
            .map(|proj| proj.path.clone())
            .collect();

        // Add the project at the first
        // If the same project already exists, remove the existing one and add new one at the first
        let project_path_buf = project_path.to_path_buf();
        if let Some(existing_index) = project_paths
            .iter()
            .position(|path_buf| path_buf == &project_path_buf)
        {
            project_paths.remove(existing_index);
        }
        project_paths.insert(0, project_path_buf);
        // Limit the number of recent projcets
        project_paths.truncate(RECENT_PROJCETS_MAX_NUM);
        project_paths.shrink_to_fit();

        // Save the paths to the disk
        save_recent_projects(&project_paths);
    }
}
