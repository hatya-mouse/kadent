mod file_browser;
mod kasl_editor;

use crate::ui::{
    EditorState,
    editor::code_editor::{file_browser::file_browser, kasl_editor::kasl_editor},
};
use eframe::egui;

pub(super) fn code_editor(ui: &mut egui::Ui, state: &mut EditorState, panel_id: egui::Id) {
    // Ensure a buffer entry exists for this panel
    state
        .ui_state
        .code_editor_state
        .code_buffers
        .entry(panel_id)
        .or_insert(None);

    egui::Panel::left(panel_id.with("code_editor_left")).show_inside(ui, |ui| {
        file_browser(ui, state, panel_id);
    });

    egui::CentralPanel::default()
        .frame(egui::Frame::new())
        .show_inside(ui, |ui| {
            kasl_editor(ui, state, panel_id);
        });
}
