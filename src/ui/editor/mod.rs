mod node_graph;
mod panel;
mod piano_roll;
mod playhead_calculation;
mod project_setup;
mod timeline;
mod toolbar;

use crate::{colors, metadata::ProjectMeta, ui_state::editor_state::EditorUIState};
use eframe::egui;
use knodiq_engine::{
    audio_thread::{AudioError, AudioThread, AudioThreadHandle},
    data_types::AudioContext,
    mixer::Project,
};
use std::path::PathBuf;

pub struct EditorUi {
    /// The directory where the project is saved.
    pub project_dir: PathBuf,
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
    pub ui_state: EditorUIState,
}

impl EditorUi {
    pub fn new(
        project_dir: PathBuf,
        audio_ctx: AudioContext,
        project: Project,
        project_meta: ProjectMeta,
    ) -> Self {
        let thread_handle = AudioThread::spawn(audio_ctx, project.clone()).unwrap();

        Self {
            project_dir,
            project,
            is_playing: false,
            thread_handle,
            errors: Vec::new(),
            project_meta,
            ui_state: EditorUIState::default(),
        }
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.calculate_playhead();

        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(colors::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show_inside(ui, |ui| {
                self.toolbar(ui);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(colors::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(0),
            )
            .show_inside(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                self.render_panels(ui, rect);
            });

        self.track_dialog(ui);
        self.update_project();

        while let Ok(Err(err)) = self.thread_handle.result_rx.try_recv() {
            eprintln!("Audio thread error occurred");
            self.errors.push(err);
        }
    }

    pub(crate) fn system_kasl_search_paths() -> Vec<String> {
        let mut paths = Vec::new();
        if let Some(app_data) = dirs::data_dir().map(|d| d.join("knodiq"))
            && let Some(s) = app_data.to_str()
        {
            paths.push(s.to_string());
        }
        if let Some(mut home) = dirs::home_dir() {
            home.push(".kasl/std/");
            if let Some(s) = home.to_str() {
                paths.push(s.to_string());
            }
        }
        paths
    }
}
