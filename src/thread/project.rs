use crate::{core::metadata::ProjectMeta, storage::project::save_project};
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::{Path, PathBuf};

pub(super) fn run_save_project(
    path: &Path,
    project: &Project,
    proj_meta: &ProjectMeta,
    code_buffers: &[(PathBuf, String)],
) -> std::io::Result<()> {
    let program_res = save_programs(code_buffers);
    let proj_res = save_project(path, project, proj_meta);
    program_res.and(proj_res)
}

/// Save all opened programs to their respective file paths.
fn save_programs(code_buffers: &[(PathBuf, String)]) -> std::io::Result<()> {
    for (path, content) in code_buffers {
        std::fs::write(path, content)?;
    }
    Ok(())
}

pub(super) fn run_write_wav(
    path: &Path,
    samples: &[f32],
    audio_ctx: &AudioContext,
) -> hound::Result<()> {
    let spec = hound::WavSpec {
        channels: audio_ctx.channels as u16,
        sample_rate: audio_ctx.sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    for &sample in samples {
        let clamped = (sample * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32);
        writer.write_sample(clamped as i16)?;
    }
    writer.finalize()?;

    Ok(())
}
