use crate::{
    core::{metadata::ProjectMeta, project_setup::setup_project},
    ui::workspaces::{EditorTransition, EditorUi, SplashUi},
};
use eframe::egui;
use kadent_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
};
use std::path::PathBuf;

impl SplashUi {
    pub(super) fn splash_controls(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("Kadent");
            ui.add_space(16.0);
            if ui.button("New Project").clicked()
                && let Some(project_dir) = rfd::FileDialog::new().save_file()
            {
                return Some(self.create_new_project(project_dir));
            }
            if ui.button("Open Project").clicked()
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
