use eframe::egui;

use crate::{
    consts::{TIMELINE_LEFT_PADDING, TIMELINE_MAX_PPB, TIMELINE_MIN_PPB, TIMELINE_RIGHT_PADDING},
    ui::{EditorState, editor::TimelineCoord},
};

impl EditorState {
    /// Calculates the width of the entire timeline based on the current pixels-per-beat and project range.
    ///
    /// ```
    /// |<--                               timeline_content_width                           -->|
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

    /// Returns the reasonable horizontal scroll offset that keeps the playhead centered within a viewport
    /// of the given visible width, clamped so it never scrolls past the scrollable content.
    pub(super) fn follow_playhead_scroll_offset(
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
}

/// Handles timeline gesture and returns the updated timeline coordinate.
pub(super) fn handle_timeline_zoom(
    ui: &egui::Ui,
    timeline_rect: egui::Rect,
    timeline_coord: &TimelineCoord,
    left_padding: f32,
    min_y_scale: f32,
    max_y_scale: f32,
) -> Option<TimelineCoord> {
    let ppb = timeline_coord.ppb;
    let y_scale = timeline_coord.y_scale;
    let scroll_amount = timeline_coord.scroll;

    let cursor_pos = ui.input(|i| i.pointer.hover_pos())?;
    if !timeline_rect.contains(cursor_pos) {
        return None;
    }

    // Get the zoom amount from the input
    let zoom_delta = ui.input(|i| i.zoom_delta());
    if zoom_delta == 1.0 {
        return None;
    }

    // Only zoom to adjust pixels per beat, and press shift to adjust the note height
    let shift = ui.input(|i| i.modifiers.shift);

    if shift {
        let rows_from_top_at_cursor =
            (scroll_amount.y + cursor_pos.y - timeline_rect.min.y) / y_scale;
        let new_y_scale = (y_scale * zoom_delta).clamp(min_y_scale, max_y_scale);
        let new_scroll_y = zoom_scroll_offset(
            timeline_coord.scroll.y,
            rows_from_top_at_cursor,
            y_scale,
            new_y_scale,
        );

        Some(timeline_coord.with_zoom_and_scroll(
            new_y_scale,
            egui::vec2(scroll_amount.x, new_scroll_y).max(egui::Vec2::ZERO),
        ))
    } else {
        // Horizontal zoom (pixels per beat), centered on the cursor
        let beats_at_cursor =
            (scroll_amount.x + cursor_pos.x - timeline_rect.min.x - left_padding) / ppb;
        let new_ppb = (ppb * zoom_delta).clamp(TIMELINE_MIN_PPB, TIMELINE_MAX_PPB);
        let new_scroll_x = zoom_scroll_offset(scroll_amount.x, beats_at_cursor, ppb, new_ppb);

        Some(timeline_coord.with_ppb_and_scroll(
            new_ppb,
            egui::vec2(new_scroll_x, scroll_amount.y).max(egui::Vec2::ZERO),
        ))
    }
}

/// Returns a new zoom scroll offset so that it zooms around the cursor.
pub(crate) fn zoom_scroll_offset(
    current_offset: f32,
    value_at_cursor: f32,
    old_scale: f32,
    new_scale: f32,
) -> f32 {
    current_offset + value_at_cursor * (new_scale - old_scale)
}
