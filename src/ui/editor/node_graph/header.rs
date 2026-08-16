use crate::{
    actions::{AddibleNodes, EditorAction},
    ui::{EditorState, components::icon_button::small_icon_button, theme},
};
use eframe::egui;

impl EditorState {
    pub(super) fn draw_node_graph_header(&mut self, ui: &mut egui::Ui) {
        let mut node_to_add: Option<AddibleNodes> = None;
        let mut jump_to_random = false;

        ui.set_min_width(ui.available_width());

        ui.horizontal_centered(|ui| {
            let plus_button = small_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/plus.svg")),
            );
            egui::Popup::menu(&plus_button)
                .style(theme::menu_style(ui))
                .show(|ui| {
                    AddibleNodes::all().iter().for_each(|node| {
                        if ui.button(node.name()).clicked() {
                            node_to_add = Some(node.clone());
                        }
                    });
                });

            let jump_button = small_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../assets/icons/crosshair.svg"
                )),
            )
            .on_hover_text("Jump to a random node");
            if jump_button.clicked() {
                jump_to_random = true;
            }
        });

        // Jump to a random node's position
        if jump_to_random
            && let Some(track_id) = self.ui_state.selection.track_id()
            && let Some(track_meta) = self.ui_state.proj_ctx.project_meta.get_track(&track_id)
            && let Some(node_meta) = track_meta.graph.nodes.values().next()
        {
            self.ui_state.node_graph_state.jump_to_pos = Some(node_meta.pos);
        }

        // Add a new node if the node is clicked on the add list
        if let Some(node_type) = node_to_add {
            // Get the currently selected track
            let Some(track_id) = self.ui_state.selection.track_id() else {
                return;
            };

            let pan = self.ui_state.node_graph_state.pan_offset;
            let pos = egui::pos2(-pan.x + 20.0, -pan.y + 20.0);
            self.push_action(EditorAction::AddNode(track_id, node_type, pos));
        }
    }
}
