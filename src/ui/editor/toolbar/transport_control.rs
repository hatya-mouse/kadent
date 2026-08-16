use crate::{
    actions::EditorAction,
    ui::{
        EditorUi,
        components::{
            icon_button::{toolbar_icon_button, toolbar_icon_button_colored},
            toolbar_group::toolbar_group,
        },
        theme,
    },
};
use eframe::egui;

impl EditorUi {
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
                self.push_action(EditorAction::Seek(
                    self.proj_ctx.project_meta.export_range.start_time(),
                ));
            }

            let play_button_color = if self.ui_state.is_playing {
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
                && !self.ui_state.is_playing
            {
                self.push_action(EditorAction::Play);
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/pause.png")),
            )
            .clicked()
                && self.ui_state.is_playing
            {
                self.push_action(EditorAction::Pause);
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/forward.png")),
            )
            .clicked()
            {
                self.push_action(EditorAction::Seek(
                    self.proj_ctx.project_meta.export_range.end_time(),
                ));
            }
        });
    }
}
