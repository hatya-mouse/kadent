use crate::ui::{
    EditorState,
    editor::{StatusHint, TimelineCoord, actions::EditorAction, utils::handle_timeline_zoom},
    theme,
};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    mixer::TrackID,
    track::{
        RegionID,
        note_track::{Note, NoteID, NoteTrack},
    },
};

const MIN_NOTE_HEIGHT: f32 = 2.0;
const MAX_NOTE_HEIGHT: f32 = 30.0;

impl EditorState {
    pub(super) fn draw_notes(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        note_grid_rect: egui::Rect,
        scroll_content_size: egui::Vec2,
        track_id: TrackID,
        region_id: RegionID,
    ) -> Option<TimelineCoord> {
        // Get the target region
        let track = self
            .project
            .data
            .tracks
            .get_mut(&track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())?;
        let region = track.get_region_mut(&region_id)?;
        let origin = note_grid_rect.min - timeline_coord.scroll;

        // Get the color of the track
        let track_color = self
            .project
            .meta
            .get_track(&track_id)
            .map(|track| track.color)?;

        let ppb = timeline_coord.ppb;
        let ppt = ppb / self.project.data.audio_ctx.resolution as f32;
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

            // Create a rect on the right side of the note to drag and resize the note
            let draggable_width = 5.0;
            let resize_rect = egui::Rect::from_min_size(
                egui::pos2(note_x + note_width - draggable_width, note_y + 2.0),
                egui::vec2(draggable_width, note_height - 4.0),
            );

            // Handle note gestures
            self.note_controls(
                ui,
                timeline_coord,
                (&track_id, &region_id, &note_id),
                &note,
                note_rect,
                resize_rect,
            );

            // Highlight the selected note
            let stroke = if self.selection.note_id() == Some(note_id) {
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

        // Handle zoom and note adding gestures
        self.note_grid_gestures(
            ui,
            timeline_coord,
            note_grid_rect,
            scroll_content_size.y,
            &track_id,
            &region_id,
        )
    }

    // Handle gestures on the note grid, such as adding notes and zooming,
    // and returns the new scroll timeline coordinate that preserves the scroll position after zooming.
    fn note_grid_gestures(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        track_id: &TrackID,
        region_id: &RegionID,
    ) -> Option<TimelineCoord> {
        let resolution = self.project.data.audio_ctx.resolution;
        let note_height = timeline_coord.y_scale;
        let scroll_amount = timeline_coord.scroll;
        let response = ui.allocate_rect(note_grid_rect, egui::Sense::click());

        if response.double_clicked() {
            // Add a new note when double clicked
            if let Some(click_pos) = response.interact_pointer_pos() {
                // Calculate the note start beats and the pitch
                let (start, pitch) = calc_note_position(
                    timeline_coord.tpp(resolution),
                    click_pos,
                    note_grid_rect,
                    note_height,
                    scroll_content_height,
                    scroll_amount,
                );

                // Set the length of the note to the lenght of the last edited note
                let note_duration = self
                    .views
                    .piano_roll
                    .last_edited_note_length
                    .unwrap_or(Ticks(self.project.data.audio_ctx.resolution as i64));

                // Add a note at the position
                let note = Note::new(start, note_duration, pitch, 1.0);
                self.push_action(EditorAction::AddNote(*track_id, *region_id, note));

                // Play the note for feedback
                self.play_note_feedback(pitch.round() as u8, 255u8);
            }
            return None;
        } else if !response.hovered() {
            return None;
        }

        handle_timeline_zoom(
            ui,
            note_grid_rect,
            timeline_coord,
            0.0,
            MIN_NOTE_HEIGHT,
            MAX_NOTE_HEIGHT,
        )
    }

    fn note_controls(
        &mut self,
        ui: &mut egui::Ui,
        timeline_coord: &TimelineCoord,
        note_id: (&TrackID, &RegionID, &NoteID),
        note: &Note,
        note_rect: egui::Rect,
        resize_rect: egui::Rect,
    ) {
        let note_height = timeline_coord.y_scale;
        let resolution = self.project.data.audio_ctx.resolution;

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
            self.selection
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            // Calculate the new duration from the drag amount
            let delta_ticks =
                Ticks((resize_res.drag_delta().x * timeline_coord.tpp(resolution)) as i64);

            if let Some(region) = self
                .project
                .data
                .get_track_mut(note_id.0)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(note_id.1))
            {
                let new_duration = (note.duration + delta_ticks).max(Ticks::ZERO);
                region.set_duration(note_id.2, new_duration);

                self.views
                    .status_bar
                    .set_status_hint(StatusHint::NotePosition(
                        note.start,
                        note.duration + delta_ticks,
                        note.pitch,
                    ));
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .project
                .data
                .get_track(note_id.0)
                .and_then(|track| track.as_any().downcast_ref::<NoteTrack>())
                .and_then(|track| track.get_region(note_id.1))
                .and_then(|region| region.get_duration(note_id.2))
        {
            self.views.piano_roll.last_edited_note_length = Some(new_duration);
            self.push_action(EditorAction::SetNoteDuration(
                *note_id.0,
                *note_id.1,
                *note_id.2,
                new_duration,
            ));
        }

        if move_res.dragged() {
            let delta_ticks =
                Ticks((move_res.drag_delta().x * timeline_coord.tpp(resolution)) as i64);
            let delta_pitch = -move_res.drag_delta().y / note_height;

            self.selection
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            if let Some(region) = self
                .project
                .data
                .get_track_mut(note_id.0)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(note_id.1))
            {
                let new_start = note.start + delta_ticks;
                let new_pitch = (note.pitch + delta_pitch).clamp(0.0, 127.0);
                region.set_start(note_id.2, new_start);
                region.set_pitch(note_id.2, new_pitch);

                self.views
                    .status_bar
                    .set_status_hint(StatusHint::NotePosition(
                        new_start,
                        note.duration,
                        new_pitch,
                    ));
            }
        } else if move_res.drag_stopped() {
            let committed = self
                .project
                .data
                .get_track(note_id.0)
                .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
                .and_then(|t| t.get_region(note_id.1))
                .and_then(|r| Some((r.get_start(note_id.2)?, r.get_pitch(note_id.2)?)));
            if let Some((new_start, new_pitch)) = committed {
                self.views.piano_roll.last_edited_note_length = Some(note.duration);
                self.push_action(EditorAction::MoveNote(
                    *note_id.0, *note_id.1, *note_id.2, new_start,
                ));
                self.push_action(EditorAction::SetNotePitch(
                    *note_id.0,
                    *note_id.1,
                    *note_id.2,
                    new_pitch.round(),
                ));

                // Play the note for feedback
                self.play_note_feedback(new_pitch.round() as u8, (note.velocity * 127.0) as u8);
            }
        }
    }
}

fn calc_note_position(
    tpp: f32,
    click_pos: egui::Pos2,
    note_grid_rect: egui::Rect,
    note_height: f32,
    scroll_content_height: f32,
    scroll_amount: egui::Vec2,
) -> (Ticks, f32) {
    let start = Ticks(((scroll_amount.x + click_pos.x - note_grid_rect.min.x) * tpp) as i64);
    let pitch = calc_note_pitch(
        click_pos.y,
        note_grid_rect,
        note_height,
        scroll_content_height,
        scroll_amount,
    );

    (start, pitch)
}

fn calc_note_pitch(
    y_pos: f32,
    note_grid_rect: egui::Rect,
    note_height: f32,
    scroll_content_height: f32,
    scroll_amount: egui::Vec2,
) -> f32 {
    ((scroll_content_height - scroll_amount.y - y_pos + note_grid_rect.min.y) / note_height).floor()
}
