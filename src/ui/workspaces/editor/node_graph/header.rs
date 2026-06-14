use crate::ui::{
    components::{icon_button::small_icon_button, text_button::text_button},
    theme,
    workspaces::EditorUi,
};
use eframe::egui;
use std::fmt::Display;

enum AddibleNodes {
    Kasl,
}

impl Display for AddibleNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddibleNodes::Kasl => write!(f, "KASL Node"),
        }
    }
}

impl EditorUi {
    pub(super) fn draw_node_graph_header(&mut self, ui: &mut egui::Ui) {
        let mut node_to_add: Option<AddibleNodes> = None;
        let mut jump_to_random = false;

        let response = egui::Frame::new()
            .fill(theme::tertiary_bg(ui.visuals().dark_mode))
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);
                ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;
                ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::icon_button_hovered();
                ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;
                ui.visuals_mut().widgets.active.weak_bg_fill = theme::icon_button_active();
                ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;

                ui.horizontal(|ui| {
                    let btn = small_icon_button(
                        ui,
                        egui::Image::new(egui::include_image!(
                            "../../../../../assets/icons/plus.svg"
                        )),
                    );
                    egui::Popup::menu(&btn).show(|ui| {
                        if text_button(ui, "kasl_node", AddibleNodes::Kasl.to_string()).clicked() {
                            node_to_add = Some(AddibleNodes::Kasl);
                        }
                    });

                    let jump_btn = small_icon_button(
                        ui,
                        egui::Image::new(egui::include_image!(
                            "../../../../../assets/icons/crosshair.svg"
                        )),
                    )
                    .on_hover_text("Jump to a random node");
                    if jump_btn.clicked() {
                        jump_to_random = true;
                    }
                });
            });

        let rect = response.response.rect;
        ui.painter().line_segment(
            [
                rect.left_bottom() - egui::vec2(0.0, 0.5),
                rect.right_bottom() - egui::vec2(0.0, 0.5),
            ],
            egui::Stroke::new(0.5, theme::border_color(ui.visuals().dark_mode)),
        );

        // Jump to a random node's position
        if jump_to_random
            && let Some(track_id) = self.ui_state.selected_track
            && let Some(track_meta) = self.project_meta.get_track(&track_id)
            && let Some(node_meta) = track_meta.graph.nodes.values().next()
        {
            self.ui_state.node_graph_state.jump_to_pos = Some(node_meta.pos);
        }

        // Add a new node if the node is clicked on the add list
        if let Some(node_type) = node_to_add {
            // Get the currently selected track
            let Some(track_id) = self.ui_state.selected_track else {
                return;
            };

            let pan = self.ui_state.node_graph_state.pan_offset;
            let pos = egui::pos2(-pan.x + 20.0, -pan.y + 20.0);
            match node_type {
                AddibleNodes::Kasl => self.add_kasl_node(&track_id, pos),
            }
        }
    }
}
