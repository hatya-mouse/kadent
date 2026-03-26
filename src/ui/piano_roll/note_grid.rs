use crate::{app::KnodiqApp, colors};
use eframe::egui;
use knodiq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteID, NoteTrack},
    },
};

impl KnodiqApp {
    pub(super) fn note_grid(
        &mut self,
        ui: &mut egui::Ui,
        grid_rect: egui::Rect,
        track_id: TrackID,
        region_id: RegionID,
    ) {
        // Get the target region
        let Some(track) = self
            .project
            .get_track_mut(&track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
        else {
            ui.label("Select a note region to edit");
            return;
        };
        let Some(region) = track.get_region_mut(&region_id) else {
            return;
        };

        // Get the color of the track
        let Some(track_color) = self
            .project_meta
            .get_track(&track_id)
            .map(|track| track.color)
        else {
            return;
        };

        // Calculate the total size by multiplying the duration of the region
        let total_width = (region.duration.0 as f32
            * self.ui_state.piano_roll_state.pixels_per_beat)
            .max(grid_rect.width());
        // Calculate the total height
        let total_height = 128.0 * self.ui_state.piano_roll_state.note_height;

        let notes = region.notes.clone();

        // Draw the notes
        egui::ScrollArea::both().show(ui, |ui| {
            // Allocate a painter
            let (response, painter) =
                ui.allocate_painter(egui::vec2(total_width, total_height), egui::Sense::click());
            let offset = response.rect.min;

            for (note_id, note) in notes {
                // Calculate the note rect
                let note_x =
                    offset.x + note.start.0 as f32 * self.ui_state.piano_roll_state.pixels_per_beat;
                let note_y =
                    offset.y + (128.0 - note.pitch) * self.ui_state.piano_roll_state.note_height;
                let note_width =
                    note.duration.0 as f32 * self.ui_state.piano_roll_state.pixels_per_beat;
                let note_rect = egui::Rect::from_min_size(
                    egui::pos2(note_x, note_y),
                    egui::vec2(note_width, self.ui_state.piano_roll_state.note_height),
                );

                // Highlight the selected note
                let stroke = if self.ui_state.piano_roll_state.selected_note == Some(note_id) {
                    egui::Stroke::new(2.0, colors::region_selected())
                } else {
                    egui::Stroke::new(1.0, colors::region_stroke())
                };

                // Draw the note
                painter.rect(
                    note_rect,
                    2.0,
                    track_color,
                    stroke,
                    egui::StrokeKind::Inside,
                );

                // Handle note gestures
                self.note_controls(ui, &track_id, &region_id, &note_id, note, note_rect);
            }

            // Handle zoom and track adding gestures
            self.note_grid_gestures(ui, grid_rect, total_height, &track_id, &region_id);
        });
    }

    fn note_grid_gestures(
        &mut self,
        ui: &mut egui::Ui,
        grid_rect: egui::Rect,
        scroll_height: f32,
        track_id: &TrackID,
        region_id: &RegionID,
    ) {
        let response = ui.allocate_rect(grid_rect, egui::Sense::click());

        if response.double_clicked() {
            // Add a new note when double clicked
            if let Some(click_pos) = response.interact_pointer_pos() {
                // Calculate the note start beats and the pitch
                let start = Beats(
                    ((click_pos.x - grid_rect.min.x)
                        / self.ui_state.piano_roll_state.pixels_per_beat)
                        as f64,
                );
                let pitch = ((scroll_height - click_pos.y)
                    / self.ui_state.piano_roll_state.note_height)
                    .floor();

                // Add a note at the position
                let note = Note::new(start, Beats(1.0), pitch, 1.0);
                self.add_note(track_id, region_id, note);
            }
        } else if response.hovered() {
            // Handle pinch zoom gesture
            let zoom_delta = ui.input(|i| i.zoom_delta());

            // Only zoom to adjust note height, and press shift in the meantime to adjust pixels per beat
            if zoom_delta != 1.0 {
                let shift = ui.input(|i| i.modifiers.shift);

                if shift {
                    let pixels_per_beat =
                        self.ui_state.piano_roll_state.pixels_per_beat * zoom_delta;
                    self.ui_state.piano_roll_state.pixels_per_beat =
                        pixels_per_beat.clamp(10.0, 500.0);
                } else {
                    let note_height = self.ui_state.piano_roll_state.note_height * zoom_delta;
                    self.ui_state.piano_roll_state.note_height = note_height.clamp(5.0, 30.0);
                }
            }
        }
    }

    fn note_controls(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
        note: Note,
        note_rect: egui::Rect,
    ) {
        // Check for the delete key input
        if self.ui_state.piano_roll_state.selected_note == Some(*note_id) {
            let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
            let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));

            if delete || backspace {
                // Remove the note from the region
                self.remove_note(track_id, region_id, note_id);
                self.ui_state.piano_roll_state.selected_note = None;
            }
        }

        // Handle note click gesture
        let response = ui.allocate_rect(note_rect, egui::Sense::drag());

        // Calculate the beats from the drag amount
        let delta_beats =
            Beats((response.drag_delta().x / self.ui_state.timeline_state.pixels_per_beat) as f64);
        let new_start = note.start + delta_beats;
        if response.dragged() {
            self.ui_state.piano_roll_state.selected_note = Some(*note_id);

            if let Some(region) = self
                .project
                .get_track_mut(track_id)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(region_id))
            {
                region.set_start(note_id, new_start);
            }
        } else if response.drag_stopped() && new_start != note.start {
            self.set_note_start(track_id, region_id, note_id, new_start);
        }
    }
}
