mod track_row;

use crate::ui::{theme, workspaces::EditorUi};
use eframe::egui;
use kadent_engine::{
    data_types::Ticks,
    thread::{AudioCommand, AudioError},
};

impl EditorUi {
    pub(crate) fn track_edit_panel(&mut self, ui: &mut egui::Ui) {
        let track_height = self.ui_state.timeline_state.track_height;
        let available = ui.available_rect_before_wrap();

        // Draw each tracks
        let track_order = self.proj_ctx.project_meta.track_order.clone();
        for (i, track_id) in track_order.iter().enumerate() {
            let y = available.min.y + i as f32 * track_height;
            let row_rect = egui::Rect::from_min_size(
                egui::pos2(available.min.x, y),
                egui::vec2(available.width(), track_height),
            );

            self.track_row(ui, track_id, row_rect);

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
        self.playhead(ui, available);
    }

    pub(super) fn beat_ruler(
        &mut self,
        ui: &mut egui::Ui,
        ruler_screen_rect: egui::Rect,
        scroll_x: f32,
    ) {
        let ppb = self.ui_state.timeline_state.pixels_per_beat;
        let ppt = self.ui_state.timeline_state.pixels_per_beat
            / self.ui_state.audio_ctx.resolution as f32;
        let dark_mode = ui.visuals().dark_mode;
        let origin_x = ruler_screen_rect.min.x - scroll_x;

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

        let seek_key = egui::Id::new("ruler_seeking").with((
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
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        if seeking {
            if primary_down && let Some(pos) = hover_pos {
                let ticks = Ticks(((pos.x - origin_x) / ppt) as u64);
                self.ui_state.playhead_ticks = ticks;
            }

            if primary_released {
                if let Some(pos) = hover_pos {
                    let ticks = Ticks(((pos.x - origin_x) / ppt) as u64);
                    self.ui_state.playhead_ticks = ticks;
                    let command = AudioCommand::Seek(ticks);
                    if self
                        .thread_handle
                        .audio_command_tx
                        .send(command.clone())
                        .is_err()
                    {
                        self.errors.push(AudioError::CommandFailed(command));
                    }
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

        let tick_color = theme::ruler_tick(dark_mode);
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
                    egui::pos2(x + 3.0, ruler_screen_rect.min.y + 3.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", beat),
                    egui::FontId::proportional(11.0),
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

    fn playhead(&mut self, ui: &mut egui::Ui, editor_rect: egui::Rect) {
        let playhead_x = self.ui_state.timeline_state.pixels_per_beat
            * (self.ui_state.playhead_ticks.0 as f32 / self.ui_state.audio_ctx.resolution as f32);

        // Create a new painter to draw on the foreground layer
        ui.painter().vline(
            editor_rect.min.x + playhead_x,
            egui::Rangef {
                min: editor_rect.min.y,
                max: editor_rect.max.y,
            },
            egui::Stroke::new(2.0, theme::primary_fg(ui.visuals().dark_mode)),
        );
    }
}
