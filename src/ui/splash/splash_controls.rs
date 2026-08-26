use crate::{
    core::project_ctx::ProjectContext,
    fonts::RichTextExt,
    storage::{app_state::AppPreferences, project::open_project_to_ctx},
    ui::{
        SplashUi,
        components::card_button::card_button,
        splash::{
            install_std_dialog::InstallStdLibDialogState,
            new_project_dialog::NewProjectDialogState, state::SplashDialogState,
        },
        theme,
    },
};
use eframe::egui;

const BUTTON_WIDTH: f32 = 300.0;
const BUTTON_HEIGHT: f32 = 44.0;
const CONTENT_HEIGHT: f32 = 60.0 + 12.0 + 16.0 + 24.0 + 12.0 + BUTTON_HEIGHT * 2.0;

impl SplashUi {
    pub(super) fn splash_controls(
        &mut self,
        ui: &mut egui::Ui,
        preferences: &AppPreferences,
    ) -> Option<ProjectContext> {
        ui.vertical_centered(|ui| {
            let full_width = ui.available_width();
            let full_height = ui.available_height();

            ui.add_space(full_height * 0.5 - CONTENT_HEIGHT * 0.5);

            let logo_image = egui::Image::new(if ui.visuals().dark_mode {
                egui::include_image!("../../../assets/logo/kadent_logo_black_on_white.png")
            } else {
                egui::include_image!("../../../assets/logo/kadent_logo_white_on_black.png")
            });
            ui.add(logo_image.max_height(60.0));
            ui.add_space(12.0);

            ui.add_sized(
                egui::vec2(full_width, 16.0),
                egui::Label::new(egui::RichText::new(&self.version_string).bold().weak()),
            );
            ui.add_space(24.0);

            let button_size = egui::vec2(BUTTON_WIDTH, BUTTON_HEIGHT);

            // --- NEW PROJECT BUTTON ---
            let new_project_res =
                card_button(ui, ui.id().with("new-project"), Some(button_size), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("New Project")
                                .strong()
                                .bold()
                                .size(theme::large_font_size()),
                        );
                        ui.label(egui::RichText::new("Create a fresh .kdp project.").weak());
                    });
                });
            if new_project_res.clicked() {
                self.splash_state.dialog =
                    SplashDialogState::NewProject(NewProjectDialogState::default());
            }

            ui.add_space(12.0);

            // --- OPEN PROJECT BUTTON ---
            let open_project_res =
                card_button(ui, ui.id().with("open-project"), Some(button_size), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Open Project")
                                .strong()
                                .bold()
                                .size(theme::large_font_size()),
                        );
                        ui.label(
                            egui::RichText::new("Open an existing project from the disk.").weak(),
                        );
                    });
                });
            if open_project_res.clicked()
                && let Some(project_dir) = rfd::FileDialog::new().pick_file()
            {
                return open_project_to_ctx(project_dir, preferences);
            }

            // --- KASL STANDARD LIBRARY INSTALLATION ---
            let install_std_res = card_button(
                ui,
                ui.id().with("install-kasl-std"),
                Some(button_size),
                |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Install KASL Standard Library")
                                .strong()
                                .bold()
                                .size(theme::large_font_size()),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Install the KASL standard library to your system.",
                            )
                            .weak(),
                        );
                    });
                },
            );
            if install_std_res.clicked() {
                self.splash_state.dialog =
                    SplashDialogState::InstallStdLib(InstallStdLibDialogState::default());
            }

            None
        })
        .inner
    }
}
