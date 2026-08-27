use crate::{
    background_thread::BackgroundThreadResult,
    ui::{
        editor::{EditorUi, UiCommand},
        theme,
    },
};

impl EditorUi {
    pub(super) fn process_background_results(&mut self) {
        while let Ok(result) = self.state.actions.background_handle.result_rx.try_recv() {
            self.views.status_bar.current_task = None;
            match result {
                BackgroundThreadResult::SavedProject(result) => match result {
                    Ok(_) => {
                        for code_buffer in self.views.code_editor.code_buffers.values_mut() {
                            code_buffer.is_modified = false;
                        }

                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Saved project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                    Err(_) => {
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Failed to save project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                },
                BackgroundThreadResult::OpenedProject(ctx) => match ctx {
                    Some(proj_ctx) => {
                        self.set_proj_ctx(*proj_ctx);
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Opened project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                    None => {
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Failed to open project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                },
                BackgroundThreadResult::WroteWav(result) => match result {
                    Ok(_) => {
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Exported project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                    Err(_) => {
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Failed to export project".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                },
                BackgroundThreadResult::ImportedAudio {
                    file_name,
                    start,
                    result,
                } => match result {
                    Ok(decoded) => {
                        self.finish_audio_import(file_name, start, decoded);
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Imported audio".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                    Err(_) => {
                        self.state
                            .ui_commands
                            .push_command(UiCommand::ShowTempStatus(
                                "Failed to import audio".to_string(),
                                theme::successful_fg(),
                            ));
                    }
                },
                BackgroundThreadResult::GeneratedWaveform {
                    track_id,
                    region_id,
                    waveform,
                } => {
                    self.views
                        .timeline
                        .waveforms
                        .insert((track_id, region_id), waveform);
                }
                BackgroundThreadResult::LintedKasl {
                    buffer_id,
                    byte_offsets,
                    errors,
                } => {
                    self.views
                        .code_editor
                        .set_lint_errors(buffer_id, byte_offsets, errors);
                }
            }
        }
    }
}
