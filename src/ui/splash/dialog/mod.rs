mod install_std_dialog;
mod license_dialog;
mod new_project_dialog;

pub(super) use install_std_dialog::InstallStdLibDialogState;
pub(super) use license_dialog::LicenseDialogState;
pub(super) use new_project_dialog::NewProjectDialogState;

use crate::{core::project_ctx::ProjectContext, storage::app_state::AppPreferences};
use eframe::egui;

#[derive(Default)]
pub(super) enum SplashDialogState {
    /// No dialog is open.
    #[default]
    None,
    /// The new project dialog is open.
    NewProject(NewProjectDialogState),
    /// The install stdlib dialog is open.
    InstallStdLib(InstallStdLibDialogState),
    /// The license dialog is open.
    License(LicenseDialogState),
}

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
            SplashDialogState::License(_) => {
                self.license_dialog(ui);
                None
            }
        }
    }
}
