mod audio_drop;
mod region_notes;
mod track_row;
mod waveform;

use super::TIMELINE_LEFT_PADDING;
use crate::{
    consts::{TIMELINE_MAX_PPB, TIMELINE_MIN_PPB},
    ui::{EditorState, editor::state::TimelineCoord, theme, zoom::zoom_scroll_offset},
};
use eframe::egui;
use kadent_engine::data_types::Ticks;

impl EditorState {
    pub(crate) fn track_edit_panel(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &mut TimelineCoord,
        timeline_width: f32,
    ) -> Option<f32> {
        let track_height = timeline_coord.y_scale;

        // Ensure the scroll area extends past the project range end (or last region end)
        ui.set_min_width(timeline_width);

        let available = ui.available_rect_before_wrap();

        // Draw each tracks
        let track_order = self.project.meta.track_order.clone();
        for (i, track_id) in track_order.iter().enumerate() {
            let y = available.min.y + i as f32 * track_height;
            let row_rect = egui::Rect::from_min_size(
                egui::pos2(available.min.x, y),
                egui::vec2(available.width(), track_height),
            );

            self.track_row(ui, timeline_coord, track_id, row_rect, available.min.y);

            // Draw a separator
            ui.painter().hline(
                egui::Rangef {
                    min: available.min.x,
                    max: available.min.x + available.width(),
                },
                y + track_height,
                theme::border(ui.visuals().dark_mode),
            );
        }

        // Draw the playhead
        self.playhead(ui, timeline_coord, available);

        // Handle pinch / zoom gesture for timeline zoooooming
        let scroll_override = self.handle_timeline_zoom(ui, timeline_coord, available);

        // Handle dragged or dropped file
        self.try_resolve_audio_drop();
        self.show_dragged_hint(ui);

        scroll_override
    }

    fn playhead(&self, ui: &mut egui::Ui, timeline_coord: &TimelineCoord, editor_rect: egui::Rect) {
        let playhead_x = timeline_coord.ppb
            * (self.transport.playhead_tick.0 as f32
                / self.project.data.audio_ctx.resolution as f32);

        // Create a new painter to draw on the foreground layer
        ui.painter().vline(
            editor_rect.min.x + playhead_x + TIMELINE_LEFT_PADDING,
            egui::Rangef {
                min: editor_rect.min.y,
                max: editor_rect.max.y,
            },
            egui::Stroke::new(2.0, theme::primary_fg(ui.visuals().dark_mode)),
        );
    }

    /// Returns the new scroll offset when a zoom gesture happened this frame and the
    /// horizontal scroll offset needs to be corrected.
    fn handle_timeline_zoom(
        &self,
        ui: &mut egui::Ui,
        timeline_coord: &mut TimelineCoord,
        editor_rect: egui::Rect,
    ) -> Option<f32> {
        let scroll_x = timeline_coord.scroll.x;

        let editor_res = ui.allocate_rect(editor_rect, egui::Sense::hover());
        if !editor_res.hovered() {
            return None;
        }
        let zoom_delta = ui.input(|i| i.zoom_delta());
        if zoom_delta == 1.0 {
            return None;
        }
        let cursor_x = ui.input(|i| i.pointer.hover_pos())?.x;

        // Ticks under the cursor before changing the zoom level
        let ticks_at_cursor = x_to_ticks(
            timeline_coord,
            self.project.data.audio_ctx.resolution,
            cursor_x,
            editor_rect,
        );

        let old_ppb = timeline_coord.ppb;
        let new_ppb = (old_ppb * zoom_delta).clamp(TIMELINE_MIN_PPB, TIMELINE_MAX_PPB);
        timeline_coord.ppb = new_ppb;

        // Shift the scroll offset by however much the position of `ticks_at_cursor` moved due to
        // the zoom change, so it stays under the cursor
        let resolution = self.project.data.audio_ctx.resolution as f32;
        let beats_at_cursor = ticks_at_cursor.0 as f32 / resolution;
        let new_offset = zoom_scroll_offset(scroll_x, beats_at_cursor, old_ppb, new_ppb);
        Some(new_offset.max(0.0))
    }
}

fn x_to_ticks(
    timeline_coord: &TimelineCoord,
    resolution: u64,
    x: f32,
    row_rect: egui::Rect,
) -> Ticks {
    Ticks(((x - row_rect.min.x - TIMELINE_LEFT_PADDING) * timeline_coord.tpp(resolution)) as i64)
        .max(Ticks::ZERO)
}
