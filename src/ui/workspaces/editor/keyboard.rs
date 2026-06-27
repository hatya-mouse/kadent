//! Handles keyboard inputs.

use crate::ui::workspaces::{EditorUi, editor::state::Selection};
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
                    self.remove_region(&track_id, &region_id);
                    self.ui_state.select_track(track_id);
                }
                Selection::Node(track_id, node_id) => {
                    self.remove_node(&track_id, &node_id);
                    self.ui_state.select_track(track_id);
                }
                Selection::Note(track_id, region_id, note_id) => {
                    self.remove_note(&track_id, &region_id, &note_id);
                    self.ui_state.select_region(track_id, region_id);
                }
                _ => (),
            }
        }
    }
}
