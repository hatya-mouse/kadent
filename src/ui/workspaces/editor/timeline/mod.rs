mod edit_panel;
mod track_list;

use crate::ui::{components::scrolled_panel::scrolled_panel, theme, workspaces::EditorUi};
use eframe::egui;

const RULER_HEIGHT: f32 = 20.0;

impl EditorUi {
    pub fn timeline(&mut self, ui: &mut egui::Ui) {
        let total_rect = ui.available_rect_before_wrap();
        let track_list_width = self.ui_state.timeline_state.track_list_width;

        // Process inputs only when the pointer is over the timeline
        if ui.rect_contains_pointer(total_rect) {
            // Pinch or Ctrl+scroll: horizontal zoom
            let zoom_delta = ui.input(|i| i.zoom_delta());
            if zoom_delta != 1.0 {
                let ppb = self.ui_state.timeline_state.pixels_per_beat;
                self.ui_state.timeline_state.pixels_per_beat =
                    (ppb * zoom_delta).clamp(10.0, 500.0);
            } else {
                // Regular scroll: vertical scroll
                let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                self.ui_state.timeline_state.timeline_scroll_y -= scroll_delta;
            }
        }

        // Clamp the vertical scroll amount
        let track_count = self.project_meta.track_order.len();
        let track_height = self.ui_state.timeline_state.track_height;
        let content_height = track_count as f32 * track_height;
        let visible_height = total_rect.height() - RULER_HEIGHT;
        let max_scroll = (content_height - visible_height).max(0.0);
        self.ui_state.timeline_state.timeline_scroll_y = self
            .ui_state
            .timeline_state
            .timeline_scroll_y
            .clamp(0.0, max_scroll);

        let scroll_y = self.ui_state.timeline_state.timeline_scroll_y;

        // Split total_rect into ruler zone (top) and content zone (below)
        let content_rect = egui::Rect::from_min_max(
            egui::pos2(total_rect.min.x, total_rect.min.y + RULER_HEIGHT),
            total_rect.max,
        );

        // Draw ruler background across the full width
        let ruler_bg_rect =
            egui::Rect::from_min_size(total_rect.min, egui::vec2(total_rect.width(), RULER_HEIGHT));
        ui.painter().rect_filled(
            ruler_bg_rect,
            0.0,
            theme::secondary_bg(ui.visuals().dark_mode),
        );

        // Screen rect where beat markers are drawn (right side, aligned with edit panel)
        let ruler_screen_rect = egui::Rect::from_min_max(
            egui::pos2(total_rect.min.x + track_list_width, total_rect.min.y),
            egui::pos2(total_rect.max.x, total_rect.min.y + RULER_HEIGHT),
        );

        // Add the left track list panel
        let list_rect = egui::Rect::from_min_size(
            content_rect.min,
            egui::vec2(track_list_width, content_rect.height()),
        );
        scrolled_panel(ui, list_rect, scroll_y, |ui| {
            self.track_list_panel(ui);
        });

        // Add the right edit panel
        let edit_rect = egui::Rect::from_min_max(
            egui::pos2(content_rect.min.x + track_list_width, content_rect.min.y),
            content_rect.max,
        );
        scrolled_panel(ui, edit_rect, scroll_y, |ui| {
            egui::ScrollArea::horizontal()
                .min_scrolled_height(edit_rect.height())
                .show(ui, |ui| {
                    let min_height = ui.available_height();
                    ui.set_min_height(min_height);
                    // Beat ruler shares horizontal scroll with edit panel
                    self.beat_ruler(ui, ruler_screen_rect);
                    self.track_edit_panel(ui, edit_rect);
                });
        });

        // Draw the horizontal separator line below the ruler
        ui.painter().hline(
            ruler_bg_rect.x_range(),
            ruler_bg_rect.max.y,
            egui::Stroke::new(1.0, theme::separator()),
        );

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
            .rect_filled(divider_rect, 0.0, theme::separator());
    }
}
