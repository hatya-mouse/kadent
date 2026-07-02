use crate::ui::workspaces::EditorUi;
use kadent_engine::{
    data_types::Ticks,
    thread::{AudioCommand, AudioError},
};

impl EditorUi {
    pub(super) fn play(&mut self) {
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

    pub(super) fn pause(&mut self) {
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

    pub(super) fn seek(&mut self, target_ticks: Ticks) {
        let command = AudioCommand::Seek(target_ticks);
        if self
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.errors.push(AudioError::CommandFailed(command));
        } else {
            self.ui_state.playhead_ticks = target_ticks;
        }
    }
}
