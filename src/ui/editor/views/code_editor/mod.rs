mod error_panel;
mod file_browser;
mod header;
mod kasl_editor;
mod tree_item;

use crate::{
    background_thread::BackgroundThreadCommand,
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    storage::app_state::AppPreferences,
    ui::{
        EditorState,
        components::{h_splitter::HSplitter, v_splitter::VSplitter},
        editor::actions::FileNode,
        theme,
    },
};
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use kasl::core::error::ErrorRecord;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};
use uuid::Uuid;

const LINT_DELAY_MS: u64 = 1000;
const CODE_EDITOR_FONT_SIZE: f32 = 14.0;

#[derive(Default)]
pub(crate) struct CodeEditorView {
    /// Syntect settings which supports KASL language.
    pub(crate) syntect_settings: Option<Arc<SyntectSettings>>,
    /// Cached graph of the project directory structure.
    pub(crate) project_dir_cache: Vec<FileNode>,
    /// Buffers for each open file, keyed by their panel ID.
    pub(crate) code_buffers: HashMap<Uuid, CodeBuffer>,
}

#[derive(Default, Clone)]
pub(crate) struct CodeBuffer {
    /// The path to the file being edited, if any.
    pub(crate) path: Option<PathBuf>,
    /// The content of the file being edited.
    content: String,
    /// Whether the buffer has been modified since last save.
    pub(crate) is_modified: bool,
    /// The scroll offset of the editor, used to render line numbers.
    scroll_offset: egui::Vec2,
    /// The error state of the buffer.
    errors: BufferErrorState,
}

impl CodeBuffer {
    pub(crate) fn save_to_file(&self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            std::fs::write(path, &self.content)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct BufferErrorState {
    /// Whether the buffer has been modified since last lint.
    has_modified_since_last_lint: bool,
    /// Whether it is currently linting the buffer.
    is_linting: bool,
    /// The list of errors found in the buffer.
    records: Vec<ErrorRecord>,
    /// The position of the character in bytes to calculate where to shown an error.
    byte_offsets: Vec<usize>,
    /// The time of the last edit, used to determine when to send a lint request.
    last_edit_time: Option<Instant>,
}

impl Default for BufferErrorState {
    fn default() -> Self {
        Self {
            has_modified_since_last_lint: false,
            is_linting: false,
            records: Vec::new(),
            byte_offsets: Vec::new(),
            last_edit_time: Some(Instant::now()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CodeEditorPanelState {
    pub(crate) file_list_width: f32,
    pub(crate) is_error_panel_open: bool,
    pub(crate) error_panel_height: f32,
    pub(crate) jump_index: Option<(usize, usize)>,
}

impl Default for CodeEditorPanelState {
    fn default() -> Self {
        Self {
            file_list_width: 200.0,
            is_error_panel_open: true,
            error_panel_height: 200.0,
            jump_index: None,
        }
    }
}

impl CodeEditorView {
    pub(in crate::ui::editor) fn ui(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
        preferences: &AppPreferences,
    ) {
        let panel_rect = ui.available_rect_before_wrap();

        ui.horizontal(|ui| {
            ui.set_height(panel_rect.height());
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

            egui::Frame::new()
                .fill(theme::secondary_bg(ui.visuals().dark_mode))
                .show(ui, |ui| {
                    ui.set_height(panel_rect.height().max(0.0));
                    egui::ScrollArea::vertical()
                        .id_salt("file_browser")
                        .max_width(panel_state.file_list_width)
                        .show(ui, |ui| {
                            self.file_browser(ui, state, panel_id, panel_state.file_list_width);
                        });
                });

            VSplitter::new(&mut panel_state.file_list_width)
                .with_min(MIN_SIDEBAR_WIDTH)
                .with_max(MAX_SIDEBAR_WIDTH)
                .with_height(panel_rect.height())
                .show(ui);

            ui.vertical(|ui| {
                let kasl_editor_height = (panel_rect.height()
                    - if panel_state.is_error_panel_open {
                        panel_state.error_panel_height
                    } else {
                        0.0
                    })
                .max(0.0);
                egui::Frame::new().show(ui, |ui| {
                    ui.set_height(kasl_editor_height);
                    self.kasl_editor(ui, panel_id, panel_state);
                });

                if panel_state.is_error_panel_open {
                    HSplitter::new(&mut panel_state.error_panel_height)
                        .with_min(MIN_SIDEBAR_WIDTH)
                        .with_max(MAX_SIDEBAR_WIDTH)
                        .show(ui);

                    self.error_panel(ui, panel_id, panel_state);
                }
            });
        });

        self.lint_buffers(state, preferences);
    }

    pub(in crate::ui::editor) fn set_lint_errors(
        &mut self,
        buffer_id: Uuid,
        byte_offsets: Vec<usize>,
        records: Vec<ErrorRecord>,
    ) {
        if let Some(buffer) = self.code_buffers.get_mut(&buffer_id) {
            buffer.errors.records = records;
            buffer.errors.byte_offsets = byte_offsets;
        }
    }

    /// Checks if the buffer has been modified recently and sends an lint request to the background thread if necessary.
    fn lint_buffers(&mut self, state: &mut EditorState, preferences: &AppPreferences) {
        for (buffer_id, buffer) in self.code_buffers.iter_mut() {
            if let Some(t) = buffer.errors.last_edit_time
                && t.elapsed() > std::time::Duration::from_millis(LINT_DELAY_MS)
            {
                buffer.errors.last_edit_time = None;
                buffer.errors.has_modified_since_last_lint = false;
                buffer.errors.is_linting = true;

                // Send a lint request to the background thread
                if let Some(file_path) = buffer.path.clone() {
                    state
                        .actions
                        .push_background_job(BackgroundThreadCommand::LintKasl {
                            buffer_id: *buffer_id,
                            code: buffer.content.clone(),
                            search_paths: preferences
                                .kasl_std_path
                                .clone()
                                .map(|p| vec![PathBuf::from(p)])
                                .unwrap_or_default(),
                            file_path,
                        });
                }
            }
        }
    }
}
