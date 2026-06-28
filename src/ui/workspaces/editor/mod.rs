mod code_editor;
mod device_fetching;
mod error_list;
mod inspector;
mod keyboard;
mod node_graph;
mod panel;
mod piano_roll;
mod state;
mod status_bar;
mod timeline;
mod toolbar;

use crate::{
    core::{
        kasl_node::kasl_syntax_set,
        midi_input::{MidiCommand, spawn_midi_thread},
        project_ctx::{EditorContext, ProjectContext},
    },
    ui::{theme, workspaces::editor::state::EditorUiState},
};
use cpal::traits::DeviceTrait;
use eframe::egui;
use kadent_engine::thread::{AudioError, AudioThread, AudioThreadHandle};
use std::{sync::mpsc, time::Duration};
use syntect::parsing::SyntaxSet;

pub struct EditorUi {
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// A channel to send MIDI commands to the MIDI thread.
    pub midi_command_tx: mpsc::Sender<MidiCommand>,
    /// The current project context.
    pub proj_ctx: ProjectContext,
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
    /// UI states to store the current UI state.
    pub ui_state: EditorUiState,
    /// Syntax set for KASL language.
    pub syntax_set: SyntaxSet,
    /// Whether the editor is in the debug mode.
    pub debug_mode: bool,
}

impl EditorUi {
    pub fn new(editor_ctx: EditorContext) -> EditorUi {
        let (thread_handle, midi_producer) =
            AudioThread::spawn(editor_ctx.audio_ctx, editor_ctx.proj_ctx.project.clone());
        let midi_command_tx = spawn_midi_thread(midi_producer);

        let mut editor_ui = EditorUi {
            thread_handle,
            midi_command_tx,
            proj_ctx: editor_ctx.proj_ctx,
            errors: Vec::new(),
            ui_state: EditorUiState::default(),
            syntax_set: kasl_syntax_set(),
            debug_mode: true,
        };

        editor_ui.fetch_devices();
        editor_ui.ui_state.selected_output_device = editor_ui
            .ui_state
            .default_output_device
            .as_ref()
            .and_then(|device| device.id().ok());

        editor_ui
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.calculate_playhead();
        self.process_vu_value();
        self.handle_keyboard(ui);

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

        egui::Panel::bottom("status_har")
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(32.0)
            .show_inside(ui, |ui| {
                self.status_bar(ui);
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
