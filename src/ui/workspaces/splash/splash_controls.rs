use crate::{
    core::{metadata::ProjectMeta, project_setup::setup_project},
    fonts::RichTextExt,
    ui::{
        components::card_button::card_button,
        workspaces::{EditorTransition, EditorUi, SplashUi},
    },
};
use eframe::egui;
use kadent_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
};
use std::path::PathBuf;

const BUTTON_WIDTH: f32 = 300.0;
const BUTTON_HEIGHT: f32 = 42.0;
const CONTENT_HEIGHT: f32 = 60.0 + 12.0 + 16.0 + 24.0 + 12.0 + BUTTON_HEIGHT * 2.0;

impl SplashUi {
    pub(super) fn splash_controls(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        ui.vertical_centered(|ui| {
            let full_width = ui.available_width();
            let full_height = ui.available_height();

            ui.add_space(full_height / 2.0 - CONTENT_HEIGHT / 2.0);

            ui.add(
                egui::Image::new(egui::include_image!(
                    "../../../../assets/logo/kadent_logo_white_on_black_plate.png"
                ))
                .max_height(60.0),
            );
            ui.add_space(12.0);

            ui.add_sized(
                egui::vec2(full_width, 16.0),
                egui::Label::new(egui::RichText::new(&self.version_string).bold().weak()),
            );
            ui.add_space(24.0);

            let button_size = egui::vec2(BUTTON_WIDTH, BUTTON_HEIGHT);

            let new_project =
                card_button(ui, ui.id().with("New Project"), Some(button_size), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("New Project")
                                .strong()
                                .bold()
                                .size(14.0),
                        );
                        ui.label(egui::RichText::new("Create a fresh .kdp project.").weak());
                    });
                });
            ui.add_space(12.0);

            let open_project =
                card_button(ui, ui.id().with("Open Project"), Some(button_size), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("Open Project")
                                .strong()
                                .bold()
                                .size(14.0),
                        );
                        ui.label(
                            egui::RichText::new("Open an existing project from the disk.").weak(),
                        );
                    });
                });

            if new_project.clicked()
                && let Some(project_dir) = rfd::FileDialog::new().save_file()
            {
                return Some(self.create_new_project(project_dir));
            }

            if open_project.clicked()
                && let Some(project_dir) = rfd::FileDialog::new().pick_folder()
            {
                return self.open_project(project_dir);
            }

            None
        })
        .inner
    }

    fn create_new_project(&self, project_dir: PathBuf) -> EditorTransition {
        let audio_ctx = AudioContext {
            channels: 2,
            sample_rate: 48000,
            buffer_size: 512,
            max_voices: 32,
        };
        let mut project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
        let mut project_meta = ProjectMeta {
            kasl_search_paths: EditorUi::system_kasl_search_paths(),
            ..Default::default()
        };
        setup_project(&project_dir, &mut project, &mut project_meta, &audio_ctx);
        EditorTransition {
            project_dir,
            audio_ctx,
            project,
            project_meta,
        }
    }
}
