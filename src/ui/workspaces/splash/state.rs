use crate::spawn_background_init;
use std::path::PathBuf;

pub(crate) struct RecentProjData {
    pub name: String,
    pub path_str: String,
    pub path: PathBuf,
}

pub(super) struct SplashUiState {
    /// Recently opened projects.
    pub recent_projects: Vec<RecentProjData>,
}

impl Default for SplashUiState {
    fn default() -> Self {
        Self {
            recent_projects: spawn_background_init!({ load_recent_projects() }),
        }
    }
}
