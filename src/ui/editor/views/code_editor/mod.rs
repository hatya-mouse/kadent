mod file_browser;
mod kasl_editor;

use crate::{
    consts::{MAX_SIDEBAR_WIDTH, MIN_SIDEBAR_WIDTH},
    ui::{
        components::splitter::Splitter,
        editor::{actions::FileNode, views::PanelViewState},
    },
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
        let sidebar_rect = panel_rect.with_max_x(panel_rect.min.x + panel_state.file_list_width);
        egui::ScrollArea::vertical()
            .max_width(panel_state.file_list_width)
            .show(ui, |ui| {
                self.file_browser(ui, panel_id);
            });

        Splitter::new(&mut panel_state.file_list_width)
            .with_min(MIN_SIDEBAR_WIDTH)
            .with_max(MAX_SIDEBAR_WIDTH)
            .show(ui);

        self.kasl_editor(ui, panel_id);
    }
}
