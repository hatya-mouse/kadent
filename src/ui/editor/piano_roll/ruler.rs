use crate::ui::{editor::state::TimelineCoord, theme};
use eframe::egui;

const NOTE_GRID_FACTOR: f32 = 6.0;

pub(super) fn note_pitch_ruler(
    ui: &mut egui::Ui,
    timeline_coord: &TimelineCoord,
    note_grid_rect: egui::Rect,
) {
    let painter = ui.painter_at(note_grid_rect);
    let grid_color_note = theme::border_color(ui.visuals().dark_mode);
    let grid_color_octave = ui.visuals().window_stroke().color;

    let note_height = timeline_coord.y_zoom;
    let origin_y = note_grid_rect.min.y - timeline_coord.scroll.y;
    // Only show per note grid lines if the note height is large enough
    let show_per_note_grid = note_height >= NOTE_GRID_FACTOR;

    for midi_note in 0u32..=128 {
        let y = origin_y + (128.0 - midi_note as f32) * note_height;
        let is_octave_boundary = midi_note % 12 == 0;

        if is_octave_boundary {
            painter.hline(
                note_grid_rect.min.x..=note_grid_rect.max.x,
                y,
                egui::Stroke::new(1.0, grid_color_octave),
            );
        } else if show_per_note_grid {
            painter.hline(
                note_grid_rect.min.x..=note_grid_rect.max.x,
                y,
                egui::Stroke::new(0.5, grid_color_note),
            );
        }
    }
}

pub(super) fn note_grid_ruler(
    ui: &egui::Ui,
    timeline_coord: &TimelineCoord,
    note_grid_rect: egui::Rect,
    total_beats: i32,
) {
    let painter = ui.painter_at(note_grid_rect);
    let grid_color = theme::border_color(ui.visuals().dark_mode);

    // Vertical beat grid lines, interval adapts to zoom level
    let ppb = timeline_coord.ppb;
    let origin_x = note_grid_rect.min.x - timeline_coord.scroll.x;
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

    // Major beat lines
    let mut beat = 0;
    while beat <= total_beats {
        let x = origin_x + beat as f32 * ppb;
        painter.vline(
            x,
            note_grid_rect.min.y..=note_grid_rect.max.y,
            egui::Stroke::new(1.0, grid_color),
        );
        beat += beats_per_line;
    }

    // Minor beat lines between major lines when zoomed in enough
    if ppb >= 30.0 && beats_per_line > 1 {
        for sub_beat in 0..=total_beats {
            if sub_beat % beats_per_line != 0 {
                let x = origin_x + sub_beat as f32 * ppb;
                painter.vline(
                    x,
                    note_grid_rect.min.y..=note_grid_rect.max.y,
                    egui::Stroke::new(0.5, grid_color.gamma_multiply(0.5)),
                );
            }
        }
    }
}
