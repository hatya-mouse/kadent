mod file_browser;
mod kasl_editor;
mod tree_item;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{components::splitter::Splitter, editor::actions::FileNode},
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
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
    ) {
        let panel_rect = ui.available_rect_before_wrap();
        ui.horizontal(|ui| {
            ui.set_height(panel_rect.height());
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

            egui::ScrollArea::vertical()
                .id_salt("file_browser")
                .max_width(panel_state.file_list_width)
                .show(ui, |ui| {
                    self.file_browser(ui, panel_id, panel_state.file_list_width);
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
