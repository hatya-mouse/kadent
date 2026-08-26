use crate::{core::project_ctx::ProjectContext, ui::splash::state::SplashDialogState};

impl SplashDialogState {
    pub(super) fn show_dialog(&mut self, ui: &mut eframe::egui::Ui) -> Option<ProjectContext> {
        match self {
            SplashDialogState::None => None,
            SplashDialogState::NewProject(_) => self.new_project_dialog(ui),
            SplashDialogState::InstallStdLib(_) => {
                self.install_std_lib_dialog(ui);
                None
            }
        }
    }
}
