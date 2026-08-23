use crate::spawn_background_init;
use crate::storage::app_state::load_recent_projects;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub(crate) struct RecentProjData {
    pub(crate) name: String,
    pub(crate) path_str: String,
    pub(crate) path: PathBuf,
}

pub(super) struct NewProjectDialogState {
    /// The name of the new project.
    pub(crate) project_name: String,
    /// The directory where the new project will be created.
    pub(crate) project_dir: Option<PathBuf>,
}

impl Default for NewProjectDialogState {
    fn default() -> Self {
        Self {
            project_name: "Project".to_string(),
            project_dir: None,
        }
    }
}

pub(super) struct SplashUiState {
    /// Recently opened projects.
    pub(crate) recent_projects: Arc<Mutex<Vec<RecentProjData>>>,
    /// Whether to show the new track dialog.
    pub(crate) new_project_state: Option<NewProjectDialogState>,
}

impl Default for SplashUiState {
    fn default() -> Self {
        Self {
            recent_projects: spawn_background_init!({ load_recent_projects() }),
            new_project_state: None,
        }
    }
}
