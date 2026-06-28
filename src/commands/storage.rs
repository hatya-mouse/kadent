use crate::{
    core::project_ctx::EditorContext,
    storage::project::{open_project_to_ctx, save_project},
    ui::workspaces::EditorUi,
};
use kadent_engine::{
    data_types::AudioContext,
    thread::{AudioCommand, AudioError, AudioResult},
};
use std::path::{Path, PathBuf};

impl EditorUi {
    pub(super) fn save_project(&mut self) {
        match save_project(
            &self.proj_ctx.project_path,
            &self.proj_ctx.project,
            &self.proj_ctx.project_meta,
        ) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Failed to save project: {:?}", e);
            }
        }
    }

    pub(super) fn open_project(&mut self, proj_path: PathBuf) {
        let Some(editor_ctx) = open_project_to_ctx(proj_path) else {
            return;
        };
        self.set_editor_ctx(editor_ctx);
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
                    eprintln!("Error exporting audio");
                }
                Ok(AudioResult::ExportedAudio(samples)) => {
                    write_samples_to_wav(path, &samples, &self.proj_ctx.project.audio_ctx);
                }
            }
        }
    }

    /// Set the editor context and update the project context accordingly.
    pub(crate) fn set_editor_ctx(&mut self, editor_ctx: EditorContext) {
        self.proj_ctx = editor_ctx.proj_ctx;

        // Seek to the start of the project after loading
        let command = AudioCommand::Seek(self.proj_ctx.project.range_start);
        if self
            .thread_handle
            .audio_command_tx
            .send(command.clone())
            .is_err()
        {
            self.errors.push(AudioError::CommandFailed(command));
        }

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
