use crate::app::KnodiqApp;
use knodiq_engine::audio_thread::AudioCommand;

impl KnodiqApp {
    pub(crate) fn update_project(&mut self) {
        // Clone the project and send it to the audio thread
        let project = self.project.clone();
        self.thread_handle
            .command_tx
            .send(AudioCommand::UpdateProject(project))
            .unwrap();
    }
}
