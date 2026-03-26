mod edit_panel;
mod track_list;

use crate::{app::KnodiqApp, components::scrolled_panel::scrolled_panel};
use eframe::egui;

impl KnodiqApp {
    pub(super) fn timeline(&mut self, ui: &mut egui::Ui) {
        let total_rect = ui.available_rect_before_wrap();
        let track_list_width = self.ui_state.timeline_state.track_list_width;
        let timeline_scroll_y = self.ui_state.timeline_state.timeline_scroll_y;

        // Get the scroll delta
        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
        self.ui_state.timeline_state.timeline_scroll_y -= scroll_delta;

        // Clamp the scroll amount
        let track_count = self.project_meta.track_order.len();
        let track_height = 40.0;
        let content_height = track_count as f32 * track_height;
        let max_scroll = (content_height - total_rect.height()).max(0.0);
        self.ui_state.timeline_state.timeline_scroll_y = timeline_scroll_y.clamp(0.0, max_scroll);

        // Synchronize the scroll amount
        let scroll_y = timeline_scroll_y;

        // Add the left track list panel
        let list_rect = egui::Rect::from_min_size(
            total_rect.min,
            egui::vec2(track_list_width, total_rect.height()),
        );
        scrolled_panel(ui, list_rect, scroll_y, |ui| {
            self.track_list_panel(ui);
        });

        // Add the right edit panel
        let edit_rect = egui::Rect::from_min_max(
            egui::pos2(total_rect.min.x + track_list_width, total_rect.min.y),
            total_rect.max,
        );
        scrolled_panel(ui, edit_rect, scroll_y, |ui| {
            egui::ScrollArea::horizontal()
                .min_scrolled_height(edit_rect.height())
                .show(ui, |ui| {
                    let min_height = ui.available_height();
                    ui.set_min_height(min_height);
                    self.track_edit_panel(ui, edit_rect);
                });
        });

        // Add a divider and make it draggable
        let divider_rect = egui::Rect::from_min_size(
            egui::pos2(total_rect.min.x + track_list_width - 1.0, total_rect.min.y),
            egui::vec2(2.0, total_rect.height()),
        );
        let divider_resp = ui.allocate_rect(divider_rect, egui::Sense::drag());
        if divider_resp.dragged() {
            self.ui_state.timeline_state.track_list_width += divider_resp.drag_delta().x;
        }
        ui.painter()
            .rect_filled(divider_rect, 0.0, egui::Color32::DARK_GRAY);
    }
}
