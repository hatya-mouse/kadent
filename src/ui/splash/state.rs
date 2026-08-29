use crate::spawn_background_init;
use crate::storage::app_state::load_recent_projects;
use crate::ui::splash::dialog::SplashDialogState;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub(crate) struct RecentProjData {
    pub(crate) name: String,
    pub(crate) path_str: String,
    pub(crate) path: PathBuf,
}

pub(super) struct SplashUiState {
    /// Recently opened projects.
    pub(crate) recent_projects: Arc<Mutex<Vec<RecentProjData>>>,
    /// New track dialog state.
    pub(crate) dialog: SplashDialogState,
}

impl Default for SplashUiState {
    fn default() -> Self {
        Self {
            recent_projects: spawn_background_init!({ load_recent_projects() }),
            dialog: SplashDialogState::None,
        }
    }
}
