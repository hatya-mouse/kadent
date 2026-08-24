mod edit_panel;
mod ruler_area;
mod state;
mod track_list;

pub(crate) use state::TimelineState;

use crate::{
    consts::{
        MAX_TRACK_HEIGHT, MAX_TRACK_LIST_WIDTH, MIN_TRACK_HEIGHT, MIN_TRACK_LIST_WIDTH,
        TIMELINE_LEFT_PADDING,
    },
    ui::{
        EditorState,
        components::panel_header::panel_header,
        editor::{utils::handle_timeline_zoom, views::PanelViewState},
        theme,
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};

impl TimelineState {
    pub(in crate::ui::editor) fn ui(
        &mut self,
        ui: &mut egui::Ui,
        panel_state: &mut PanelViewState,
        state: &mut EditorState,
    ) {
        let panel_width = ui.available_width();
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let PanelViewState::Timeline {
            follow_playhead,
            track_list_width,
            timeline_coord,
        } = panel_state
        else {
            return;
        };

        // While following, keep the playhead centered in the visible track area
        let visible_width = (panel_width - *track_list_width).max(0.0);
        if *follow_playhead && state.transport.is_playing {
            timeline_coord.scroll.x =
                self.follow_playhead_scroll_offset(&timeline_coord, visible_width);
        }

        // Clamp the timeline_scroll by zero and the end of the timeline content width
        // so that it never scrolls past the scrollable area
        let timeline_width = state.timeline_content_width(&timeline_coord);
        let max_scroll = (timeline_width - visible_width).max(0.0);
        timeline_coord.scroll.x = timeline_coord.scroll.x.clamp(0.0, max_scroll);

        let bar_scroll_x = panel_header(ui, egui::Margin::ZERO, |ui| {
            self.ruler_area(
                ui,
                &timeline_coord,
                visible_width,
                timeline_width,
                track_list_width,
                &mut follow_playhead,
            )
        })
        .inner;

        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let panel_rect = ui.available_rect_before_wrap();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        self.track_list_panel(ui, &timeline_coord, track_list_width);

                        // Add a divider and make it draggable
                        let divider_rect = egui::Rect::from_min_size(
                            egui::pos2(panel_rect.min.x + track_list_width - 1.0, panel_rect.min.y),
                            egui::vec2(2.0, panel_rect.height()),
                        );

                        // Just allocate rect for the divider
                        // Draw divider later to avoid ScrollArea overlapping the divider
                        let divider_resp = ui.allocate_rect(divider_rect, egui::Sense::drag());

                        let scroll_res = egui::ScrollArea::horizontal()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                            .scroll_offset(timeline_coord.scroll)
                            .show(ui, |ui| {
                                ui.set_min_height(panel_rect.height());
                                self.track_edit_panel(ui, &timeline_coord, timeline_width)
                            });
                        let zoom_gesture_res = handle_timeline_zoom(
                            ui,
                            panel_rect.with_min_x(panel_rect.min.x + *track_list_width),
                            &timeline_coord,
                            TIMELINE_LEFT_PADDING,
                            MIN_TRACK_HEIGHT,
                            MAX_TRACK_HEIGHT,
                        );
                        timeline_coord.apply_scroll(
                            bar_scroll_x,
                            zoom_gesture_res,
                            scroll_res.state.offset,
                        );

                        // Handle divider drag and then draw the divider
                        if divider_resp.dragged() {
                            *track_list_width = (*track_list_width + divider_resp.drag_delta().x)
                                .min(panel_width * 0.5)
                                .clamp(MIN_TRACK_LIST_WIDTH, MAX_TRACK_LIST_WIDTH);
                        }
                        if divider_resp.hovered() {
                            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                            ui.painter().rect_filled(
                                divider_rect,
                                0.0,
                                theme::separator_hovered(ui.visuals().dark_mode),
                            );
                        } else {
                            ui.painter().rect_filled(
                                divider_rect,
                                0.0,
                                theme::separator(ui.visuals().dark_mode),
                            );
                        }
                    });
                });
            });
    }
}
