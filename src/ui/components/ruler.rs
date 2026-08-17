use crate::{
    actions::EditorAction,
    ui::{EditorState, theme},
};
use eframe::egui;
use kadent_engine::{data_types::Ticks, timing::TimePosition};

pub(crate) fn beat_ruler(
    ui: &mut egui::Ui,
    state: &mut EditorState,
    ruler_screen_rect: egui::Rect,
    origin_x: f32,
) {
    let ppb = state.ui_state.timeline_state.pixels_per_beat;
    let ppt = ppb / state.ui_state.audio_ctx.resolution as f32;
    let dark_mode = ui.visuals().dark_mode;

    // --- Gesture handling ---
    let (hover_pos, press_origin, primary_pressed, primary_down, primary_released) =
        ui.input(|i| {
            (
                i.pointer.hover_pos(),
                i.pointer.press_origin(),
                i.pointer.primary_pressed(),
                i.pointer.primary_down(),
                i.pointer.primary_released(),
            )
        });

    let seek_key = ui.id().with("ruler_seeking").with((
        ruler_screen_rect.min.x as i32,
        ruler_screen_rect.min.y as i32,
    ));
    let seeking: bool = ui.data(|data| data.get_temp(seek_key).unwrap_or(false));

    if primary_pressed
        && let Some(origin) = press_origin
        && ruler_screen_rect.contains(origin)
    {
        ui.data_mut(|data| data.insert_temp(seek_key, true));
    }

    if let Some(pos) = hover_pos
        && ruler_screen_rect.contains(pos)
    {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }

    if seeking {
        if primary_down && let Some(pos) = hover_pos {
            let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
            state.ui_state.playhead_tick = ticks;
        }

        if primary_released {
            if let Some(pos) = hover_pos {
                let ticks = Ticks(((pos.x - origin_x) / ppt) as i64).max(Ticks(0));
                let time = TimePosition::Musical(ticks);
                state.push_action(EditorAction::Seek(time));
            }
            ui.data_mut(|data| data.remove::<bool>(seek_key));
        }

        if !primary_down {
            ui.data_mut(|data| data.remove::<bool>(seek_key));
        }
    }

    // --- Drawing ---
    let painter = ui.painter().with_clip_rect(ruler_screen_rect);

    let raw_interval = (60.0_f32 / ppb).ceil() as i32;
    let beats_per_label = if raw_interval <= 1 {
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

    // Visible beat range
    let left_beat = ((ruler_screen_rect.min.x - origin_x) / ppb).floor() as i32;
    let right_beat = ((ruler_screen_rect.max.x - origin_x) / ppb).ceil() as i32;
    let first_label_beat = (left_beat / beats_per_label) * beats_per_label;

    let tick_color = theme::border_color(dark_mode);
    let text_color = theme::ruler_label(dark_mode);

    // Major ticks and labels
    let mut beat = first_label_beat;
    while beat <= right_beat {
        if beat >= 0 {
            let x = origin_x + beat as f32 * ppb;

            painter.vline(
                x,
                egui::Rangef::new(ruler_screen_rect.min.y, ruler_screen_rect.max.y),
                egui::Stroke::new(1.0, tick_color),
            );

            painter.text(
                egui::pos2(x + 3.0, ruler_screen_rect.max.y - 2.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{}", beat),
                egui::FontId::proportional(12.0),
                text_color,
            );
        }
        beat += beats_per_label;
    }

    // Minor ticks between major ticks
    if ppb >= 30.0 && beats_per_label > 1 {
        for sub_beat in left_beat..=right_beat {
            if sub_beat >= 0 && sub_beat % beats_per_label != 0 {
                let x = origin_x + sub_beat as f32 * ppb;
                painter.vline(
                    x,
                    egui::Rangef::new(ruler_screen_rect.max.y - 5.0, ruler_screen_rect.max.y),
                    egui::Stroke::new(1.0, tick_color.gamma_multiply(0.5)),
                );
            }
        }
    }
}
