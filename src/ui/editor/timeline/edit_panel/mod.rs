mod track_row;

use crate::{theme, ui::EditorUi};
use eframe::egui;
use kreniq_engine::{
    data_types::Beats,
    thread::{AudioCommand, AudioError},
};

impl EditorUi {
    pub(crate) fn track_edit_panel(&mut self, ui: &mut egui::Ui, edit_rect: egui::Rect) {
        egui::Frame::new()
            .fill(theme::tertiary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                // Draw the playhead
                self.playhead(ui, edit_rect);

                // Draw each tracks
                let track_height = self.ui_state.timeline_state.track_height;
                let available = ui.available_rect_before_wrap();

                // Delete selected region when the pointer is over the edit panel.
                // Checked here because ui.ui_contains_pointer() is unreliable inside
                // region_gestures (min_rect hasn't expanded yet when the check runs).
                if let Some((track_id, region_id)) = self.ui_state.selected_region {
                    let hovered = ui
                        .input(|i| i.pointer.hover_pos())
                        .is_some_and(|p| available.contains(p));
                    if hovered {
                        let delete = ui.input(|i| i.key_pressed(egui::Key::Delete));
                        let backspace = ui.input(|i| i.key_pressed(egui::Key::Backspace));
                        if delete || backspace {
                            self.remove_region(&track_id, &region_id);
                            self.ui_state.selected_region = None;
                        }
                    }
                }

                let track_order = self.project_meta.track_order.clone();
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
                        egui::Stroke::new(1.0, theme::border(ui.visuals().dark_mode)),
                    );
                }
            });
    }

    /// Draw beat markers on the ruler area and handle click/drag to seek.
    /// Must be called inside the horizontal ScrollArea so that `available.min.x`
    /// reflects the current horizontal scroll offset.
    pub(super) fn beat_ruler(&mut self, ui: &mut egui::Ui, ruler_screen_rect: egui::Rect) {
        let available = ui.available_rect_before_wrap();
        let ppb = self.ui_state.timeline_state.pixels_per_beat;
        let dark_mode = ui.visuals().dark_mode;

        // --- Gesture handling ---
        // During drag: update playhead_beats visually only
        // On release: send AudioCommand::Seek once to avoid spamming the audio thread
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

        // Per-panel seeking flag stored in egui temp data, keyed by ruler position
        let seek_key = egui::Id::new("ruler_seeking").with((
            ruler_screen_rect.min.x as i32,
            ruler_screen_rect.min.y as i32,
        ));
        let seeking: bool = ui.data(|data| data.get_temp(seek_key).unwrap_or(false));

        // Mark drag start when the button is first pressed inside this panel's ruler
        // * `ui` variable here is the local one for the panel
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
            if primary_down {
                // Visual-only update during drag
                if let Some(pos) = hover_pos {
                    let beat = Beats(((pos.x - available.min.x) / ppb).max(0.0) as f64);
                    self.ui_state.playhead_beats = beat;
                }
            }

            if primary_released {
                // Send seek command once on mouse release
                if let Some(pos) = hover_pos {
                    let beat = Beats(((pos.x - available.min.x) / ppb).max(0.0) as f64);
                    self.ui_state.playhead_beats = beat;
                    let command = AudioCommand::Seek(beat);
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
        let mut painter = ui.ctx().layer_painter(egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("beat_ruler"),
        ));
        painter.set_clip_rect(ruler_screen_rect);

        // Determine label interval so labels are at least 60px apart
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
        let left_beat = ((ruler_screen_rect.min.x - available.min.x) / ppb).floor() as i32;
        let right_beat = ((ruler_screen_rect.max.x - available.min.x) / ppb).ceil() as i32;
        let first_label_beat = (left_beat / beats_per_label) * beats_per_label;

        let tick_color = theme::ruler_tick(dark_mode);
        let text_color = theme::ruler_label(dark_mode);

        // Major ticks and labels
        let mut beat = first_label_beat;
        while beat <= right_beat {
            if beat >= 0 {
                let x = available.min.x + beat as f32 * ppb;

                painter.vline(
                    x,
                    egui::Rangef::new(ruler_screen_rect.min.y, ruler_screen_rect.max.y),
                    egui::Stroke::new(1.0, tick_color),
                );

                // Display beat numbers as 1-indexed
                painter.text(
                    egui::pos2(x + 3.0, ruler_screen_rect.min.y + 3.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", beat + 1),
                    egui::FontId::proportional(11.0),
                    text_color,
                );
            }
            beat += beats_per_label;
        }

        // Minor ticks between major ticks (only when zoomed in enough)
        if ppb >= 30.0 && beats_per_label > 1 {
            for sub_beat in left_beat..=right_beat {
                if sub_beat >= 0 && sub_beat % beats_per_label != 0 {
                    let x = available.min.x + sub_beat as f32 * ppb;
                    painter.vline(
                        x,
                        egui::Rangef::new(ruler_screen_rect.max.y - 5.0, ruler_screen_rect.max.y),
                        egui::Stroke::new(1.0, tick_color.gamma_multiply(0.5)),
                    );
                }
            }
        }
    }

    fn playhead(&mut self, ui: &mut egui::Ui, edit_rect: egui::Rect) {
        let available = ui.available_rect_before_wrap();

        let playhead_x =
            self.ui_state.timeline_state.pixels_per_beat * self.ui_state.playhead_beats.0 as f32;

        // Create a new painter to draw on the foreground layer
        let mut painter = ui.ctx().layer_painter(egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("playhead"),
        ));
        painter.set_clip_rect(edit_rect);

        painter.vline(
            available.min.x + playhead_x,
            egui::Rangef {
                min: available.min.y,
                max: available.max.y,
            },
            egui::Stroke::new(2.0, theme::primary_fg(ui.visuals().dark_mode)),
        );
    }
}
