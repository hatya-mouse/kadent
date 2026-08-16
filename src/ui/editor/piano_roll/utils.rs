use crate::ui::EditorState;
use eframe::egui;
use kadent_engine::data_types::Ticks;

impl EditorState {
    pub(in crate::ui::editor) fn calc_note_position(
        &self,
        click_pos: egui::Pos2,
        note_grid_rect: egui::Rect,
        scroll_content_height: f32,
        scroll_amount: egui::Vec2,
    ) -> (Ticks, f32) {
        let start = Ticks(
            ((scroll_amount.x + click_pos.x - note_grid_rect.min.x)
                * self.ui_state.piano_roll_ticks_per_pixel()) as i64,
        );
        let pitch = ((scroll_content_height - scroll_amount.y - click_pos.y
            + note_grid_rect.min.y)
            / self.ui_state.piano_roll_state.note_height)
            .ceil();

        (start, pitch)
    }
}
