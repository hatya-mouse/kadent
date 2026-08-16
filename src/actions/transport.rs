use crate::ui::EditorUi;
use kadent_engine::{
    thread::{AudioCommand, AudioError},
    timing::TimePosition,
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
            self.ui_state
                .errors
                .push(AudioError::CommandFailed(command));
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
            self.ui_state
                .errors
                .push(AudioError::CommandFailed(command));
        } else {
            self.ui_state.is_playing = false;
        }
    }

    pub(super) fn seek(&mut self, seek_position: TimePosition) {
        let command = AudioCommand::Seek(seek_position);
        if self
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.ui_state
                .errors
                .push(AudioError::CommandFailed(command));
        }
    }
}
