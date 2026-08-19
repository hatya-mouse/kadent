use crate::{
    consts::{TIMELINE_LEFT_PADDING, TIMELINE_RIGHT_PADDING},
    ui::{EditorState, editor::TimelineCoord},
};

impl EditorState {
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
