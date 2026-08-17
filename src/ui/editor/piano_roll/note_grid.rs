use crate::{
    actions::EditorAction,
    consts::PANEL_HEADER_HEIGHT,
    ui::{
        EditorState,
        components::ruler::ruler_and_scroll_bar,
        editor::state::{Modification, TimelineCoord},
        theme,
        zoom::zoom_scroll_offset,
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

pub(super) fn note_grid(
    ui: &mut egui::Ui,
    state: &mut EditorState,
    rect: egui::Rect,
    track_id: TrackID,
    region_id: RegionID,
) {
    // Get the target region
    let Some(track) = state
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
    let Some(track_color) = state
        .ui_state
        .proj_ctx
        .project_meta
        .get_track(&track_id)
        .map(|track| track.color)
    else {
        return;
    };

    // Get the timeline coordinate and calculate other variables
    let ruler_bottom_y = rect.min.y + PANEL_HEADER_HEIGHT;
    let ruler_screen_rect = rect.with_max_y(ruler_bottom_y);
    let note_grid_rect = rect.with_min_y(ruler_bottom_y);

    let timeline_coord_key = ui.id().with("timeline_coord");
    let timeline_coord: TimelineCoord = ui.data(|data| {
        data.get_temp(timeline_coord_key)
            .unwrap_or(TimelineCoord::new(80.0, 10.0, egui::vec2(0.0, 0.0)))
    });

    let ppb = timeline_coord.ppb;
    let note_height = timeline_coord.y_zoom;
    let scroll_amount = timeline_coord.scroll;
    let ppt = ppb / state.ui_state.audio_ctx.resolution as f32;

    // Calculate the total size of the scroll area content
    let region_duration = region
        .bounds
        .duration_ticks(&state.ui_state.proj_ctx.project.tempo_map);
    let last_note_end = region
        .notes
        .values()
        .map(|note| (note.start + note.duration).0)
        .max()
        .unwrap_or(0);
    let content_end_ticks = region_duration.0.max(last_note_end);
    let scroll_content_width = (content_end_ticks as f32 * ppt).max(note_grid_rect.width());

    // Calculate the total height of the scroll area content (128 MIDI notes)
    let scroll_content_height = (128.0 * note_height).max(note_grid_rect.height());

    let notes = region.notes.clone();

    // Show the ruler at the top of the note grid
    let (new_scroll_x, ruler_res) = ruler_and_scroll_bar(
        ui,
        ruler_screen_rect,
        &timeline_coord,
        state.ui_state.audio_ctx.resolution,
        scroll_content_width,
        ruler_screen_rect.width(),
    );
    state.apply_ruler_res(&ruler_res);

    // Draw the notes
    let scroll_output = egui::ScrollArea::both()
        .scroll_offset(scroll_amount)
        .show(ui, |ui| {
            // Allocate a painter
            let (response, painter) = ui.allocate_painter(
                egui::vec2(scroll_content_width, scroll_content_height),
                egui::Sense::click(),
            );
            let offset = response.rect.min;

            // Draw the note grid
            draw_note_grid(
                ui,
                &painter,
                &timeline_coord,
                offset,
                scroll_content_width,
                scroll_content_height,
            );

            for (note_id, note) in notes {
                // Calculate the note rect
                let note_x = offset.x + note.start.0 as f32 * ppt;
                let note_y = offset.y + (128.0 - note.pitch) * note_height;
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
                note_controls(
                    ui,
                    state,
                    &timeline_coord,
                    (&track_id, &region_id, &note_id),
                    &note,
                    note_rect,
                    resize_rect,
                );

                // Highlight the selected note
                let stroke = if state.ui_state.selection.note_id() == Some(note_id) {
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
            note_grid_gestures(
                ui,
                state,
                note_grid_rect,
                scroll_content_height,
                &timeline_coord,
                &track_id,
                &region_id,
            )
        });

    // Prioritize scroll bar click over the scroll area's own offset
    let scroll_timeline_coord = scroll_output
        .inner
        .unwrap_or_else(|| timeline_coord.clone());
    let mut new_timeline_coord = match new_scroll_x {
        Some(new_scroll_x) => {
            timeline_coord.with_scroll(egui::vec2(new_scroll_x, timeline_coord.scroll.y))
        }
        None => scroll_timeline_coord,
    };

    // Clamp the scroll by zero and the end of the content so that it never exceeds the content
    // especially when zooming out
    let max_scroll_x = (scroll_content_width - note_grid_rect.width()).max(0.0);
    let max_scroll_y = (scroll_content_height - note_grid_rect.height()).max(0.0);
    new_timeline_coord.scroll = egui::vec2(
        new_timeline_coord.scroll.x.clamp(0.0, max_scroll_x),
        new_timeline_coord.scroll.y.clamp(0.0, max_scroll_y),
    );

    ui.data_mut(|data| data.insert_temp(timeline_coord_key, new_timeline_coord));
}

fn draw_note_grid(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    timeline_coord: &TimelineCoord,
    offset: egui::Pos2,
    scroll_content_width: f32,
    scroll_content_height: f32,
) {
    let grid_color_note = theme::border_color(ui.visuals().dark_mode);
    let grid_color_octave = ui.visuals().window_stroke().color;

    let note_height = timeline_coord.y_zoom;
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

    note_grid_ruler(
        painter,
        timeline_coord,
        scroll_content_width,
        scroll_content_height,
        offset,
        grid_color_note,
    );
}

fn note_grid_ruler(
    painter: &egui::Painter,
    timeline_coord: &TimelineCoord,
    scroll_content_width: f32,
    scroll_content_height: f32,
    offset: egui::Pos2,
    grid_color_note: egui::Color32,
) {
    // Vertical beat grid lines, interval adapts to zoom level
    let ppb = timeline_coord.ppb;
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
// and returns the new scroll timeline coordinate that preserves the scroll position after zooming.
fn note_grid_gestures(
    ui: &mut egui::Ui,
    state: &mut EditorState,
    note_grid_rect: egui::Rect,
    scroll_content_height: f32,
    timeline_coord: &TimelineCoord,
    track_id: &TrackID,
    region_id: &RegionID,
) -> Option<TimelineCoord> {
    let resolution = state.ui_state.audio_ctx.resolution;
    let ppb = timeline_coord.ppb;
    let note_height = timeline_coord.y_zoom;
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
            let note_duration = state
                .ui_state
                .piano_roll_state
                .last_edited_note_length
                .unwrap_or(Ticks(state.ui_state.audio_ctx.resolution as i64));

            // Add a note at the position
            let note = Note::new(start, note_duration, pitch, 1.0);
            state.push_action(EditorAction::AddNote(*track_id, *region_id, note));

            // Play the note for feedback
            state.play_note_feedback(pitch.round() as u8, 255u8);
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
        let rows_from_top_at_cursor =
            (scroll_amount.y + cursor_pos.y - note_grid_rect.min.y) / note_height;
        let new_note_height = (note_height * zoom_delta).clamp(5.0, 30.0);
        let new_scroll_y = zoom_scroll_offset(
            timeline_coord.scroll.y,
            rows_from_top_at_cursor,
            note_height,
            new_note_height,
        );

        Some(timeline_coord.with_zoom_and_scroll(
            new_note_height,
            egui::vec2(scroll_amount.x, new_scroll_y.max(0.0)),
        ))
    } else {
        // Horizontal zoom (pixels per beat), centered on the cursor
        let beats_at_cursor = (scroll_amount.x + cursor_pos.x - note_grid_rect.min.x) / ppb;
        let new_ppb = (ppb * zoom_delta).clamp(10.0, 500.0);
        let new_scroll_x = zoom_scroll_offset(scroll_amount.x, beats_at_cursor, ppb, new_ppb);

        Some(timeline_coord.with_ppb_and_scroll(new_ppb, egui::vec2(new_scroll_x, scroll_amount.y)))
    }
}

fn note_controls(
    ui: &mut egui::Ui,
    state: &mut EditorState,
    timeline_coord: &TimelineCoord,
    note_id: (&TrackID, &RegionID, &NoteID),
    note: &Note,
    note_rect: egui::Rect,
    resize_rect: egui::Rect,
) {
    let note_height = timeline_coord.y_zoom;
    let resolution = state.ui_state.audio_ctx.resolution;

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
            .ui_state
            .select_note(*note_id.0, *note_id.1, *note_id.2);

        // Calculate the new duration from the drag amount
        let delta_ticks =
            Ticks((resize_res.drag_delta().x * timeline_coord.tpp(resolution)) as i64);

        if let Some(region) = state
            .ui_state
            .proj_ctx
            .project
            .get_track_mut(note_id.0)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
            .and_then(|track| track.get_region_mut(note_id.1))
        {
            let new_duration = (note.duration + delta_ticks).max(Ticks(0));
            region.set_duration(note_id.2, new_duration);

            state.ui_state.set_modification(Modification::NotePosition(
                note.start,
                note.duration + delta_ticks,
                note.pitch,
            ));
        }
    } else if resize_res.drag_stopped()
        && let Some(new_duration) = state
            .ui_state
            .proj_ctx
            .project
            .get_track(note_id.0)
            .and_then(|track| track.as_any().downcast_ref::<NoteTrack>())
            .and_then(|track| track.get_region(note_id.1))
            .and_then(|region| region.get_duration(note_id.2))
    {
        state.ui_state.piano_roll_state.last_edited_note_length = Some(new_duration);
        state.push_action(EditorAction::SetNoteDuration(
            *note_id.0,
            *note_id.1,
            *note_id.2,
            new_duration,
        ));
    }

    if move_res.dragged() {
        let delta_ticks = Ticks((move_res.drag_delta().x * timeline_coord.tpp(resolution)) as i64);
        let delta_pitch = -move_res.drag_delta().y / note_height;

        state
            .ui_state
            .select_note(*note_id.0, *note_id.1, *note_id.2);

        if let Some(region) = state
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

            state.ui_state.set_modification(Modification::NotePosition(
                new_start,
                note.duration,
                new_pitch,
            ));
        }
    } else if move_res.drag_stopped() {
        let committed = state
            .ui_state
            .proj_ctx
            .project
            .get_track(note_id.0)
            .and_then(|t| t.as_any().downcast_ref::<NoteTrack>())
            .and_then(|t| t.get_region(note_id.1))
            .and_then(|r| Some((r.get_start(note_id.2)?, r.get_pitch(note_id.2)?)));
        if let Some((new_start, new_pitch)) = committed {
            state.ui_state.piano_roll_state.last_edited_note_length = Some(note.duration);
            state.push_action(EditorAction::MoveNote(
                *note_id.0, *note_id.1, *note_id.2, new_start,
            ));
            state.push_action(EditorAction::SetNotePitch(
                *note_id.0,
                *note_id.1,
                *note_id.2,
                new_pitch.round(),
            ));

            // Play the note for feedback
            state.play_note_feedback(new_pitch.round() as u8, (note.velocity * 127.0) as u8);
        }
    }
}

pub(in crate::ui::editor) fn calc_note_position(
    tpp: f32,
    click_pos: egui::Pos2,
    note_grid_rect: egui::Rect,
    note_height: f32,
    scroll_content_height: f32,
    scroll_amount: egui::Vec2,
) -> (Ticks, f32) {
    let start = Ticks(((scroll_amount.x + click_pos.x - note_grid_rect.min.x) * tpp) as i64);
    let pitch = ((scroll_content_height - scroll_amount.y - click_pos.y + note_grid_rect.min.y)
        / note_height)
        .ceil();

    (start, pitch)
}
