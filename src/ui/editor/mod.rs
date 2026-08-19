mod actions;
mod background_results;
mod device_fetching;
mod keyboard;
mod panel;
mod preview_notes;
mod state;
mod status_bar;
mod toolbar;
mod utils;
mod views;

pub(crate) use panel::{PanelNode, PanelView, SplitDir};
pub(crate) use state::*;
pub(crate) use status_bar::{StatusBarState, StatusHint};
pub(crate) use views::{
    CodeEditorState, DialogState, NodeGraphState, PianoRollState, TimelineState, ViewStates,
};

use crate::{
    background_thread::{BackgroundThreadCommand, spawn_background_thread},
    core::{
        kasl_node::kasl_syntax_set,
        midi_thread::{MidiCommand, spawn_midi_thread},
        project_ctx::ProjectContext,
    },
    ui::editor::actions::EditorAction,
    ui::{
        components::ruler::RulerResponse,
        editor::state::{ActionDispatcher, AudioDeviceManager, MidiDeviceManager, TransportState},
        theme,
    },
};
use cpal::traits::DeviceTrait;
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use kadent_engine::{
    thread::{AudioCommand, AudioThread, AudioThreadHandle},
    timing::TimePosition,
};
use std::{
    sync::{Arc, mpsc},
    time::Duration,
};
use syntect::highlighting::ThemeSet;

pub struct EditorState {
    // --- PROJECT ---
    /// The current project context that stores the document data.
    pub project: ProjectContext,

    // --- THREAD COMMUNIATION & HANDLES ---
    /// A thread handle to communicate with the audio thread.
    pub thread_handle: AudioThreadHandle,
    /// A channel to send MIDI commands to the MIDI thread.
    pub midi_tx: mpsc::Sender<MidiCommand>,

    // --- AUDIO & MIDI DEVICE ---
    /// The audio device manager to store the available audio devices and the selected device.
    pub audio_device: AudioDeviceManager,
    /// The MIDI device manager to store the available MIDI devices and the selected device.
    pub midi_device: MidiDeviceManager,

    // --- TRANSPORT STATE ---
    /// The transport state that stores the current playhead position.
    pub transport: TransportState,

    // --- UI LAYOUT & VIEW STATES ---
    /// Panel layout tree.
    pub layout: PanelNode,
    /// The states for the each panel views.
    pub views: ViewStates,

    // --- BACKEND LOGIC ---
    /// Currently selected content.
    pub selection: Selection,
    /// Action dispatcher that stores the pending actions to be executed at the end of the frame.
    pub actions: ActionDispatcher,

    // --- DEBUG MODE ---
    /// Whether the editor is in the debug mode.
    pub debug_mode: bool,
}

impl EditorState {
    pub fn new(proj_ctx: ProjectContext) -> EditorState {
        let (thread_handle, midi_producer) = AudioThread::spawn(proj_ctx.meta.export_ctx.clone());
        let background_handle = spawn_background_thread();
        let midi_tx = spawn_midi_thread(midi_producer);

        let mut editor_ui = EditorState {
            project: proj_ctx,
            thread_handle,
            midi_tx,
            audio_device: AudioDeviceManager::default(),
            midi_device: MidiDeviceManager::default(),
            transport: TransportState::default(),
            layout: PanelNode::default(),
            views: ViewStates::default(),
            selection: Selection::default(),
            actions: ActionDispatcher::new(background_handle),
            debug_mode: false,
        };

        // Load the kasl syntax set and create a syntect settings
        editor_ui.views.code_editor.syntect_settings = Some(Arc::new(SyntectSettings {
            ps: kasl_syntax_set(),
            ts: ThemeSet::load_defaults(),
        }));

        // Fetch the avaliable devices first
        editor_ui.fetch_devices();
        editor_ui.audio_device.selected_output = editor_ui
            .audio_device
            .default_output
            .as_ref()
            .and_then(|device| device.id().ok());

        // Load the project structure and cache it
        editor_ui.push_action(EditorAction::UpdateDirCache);
        editor_ui.modified_project();

        // For each audio region, generate the waveforms
        editor_ui.generate_waveforms();

        editor_ui
    }

    pub(crate) fn editor_ui(&mut self, ui: &mut egui::Ui) {
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
        self.views.status_bar.set_status_hint(StatusHint::None);

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

    pub(super) fn push_action(&mut self, action: EditorAction) {
        self.actions.pending.push_back(action);
    }

    pub(crate) fn push_background_job(&mut self, command: BackgroundThreadCommand) {
        self.actions.background_handle.command_tx.send(command).ok();
    }

    fn apply_ruler_res(&mut self, ruler_res: &RulerResponse) {
        if let Some(target_tick) = ruler_res.seek_to {
            self.transport.playhead_tick = target_tick;

            if ruler_res.drag_ended {
                self.push_action(EditorAction::Seek(TimePosition::Musical(target_tick)));
            }
        }
    }
}
