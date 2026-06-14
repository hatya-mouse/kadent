use crate::{
    storage::project::create_new_project,
    ui::{
        components::{
            dialog::dialog,
            text_button::{text_button, text_button_enabled},
            text_input::text_input,
        },
        workspaces::{EditorTransition, SplashUi},
    },
};
use eframe::egui;

impl SplashUi {
    pub(super) fn new_project_dialog(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let dialog_state = self.splash_state.new_project_state.as_mut()?;

        let mut should_close = false;
        let modal = dialog(ui, "Create Project", |ui| {
            ui.label("Project Name");
            text_input(ui, &mut dialog_state.project_name);

            ui.label("Project Folder");
            ui.label(
                dialog_state
                    .project_dir
                    .as_ref()
                    .map_or("No folder selected".to_string(), |path| {
                        path.to_string_lossy().to_string()
                    }),
            );
            text_button(ui, "select_folder", "Select Folder")
                .clicked()
                .then(|| {
                    if let Some(project_dir) = rfd::FileDialog::new().pick_folder() {
                        dialog_state.project_dir = Some(project_dir);
                    }
                });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if text_button(ui, "cancel_project_creation", "Cancel").clicked() {
                    should_close = true;
                }

                let can_create = !dialog_state.project_name.trim().is_empty()
                    && dialog_state
                        .project_dir
                        .as_ref()
                        .is_some_and(|path| path.is_dir());
                text_button_enabled(can_create, ui, "create_project", "Create Project")
                    .clicked()
                    .then(|| {
                        if let Some(project_dir) = dialog_state.project_dir.clone() {
                            should_close = true;
                            create_new_project(&dialog_state.project_name, project_dir).ok()
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
        });

        if should_close || modal.should_close() {
            self.splash_state.new_project_state = None;
        }

        modal.inner.inner
    }
}
