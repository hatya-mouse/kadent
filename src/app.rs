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
}

impl KnodiqApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Install the image loader
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // UI Setup
        Self::setup_fonts(&cc.egui_ctx);
        Self::make_unselectable(&cc.egui_ctx);

        // Create a new project and spawn an audio thread
        let audio_ctx = AudioContext::default();
        let project = Project::new(audio_ctx.clone(), 120.0, Beats(0.0), Beats(8.0));
        let thread_handle = AudioThread::spawn(audio_ctx, project.clone());

        Self {
            project,
            is_playing: false,
            thread_handle,
            errors: Vec::new(),
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

    fn make_unselectable(ctx: &egui::Context) {
        ctx.style_mut(|style| {
            // Make labels unselectable by default
            style.interaction.selectable_labels = false;
        });
    }
}
