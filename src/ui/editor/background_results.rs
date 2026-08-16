use crate::{
    background_thread::BackgroundThreadResult,
    ui::{EditorUi, theme},
};

impl EditorUi {
    pub(super) fn process_background_results(&mut self) {
        while let Ok(result) = self.background_handle.result_rx.try_recv() {
            self.ui_state.status_bar_state.current_task = None;
            match result {
                BackgroundThreadResult::SavedProject(result) => match result {
                    Ok(_) => {
                        self.show_temp_status("Saved project", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to save project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::OpenedProject(ctx) => match ctx {
                    Some(editor_ctx) => {
                        self.set_editor_ctx(*editor_ctx);
                        self.show_temp_status("Opened project", theme::successful_fg());
                    }
                    None => {
                        self.show_temp_status("Failed to open project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::WroteWav(result) => match result {
                    Ok(_) => {
                        self.show_temp_status("Exported project", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to export project", theme::error_fg());
                    }
                },
                BackgroundThreadResult::ImportedAudio {
                    file_name,
                    start,
                    result,
                } => match result {
                    Ok(decoded) => {
                        self.finish_audio_import(file_name, start, decoded);
                        self.show_temp_status("Imported audio", theme::successful_fg());
                    }
                    Err(_) => {
                        self.show_temp_status("Failed to import audio", theme::error_fg());
                    }
                },
                BackgroundThreadResult::GeneratedWaveform {
                    track_id,
                    region_id,
                    waveform,
                } => {
                    self.ui_state
                        .timeline_state
                        .waveforms
                        .insert((track_id, region_id), waveform);
                }
            }
        }
    }
}
