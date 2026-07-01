mod edit_panel;
mod track_list;

use crate::ui::{theme, workspaces::EditorUi};
use eframe::egui;

pub(crate) const RULER_HEIGHT: f32 = 20.0;
pub(crate) const MIN_TRACK_LIST_WIDTH: f32 = 100.0;
/// Extra pixels of empty space inserted before zero beat
pub(crate) const TIMELINE_LEFT_PADDING: f32 = 50.0;
/// Extra pixels of empty space appended after the last region or project range end
pub(crate) const TIMELINE_RIGHT_PADDING: f32 = 200.0;

impl EditorUi {
    pub fn timeline(&mut self, ui: &mut egui::Ui) {
        let track_list_width = self.ui_state.timeline_state.track_list_width;
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let timeline_scroll_key = ui.id().with("timeline_scroll");
        let timeline_scroll = ui.data(|data| {
            data.get_temp(timeline_scroll_key)
                .unwrap_or(TIMELINE_LEFT_PADDING)
        });

        egui::Panel::top(ui.id().with("ruler"))
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let panel_rect = ui.available_rect_before_wrap();
                let ruler_screen_rect = egui::Rect::from_min_max(
                    egui::pos2(panel_rect.min.x + track_list_width, panel_rect.min.y),
                    egui::pos2(panel_rect.max.x, panel_rect.min.y + RULER_HEIGHT),
                );
                self.beat_ruler(ui, ruler_screen_rect, timeline_scroll);

                let vertical_separator_rect = egui::Rect::from_min_size(
                    egui::pos2(panel_rect.min.x + track_list_width - 1.0, panel_rect.min.y),
                    egui::vec2(2.0, RULER_HEIGHT),
                );
                ui.painter()
                    .rect_filled(vertical_separator_rect, 0, theme::separator());
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let panel_rect = ui.available_rect_before_wrap();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        self.track_list_panel(ui);

                        // Add a divider and make it draggable
                        let divider_rect = egui::Rect::from_min_size(
                            egui::pos2(panel_rect.min.x + track_list_width - 1.0, panel_rect.min.y),
                            egui::vec2(2.0, panel_rect.height()),
                        );

                        // Just allocate rect for the divider
                        // Draw divider later to avoid ScrollArea overlapping the divider
                        let divider_resp = ui.allocate_rect(divider_rect, egui::Sense::drag());

                        let scroll_output = egui::ScrollArea::horizontal()
                            .horizontal_scroll_offset(timeline_scroll)
                            .show(ui, |ui| {
                                ui.set_min_height(panel_rect.height());
                                self.track_edit_panel(ui);
                            });
                        ui.data_mut(|data| {
                            data.insert_temp(timeline_scroll_key, scroll_output.state.offset.x)
                        });

                        // Handle dragging the divider and draw the divider
                        if divider_resp.dragged() {
                            self.ui_state.timeline_state.track_list_width =
                                (self.ui_state.timeline_state.track_list_width
                                    + divider_resp.drag_delta().x)
                                    .max(MIN_TRACK_LIST_WIDTH);
                        }
                        if divider_resp.hovered() {
                            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                            ui.painter()
                                .rect_filled(divider_rect, 0.0, theme::separator_hovered());
                        } else {
                            ui.painter()
                                .rect_filled(divider_rect, 0.0, theme::separator());
                        }
                    });
                });
            });
    }
}
