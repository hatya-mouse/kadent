pub(crate) mod error_list;
pub(crate) mod inspector;
pub(crate) mod node_graph;
pub(crate) mod panel;
pub(crate) mod piano_roll;
pub mod state;
pub(crate) mod timeline;
pub(crate) mod toolbar;

use crate::{
    core::project_ctx::{EditorContext, ProjectContext},
    ui::{theme, workspaces::editor::state::EditorUiState},
};
use eframe::egui;
use kadent_engine::thread::{AudioError, AudioThread, AudioThreadHandle};
use std::time::Duration;

pub struct EditorUi {
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// The current project context.
    pub proj_ctx: ProjectContext,
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
    /// UI states to store the current UI state.
    pub ui_state: EditorUiState,
    /// Whether the editor is in the debug mode.
    pub debug_mode: bool,
}

impl EditorUi {
    pub fn new(editor_ctx: EditorContext) -> Self {
        let thread_handle =
            AudioThread::spawn(editor_ctx.audio_ctx, editor_ctx.proj_ctx.project.clone());

        Self {
            proj_ctx: editor_ctx.proj_ctx,
            thread_handle,
            errors: Vec::new(),
            ui_state: EditorUiState::default(),
            debug_mode: true,
        }
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.calculate_playhead();
        self.process_vu_value();

        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show_inside(ui, |ui| {
                self.toolbar(ui);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(0),
            )
            .show_inside(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                self.render_panels(ui, rect);
            });

        self.track_dialog(ui);
        self.update_project();

        // Request a repaint to update the playhead and the VU meter.
        ui.ctx().request_repaint_after(Duration::from_millis(16));

        while let Ok(Err(err)) = self.thread_handle.result_rx.try_recv() {
            eprintln!("Audio thread error occurred");
            self.errors.push(err);
        }
    }

    pub(crate) fn system_kasl_search_paths() -> Vec<String> {
        let mut paths = Vec::new();
        if let Some(app_data) = dirs::data_dir().map(|d| d.join("kadent"))
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
