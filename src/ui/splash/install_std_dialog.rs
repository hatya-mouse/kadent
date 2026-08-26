use crate::{
    core::stdlib_installer::{default_kasl_lib_directory, install_kasl_stdlib},
    fonts::RichTextExt,
    storage::app_state::{AppPreferences, save_preferences},
    ui::{
        components::{
            dialog::{dialog, dialog_bold_label},
            text_button::{text_button, text_button_enabled},
        },
        splash::state::SplashDialogState,
    },
};
use eframe::egui;
use std::path::PathBuf;

pub(super) struct InstallStdLibDialogState {
    /// The error message to display if the installation fails.
    pub(super) error_message: Option<String>,
    /// The directory where the KASL standard library will be installed.
    pub(super) install_dir: Option<PathBuf>,
}

impl Default for InstallStdLibDialogState {
    fn default() -> Self {
        let install_dir = default_kasl_lib_directory();
        Self {
            error_message: None,
            install_dir,
        }
    }
}

impl SplashDialogState {
    pub(super) fn install_std_lib_dialog(
        &mut self,
        ui: &mut egui::Ui,
        preferences: &mut AppPreferences,
    ) {
        let SplashDialogState::InstallStdLib(dialog_state) = self else {
            return;
        };

        let mut should_close = false;
        let modal = dialog(ui, "Install KASL Standard Library", |ui| {
            if let Some(error_message) = &dialog_state.error_message {
                dialog_bold_label(ui, "Could not install KASL standard library");
                ui.colored_label(egui::Color32::RED, error_message);
            }

            dialog_bold_label(ui, "Install Folder");
            ui.label(
                dialog_state
                    .install_dir
                    .as_ref()
                    .map_or("No Folder Selected".to_string(), |path| {
                        path.to_string_lossy().to_string()
                    }),
            );
            if text_button(ui, "select_folder", "Select Folder").clicked() {
                let dialog = dialog_state
                    .install_dir
                    .as_ref()
                    .map_or_else(rfd::FileDialog::new, |install_dir| {
                        rfd::FileDialog::new().set_directory(install_dir)
                    });
                if let Some(install_dir) = dialog.pick_folder() {
                    dialog_state.install_dir = Some(install_dir);
                }
            }

            ui.horizontal(|ui| {
                if text_button(ui, "cancel_install", "Cancel").clicked() {
                    should_close = true;
                }

                let can_install = dialog_state.install_dir.is_some();
                if text_button_enabled(
                    can_install,
                    true,
                    ui,
                    "install",
                    egui::RichText::new("Install").bold(),
                )
                .clicked()
                    && let Some(install_dir) = &dialog_state.install_dir
                {
                    match install_kasl_stdlib(install_dir) {
                        Err(e) => {
                            dialog_state.error_message = Some(e.to_string());
                        }
                        Ok(()) => {
                            preferences.kasl_std_path =
                                install_dir.clone().into_os_string().into_string().ok();
                            match save_preferences(preferences) {
                                Err(e) => {
                                    dialog_state.error_message = Some(e.to_string());
                                }
                                Ok(()) => {
                                    should_close = true;
                                }
                            }
                        }
                    }
                }
            })
        });

        if should_close || modal.should_close() {
            *self = SplashDialogState::None;
        }
    }
}
