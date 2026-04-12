use crate::app::KnodiqApp;
use eframe::egui;
use knodiq_engine::data_types::Beats;

impl KnodiqApp {
    pub(in crate::ui::piano_roll) fn calc_note_position(
        &self,
        click_pos: egui::Pos2,
        grid_rect: egui::Rect,
        scroll_height: f32,
        scroll_amount: egui::Vec2,
    ) -> (Beats, f32) {
        let start = Beats(
            ((scroll_amount.x + click_pos.x - grid_rect.min.x)
                / self.ui_state.piano_roll_state.pixels_per_beat) as f64,
        );
        let pitch = ((scroll_height - scroll_amount.y - click_pos.y + grid_rect.min.y)
            / self.ui_state.piano_roll_state.note_height)
            .ceil();

        (start, pitch)
    }
}
