//! Handles keyboard inputs.

use crate::{
    commands::EditorAction,
    ui::workspaces::{EditorUi, editor::state::Selection},
};
use eframe::egui;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use eframe::egui::InputState;

impl EditorUi {
    pub(super) fn handle_keyboard(&mut self, ui: &egui::Ui) {
        // If other UI element has focus, I don't want to handle keybaord input here
        if ui.egui_wants_keyboard_input() {
            return;
        }

        // Handle delete and backspace keys
        let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
        let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
        if delete || backspace {
            match self.ui_state.selection {
                Selection::Region(track_id, region_id) => {
                    self.push_action(EditorAction::RemoveRegion(track_id, region_id));
                }
                Selection::Node(track_id, node_id) => {
                    self.push_action(EditorAction::RemoveNode(track_id, node_id));
                }
                Selection::Note(track_id, region_id, note_id) => {
                    self.push_action(EditorAction::RemoveNote(track_id, region_id, note_id));
                }
                _ => (),
            }
        }

        let save = ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S));
        if save {
            // Save the projects and opened KASL programs
            self.push_action(EditorAction::SaveAll);
        }
    }
}
