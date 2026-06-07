mod node;
mod note;
mod region;
mod track;

use crate::{
    fonts::RichTextExt,
    ui::{theme, workspaces::EditorUi},
};
use eframe::egui;
use std::hash::Hash;

const HEADER_HEIGHT: f32 = 32.0;

impl EditorUi {
    pub fn inspector(&mut self, ui: &mut egui::Ui) {
        // Fill the background
        ui.painter().rect_filled(
            ui.max_rect(),
            0.0,
            theme::secondary_bg(ui.visuals().dark_mode),
        );

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                if let Some(track_id) = self.ui_state.selected_track {
                    self.track_inspector(ui, &track_id);
                }
                if let Some((track_id, region_id)) = self.ui_state.selected_region {
                    self.region_inspector(ui, &track_id, &region_id);
                }
                if let Some(note_id) = self.ui_state.selected_note {
                    let Some((track_id, region_id)) = self.ui_state.selected_region else {
                        return;
                    };
                    self.note_inspector(ui, &track_id, &region_id, &note_id);
                }
                if let Some(node_id) = self.ui_state.selected_node {
                    let Some(track_id) = self.ui_state.selected_track else {
                        return;
                    };
                    self.node_inspector(ui, &track_id, &node_id);
                }
            });
    }
}

fn inspector_section(
    ui: &mut egui::Ui,
    unique_id: impl Hash,
    title: impl Into<String>,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    ui.add_space(8.0);

    // Manage content collapsing state
    let id = ui.make_persistent_id(unique_id);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true);

    // Header
    let header_response = egui::Frame::new()
        .fill(theme::tertiary_bg(ui.visuals().dark_mode))
        .inner_margin(egui::vec2(8.0, 0.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_width(ui.available_width());
                ui.set_min_height(HEADER_HEIGHT);

                let collapse_icon = if state.is_open() {
                    egui::include_image!("../../../../../assets/icons/tri_down.svg")
                } else {
                    egui::include_image!("../../../../../assets/icons/tri_right.svg")
                };
                ui.add(
                    egui::Image::new(collapse_icon)
                        .tint(theme::primary_fg(ui.visuals().dark_mode))
                        .max_size(egui::vec2(20.0, 20.0)),
                );

                ui.add(
                    egui::Label::new(
                        egui::RichText::new(title)
                            .size(theme::large_font_size())
                            .color(theme::primary_fg(ui.visuals().dark_mode))
                            .bold(),
                    )
                    .selectable(false),
                );
            });
        });
    let header_click = header_response.response.interact(egui::Sense::click());

    if header_click.clicked() {
        state.toggle(ui);
    }

    // Draw a border on the top and the bottom of the header
    let painter = ui.painter();
    let header_rect = header_response.response.rect;
    painter.line_segment(
        [
            header_rect.left_top() + egui::vec2(0.0, 0.5),
            header_rect.right_top() + egui::vec2(0.0, 0.5),
        ],
        egui::Stroke::new(1.0, theme::border(ui.visuals().dark_mode)),
    );
    painter.line_segment(
        [
            header_rect.left_bottom() - egui::vec2(0.0, 0.5),
            header_rect.right_bottom() - egui::vec2(0.0, 0.5),
        ],
        egui::Stroke::new(1.0, theme::border(ui.visuals().dark_mode)),
    );

    // Contents
    state.show_body_unindented(ui, |ui| {
        egui::Frame::new()
            .inner_margin(egui::vec2(12.0, 8.0))
            .show(ui, |ui| {
                ui.style_mut().spacing.item_spacing.y = 8.0;
                add_contents(ui);
            });
    });
}

fn inspector_item(
    ui: &mut egui::Ui,
    label: impl Into<String>,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    ui.style_mut().spacing.interact_size.y = 24.0;
    ui.horizontal(|ui| {
        let text = egui::RichText::new(label)
            .size(theme::normal_font_size())
            .color(theme::primary_fg(ui.visuals().dark_mode));

        ui.add(egui::Label::new(text).selectable(false));
        add_contents(ui);
    });
}
