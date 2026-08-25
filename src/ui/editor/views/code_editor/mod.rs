mod file_browser;
mod header;
mod kasl_editor;
mod tree_item;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{EditorState, components::splitter::Splitter, editor::actions::FileNode, theme},
};
use eframe::egui;
use egui_extras::syntax_highlighting::SyntectSettings;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use uuid::Uuid;

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
    pub(crate) path: Option<PathBuf>,
    pub(crate) content: String,
    pub(crate) is_modified: bool,
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
    }
}
