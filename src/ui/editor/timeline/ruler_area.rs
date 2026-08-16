use crate::{
    consts::{PANEL_HEADER_HEIGHT, PANEL_HEADER_MARGIN},
    ui::{EditorUi, components::icon_button::small_icon_button, theme},
};
use eframe::egui;

/// The height of the timeline scroll bar.
const SCROLL_BAR_HEIGHT: f32 = 12.0;

impl EditorUi {
    /// Returns the new scroll position if the user scrolled the timeline, otherwise returns `None`.
    pub(super) fn ruler_area(
        &mut self,
        ui: &mut egui::Ui,
        scroll_x: f32,
        visible_width: f32,
        timeline_width: f32,
        track_list_width: f32,
        follow_playhead: &mut bool,
    ) -> Option<f32> {
        let panel_rect = ui.available_rect_before_wrap();

        let corner_rect = egui::Rect::from_min_size(
            panel_rect.min,
            egui::vec2(track_list_width, PANEL_HEADER_HEIGHT),
        );
        self.follow_playhead_button(ui, corner_rect, follow_playhead);

        // Top half: shows the scroll bar for scrolling the entire timeline horizontally
        // Bottom half: shows the beat rule
        let scroll_bar_left_x = panel_rect.min.x + track_list_width;
        let scroll_bar_bottom_y = panel_rect.min.y + SCROLL_BAR_HEIGHT;
        let ruler_bottom_y = panel_rect.min.y + PANEL_HEADER_HEIGHT;
        let scroll_bar_rect = egui::Rect::from_min_max(
            egui::pos2(scroll_bar_left_x, panel_rect.min.y),
            egui::pos2(panel_rect.max.x, scroll_bar_bottom_y),
        );
        let ruler_screen_rect = egui::Rect::from_min_max(
            egui::pos2(scroll_bar_left_x, scroll_bar_bottom_y),
            egui::pos2(panel_rect.max.x, ruler_bottom_y),
        );
        let new_scroll_x =
            self.scroll_bar(ui, scroll_bar_rect, scroll_x, visible_width, timeline_width);
        self.beat_ruler(ui, ruler_screen_rect, scroll_x);

        let vertical_separator_rect = egui::Rect::from_min_size(
            egui::pos2(panel_rect.min.x + track_list_width - 1.0, panel_rect.min.y),
            egui::vec2(2.0, PANEL_HEADER_HEIGHT),
        );
        ui.painter().rect_filled(
            vertical_separator_rect,
            0,
            theme::separator(ui.visuals().dark_mode),
        );

        new_scroll_x
    }

    /// Draws the follow-playhead toggle in the corner cell above the track list, toggling
    /// `follow_playhead` when clicked.
    fn follow_playhead_button(
        &self,
        ui: &mut egui::Ui,
        corner_rect: egui::Rect,
        follow_playhead: &mut bool,
    ) {
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(corner_rect.shrink2(PANEL_HEADER_MARGIN.right_bottom())),
            |ui| {
                ui.horizontal_centered(|ui| {
                    let icon = egui::include_image!("../../../../assets/icons/crosshair.svg");
                    let response = small_icon_button(ui, egui::Image::new(icon));
                    if response.clicked() {
                        *follow_playhead = !*follow_playhead;
                    }
                    if *follow_playhead {
                        ui.painter()
                            .rect_filled(response.rect, 6.0, theme::icon_button_active());
                    }
                    response.on_hover_text("Follow playhead");
                });
            },
        );
    }
}
