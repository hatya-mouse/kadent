mod edit_panel;
mod ruler;
mod ruler_area;
mod scroll_bar;
mod track_list;

use crate::ui::{components::panel_header::panel_header, theme, workspaces::EditorUi};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};

/// The minimum width of the track list panel, in pixels.
pub(crate) const MIN_TRACK_LIST_WIDTH: f32 = 100.0;
/// The maximum width of the track list panel, in pixels.
pub(crate) const MAX_TRACK_LIST_WIDTH: f32 = 800.0;
/// Extra pixels of empty space inserted before zero beat.
pub(crate) const TIMELINE_LEFT_PADDING: f32 = 50.0;
/// Extra pixels of empty space appended after the last region or project range end.
pub(crate) const TIMELINE_RIGHT_PADDING: f32 = 200.0;
/// The minimum pixels per beat.
pub(crate) const TIMELINE_MIN_PPB: f32 = 1.0;
/// The maximum pixels per beat.
pub(crate) const TIMELINE_MAX_PPB: f32 = 4000.0;

impl EditorUi {
    pub fn timeline(&mut self, ui: &mut egui::Ui) {
        let track_list_width = self.ui_state.timeline_state.track_list_width;
        let panel_width = ui.available_width();
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        let follow_playhead_key = ui.id().with("follow_playhead");
        let timeline_scroll_key = ui.id().with("timeline_scroll");

        let mut follow_playhead =
            ui.data(|data| data.get_temp(follow_playhead_key).unwrap_or(false));
        let mut timeline_scroll = ui.data(|data| {
            data.get_temp(timeline_scroll_key)
                .unwrap_or(TIMELINE_LEFT_PADDING)
        });

        // While following, keep the playhead centered in the visible track area
        let visible_width = (panel_width - track_list_width).max(0.0);
        if follow_playhead && self.ui_state.is_playing {
            timeline_scroll = self.follow_playhead_scroll_offset(visible_width);
        }

        // Clamp the timeline_scroll by zero and the end of the timeline content width
        // so that it never scrolls past the scrollable area
        let timeline_width = self.timeline_content_width();
        let max_scroll = (timeline_width - visible_width).max(0.0);
        timeline_scroll = timeline_scroll.clamp(0.0, max_scroll);

        let mut new_scroll_x = None;
        panel_header(ui, egui::Margin::ZERO, |ui| {
            new_scroll_x = self.ruler_area(
                ui,
                timeline_scroll,
                visible_width,
                timeline_width,
                track_list_width,
                &mut follow_playhead,
            );
        });

        ui.data_mut(|data| data.insert_temp(follow_playhead_key, follow_playhead));

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
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                            .horizontal_scroll_offset(timeline_scroll)
                            .show(ui, |ui| {
                                ui.set_min_height(panel_rect.height());
                                self.track_edit_panel(ui, timeline_scroll, timeline_width)
                            });

                        // If the timeline is scrolled via the top scroll bar, prefer the `new_scroll_x`
                        // If a zoom gesture requested a specific scroll offset this frame, use
                        // that instead of the ScrollArea's own offset
                        let final_offset = new_scroll_x
                            .unwrap_or(scroll_output.inner.unwrap_or(scroll_output.state.offset.x));
                        ui.data_mut(|data| data.insert_temp(timeline_scroll_key, final_offset));

                        // Handle dragging the divider and draw the divider
                        if divider_resp.dragged() {
                            self.ui_state.timeline_state.track_list_width =
                                (self.ui_state.timeline_state.track_list_width
                                    + divider_resp.drag_delta().x)
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

    /// Returns the horizontal scroll offset that keeps the playhead centered within a viewport
    /// of the given visible width, clamped so it never scrolls past the scrollable content.
    fn follow_playhead_scroll_offset(&self, visible_width: f32) -> f32 {
        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let playhead_content_x = TIMELINE_LEFT_PADDING + self.ui_state.playhead_tick.0 as f32 * ppt;
        let visible_half_width = visible_width * 0.5;

        playhead_content_x - visible_half_width
    }

    /// Calculates the width of the entire timeline based on the current pixels-per-beat and project range.
    ///
    /// ```
    /// |<--                             timeline_content_width                         -->|
    /// |<-- TIMELINE_LEFT_PADDING -->[<-- Project Range -->]<-- TIMELINE_RIGHT_PADDING -->|
    /// ```
    pub(super) fn timeline_content_width(&self) -> f32 {
        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let tempo_map = &self.proj_ctx.project.tempo_map;
        let range_end_ticks = self.proj_ctx.project.export_range.end_tick(tempo_map).0;
        let last_region_end = self
            .proj_ctx
            .project_meta
            .track_order
            .iter()
            .filter_map(|id| self.proj_ctx.project_meta.get_track(id))
            .flat_map(|t| t.regions.values())
            .map(|r| r.bounds.start_tick(tempo_map).0 + r.bounds.duration_ticks(tempo_map).0)
            .max()
            .unwrap_or(0);
        let content_end_ticks = range_end_ticks.max(last_region_end);
        TIMELINE_LEFT_PADDING + content_end_ticks as f32 * ppt + TIMELINE_RIGHT_PADDING
    }
}
