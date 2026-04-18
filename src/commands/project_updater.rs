use crate::app::KnodiqApp;
use knodiq_engine::audio_thread::AudioCommand;
use std::time::Instant;

impl KnodiqApp {
    /// Marks the project as modified and updates the last edit time. Should be called whenever the project is modified.
    pub(crate) fn modified_project(&mut self) {
        println!("Modified project");
        self.ui_state.editor_state.last_edit_time = Some(Instant::now());
    }

    /// Checks if the project has been modified recently and sends an update command to the audio thread if necessary.
    /// Should not be called directly because this is automatically called.
    pub(crate) fn update_project(&mut self) {
        if let Some(t) = self.ui_state.editor_state.last_edit_time
            && t.elapsed() > std::time::Duration::from_millis(300)
        {
            println!("Update project");
            self.ui_state.editor_state.last_edit_time = None;

            // Clone the project and send it to the audio thread
            let project = self.project.clone();
            self.thread_handle
                .command_tx
                .send(AudioCommand::UpdateProject(project))
                .unwrap();
        }
    }
}
