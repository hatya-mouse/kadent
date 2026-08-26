use crate::{
    core::project_ctx::ProjectContext, storage::app_state::AppPreferences,
    ui::splash::state::SplashDialogState,
};
use eframe::egui;

impl SplashDialogState {
    pub(super) fn show_dialog(
        &mut self,
        ui: &mut egui::Ui,
        preferences: &mut AppPreferences,
    ) -> Option<ProjectContext> {
        match self {
            SplashDialogState::None => None,
            SplashDialogState::NewProject(_) => self.new_project_dialog(ui),
            SplashDialogState::InstallStdLib(_) => {
                self.install_std_lib_dialog(ui, preferences);
                None
            }
        }
    }
}
