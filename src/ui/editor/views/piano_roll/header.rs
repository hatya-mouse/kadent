use crate::ui::{
    components::icon_button::small_icon_button_highlighted,
    editor::{PianoRollState, views::piano_roll::PianoRollTool},
};
use eframe::egui::{self, include_image};

impl PianoRollState {
    pub(in crate::ui::editor) fn header(&mut self, ui: &mut egui::Ui) {
        // Draw tool selection
        let selected_tool = &mut self.selected_tool;

        let cursor_icon = include_image!("../../../../../assets/icons/cursor.svg");
        let pencil_icon = include_image!("../../../../../assets/icons/pencil.svg");
        let eraser_icon = include_image!("../../../../../assets/icons/eraser.svg");

        let normal_res = small_icon_button_highlighted(
            ui,
            egui::Image::new(cursor_icon),
            *selected_tool == PianoRollTool::Normal,
        );
        let pencil_res = small_icon_button_highlighted(
            ui,
            egui::Image::new(pencil_icon),
            *selected_tool == PianoRollTool::Add,
        );
        let eraser_res = small_icon_button_highlighted(
            ui,
            egui::Image::new(eraser_icon),
            *selected_tool == PianoRollTool::Remove,
        );

        if normal_res.clicked() {
            *selected_tool = PianoRollTool::Normal;
        } else if pencil_res.clicked() {
            *selected_tool = PianoRollTool::Add;
        } else if eraser_res.clicked() {
            *selected_tool = PianoRollTool::Remove;
        }
    }
}
