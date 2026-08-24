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

pub(super) fn transport_control(ui: &mut egui::Ui, state: &mut EditorState) {
    toolbar_group(ui, |ui| {
        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!(
                "../../../../assets/icons/backward.png"
            )),
        )
        .clicked()
        {
            state.actions.push_action(EditorAction::Seek(
                state.project.data.export_range.start_time(),
            ));
        }

        let play_button_color = if state.transport.is_playing {
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
            && !state.transport.is_playing
        {
            state.actions.push_action(EditorAction::Play);
        }

        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/pause.png")),
        )
        .clicked()
            && state.transport.is_playing
        {
            state.actions.push_action(EditorAction::Pause);
        }

        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/forward.png")),
        )
        .clicked()
        {
            state.actions.push_action(EditorAction::Seek(
                state.project.data.export_range.end_time(),
            ));
        }
    });
}
