mod node;
mod note;
mod region;
mod track;

use crate::{fonts::RichTextExt, theme, ui::EditorUi};
use eframe::egui;

const HEADER_HEIGHT: f32 = 32.0;

impl EditorUi {
    pub(in crate::ui) fn inspector(&mut self, ui: &mut egui::Ui) {
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

fn inspector_section(ui: &mut egui::Ui, title: String, add_contents: impl FnOnce(&mut egui::Ui)) {
    let width = ui.available_width();
    ui.add_space(16.0);

    // Header
    let header_response = egui::Frame::new()
        .fill(theme::tertiary_bg(ui.visuals().dark_mode))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(width, HEADER_HEIGHT),
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(title)
                                .size(theme::large_font_size())
                                .color(theme::primary_fg(ui.visuals().dark_mode))
                                .bold(),
                        )
                        .selectable(false),
                    );
                },
            );
        });
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
    egui::Frame::new()
        .inner_margin(egui::vec2(12.0, 8.0))
        .show(ui, |ui| {
            ui.style_mut().spacing.item_spacing.y = 8.0;
            add_contents(ui);
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
