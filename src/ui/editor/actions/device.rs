use crate::{core::audio_engine::thread::AudioCommand, ui::EditorState};
use cpal::traits::DeviceTrait;

impl EditorState {
    pub(super) fn set_audio_output_device(&mut self, device: cpal::Device) {
        self.audio_device.selected_output = device.id().ok();
        self.thread_handle
            .audio_command_tx
            .send(AudioCommand::SetOutputDevice(device.clone()))
            .ok();
    }
}
