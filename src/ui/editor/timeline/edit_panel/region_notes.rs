//! Renders the notes on the region box in the timeline.

use crate::ui::{EditorUi, theme};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    mixer::TrackID,
    track::{RegionID, note_track::NoteTrack},
};

const NOTE_HEIGHT_MULTIPLIER: f32 = 3.0;

impl EditorUi {
    pub(super) fn draw_notes_in(
        &mut self,
        ui: &egui::Ui,
        track_id: &TrackID,
        region_id: &RegionID,
        region_rect: &egui::Rect,
    ) {
        let Some(region) = self
            .proj_ctx
            .project
            .get_track(track_id)
            .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
            .and_then(|t| t.get_region(region_id))
        else {
            return;
        };
        let rect_width = region_rect.width();
        let rect_height = region_rect.height();

        // Calculate the pixels per ticks based on the region's duration and the width of the region rectangle
        let region_duration = region
            .bounds
            .duration_ticks(&self.proj_ctx.project.tempo_map);
        let ppt = rect_width / region_duration.0 as f32;
        // Also calculate the y position per pitch
        let y_per_pitch = rect_height / 128.0;

        let painter = ui.painter_at(*region_rect);

        // Draw each note in the region
        for note in region.notes.values() {
            if note.start < Ticks(0) || note.start >= region_duration {
                // Skip notes that are outside the visible range
                continue;
            }

            let note_x = region_rect.left() + (note.start.0 as f32 * ppt);
            let note_y = region_rect.bottom() - (note.pitch * y_per_pitch);
            let note_width = note.duration.0 as f32 * ppt;
            let note_height = y_per_pitch * NOTE_HEIGHT_MULTIPLIER;

            let note_rect = egui::Rect::from_min_size(
                egui::pos2(note_x, note_y - note_height),
                egui::vec2(note_width, note_height),
            );

            painter.rect_filled(note_rect, 0.0, theme::waveform_color());
        }
    }
}
