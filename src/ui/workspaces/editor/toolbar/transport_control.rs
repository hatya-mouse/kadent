use crate::ui::{
    components::{
        icon_button::{toolbar_icon_button, toolbar_icon_button_colored},
        toolbar_group::toolbar_group,
    },
    theme,
    workspaces::EditorUi,
};
use eframe::egui;
use kadent_engine::thread::{AudioCommand, AudioError};

impl EditorUi {
    pub(super) fn transport_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../../assets/icons/backward.png"
                )),
            )
            .clicked()
            {
                let command = AudioCommand::Seek(self.proj_ctx.project.range_start);
                if self
                    .thread_handle
                    .audio_command_tx
                    .send(command.clone())
                    .is_err()
                {
                    self.errors.push(AudioError::CommandFailed(command));
                }
            }

            let play_button_color = if self.ui_state.is_playing {
                Some(theme::transport_green(ui.visuals().dark_mode))
            } else {
                None
            };
            if toolbar_icon_button_colored(
                ui,
                egui::Image::new(egui::include_image!("../../../../../assets/icons/play.png")),
                play_button_color,
            )
            .clicked()
                && !self.ui_state.is_playing
            {
                let command = AudioCommand::Play;
                if self
                    .thread_handle
                    .audio_command_tx
                    .send(command.clone())
                    .is_err()
                {
                    self.errors.push(AudioError::CommandFailed(command));
                } else {
                    self.ui_state.is_playing = true;
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../../assets/icons/pause.png"
                )),
            )
            .clicked()
                && self.ui_state.is_playing
            {
                let command = AudioCommand::Pause;
                if self
                    .thread_handle
                    .audio_command_tx
                    .send(command.clone())
                    .is_err()
                {
                    self.errors.push(AudioError::CommandFailed(command));
                } else {
                    self.ui_state.is_playing = false;
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../../assets/icons/forward.png"
                )),
            )
            .clicked()
            {
                let command = AudioCommand::Seek(
                    self.proj_ctx.project.range_start + self.proj_ctx.project.range_duration,
                );
                if self
                    .thread_handle
                    .audio_command_tx
                    .send(command.clone())
                    .is_err()
                {
                    self.errors.push(AudioError::CommandFailed(command));
                }
            }
        });
    }
}
