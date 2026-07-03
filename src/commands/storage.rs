use crate::{
    core::project_ctx::EditorContext,
    storage::project::{open_project_to_ctx, save_project},
    ui::{theme, workspaces::EditorUi},
};
use kadent_engine::{
    data_types::AudioContext,
    thread::{AudioCommand, AudioResult},
};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum FileNodeKind {
    File,
    Dir { children: Vec<FileNode> },
}

#[derive(Debug)]
pub(crate) struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileNodeKind,
}

impl EditorUi {
    pub(super) fn save_all(&mut self) {
        let program_res = self.save_programs();
        let proj_res = save_project(
            &self.proj_ctx.project_path,
            &self.proj_ctx.project,
            &self.proj_ctx.project_meta,
        );

        if program_res.is_ok() && proj_res.is_ok() {
            self.show_temp_status("Saved Project", theme::successful_fg());
        } else {
            self.show_temp_status("Failed to Save Project", theme::error_fg());
        }
    }

    /// Save all opened programs to their respective file paths.
    fn save_programs(&mut self) -> std::io::Result<()> {
        for (path, buffer) in self
            .ui_state
            .code_editor_state
            .code_buffers
            .iter()
            .flatten()
        {
            std::fs::write(path, buffer)?;
        }
        Ok(())
    }

    pub(super) fn open_project(&mut self, proj_path: PathBuf) {
        let Some(editor_ctx) = open_project_to_ctx(proj_path) else {
            self.show_temp_status("Failed to Open Project", theme::error_fg());
            return;
        };
        self.set_editor_ctx(editor_ctx);
        self.show_temp_status("Opened Project", theme::successful_fg());
    }

    pub(super) fn export_project(&mut self, path: &Path) {
        // Request generation the f32 samples for the entire project
        let project = self.proj_ctx.project.clone();
        self.thread_handle
            .audio_command_tx
            .send(AudioCommand::ExportAudio(Box::new(project)))
            .unwrap();

        // Wait for the audio thread to generate the samples and send them back
        if let Ok(res) = self.thread_handle.result_rx.recv() {
            match res {
                Err(_) => {
                    self.show_temp_status("Failed to Export Project", theme::error_fg());
                }
                Ok(AudioResult::ExportedAudio(samples)) => {
                    self.show_temp_status("Exported Project", theme::successful_fg());
                    write_samples_to_wav(path, &samples, &self.ui_state.audio_ctx);
                }
            }
        }
    }

    pub(super) fn update_dir_cache(&mut self) {
        if let Some(project_dir_path) = self.proj_ctx.project_path.parent()
            && project_dir_path.is_dir()
        {
            self.ui_state.project_dir_cache = recursively_create_graph(project_dir_path);
        }
    }

    /// Set the editor context and update the project context accordingly.
    pub(crate) fn set_editor_ctx(&mut self, editor_ctx: EditorContext) {
        self.proj_ctx = editor_ctx.proj_ctx;

        // Seek to the start of the project after loading
        self.seek(self.proj_ctx.project.range_start);

        // Notify the audio thread of the project change
        self.modified_project();
    }
}

fn write_samples_to_wav(path: &Path, samples: &[f32], audio_ctx: &AudioContext) {
    let spec = hound::WavSpec {
        channels: audio_ctx.channels as u16,
        sample_rate: audio_ctx.sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for &sample in samples {
        let clamped = (sample * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32);
        writer.write_sample(clamped as i16).unwrap();
    }
    writer.finalize().unwrap();
}

/// Recursively create a graph of the project directory structure.
fn recursively_create_graph(path: &Path) -> Vec<FileNode> {
    if let Ok(files) = std::fs::read_dir(path) {
        files
            .filter_map(|file| {
                file.map(|file| FileNode {
                    path: file.path(),
                    name: file.file_name().to_string_lossy().to_string(),
                    kind: if file.path().is_dir() {
                        FileNodeKind::Dir {
                            children: recursively_create_graph(&file.path()),
                        }
                    } else {
                        FileNodeKind::File
                    },
                })
                .ok()
            })
            .collect()
    } else {
        Vec::new()
    }
}
