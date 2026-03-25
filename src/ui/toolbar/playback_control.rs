use crate::{
    app::KnodiqApp, components::icon_button::icon_button, ui::toolbar::toolbar_group::toolbar_group,
};
use eframe::egui;
use knodiq_engine::audio_thread::{AudioCommand, error::AudioError};

impl KnodiqApp {
    pub(super) fn playback_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/backward.png")),
            )
            .clicked()
            {
                println!("Backward");
            }

            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/play.png")),
            )
            .clicked()
                && !self.is_playing
            {
                if self
                    .thread_handle
                    .command_tx
                    .send(AudioCommand::Play)
                    .is_err()
                {
                    self.errors
                        .push(AudioError::CommandFailed(AudioCommand::Play));
                } else {
                    self.is_playing = true;
                }
            }

            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/pause.png")),
            )
            .clicked()
                && self.is_playing
            {
                if self
                    .thread_handle
                    .command_tx
                    .send(AudioCommand::Pause)
                    .is_err()
                {
                    self.errors
                        .push(AudioError::CommandFailed(AudioCommand::Pause));
                } else {
                    self.is_playing = false;
                }
            }

            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/forward.png")),
            )
            .clicked()
            {
                println!("Forward");
            }
        });
    }
}
