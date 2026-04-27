use crate::{theme, ui::EditorUi};
use eframe::egui;
use knodiq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteID, NoteTrack},
    },
};

const NOTE_GRID_FACTOR: f32 = 6.0;

impl EditorUi {
    pub(super) fn note_grid(
        &mut self,
        ui: &mut egui::Ui,
        note_grid_rect: egui::Rect,
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

        // Calculate the total size of the scroll area content
        let scroll_content_width = (region.duration.0 as f32
            * self.ui_state.piano_roll_state.pixels_per_beat)
            .max(note_grid_rect.width());
        // Calculate the total height of the scroll area content (128 MIDI notes)
        let scroll_content_height =
            (128.0 * self.ui_state.piano_roll_state.note_height).max(note_grid_rect.height());

        let notes = region.notes.clone();

        // Draw the notes
        let scroll_area = egui::ScrollArea::both().show(ui, |ui| {
            // Allocate a painter
            let (response, painter) = ui.allocate_painter(
                egui::vec2(scroll_content_width, scroll_content_height),
                egui::Sense::click(),
            );
            let offset = response.rect.min;

            // Draw the note grid
            self.draw_note_grid(ui, &painter, offset, scroll_content_width);

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

                // Create a rect on the right side of the note to drag and resize the note
                let draggable_width = 5.0;
                let resize_rect = egui::Rect::from_min_size(
                    egui::pos2(note_x + note_width - draggable_width, note_y + 2.0),
                    egui::vec2(
                        draggable_width,
                        self.ui_state.piano_roll_state.note_height - 4.0,
                    ),
                );

                // Handle note gestures
                self.note_controls(
                    ui,
                    &track_id,
                    &region_id,
                    &note_id,
                    &note,
                    note_rect,
                    resize_rect,
                );

                // Highlight the selected note
                let stroke = if self.ui_state.selected_note == Some(note_id) {
                    egui::Stroke::new(2.0, theme::region_selected(ui.visuals().dark_mode))
                } else {
                    egui::Stroke::new(1.0, theme::border(ui.visuals().dark_mode))
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
        });

        // Handle zoom and track adding gestures
        self.note_grid_gestures(
            ui,
            note_grid_rect,
            scroll_content_height,
            scroll_area.state.offset,
            &track_id,
            &region_id,
        );
    }

    fn draw_note_grid(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        offset: egui::Pos2,
        scroll_content_width: f32,
    ) {
        let grid_color_note = theme::border(ui.visuals().dark_mode);
        let grid_color_octave = ui.visuals().window_stroke().color;

        let note_height = self.ui_state.piano_roll_state.note_height;
        // Only show per note grid lines if the note height is large enough
        let show_per_note_grid = note_height >= NOTE_GRID_FACTOR;

        for midi_note in 0u32..=128 {
            let y = offset.y + (128.0 - midi_note as f32) * note_height;
            let is_octave_boundary = midi_note % 12 == 0;

            if is_octave_boundary {
                painter.hline(
                    offset.x..=(offset.x + scroll_content_width),
                    y,
                    egui::Stroke::new(1.0, grid_color_octave),
                );
            } else if show_per_note_grid {
                painter.hline(
                    offset.x..=(offset.x + scroll_content_width),
                    y,
                    egui::Stroke::new(0.5, grid_color_note),
                );
            }
        }
    }

    fn note_grid_gestures(
        &mut self,
        ui: &mut egui::Ui,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        scroll_amount: egui::Vec2,
        track_id: &TrackID,
        region_id: &RegionID,
    ) {
        let response = ui.allocate_rect(note_grid_rect, egui::Sense::click());

        if response.double_clicked() {
            // Add a new note when double clicked
            if let Some(click_pos) = response.interact_pointer_pos() {
                // Calculate the note start beats and the pitch
                let (start, pitch) = self.calc_note_position(
                    click_pos,
                    note_grid_rect,
                    scroll_content_height,
                    scroll_amount,
                );

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
        note: &Note,
        note_rect: egui::Rect,
        resize_rect: egui::Rect,
    ) {
        // Check for the delete key input
        if self.ui_state.selected_note == Some(*note_id) && ui.ui_contains_pointer() {
            let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
            let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));

            if delete || backspace {
                // Remove the note from the region
                self.remove_note(track_id, region_id, note_id);
                self.ui_state.selected_note = None;
            }
        }

        // Get gestures on the note
        let move_res = ui.allocate_rect(note_rect, egui::Sense::drag());
        let resize_res = ui.allocate_rect(resize_rect, egui::Sense::drag());

        // If the resize area is hovered, show the resize cursor
        if resize_res.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Handle resize
        if resize_res.dragged() {
            // Select the note
            self.ui_state.set_selected_note(*note_id);

            // Calculate the new duration from the drag amount
            let delta_beats = Beats(
                (resize_res.drag_delta().x / self.ui_state.piano_roll_state.pixels_per_beat) as f64,
            );
            if let Some(region) = self
                .project
                .get_track_mut(track_id)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(region_id))
            {
                region.set_duration(note_id, Beats((note.duration.0 + delta_beats.0).max(0.0)));
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .project
                .get_track(track_id)
                .and_then(|track| track.as_any().downcast_ref::<NoteTrack>())
                .and_then(|track| track.get_region(region_id))
                .and_then(|region| region.get_duration(note_id))
        {
            self.set_note_duration(track_id, region_id, note_id, new_duration);
        }

        if move_res.dragged() {
            let delta_beats = Beats(
                (move_res.drag_delta().x / self.ui_state.piano_roll_state.pixels_per_beat) as f64,
            );
            let delta_pitch = -move_res.drag_delta().y / self.ui_state.piano_roll_state.note_height;
            let new_start = note.start + delta_beats;
            let new_pitch = (note.pitch + delta_pitch).clamp(0.0, 127.0);

            self.ui_state.set_selected_note(*note_id);

            if let Some(region) = self
                .project
                .get_track_mut(track_id)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(region_id))
            {
                region.set_start(note_id, new_start);
                region.set_pitch(note_id, new_pitch);
            }
        } else if move_res.drag_stopped() {
            let committed = self
                .project
                .get_track(track_id)
                .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
                .and_then(|t| t.get_region(region_id))
                .and_then(|r| Some((r.get_start(note_id)?, r.get_pitch(note_id)?)));
            if let Some((new_start, new_pitch)) = committed {
                self.set_note_start(track_id, region_id, note_id, new_start);
                self.set_note_pitch(track_id, region_id, note_id, new_pitch.round());
            }
        }
    }
}
