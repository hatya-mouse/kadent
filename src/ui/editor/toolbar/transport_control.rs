use crate::{
    ui::editor::actions::EditorAction,
    ui::{
        EditorState,
        components::{
            icon_button::{toolbar_icon_button, toolbar_icon_button_colored},
            toolbar_group::toolbar_group,
        },
        theme,
    },
};
use eframe::egui;

impl EditorState {
    pub(super) fn transport_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../assets/icons/backward.png"
                )),
            )
            .clicked()
            {
                self.actions.push_action(EditorAction::Seek(
                    self.project.data.export_range.start_time(),
                ));
            }

            let play_button_color = if self.transport.is_playing {
                Some(theme::transport_green(ui.visuals().dark_mode))
            } else {
                None
            };
            if toolbar_icon_button_colored(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/play.png")),
                play_button_color,
            )
            .clicked()
                && !self.transport.is_playing
            {
                self.actions.push_action(EditorAction::Play);
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/pause.png")),
            )
            .clicked()
                && self.transport.is_playing
            {
                self.actions.push_action(EditorAction::Pause);
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/forward.png")),
            )
            .clicked()
            {
                self.actions.push_action(EditorAction::Seek(
                    self.project.data.export_range.end_time(),
                ));
            }
        });
    }
}
