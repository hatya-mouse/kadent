use crate::ui::EditorUi;
use eframe::egui;
use kreniq_engine::data_types::Beats;

impl EditorUi {
    pub(in crate::ui::editor::piano_roll) fn calc_note_position(
        &self,
        click_pos: egui::Pos2,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        scroll_amount: egui::Vec2,
    ) -> (Beats, f32) {
        let start = Beats(
            ((scroll_amount.x + click_pos.x - note_grid_rect.min.x)
                / self.ui_state.piano_roll_state.pixels_per_beat) as f64,
        );
        let pitch = ((scroll_content_height - scroll_amount.y - click_pos.y
            + note_grid_rect.min.y)
            / self.ui_state.piano_roll_state.note_height)
            .ceil();

        (start, pitch)
    }
}
