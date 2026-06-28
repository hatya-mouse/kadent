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

        let save = ui.input(|i| is_command_pressed(ui) && i.key_pressed(egui::Key::S));
        if save {
            // Save the projects and opened KASL programs
            self.push_action(EditorAction::SaveAll);
        }
    }
}

/// Returns true if the command key is pressed on macOS/iOS, or the control key is pressed on other platforms.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn is_command_pressed(ui: &egui::Ui) -> bool {
    ui.input(|i| i.modifiers.command)
}

/// Returns true if the command key is pressed on macOS/iOS, or the control key is pressed on other platforms.
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn is_command_pressed(ui: &egui::Ui) -> bool {
    ui.input(|i| i.modifiers.ctrl)
}
