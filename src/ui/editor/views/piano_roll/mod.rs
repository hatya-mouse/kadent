mod gestures;
mod header;
mod note_grid;
mod ruler;

use crate::consts::PANEL_HEADER_HEIGHT;
use crate::core::{
    audio_engine::{data_types::Ticks, track::note_track::NoteTrack},
    metadata::TrackType,
};
use crate::ui::editor::utils::handle_timeline_zoom;
use crate::ui::{
    EditorState,
    components::not_available_text::not_available_text,
    components::{
        panel_header::panel_header,
        ruler::{RulerConfig, ruler_and_scroll_bar},
    },
    editor::TimelineCoord,
};
use eframe::egui::{self, scroll_area::ScrollBarVisibility};
use ruler::{note_grid_ruler, note_pitch_ruler};
use std::time::Instant;

const MIN_NOTE_HEIGHT: f32 = 2.0;
const MAX_NOTE_HEIGHT: f32 = 30.0;

#[derive(Default)]
pub(crate) struct PianoRollState {
    /// MIDI note numbers and Instants for currently playing preview notes.
    pub(crate) preview_notes: Vec<(u8, Instant)>,
    /// Length of the last edited note.
    last_edited_note_length: Option<Ticks>,
    /// The currently selected tool.
    selected_tool: PianoRollTool,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum PianoRollTool {
    #[default]
    Normal,
    Add,
    Remove,
}

#[derive(Clone, Debug)]
pub(crate) struct PianoRollPanelState {
    pub(crate) timeline_coord: TimelineCoord,
}

impl Default for PianoRollPanelState {
    fn default() -> Self {
        Self {
            timeline_coord: TimelineCoord::new(100.0, 200.0, egui::Vec2::ZERO),
        }
    }
}

impl PianoRollState {
    pub(in crate::ui::editor) fn ui(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_state: &mut PianoRollPanelState,
    ) {
        let timeline_coord = &mut panel_state.timeline_coord;

        let Some((track_id, region_id)) = state.selection.track_and_region_id() else {
            not_available_text(ui, "Select a note region to edit");
            return;
        };

        // If the selected track is not a note track, we cannot edit it in the piano roll
        if state
            .project
            .meta
            .get_track(&track_id)
            .is_none_or(|track| track.track_type != TrackType::Note)
        {
            not_available_text(ui, "Select a note region to edit");
            return;
        }

        // Get the target region
        let Some(track) = state
            .project
            .data
            .tracks
            .get_mut(&track_id)
            .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
        else {
            not_available_text(ui, "Select a note region to edit");
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

        // Calculate the total width and height of the scroll area content (128 MIDI notes)
        let (region_start, region_end) = region.bounds.tick_range(&state.project.data.tempo_map);
        let region_duration = region_end - region_start;
        let last_note_end = region
            .notes
            .values()
            .map(|note| (note.start + note.duration).0)
            .max()
            .unwrap_or(0);
        let content_end_ticks = region_duration.0.max(last_note_end);
        let scroll_content_width = (content_end_ticks as f32
            * timeline_coord.ppt(state.project.data.audio_ctx.resolution))
        .max(note_grid_rect.width());
        let scroll_content_height = (128.0 * timeline_coord.y_scale).max(note_grid_rect.height());
        let scroll_content_size = egui::vec2(scroll_content_width, scroll_content_height);

        let (bar_scroll_x, ruler_res) = panel_header(ui, egui::Margin::ZERO, |ui| {
            let ruler_config =
                RulerConfig::new(region_start, 0.0, state.project.data.audio_ctx.resolution);

            // Show the ruler at the top of the note grid
            ruler_and_scroll_bar(
                ui,
                ruler_screen_rect,
                timeline_coord,
                &ruler_config,
                scroll_content_width,
                ruler_screen_rect.width(),
            )
        })
        .inner;
        state.apply_ruler_res(&ruler_res);

        // Draw the notes
        let scroll_res = egui::ScrollArea::both()
            .id_salt("piano_roll")
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .scroll_offset(timeline_coord.scroll)
            .show(ui, |ui| {
                ui.set_min_size(scroll_content_size);

                // Draw the note grid
                let region_duration_beats = (region_duration.0 as f32
                    / state.project.data.audio_ctx.resolution as f32)
                    .ceil() as i32;
                note_pitch_ruler(ui, timeline_coord, note_grid_rect);
                note_grid_ruler(ui, timeline_coord, note_grid_rect, region_duration_beats);

                // Then draw the notes on top of the note grid
                self.draw_notes(
                    ui,
                    state,
                    timeline_coord,
                    note_grid_rect,
                    track_id,
                    region_id,
                );
            });

        // Handle note tool gestures
        self.note_grid_gestures(
            ui,
            state,
            timeline_coord,
            note_grid_rect,
            scroll_content_size.y,
            &(track_id, region_id),
        );

        // Handle the zoom gesture and apply the scroll offset
        let zoom_gesture_res = handle_timeline_zoom(
            ui,
            note_grid_rect,
            timeline_coord,
            0.0,
            MIN_NOTE_HEIGHT,
            MAX_NOTE_HEIGHT,
        );
        timeline_coord.apply_scroll(bar_scroll_x, zoom_gesture_res, scroll_res.state.offset);

        // Clamp the scroll by zero and the end of the content so that it never exceeds the content
        // especially when zooming out
        let max_scroll_x = (scroll_content_width - note_grid_rect.width()).max(0.0);
        let max_scroll_y = (scroll_content_height - note_grid_rect.height()).max(0.0);
        timeline_coord.scroll = egui::vec2(
            timeline_coord.scroll.x.clamp(0.0, max_scroll_x),
            timeline_coord.scroll.y.clamp(0.0, max_scroll_y),
        );
    }
}
