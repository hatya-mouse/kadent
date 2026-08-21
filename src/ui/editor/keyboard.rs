//! Handles keyboard inputs.

use crate::{
    ui::editor::actions::EditorAction,
    ui::{EditorState, editor::state::Selection},
};
use eframe::egui;

impl EditorState {
    pub(super) fn handle_keyboard(&mut self, ui: &egui::Ui) {
        // If other UI element has focus, I don't want to handle keybaord input here
        if ui.egui_wants_keyboard_input() {
            return;
        }

        // Handle delete and backspace keys
        let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
        let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
        if delete || backspace {
            match self.selection {
                Selection::Region(track_id, region_id) => {
                    self.actions
                        .push_action(EditorAction::RemoveRegion(track_id, region_id));
                }
                Selection::Node(track_id, node_id) => {
                    self.actions
                        .push_action(EditorAction::RemoveNode(track_id, node_id));
                }
                Selection::Note(track_id, region_id, note_id) => {
                    self.actions
                        .push_action(EditorAction::RemoveNote(track_id, region_id, note_id));
                }
                Selection::Keyframe(track_id, region_id, keyframe_index) => {
                    self.actions.push_action(EditorAction::RemoveKeyframe(
                        track_id,
                        region_id,
                        keyframe_index,
                    ));
                }
                _ => (),
            }
        }

        let save = ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S));
        if save {
            // Save the projects and opened KASL programs
            self.actions.push_action(EditorAction::SaveAll);
        }

        let play_pause = ui.input(|i| i.key_pressed(egui::Key::Space));
        if play_pause {
            if self.transport.is_playing {
                self.actions.push_action(EditorAction::Pause);
            } else {
                self.actions.push_action(EditorAction::Play);
            }
        }

        let seek_forward = ui.input(|i| i.key_pressed(egui::Key::ArrowRight));
        if seek_forward {
            self.actions.push_action(EditorAction::Seek(
                self.project.data.export_range.end_time(),
            ));
        }

        let seek_back = ui.input(|i| i.key_pressed(egui::Key::ArrowLeft));
        if seek_back {
            self.actions.push_action(EditorAction::Seek(
                self.project.data.export_range.start_time(),
            ));
        }
    }
}
