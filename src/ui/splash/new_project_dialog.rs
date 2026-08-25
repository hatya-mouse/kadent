use crate::{
    consts::PROJECT_FILE_EXTENSION,
    core::project_ctx::ProjectContext,
    fonts::RichTextExt,
    storage::{app_state::add_and_store_recent_projects, project::create_new_project},
    ui::{
        SplashUi,
        components::{
            dialog::{dialog, dialog_bold_label},
            text_button::{text_button, text_button_enabled},
            text_input::text_input,
        },
    },
};
use eframe::egui;

impl SplashUi {
    pub(super) fn new_project_dialog(&mut self, ui: &mut egui::Ui) -> Option<ProjectContext> {
        let mut dialog_state = self.splash_state.new_project_state.take()?;

        let mut should_close = false;
        let modal = dialog(ui, "Create Project", |ui| {
            dialog_bold_label(ui, "Project Name");
            text_input(ui, &mut dialog_state.project_name);

            dialog_bold_label(ui, "Project Folder");
            ui.label(
                egui::RichText::new(
                    dialog_state
                        .project_dir
                        .as_ref()
                        .map_or("No folder selected".to_string(), |path| {
                            path.to_string_lossy().to_string()
                        }),
                )
                .strong(),
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
                text_button_enabled(
                    can_create,
                    true,
                    ui,
                    "create_project",
                    egui::RichText::new("Create Project").bold(),
                )
                .clicked()
                .then(|| {
                    if let Some(parent_dir) = dialog_state.project_dir.clone() {
                        should_close = true;
                        let root_path = parent_dir.join(&dialog_state.project_name);
                        let project_path = root_path
                            .join(&dialog_state.project_name)
                            .with_added_extension(PROJECT_FILE_EXTENSION);
                        add_and_store_recent_projects(&project_path);
                        create_new_project(&dialog_state.project_name, root_path).ok()
                    } else {
                        None
                    }
                })
                .flatten()
            })
        });

        if should_close || modal.should_close() {
            self.splash_state.new_project_state = None;
        } else {
            self.splash_state.new_project_state = Some(dialog_state);
        }

        modal.inner.inner
    }
}
