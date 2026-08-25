use crate::{
    core::{
        audio_engine::{
            data_types::{MidiEvent, Ticks},
            mixer::TrackID,
            track::{
                RegionID,
                note_track::{Note, NoteID, NoteTrack},
            },
        },
        midi_thread::MidiCommand,
    },
    ui::{
        EditorState,
        editor::{
            PianoRollState, TimelineCoord, actions::EditorAction, views::piano_roll::PianoRollTool,
        },
    },
};
use eframe::egui;

impl PianoRollState {
    // --- CLICK GESTURES ---

    // Handles note grid clicking gestures.
    pub(super) fn note_grid_gestures(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        region_id: &(TrackID, RegionID),
    ) {
        let response = ui.allocate_rect(note_grid_rect, egui::Sense::click());
        let should_add = match self.selected_tool {
            PianoRollTool::Normal => response.double_clicked(),
            PianoRollTool::Add => response.clicked(),
            _ => false,
        };

        if should_add {
            self.add_note_by_response(
                &response,
                state,
                timeline_coord,
                note_grid_rect,
                scroll_content_height,
                region_id,
            );
        }
    }

    fn add_note_by_response(
        &mut self,
        response: &egui::Response,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        region_id: &(TrackID, RegionID),
    ) {
        let resolution = state.project.data.audio_ctx.resolution;
        let note_height = timeline_coord.y_scale;
        let scroll_amount = timeline_coord.scroll;

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
                .last_edited_note_length
                .unwrap_or(Ticks(state.project.data.audio_ctx.resolution as i64));

            // Add a note at the position
            let note = Note::new(start, note_duration, pitch, 1.0);
            state
                .actions
                .push_action(EditorAction::AddNote(region_id.0, region_id.1, note));

            // Play the note for feedback
            self.play_note_feedback(state, pitch, 1.0);
        }
    }

    // --- NOTE GESTURES ---

    pub(super) fn note_controls(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        timeline_coord: &TimelineCoord,
        note_id: (&TrackID, &RegionID, &NoteID),
        note: &Note,
        note_rect: egui::Rect,
        resize_rect: egui::Rect,
    ) {
        let note_height = timeline_coord.y_scale;
        let resolution = state.project.data.audio_ctx.resolution;

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
            state
                .selection
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            // Calculate the new duration from the drag amount
            let delta_ticks =
                Ticks((resize_res.drag_delta().x * timeline_coord.tpp(resolution)) as i64);

            if let Some(region) = state
                .project
                .data
                .get_track_mut(note_id.0)
                .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
                .and_then(|track| track.get_region_mut(note_id.1))
            {
                let new_duration = (note.duration + delta_ticks).max(Ticks::ZERO);
                region.set_duration(note_id.2, new_duration);
            }
        } else if resize_res.drag_stopped()
            && let Some(new_duration) = state
                .project
                .data
                .get_track(note_id.0)
                .and_then(|track| track.as_any().downcast_ref::<NoteTrack>())
                .and_then(|track| track.get_region(note_id.1))
                .and_then(|region| region.get_duration(note_id.2))
        {
            self.last_edited_note_length = Some(new_duration);
            state.actions.push_action(EditorAction::SetNoteDuration(
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

            state
                .selection
                .select_note(*note_id.0, *note_id.1, *note_id.2);

            if let Some(region) = state
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
            }
        } else if move_res.drag_stopped() {
            let committed = state
                .project
                .data
                .get_track(note_id.0)
                .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
                .and_then(|t| t.get_region(note_id.1))
                .and_then(|r| Some((r.get_start(note_id.2)?, r.get_pitch(note_id.2)?)));
            if let Some((new_start, new_pitch)) = committed {
                self.last_edited_note_length = Some(note.duration);
                state.actions.push_action(EditorAction::MoveNote(
                    *note_id.0, *note_id.1, *note_id.2, new_start,
                ));
                state.actions.push_action(EditorAction::SetNotePitch(
                    *note_id.0,
                    *note_id.1,
                    *note_id.2,
                    new_pitch.round(),
                ));

                // Play the note for feedback
                self.play_note_feedback(state, new_pitch, note.velocity);
            }
        }
    }

    fn play_note_feedback(&mut self, state: &EditorState, pitch: f32, velocity: f32) {
        // Play the note for feedback
        let pitch = pitch.clamp(0.0, 127.0).round() as u8;
        let velocity = (velocity.clamp(0.0, 1.0) * 128.0).round() as u8;
        state
            .midi_tx
            .send(MidiCommand::SendEvent(MidiEvent::NoteOn {
                pitch,
                velocity,
            }))
            .ok();
        // Add the note to the preview notes with the current timestamp
        self.preview_notes.push((pitch, std::time::Instant::now()));
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
