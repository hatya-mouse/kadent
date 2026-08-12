mod background_results;
mod code_editor;
mod device_fetching;
mod error_list;
mod inspector;
mod keyboard;
mod node_graph;
mod panel;
mod piano_roll;
mod preview_notes;
mod state;
mod status_bar;
mod timeline;
mod toolbar;

use crate::{
    actions::EditorAction,
    background_thread::{BackgroundThreadCommand, BackgroundThreadHandle, spawn_background_thread},
    core::{
        kasl_node::kasl_syntax_set,
        midi_thread::{MidiCommand, spawn_midi_thread},
        project_ctx::{EditorContext, ProjectContext},
    },
    ui::{
        theme,
        workspaces::editor::state::{EditorUiState, Modification},
    },
};
use cpal::traits::DeviceTrait;
use eframe::egui;
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use kadent_engine::thread::{AudioCommand, AudioError, AudioThread, AudioThreadHandle};
use std::{collections::VecDeque, path::PathBuf, sync::mpsc, time::Duration};
use syntect::highlighting::ThemeSet;

pub struct EditorUi {
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// A thread handle to communicate with the background processing thread.
    pub background_handle: BackgroundThreadHandle,
    /// A channel to send MIDI commands to the MIDI thread.
    pub midi_command_tx: mpsc::Sender<MidiCommand>,
    /// The current project context.
    pub proj_ctx: ProjectContext,
    /// Errors to be shown.
    pub errors: Vec<AudioError>,
    /// UI states to store the current UI state.
    pub ui_state: EditorUiState,
    /// Pending actions to be executed in the frame.
    pub pending_actions: VecDeque<EditorAction>,
    /// The path to write the currently exported project to.
    pub pending_export_path: Option<PathBuf>,
    /// Whether the editor is in the debug mode.
    pub debug_mode: bool,
}

impl EditorUi {
    pub fn new(editor_ctx: EditorContext) -> EditorUi {
        let (thread_handle, midi_producer) =
            AudioThread::spawn(editor_ctx.proj_ctx.project_meta.export_ctx.clone());
        let background_handle = spawn_background_thread();
        let midi_command_tx = spawn_midi_thread(midi_producer);

        let mut editor_ui = EditorUi {
            thread_handle,
            background_handle,
            midi_command_tx,
            proj_ctx: editor_ctx.proj_ctx,
            errors: Vec::new(),
            ui_state: EditorUiState::with_audio_ctx(editor_ctx.audio_ctx),
            pending_actions: VecDeque::new(),
            pending_export_path: None,
            debug_mode: false,
        };

        // Load the kasl syntax set and create a syntect settings
        editor_ui.ui_state.code_editor_state.syntect_settings = Some(SyntectSettings {
            ps: kasl_syntax_set(),
            ts: ThemeSet::load_defaults(),
        });

        // Fetch the avaliable devices first
        editor_ui.fetch_devices();
        editor_ui.ui_state.selected_output_device = editor_ui
            .ui_state
            .default_output_device
            .as_ref()
            .and_then(|device| device.id().ok());

        // Load the project structure and cache it
        editor_ui.push_action(EditorAction::UpdateDirCache);
        editor_ui.modified_project();

        // For each audio region, generate the waveforms
        editor_ui.generate_waveforms();

        editor_ui
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui_state.code_editor_state.theme = Some(CodeTheme::from_memory(ui.ctx(), ui.style()));

        self.calculate_playhead();
        self.process_vu_value();
        self.update_preview_notes();
        self.handle_keyboard(ui);

        egui::Panel::top(ui.id().with("toolbar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show_inside(ui, |ui| {
                self.toolbar(ui);
            });

        // The status bar should display the modification state from the last frame
        egui::Panel::bottom(ui.id().with("status_bar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(32.0)
            .show_inside(ui, |ui| {
                self.status_bar(ui);
            });

        // Reset the modification state from the last frame
        self.ui_state.set_modification(Modification::None);

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

        // Request a repaint to update the playhead and the VU meter
        ui.ctx().request_repaint_after(Duration::from_millis(16));

        // Execute all pending actions
        self.consume_actions();
        self.process_audio_thread_result();
        // Process the result of the background thread
        self.process_background_results();
    }

    /// Checks if the project has been modified recently and sends an update command to the audio thread if necessary.
    /// Should not be called directly because this is automatically called.
    fn update_project(&mut self) {
        if let Some(t) = self.ui_state.last_edit_time
            && t.elapsed() > std::time::Duration::from_millis(300)
        {
            self.ui_state.last_edit_time = None;

            // Clone the project and send it to the audio thread
            let project = self.proj_ctx.project.clone();
            if let Err(err) = self
                .thread_handle
                .audio_command_tx
                .send(AudioCommand::UpdateProject(Box::new(project)))
            {
                println!("Failed to send project update command: {err}");
            }
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

    fn push_action(&mut self, action: EditorAction) {
        self.pending_actions.push_back(action);
    }

    pub(crate) fn push_background_job(&mut self, command: BackgroundThreadCommand) {
        self.background_handle.command_tx.send(command).ok();
    }
}
