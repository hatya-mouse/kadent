use crate::{metadata::ProjectMeta, ui_state::KnodiqUIState};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use knodiq_engine::{
    audio_thread::{AudioThread, AudioThreadHandle, error::AudioError},
    data_types::{AudioContext, Beats},
    mixer::Project,
};
use std::sync::Arc;

pub struct KnodiqApp {
    /// A master source of the project.
    pub project: Project,
    /// Whether the audio is playing.
    pub is_playing: bool,
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
    /// The metadata of the project.
    pub project_meta: ProjectMeta,
    /// UI states to store the current UI state.
    pub ui_state: KnodiqUIState,
}

impl KnodiqApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Install the image loader
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // UI Setup
        Self::setup_fonts(&cc.egui_ctx);
        Self::base_style(&cc.egui_ctx);

        // Create a new project and spawn an audio thread
        let audio_ctx = AudioContext {
            channels: 2,
            sample_rate: 48000,
            buffer_size: 512,
            max_voices: 32,
        };
        let project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
        let thread_handle = AudioThread::spawn(audio_ctx, project.clone());

        Self {
            project,
            is_playing: false,
            thread_handle,
            errors: Vec::new(),
            project_meta: ProjectMeta::new("Project".to_string()),
            ui_state: KnodiqUIState::default(),
        }
    }

    fn setup_fonts(ctx: &egui::Context) {
        // Set up the fonts
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Inter".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/Inter.ttf"
            ))),
        );
        fonts.font_data.insert(
            "NotoSansJP".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/fonts/NotoSansJP.ttf"
            ))),
        );

        // Set the font data
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "Inter".to_owned());
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("NotoSansJP".to_owned());

        // Set the fonts to the specified fonts
        ctx.set_fonts(fonts);
    }

    fn base_style(ctx: &egui::Context) {
        ctx.style_mut(|style| {
            // Make labels unselectable by default
            style.interaction.selectable_labels = false;

            // Remove window shadows
            style.visuals.window_shadow = egui::Shadow::NONE;
        });
    }
}
