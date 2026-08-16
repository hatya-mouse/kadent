use crate::{
    actions::EditorAction,
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
            state.push_action(EditorAction::Seek(
                state
                    .ui_state
                    .proj_ctx
                    .project_meta
                    .export_range
                    .start_time(),
            ));
        }

        let play_button_color = if state.ui_state.is_playing {
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
            && !state.ui_state.is_playing
        {
            state.push_action(EditorAction::Play);
        }

        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/pause.png")),
        )
        .clicked()
            && state.ui_state.is_playing
        {
            state.push_action(EditorAction::Pause);
        }

        if toolbar_icon_button(
            ui,
            egui::Image::new(egui::include_image!("../../../../assets/icons/forward.png")),
        )
        .clicked()
        {
            state.push_action(EditorAction::Seek(
                state.ui_state.proj_ctx.project_meta.export_range.end_time(),
            ));
        }
    });
}
