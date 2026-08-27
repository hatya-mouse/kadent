use crate::{
    core::audio_engine::{
        thread::{AudioCommand, AudioError},
        timing::TimePosition,
    },
    ui::editor::EditorUi,
};

impl EditorUi {
    pub(super) fn play(&mut self) {
        let command = AudioCommand::Play;
        if self
            .state
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.views
                .error_list
                .push_error(AudioError::CommandFailed(command));
        } else {
            self.state.transport.is_playing = true;
        }
    }

    pub(super) fn pause(&mut self) {
        let command = AudioCommand::Pause;
        if self
            .state
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.views
                .error_list
                .push_error(AudioError::CommandFailed(command));
        } else {
            self.state.transport.is_playing = false;
        }
    }

    pub(super) fn seek(&mut self, seek_position: TimePosition) {
        let command = AudioCommand::Seek(seek_position);
        if self
            .state
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.views
                .error_list
                .push_error(AudioError::CommandFailed(command));
        }
    }
}
