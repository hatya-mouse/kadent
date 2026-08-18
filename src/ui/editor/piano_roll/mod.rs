mod note_grid;
mod ruler;

use crate::{
    consts::PANEL_HEADER_HEIGHT,
    core::metadata::TrackType,
    ui::{
        EditorState,
        components::{
            panel_header::panel_header,
            ruler::{RulerConfig, ruler_and_scroll_bar},
        },
        editor::{
            piano_roll::ruler::{note_grid_ruler, note_pitch_ruler},
            state::TimelineCoord,
        },
    },
};
use eframe::egui;
use kadent_engine::track::note_track::NoteTrack;

impl EditorState {
    pub fn piano_roll(&mut self, ui: &mut egui::Ui) {
        let Some((track_id, region_id)) = self.selection.track_and_region_id() else {
            ui.label("Select a note region to edit");
            return;
        };

        // If the selected track is not a note track, we cannot edit it in the piano roll
        if self
            .project
            .meta
            .get_track(&track_id)
            .is_none_or(|track| track.track_type != TrackType::Note)
        {
            ui.label("Select a note region to edit");
            return;
        }

        // Get the target region
        let Some(track) = self
            .project
            .data
            .tracks
            .get_mut(&track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
        else {
            ui.label("Select a note region to edit");
            return;
        };
        let Some(region) = track.get_region_mut(&region_id) else {
            return;
        };

        // Calculate the rect to show the ruler and the note grid
        let total_rect = ui.available_rect_before_wrap();
        let ruler_bottom_y = total_rect.min.y + PANEL_HEADER_HEIGHT;
        let ruler_screen_rect = total_rect.with_max_y(ruler_bottom_y);
        let note_grid_rect = total_rect.with_min_y(ruler_bottom_y);

        let timeline_coord_key = ui.id().with("timeline_coord");
        let timeline_coord = ui.data(|data| {
            data.get_temp(timeline_coord_key)
                .unwrap_or(TimelineCoord::new(80.0, 10.0, egui::vec2(0.0, 0.0)))
        });

        // Calculate the total width and height of the scroll area content (128 MIDI notes)
        let (region_start, region_end) = region.bounds.tick_range(&self.project.data.tempo_map);
        let region_duration = region_end - region_start;
        let last_note_end = region
            .notes
            .values()
            .map(|note| (note.start + note.duration).0)
            .max()
            .unwrap_or(0);
        let content_end_ticks = region_duration.0.max(last_note_end);
        let scroll_content_width = (content_end_ticks as f32
            * timeline_coord.ppt(self.project.data.audio_ctx.resolution))
        .max(note_grid_rect.width());
        let scroll_content_height = (128.0 * timeline_coord.y_zoom).max(note_grid_rect.height());
        let scroll_content_size = egui::vec2(scroll_content_width, scroll_content_height);

        let (new_scroll_x, ruler_res) = panel_header(ui, egui::Margin::ZERO, |ui| {
            let ruler_config =
                RulerConfig::new(region_start, 0.0, self.project.data.audio_ctx.resolution);

            // Show the ruler at the top of the note grid
            ruler_and_scroll_bar(
                ui,
                ruler_screen_rect,
                &timeline_coord,
                &ruler_config,
                scroll_content_width,
                ruler_screen_rect.width(),
            )
        })
        .inner;
        self.apply_ruler_res(&ruler_res);

        // Draw the notes
        let scroll_output = egui::ScrollArea::both()
            .scroll_offset(timeline_coord.scroll)
            .show(ui, |ui| {
                ui.set_min_size(scroll_content_size);

                // Draw the note grid
                let region_duration_beats = (region_duration.0 as f32
                    / self.project.data.audio_ctx.resolution as f32)
                    .ceil() as i32;
                note_pitch_ruler(ui, &timeline_coord, note_grid_rect);
                note_grid_ruler(ui, &timeline_coord, note_grid_rect, region_duration_beats);

                // Then draw the notes on top of the note grid
                self.draw_notes(
                    ui,
                    &timeline_coord,
                    note_grid_rect,
                    scroll_content_size,
                    track_id,
                    region_id,
                )
            });

        // Prioritize scroll bar click over the scroll area's own offset
        let mut new_timeline_coord = match new_scroll_x {
            Some(new_scroll_x) => {
                timeline_coord.with_scroll(egui::vec2(new_scroll_x, timeline_coord.scroll.y))
            }
            None => scroll_output
                .inner
                .unwrap_or_else(|| timeline_coord.with_scroll(scroll_output.state.offset)),
        };

        // Clamp the scroll by zero and the end of the content so that it never exceeds the content
        // especially when zooming out
        let max_scroll_x = (scroll_content_width - note_grid_rect.width()).max(0.0);
        let max_scroll_y = (scroll_content_height - note_grid_rect.height()).max(0.0);
        new_timeline_coord.scroll = egui::vec2(
            new_timeline_coord.scroll.x.clamp(0.0, max_scroll_x),
            new_timeline_coord.scroll.y.clamp(0.0, max_scroll_y),
        );

        println!("new_timeline_coord: {:?}", new_timeline_coord);

        ui.data_mut(|data| data.insert_temp(timeline_coord_key, new_timeline_coord));
    }
}
