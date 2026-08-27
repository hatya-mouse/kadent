use crate::ui::{
    EditorState,
    components::toolbar_button::small_icon_button,
    editor::{
        NodeGraphState,
        actions::{AddibleNodes, EditorAction},
    },
    theme,
};
use eframe::egui;
use rand::seq::IteratorRandom;

impl NodeGraphState {
    pub(in crate::ui::editor) fn header(&mut self, ui: &mut egui::Ui, state: &mut EditorState) {
        let mut node_to_add: Option<AddibleNodes> = None;
        let mut jump_to_random = false;

        ui.set_min_width(ui.available_width());

        ui.horizontal_centered(|ui| {
            let plus_button = small_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../../assets/icons/plus.svg")),
                true,
            );
            egui::Popup::menu(&plus_button)
                .style(theme::menu_style(ui))
                .show(|ui| {
                    AddibleNodes::all().iter().for_each(|node| {
                        if ui.selectable_label(false, node.name()).clicked() {
                            node_to_add = Some(node.clone());
                        }
                    });
                });

            let jump_button = small_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../../assets/icons/crosshair.svg"
                )),
                true,
            )
            .on_hover_text("Jump to a random node");
            if jump_button.clicked() {
                jump_to_random = true;
            }
        });

        // Jump to a random node's position
        if jump_to_random
            && let Some(track_id) = state.selection.track_id()
            && let Some(track_meta) = state.project.meta.get_track(&track_id)
            && let Some(node_meta) = track_meta.graph.nodes.values().choose(&mut rand::rng())
        {
            self.jump_to_pos = Some(node_meta.pos);
        }

        // Add a new node if the node is clicked on the add list
        if let Some(node_type) = node_to_add {
            // Get the currently selected track
            let Some(track_id) = state.selection.track_id() else {
                return;
            };

            let pan = self.pan_offset;
            let pos = egui::pos2(-pan.x + 20.0, -pan.y + 20.0);
            state
                .actions
                .push_action(EditorAction::AddNode(track_id, node_type, pos));
        }
    }
}
