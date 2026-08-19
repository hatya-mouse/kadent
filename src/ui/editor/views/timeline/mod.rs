mod edit_panel;
mod ruler_area;
mod state;
mod track_list;

pub(crate) use state::TimelineState;

use crate::{
    consts::{
        MAX_TRACK_LIST_WIDTH, MIN_TRACK_LIST_WIDTH, TIMELINE_LEFT_PADDING, TIMELINE_RIGHT_PADDING,
    },
    ui::{
        EditorState,
        components::panel_header::panel_header,
        editor::{PanelView, state::TimelineCoord, views::PanelViewState},
        theme,
    },
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};
use uuid::Uuid;

impl EditorState {
    pub(in crate::ui::editor) fn timeline(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let panel_width = ui.available_width();
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let PanelViewState::Timeline {
            mut follow_playhead,
            mut track_list_width,
            mut timeline_coord,
        } = self
            .views
            .get_panel_state_or_insert(panel_id, PanelView::Timeline, || PanelViewState::Timeline {
                follow_playhead: false,
                track_list_width: 200.0,
                timeline_coord: TimelineCoord::new(
                    80.0,
                    50.0,
                    egui::vec2(TIMELINE_LEFT_PADDING, 0.0),
                ),
            })
            .clone()
        else {
            return;
        };

        // While following, keep the playhead centered in the visible track area
        let visible_width = (panel_width - track_list_width).max(0.0);
        if follow_playhead && self.transport.is_playing {
            timeline_coord.scroll.x =
                self.follow_playhead_scroll_offset(&timeline_coord, visible_width);
        }

        // Clamp the timeline_scroll by zero and the end of the timeline content width
        // so that it never scrolls past the scrollable area
        let timeline_width = self.timeline_content_width(&timeline_coord);
        let max_scroll = (timeline_width - visible_width).max(0.0);
        timeline_coord.scroll.x = timeline_coord.scroll.x.clamp(0.0, max_scroll);

        let new_scroll_x = panel_header(ui, egui::Margin::ZERO, |ui| {
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

                        let scroll_output = egui::ScrollArea::horizontal()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                            .horizontal_scroll_offset(timeline_coord.scroll.x)
                            .show(ui, |ui| {
                                ui.set_min_height(panel_rect.height());
                                self.track_edit_panel(ui, &mut timeline_coord, timeline_width)
                            });

                        // If the timeline is scrolled via the top scroll bar, prefer the `new_scroll_x`
                        // If a zoom gesture requested a specific scroll offset this frame, use
                        // that instead of the ScrollArea's own offset
                        let final_offset = new_scroll_x
                            .unwrap_or(scroll_output.inner.unwrap_or(scroll_output.state.offset.x));
                        timeline_coord.scroll.x = final_offset;

                        // Handle dragging the divider and draw the divider
                        if divider_resp.dragged() {
                            track_list_width = (track_list_width + divider_resp.drag_delta().x)
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

        let new_panel_state = PanelViewState::Timeline {
            follow_playhead,
            track_list_width,
            timeline_coord,
        };
        self.views.insert_panel_state(panel_id, new_panel_state);
    }

    /// Returns the horizontal scroll offset that keeps the playhead centered within a viewport
    /// of the given visible width, clamped so it never scrolls past the scrollable content.
    fn follow_playhead_scroll_offset(
        &self,
        timeline_coord: &TimelineCoord,
        visible_width: f32,
    ) -> f32 {
        let ppt = timeline_coord.ppt(self.project.data.audio_ctx.resolution);
        let playhead_content_x =
            TIMELINE_LEFT_PADDING + self.transport.playhead_tick.0 as f32 * ppt;
        let visible_half_width = visible_width * 0.5;

        playhead_content_x - visible_half_width
    }

    /// Calculates the width of the entire timeline based on the current pixels-per-beat and project range.
    ///
    /// ```
    /// |<--                             timeline_content_width                         -->|
    /// |<-- TIMELINE_LEFT_PADDING -->[<-- ProjectData Range -->]<-- TIMELINE_RIGHT_PADDING -->|
    /// ```
    pub(super) fn timeline_content_width(&self, timeline_coord: &TimelineCoord) -> f32 {
        let ppt = timeline_coord.ppt(self.project.data.audio_ctx.resolution);
        let tempo_map = &self.project.data.tempo_map;
        let range_end_ticks = self.project.data.export_range.end_tick(tempo_map).0;
        let last_region_end = self
            .project
            .meta
            .track_order
            .iter()
            .filter_map(|id| self.project.meta.get_track(id))
            .flat_map(|t| t.regions.values())
            .map(|r| r.bounds.start_tick(tempo_map).0 + r.bounds.duration_ticks(tempo_map).0)
            .max()
            .unwrap_or(0);
        let content_end_ticks = range_end_ticks.max(last_region_end);
        TIMELINE_LEFT_PADDING + content_end_ticks as f32 * ppt + TIMELINE_RIGHT_PADDING
    }
}
