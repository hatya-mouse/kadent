use crate::{
    background_thread::BackgroundThreadResult,
    ui::{theme, workspaces::EditorUi},
};

impl EditorUi {
    pub(super) fn process_background_results(&mut self) {
        while let Ok(result) = self.background_handle.result_rx.try_recv() {
            self.ui_state.status_bar_state.current_task = None;
            match result {
                BackgroundThreadResult::SavedProject(result) => match result {
                    Ok(_) => {
                        self.show_temp_status("Saved Project", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to Save Project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::OpenedProject(ctx) => match ctx {
                    Some(editor_ctx) => {
                        self.set_editor_ctx(*editor_ctx);
                        self.show_temp_status("Opened Project", theme::successful_fg());
                    }
                    None => {
                        self.show_temp_status("Failed to Open Project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::WroteWav(result) => match result {
                    Ok(_) => {
                        self.show_temp_status("Exported Project", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to Export Project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::ImportedAudio {
                    track_id,
                    start,
                    result,
                } => match result {
                    Ok(decoded) => {
                        self.finish_audio_import(track_id, start, decoded);
                        self.show_temp_status("Imported Audio", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to Import Audio", theme::error_fg());
                    }
                },
            }
        }
    }
}
