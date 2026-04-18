use crate::{app::KnodiqApp, metadata::ProjectMeta};
use eframe::egui;
use knodiq_engine::{
    data_types::{AudioContext, Beats},
    mixer::Project,
};

impl KnodiqApp {
    pub(in crate::ui) fn splash_screen(&mut self, ui: &mut egui::Ui) {}

    fn create_project(&self) -> (Project, ProjectMeta) {
        // Create a new project and spawn an audio thread
        let audio_ctx = AudioContext {
            channels: 2,
            sample_rate: 48000,
            buffer_size: 512,
            max_voices: 32,
        };
        let mut project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
        let mut project_meta = ProjectMeta {
            kasl_search_paths: Self::system_kasl_search_paths(),
            ..Default::default()
        };

        (project, project_meta)
    }
}
