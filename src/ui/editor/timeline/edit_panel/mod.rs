mod audio_drop;
mod region_notes;
mod track_row;
mod waveform;

use crate::{
    consts::{TIMELINE_MAX_PPB, TIMELINE_MIN_PPB},
    ui::{
        EditorState,
        editor::{
            state::EditorUiState,
            timeline::{
                TIMELINE_LEFT_PADDING,
                edit_panel::{audio_drop::show_dragged_hint, track_row::track_row},
            },
        },
        theme,
        zoom::zoom_scroll_offset,
    },
};
use eframe::egui;
use kadent_engine::{data_types::Ticks, mixer::TrackID};

pub(crate) fn track_edit_panel(
    ui: &mut egui::Ui,
    state: &mut EditorState,
    scroll_x: f32,
    timeline_width: f32,
) -> Option<f32> {
    let track_height = state.ui_state.timeline_state.track_height;

    // Ensure the scroll area extends past the project range end (or last region end)
    ui.set_min_width(timeline_width);

    let available = ui.available_rect_before_wrap();

    // Draw each tracks
    let track_order = state.ui_state.proj_ctx.project_meta.track_order.clone();
    for (i, track_id) in track_order.iter().enumerate() {
        let y = available.min.y + i as f32 * track_height;
        let row_rect = egui::Rect::from_min_size(
            egui::pos2(available.min.x, y),
            egui::vec2(available.width(), track_height),
        );

        track_row(ui, state, track_id, row_rect, available.min.y);

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
    playhead(ui, &state.ui_state, available);

    // Handle pinch / zoom gesture for timeline zoooooming
    let scroll_override = handle_timeline_zoom(ui, &mut state.ui_state, available, scroll_x);

    // Handle dragged or dropped file
    state.try_resolve_audio_drop();
    show_dragged_hint(ui, &mut state.ui_state);

    scroll_override
}

fn playhead(ui: &mut egui::Ui, ui_state: &EditorUiState, editor_rect: egui::Rect) {
    let playhead_x = ui_state.timeline_state.pixels_per_beat
        * (ui_state.playhead_tick.0 as f32 / ui_state.audio_ctx.resolution as f32);

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
    ui: &mut egui::Ui,
    ui_state: &mut EditorUiState,
    editor_rect: egui::Rect,
    scroll_x: f32,
) -> Option<f32> {
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
    let ticks_at_cursor = x_to_ticks(ui_state, cursor_x, editor_rect);

    let old_ppb = ui_state.timeline_state.pixels_per_beat;
    let new_ppb = (old_ppb * zoom_delta).clamp(TIMELINE_MIN_PPB, TIMELINE_MAX_PPB);
    ui_state.timeline_state.pixels_per_beat = new_ppb;

    // Shift the scroll offset by however much the position of `ticks_at_cursor` moved due to
    // the zoom change, so it stays under the cursor
    let resolution = ui_state.audio_ctx.resolution as f32;
    let beats_at_cursor = ticks_at_cursor.0 as f32 / resolution;
    let new_offset = zoom_scroll_offset(scroll_x, beats_at_cursor, old_ppb, new_ppb);
    Some(new_offset.max(0.0))
}

fn x_to_ticks(ui_state: &EditorUiState, x: f32, row_rect: egui::Rect) -> Ticks {
    Ticks(
        ((x - row_rect.min.x - TIMELINE_LEFT_PADDING) * ui_state.timeline_ticks_per_pixel()) as i64,
    )
    .max(Ticks(0))
}

/// Converts a screen-space y position to the track it falls into,
/// given the y position of the top of the track list content area.
fn y_to_track_id(ui_state: &EditorUiState, y: f32, content_top: f32) -> Option<TrackID> {
    let track_height = ui_state.timeline_state.track_height;
    if y < content_top || track_height <= 0.0 {
        return None;
    }
    let index = ((y - content_top) / track_height) as usize;
    ui_state
        .proj_ctx
        .project_meta
        .track_order
        .get(index)
        .copied()
}
