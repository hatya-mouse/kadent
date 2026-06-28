//! Handles keyboard inputs.

use crate::{
    commands::EditorAction,
    ui::workspaces::{EditorUi, editor::state::Selection},
};
use eframe::egui;

impl EditorUi {
    pub(super) fn handle_keyboard(&mut self, ui: &egui::Ui) {
        // If other UI element has focus, I don't want to handle keybaord input here
        if ui.egui_wants_keyboard_input() {
            return;
        }

        let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
        let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
        if delete || backspace {
            match self.ui_state.selection {
                Selection::Region(track_id, region_id) => {
                    self.pending_actions
                        .push_back(EditorAction::RemoveRegion(track_id, region_id));
                }
                Selection::Node(track_id, node_id) => {
                    self.pending_actions
                        .push_back(EditorAction::RemoveNode(track_id, node_id));
                }
                Selection::Note(track_id, region_id, note_id) => {
                    self.pending_actions
                        .push_back(EditorAction::RemoveNote(track_id, region_id, note_id));
                }
                _ => (),
            }
        }
    }
}
