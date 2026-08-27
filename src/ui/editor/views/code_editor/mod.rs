mod file_browser;
mod header;
mod kasl_editor;
mod tree_item;

use crate::{
    background_thread::BackgroundThreadCommand,
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{EditorState, components::splitter::Splitter, editor::actions::FileNode, theme},
};
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use kasl::core::error::ErrorRecord;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};
use uuid::Uuid;

const LINT_DELAY_MS: u64 = 1000;

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
    pub(crate) content: String,
    /// Whether the buffer has been modified since last save.
    pub(crate) is_modified: bool,
    /// Whether the buffer has been modified since last lint.
    pub(crate) has_modified_since_last_lint: bool,
    /// The list of errors found in the buffer.
    pub(crate) errors: Vec<ErrorRecord>,
    /// The time of the last edit, used to determine when to send a lint request.
    pub(crate) last_edit_time: Option<Instant>,
}

impl CodeBuffer {
    pub(crate) fn save_to_file(&self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            std::fs::write(path, &self.content)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CodeEditorPanelState {
    pub(crate) file_list_width: f32,
}

impl Default for CodeEditorPanelState {
    fn default() -> Self {
        Self {
            file_list_width: 200.0,
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
    ) {
        let panel_rect = ui.available_rect_before_wrap();

        ui.horizontal(|ui| {
            ui.set_height(panel_rect.height());
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

            egui::Frame::new()
                .fill(theme::secondary_bg(ui.visuals().dark_mode))
                .show(ui, |ui| {
                    ui.set_height(panel_rect.height());
                    egui::ScrollArea::vertical()
                        .id_salt("file_browser")
                        .max_width(panel_state.file_list_width)
                        .show(ui, |ui| {
                            self.file_browser(ui, state, panel_id, panel_state.file_list_width);
                        });
                });

            Splitter::new(&mut panel_state.file_list_width)
                .with_min(MIN_SIDEBAR_WIDTH)
                .with_max(MAX_SIDEBAR_WIDTH)
                .with_height(panel_rect.height())
                .show(ui);

            self.kasl_editor(ui, panel_id);
        });

        self.lint_buffers(state);
    }

    /// Checks if the buffer has been modified recently and sends an lint request to the background thread if necessary.
    fn lint_buffers(&mut self, state: &mut EditorState) {
        for (buffer_id, buffer) in self.code_buffers.iter_mut() {
            if let Some(t) = buffer.last_edit_time
                && t.elapsed() > std::time::Duration::from_millis(LINT_DELAY_MS)
            {
                buffer.last_edit_time = None;
                buffer.has_modified_since_last_lint = false;

                // Send a lint request to the background thread
                state
                    .actions
                    .push_background_job(BackgroundThreadCommand::LintKasl {
                        buffer_id: *buffer_id,
                        code: buffer.content.clone(),
                    });
            }
        }
    }
}
