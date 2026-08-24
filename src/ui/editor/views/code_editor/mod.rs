mod file_browser;
mod kasl_editor;

use crate::ui::editor::{actions::FileNode, views::PanelViewState};
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

impl CodeEditorView {
    pub(in crate::ui::editor) fn ui(
        &mut self,
        ui: &mut egui::Ui,
        panel_id: Uuid,
        panel_state: Option<&mut PanelViewState>,
    ) {
        egui::Panel::left(ui.id().with("code_editor_left")).show_inside(ui, |ui| {
            self.file_browser(ui, panel_id);
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                self.kasl_editor(ui, panel_id);
            });
    }
}
