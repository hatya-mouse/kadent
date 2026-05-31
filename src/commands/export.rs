use crate::ui::EditorUi;
use kreniq_engine::{
    data_types::AudioContext,
    thread::{AudioCommand, AudioResult},
};
use std::path::Path;

impl EditorUi {
    pub(crate) fn export_project(&mut self, path: &Path) {
        // Request generation the f32 samples for the entire project
        let project = self.project.clone();
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
                    write_samples_to_wav(path, &samples, &self.project.audio_ctx);
                }
            }
        }
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
