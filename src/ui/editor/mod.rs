mod actions;
mod background_results;
mod device_fetching;
mod dialog;
mod frame_process;
mod keyboard;
mod panel;
mod preview_notes;
mod state;
mod status_bar;
mod toolbar;
mod utils;
mod views;

pub(crate) use frame_process::PeakHold;
pub(crate) use panel::{PanelNode, PanelVariant, SplitDir};
pub(crate) use state::*;
pub(crate) use status_bar::StatusBarView;
pub(crate) use views::{
    AutomationState, CodeBuffer, NodeGraphState, PianoRollState, TimelineState, ViewStates,
};

use crate::{
    background_thread::spawn_background_thread,
    core::{
        kasl_node::kasl_syntax_set,
        midi_thread::{MidiCommand, spawn_midi_thread},
        project_ctx::ProjectContext,
    },
    ui::editor::actions::EditorAction,
    ui::{
        editor::state::{ActionDispatcher, AudioDeviceManager, MidiDeviceManager, TransportState},
        theme,
    },
};
use crate::{
    core::{
        audio_engine::thread::{AudioCommand, AudioThread, AudioThreadHandle},
        metadata::TrackType,
    },
    storage::app_state::AppPreferences,
};
use cpal::traits::DeviceTrait;
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use std::{
    sync::{Arc, mpsc},
    time::Duration,
};
use syntect::highlighting::ThemeSet;

pub(crate) struct EditorState {
    // --- PROJECT ---
    /// The current project context that stores the document data.
    pub(crate) project: ProjectContext,

    // --- THREAD COMMUNIATION & HANDLES ---
    /// A thread handle to communicate with the audio thread.
    pub(crate) thread_handle: AudioThreadHandle,
    /// A channel to send MIDI commands to the MIDI thread.
    pub(crate) midi_tx: mpsc::Sender<MidiCommand>,

    // --- AUDIO & MIDI DEVICE ---
    /// The audio device manager to store the available audio devices and the selected device.
    pub(crate) audio_device: AudioDeviceManager,
    /// The MIDI device manager to store the available MIDI devices and the selected device.
    pub(crate) midi_device: MidiDeviceManager,

    // --- TRANSPORT STATE ---
    /// The transport state that stores the current playhead position.
    pub(crate) transport: TransportState,

    // --- BACKEND LOGIC ---
    /// Currently selected content.
    pub(crate) selection: Selection,
    /// Action dispatcher that stores the pending actions to be executed at the end of the frame.
    pub(crate) actions: ActionDispatcher,

    // --- UI COMMAND ---
    /// UI command dispatcher that stores the pending UI commands to be executed at the end of the frame.
    pub(crate) ui_commands: UiCommandDispatcher,

    // --- DEBUG MODE ---
    /// Whether the editor is in the debug mode.
    pub(crate) debug_mode: bool,
}

pub(crate) struct EditorUi {
    // --- EDITOR STATE ---
    pub(crate) state: EditorState,

    // --- UI LAYOUT & VIEW STATES ---
    /// Panel layout tree.
    pub(crate) layout: PanelNode,
    /// The states for the each panel views.
    pub(crate) views: ViewStates,
}

impl EditorUi {
    pub(crate) fn new(proj_ctx: ProjectContext) -> EditorUi {
        let (thread_handle, midi_producer) = AudioThread::spawn(proj_ctx.meta.export_ctx.clone());
        let background_handle = spawn_background_thread();
        let midi_tx = spawn_midi_thread(midi_producer);

        let mut editor_ui = EditorUi {
            state: EditorState {
                project: proj_ctx,
                thread_handle,
                midi_tx,
                audio_device: AudioDeviceManager::default(),
                midi_device: MidiDeviceManager::default(),
                transport: TransportState::default(),
                selection: Selection::default(),
                actions: ActionDispatcher::new(background_handle),
                ui_commands: UiCommandDispatcher::default(),
                debug_mode: false,
            },
            layout: PanelNode::default(),
            views: ViewStates::default(),
        };

        // Load the kasl syntax set and create a syntect settings
        editor_ui.views.code_editor.syntect_settings = Some(Arc::new(SyntectSettings {
            ps: kasl_syntax_set(),
            ts: ThemeSet::load_defaults(),
        }));

        // Fetch the avaliable devices first
        editor_ui.state.fetch_devices();
        editor_ui.state.audio_device.selected_output = editor_ui
            .state
            .audio_device
            .default_output
            .as_ref()
            .and_then(|device| device.id().ok());

        // Load the project structure and cache it
        editor_ui
            .state
            .actions
            .push_action(EditorAction::UpdateDirCache);
        editor_ui.state.actions.modified_project();

        // For each audio region, generate the waveforms
        editor_ui.generate_waveforms();

        editor_ui
    }

    pub(crate) fn ui(&mut self, ui: &mut egui::Ui, preferences: &AppPreferences) {
        self.calculate_playhead();
        self.process_vu_value();
        self.update_preview_notes();
        self.state.handle_keyboard(ui);

        egui::Panel::top(ui.id().with("toolbar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(44.0)
            .show(ui, |ui| {
                self.views.toolbar.ui(ui, &mut self.state);
            });

        // The status bar should display the modification state from the last frame
        egui::Panel::bottom(ui.id().with("status_bar"))
            .frame(
                egui::Frame::new()
                    .fill(theme::tertiary_bg(ui.visuals().dark_mode))
                    .inner_margin(egui::Margin::symmetric(12, 0)),
            )
            .exact_size(32.0)
            .show(ui, |ui| {
                self.views.status_bar.ui(ui, &self.state);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::primary_bg(ui.visuals().dark_mode))
                    .inner_margin(0),
            )
            .show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                self.render_panels(ui, rect, preferences);
            });

        self.views.dialog.dialog(ui, &mut self.state);
        self.state.update_project();

        // Request a repaint to update the playhead and the VU meter
        ui.ctx().request_repaint_after(Duration::from_millis(16));

        // Execute all pending actions
        self.consume_actions(preferences);
        // Also consume the UI commands
        self.consume_ui_commands();
        // Process the result of the background thread
        self.process_audio_thread_result();
        self.process_background_results();
    }

    fn consume_ui_commands(&mut self) {
        let pending_commands: Vec<UiCommand> = self.state.ui_commands.pending.drain(..).collect();
        for command in pending_commands {
            match command {
                UiCommand::ShowDialog(dialog_type) => match dialog_type {
                    DialogType::AddTrack => {
                        self.views.dialog = DialogState::AddTrack {
                            selected_track_type: TrackType::Audio,
                            name: String::new(),
                        };
                    }
                    DialogType::ChangeCodeBuffer { panel_id, path } => {
                        self.views.dialog = DialogState::ChangeCodeBuffer { panel_id, path };
                    }
                    DialogType::CloseCodeBuffer { panel_id } => {
                        self.views.dialog = DialogState::CloseCodeBuffer { panel_id };
                    }
                    DialogType::RenameFile { path } => {
                        let initial_name = path
                            .file_name()
                            .and_then(|file_name| file_name.to_os_string().into_string().ok())
                            .unwrap_or_default();
                        self.views.dialog = DialogState::RenameFile {
                            path,
                            new_name: initial_name,
                        };
                    }
                },
                UiCommand::ShowTempStatus(message, color) => {
                    self.views.status_bar.show_temp_status(&message, color);
                }
            }
        }
    }
}

impl EditorState {
    /// Checks if the project has been modified recently and sends an update command to the audio thread if necessary.
    /// Should not be called directly because this is automatically called.
    fn update_project(&mut self) {
        if let Some(t) = self.actions.last_edit_time
            && t.elapsed() > std::time::Duration::from_millis(300)
        {
            self.actions.last_edit_time = None;

            // Clone the project and send it to the audio thread
            let project = self.project.data.clone();
            if let Err(err) = self
                .thread_handle
                .audio_command_tx
                .send(AudioCommand::UpdateProject(Box::new(project)))
            {
                println!("Failed to send project update command: {err}");
            }
        }
    }
}
