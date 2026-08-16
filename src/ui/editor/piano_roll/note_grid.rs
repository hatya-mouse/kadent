use crate::{
    actions::EditorAction,
    ui::{
        theme,
        zoom::zoom_scroll_offset,
        {EditorState, editor::state::Modification},
    },
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

const NOTE_GRID_FACTOR: f32 = 6.0;

impl EditorState {
    pub(super) fn note_grid(
        &mut self,
        ui: &mut egui::Ui,
        note_grid_rect: egui::Rect,
        track_id: TrackID,
        region_id: RegionID,
    ) {
        // Get the target region
        let Some(track) = self
            .ui_state
            .proj_ctx
            .project
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

        // Get the color of the track
        let Some(track_color) = self
            .ui_state
            .proj_ctx
            .project_meta
            .get_track(&track_id)
            .map(|track| track.color)
        else {
            return;
        };

        let ppt = self.ui_state.piano_roll_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;

        // Calculate the total size of the scroll area content
        let region_duration = region
            .bounds
            .duration_ticks(&self.ui_state.proj_ctx.project.tempo_map);
        let scroll_content_width = (region_duration.0 as f32 * ppt).max(note_grid_rect.width());
        // Calculate the total height of the scroll area content (128 MIDI notes)
        let scroll_content_height =
            (128.0 * self.ui_state.piano_roll_state.note_height).max(note_grid_rect.height());

        let notes = region.notes.clone();

        let note_grid_scroll_key = ui.id().with("note_grid_scroll");
        let note_grid_scroll = ui.data(|data| {
            data.get_temp(note_grid_scroll_key)
                .unwrap_or(egui::Vec2::ZERO)
        });

        // Draw the notes
        let scroll_output = egui::ScrollArea::both()
            .scroll_offset(note_grid_scroll)
            .show(ui, |ui| {
                // Allocate a painter
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(scroll_content_width, scroll_content_height),
                    egui::Sense::click(),
                );
                let offset = response.rect.min;

                // Draw the note grid
                self.draw_note_grid(
                    ui,
                    &painter,
                    offset,
                    scroll_content_width,
                    scroll_content_height,
                );

                for (note_id, note) in notes {
                    // Calculate the note rect
                    let note_x = offset.x + note.start.0 as f32 * ppt;
                    let note_y = offset.y
                        + (128.0 - note.pitch) * self.ui_state.piano_roll_state.note_height;
                    let note_width = note.duration.0 as f32 * ppt;
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
                        (&track_id, &region_id, &note_id),
                        &note,
                        note_rect,
                        resize_rect,
                    );

                    // Highlight the selected note
                    let stroke = if self.ui_state.selection.note_id() == Some(note_id) {
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

                // Handle zoom and track adding gestures
                self.note_grid_gestures(
                    ui,
                    note_grid_rect,
                    scroll_content_height,
                    note_grid_scroll,
                    &track_id,
                    &region_id,
                )
            });

        let final_offset = scroll_output.inner.unwrap_or(scroll_output.state.offset);
        ui.data_mut(|data| data.insert_temp(note_grid_scroll_key, final_offset));
    }

    fn draw_note_grid(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        offset: egui::Pos2,
        scroll_content_width: f32,
        scroll_content_height: f32,
    ) {
        let grid_color_note = theme::border_color(ui.visuals().dark_mode);
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

        self.note_grid_ruler(
            painter,
            scroll_content_width,
            scroll_content_height,
            offset,
            grid_color_note,
        );
    }

    fn note_grid_ruler(
        &self,
        painter: &egui::Painter,
        scroll_content_width: f32,
        scroll_content_height: f32,
        offset: egui::Pos2,
        grid_color_note: egui::Color32,
    ) {
        // Vertical beat grid lines, interval adapts to zoom level
        let ppb = self.ui_state.piano_roll_state.pixels_per_beat;
        let raw_interval = (30.0_f32 / ppb).ceil() as i32;
        let beats_per_line = if raw_interval <= 1 {
            1
        } else if raw_interval <= 2 {
            2
        } else if raw_interval <= 4 {
            4
        } else if raw_interval <= 8 {
            8
        } else if raw_interval <= 16 {
            16
        } else {
            ((raw_interval + 31) / 32) * 32
        };

        let total_beats = (scroll_content_width / ppb).ceil() as i32;
        let y_range = egui::Rangef::new(offset.y, offset.y + scroll_content_height);

        // Major beat lines
        let mut beat = 0;
        while beat <= total_beats {
            let x = offset.x + beat as f32 * ppb;
            painter.vline(x, y_range, egui::Stroke::new(1.0, grid_color_note));
            beat += beats_per_line;
        }

        // Minor beat lines between major lines when zoomed in enough
        if ppb >= 30.0 && beats_per_line > 1 {
            for sub_beat in 0..=total_beats {
                if sub_beat % beats_per_line != 0 {
                    let x = offset.x + sub_beat as f32 * ppb;
                    painter.vline(
                        x,
                        y_range,
                        egui::Stroke::new(0.5, grid_color_note.gamma_multiply(0.5)),
                    );
                }
            }
        }
    }

    // Handle gestures on the note grid, such as adding notes and zooming,
    // and returns the new scroll offset to preserve the scroll position after zooming.
    fn note_grid_gestures(
        &mut self,
        ui: &mut egui::Ui,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        scroll_amount: egui::Vec2,
        track_id: &TrackID,
        region_id: &RegionID,
    ) -> Option<egui::Vec2> {
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

                // Set the length of the note to the lenght of the last edited note
                let note_duration = self
                    .ui_state
                    .piano_roll_state
                    .last_edited_note_length
                    .unwrap_or(Ticks(self.ui_state.audio_ctx.resolution as i64));

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

        let zoom_delta = ui.input(|i| i.zoom_delta());
        if zoom_delta == 1.0 {
            return None;
        }
        let cursor_pos = ui.input(|i| i.pointer.hover_pos())?;

        // Only zoom to adjust pixels per beat, and press shift to adjust the note height
        let shift = ui.input(|i| i.modifiers.shift);

        if shift {
            let rows_from_top_at_cursor = (scroll_amount.y + cursor_pos.y - note_grid_rect.min.y)
                / self.ui_state.piano_roll_state.note_height;

            let old_note_height = self.ui_state.piano_roll_state.note_height;
            let new_note_height = (old_note_height * zoom_delta).clamp(5.0, 30.0);
            self.ui_state.piano_roll_state.note_height = new_note_height;

            let new_scroll_y = zoom_scroll_offset(
                scroll_amount.y,
                rows_from_top_at_cursor,
                old_note_height,
                new_note_height,
            );
            Some(egui::vec2(scroll_amount.x, new_scroll_y.max(0.0)))
        } else {
            // Horizontal zoom (pixels per beat), centered on the cursor
            let beats_at_cursor = (scroll_amount.x + cursor_pos.x - note_grid_rect.min.x)
                / self.ui_state.piano_roll_state.pixels_per_beat;

            let old_ppb = self.ui_state.piano_roll_state.pixels_per_beat;
            let new_ppb = (old_ppb * zoom_delta).clamp(10.0, 500.0);
            self.ui_state.piano_roll_state.pixels_per_beat = new_ppb;

            let new_scroll_x =
                zoom_scroll_offset(scroll_amount.x, beats_at_cursor, old_ppb, new_ppb);
            Some(egui::vec2(new_scroll_x.max(0.0), scroll_amount.y))
        }
    }

    fn note_controls(
        &mut self,
        ui: &mut egui::Ui,
        note_id: (&TrackID, &RegionID, &NoteID),
        note: &Note,
        note_rect: egui::Rect,
        resize_rect: egui::Rect,
    ) {
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
            self.ui_state
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            // Calculate the new duration from the drag amount
            let delta_ticks = Ticks(
                (resize_res.drag_delta().x * self.ui_state.piano_roll_ticks_per_pixel()) as i64,
            );

            if let Some(region) = self
                .ui_state
                .proj_ctx
                .project
                .get_track_mut(note_id.0)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(note_id.1))
            {
                let new_duration = (note.duration + delta_ticks).max(Ticks(0));
                region.set_duration(note_id.2, new_duration);

                self.ui_state.set_modification(Modification::NotePosition(
                    note.start,
                    note.duration + delta_ticks,
                    note.pitch,
                ));
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = self
                .ui_state
                .proj_ctx
                .project
                .get_track(note_id.0)
                .and_then(|track| track.as_any().downcast_ref::<NoteTrack>())
                .and_then(|track| track.get_region(note_id.1))
                .and_then(|region| region.get_duration(note_id.2))
        {
            self.ui_state.piano_roll_state.last_edited_note_length = Some(new_duration);
            self.push_action(EditorAction::SetNoteDuration(
                *note_id.0,
                *note_id.1,
                *note_id.2,
                new_duration,
            ));
        }

        if move_res.dragged() {
            let delta_ticks = Ticks(
                (move_res.drag_delta().x * self.ui_state.piano_roll_ticks_per_pixel()) as i64,
            );
            let delta_pitch = -move_res.drag_delta().y / self.ui_state.piano_roll_state.note_height;

            self.ui_state
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            if let Some(region) = self
                .ui_state
                .proj_ctx
                .project
                .get_track_mut(note_id.0)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(note_id.1))
            {
                let new_start = note.start + delta_ticks;
                let new_pitch = (note.pitch + delta_pitch).clamp(0.0, 127.0);
                region.set_start(note_id.2, new_start);
                region.set_pitch(note_id.2, new_pitch);

                self.ui_state.set_modification(Modification::NotePosition(
                    new_start,
                    note.duration,
                    new_pitch,
                ));
            }
        } else if move_res.drag_stopped() {
            let committed = self
                .ui_state
                .proj_ctx
                .project
                .get_track(note_id.0)
                .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
                .and_then(|t| t.get_region(note_id.1))
                .and_then(|r| Some((r.get_start(note_id.2)?, r.get_pitch(note_id.2)?)));
            if let Some((new_start, new_pitch)) = committed {
                self.ui_state.piano_roll_state.last_edited_note_length = Some(note.duration);
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
