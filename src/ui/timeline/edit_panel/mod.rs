mod track_row;

use crate::{app::KnodiqApp, colors};
use eframe::egui;
use std::sync::atomic::Ordering;

impl KnodiqApp {
    pub(crate) fn track_edit_panel(&mut self, ui: &mut egui::Ui, edit_rect: egui::Rect) {
        egui::Frame::new()
            .fill(colors::tertiary_bg(ui.visuals().dark_mode))
            .show(ui, |ui| {
                // Draw the playhead
                self.playhead(ui, edit_rect);

                // Draw each tracks
                let track_height = self.ui_state.track_height;
                let available = ui.available_rect_before_wrap();

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
                        egui::Stroke::new(1.0, colors::region_stroke(ui.visuals().dark_mode)),
                    );
                }
            });
    }

    fn playhead(&mut self, ui: &mut egui::Ui, edit_rect: egui::Rect) {
        let playhead_sample = self.thread_handle.playhead.load(Ordering::Acquire);
        let available = ui.available_rect_before_wrap();

        // Calculate if the playhead sample has changed
        if self.ui_state.last_playhead != playhead_sample {
            let playhead_beats = self.project.tempo_map.samples_to_beats(playhead_sample);
            self.ui_state.last_playhead_x = self.ui_state.pixels_per_beat * playhead_beats.0 as f32;
            self.ui_state.last_playhead = playhead_sample;
        }

        // Create a new painter to draw on the foreground layer
        let mut painter = ui.ctx().layer_painter(egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("playhead"),
        ));
        painter.set_clip_rect(edit_rect);

        // let painter = ui.painter_at(edit_rect);
        painter.vline(
            available.min.x + self.ui_state.last_playhead_x,
            egui::Rangef {
                min: available.min.y,
                max: available.max.y,
            },
            egui::Stroke::new(2.0, colors::primary_fg(ui.visuals().dark_mode)),
        );

        if self.is_playing {
            ui.ctx().request_repaint();
        }
    }
}
