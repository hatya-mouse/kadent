use crate::{
    core::audio_engine::{
        mixer::TrackID,
        track::{RegionID, note_track::NoteTrack},
    },
    ui::{
        EditorState,
        editor::{PianoRollState, TimelineCoord},
        theme,
    },
};
use eframe::egui;

impl PianoRollState {
    pub(super) fn draw_notes(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        note_grid_rect: egui::Rect,
        track_id: TrackID,
        region_id: RegionID,
    ) {
        // Get the target region
        let Some(track) = state
            .project
            .data
            .tracks
            .get_mut(&track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
        else {
            return;
        };
        let Some(region) = track.get_region_mut(&region_id) else {
            return;
        };
        let origin = note_grid_rect.min - timeline_coord.scroll;

        // Get the color of the track
        let track_color = state
            .project
            .meta
            .get_track(&track_id)
            .map(|track| track.color)
            .unwrap_or_default();

        let ppb = timeline_coord.ppb;
        let ppt = ppb / state.project.data.audio_ctx.resolution as f32;
        let note_height = timeline_coord.y_scale;

        // Draw the notes
        let painter = ui.painter_at(note_grid_rect);

        let notes = region.notes.clone();
        for (note_id, note) in notes {
            // Calculate the note rect
            let note_x = origin.x + note.start.0 as f32 * ppt;
            let note_y = origin.y + (127.0 - note.pitch) * note_height;
            let note_width = note.duration.0 as f32 * ppt;
            let note_rect = egui::Rect::from_min_size(
                egui::pos2(note_x, note_y),
                egui::vec2(note_width, note_height),
            );

            // Handle note gestures
            self.note_controls(
                ui,
                state,
                timeline_coord,
                (&track_id, &region_id, &note_id),
                &note,
                note_rect,
            );

            // Highlight the selected note
            let stroke = if state.selection.note_id() == Some(note_id) {
                egui::Stroke::new(2.0, theme::region_selected(ui.visuals().dark_mode))
            } else {
                theme::border(ui.visuals().dark_mode)
            };

            // Draw the note
            painter.rect(
                note_rect,
                2.0,
                track_color,
                stroke,
                egui::StrokeKind::Inside,
            );
        }
    }
}
