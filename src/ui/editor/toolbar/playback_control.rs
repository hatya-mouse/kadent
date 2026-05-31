use crate::{
    components::icon_button::toolbar_icon_button, ui::EditorUi,
    ui::editor::toolbar::toolbar_group::toolbar_group,
};
use eframe::egui;
use kreniq_engine::thread::{AudioCommand, AudioError};

impl EditorUi {
    pub(super) fn playback_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!(
                    "../../../../assets/icons/backward.png"
                )),
            )
            .clicked()
            {
                let command = AudioCommand::Seek(self.project.range_start);
                if self
                    .thread_handle
                    .audio_command_tx
                    .send(command.clone())
                    .is_err()
                {
                    self.errors.push(AudioError::CommandFailed(command));
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/play.png")),
            )
            .clicked()
                && !self.is_playing
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
                    self.is_playing = true;
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/pause.png")),
            )
            .clicked()
                && self.is_playing
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
                    self.is_playing = false;
                }
            }

            if toolbar_icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../../assets/icons/forward.png")),
            )
            .clicked()
            {
                let command =
                    AudioCommand::Seek(self.project.range_start + self.project.range_duration);
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
