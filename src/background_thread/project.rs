use crate::core::audio_engine::{data_types::PlaybackContext, mixer::ProjectData};
use crate::storage::project::SaveError;
use crate::ui::editor::CodeBuffer;
use crate::{core::metadata::ProjectMeta, storage::project::save_project};
use std::path::Path;

pub(super) fn run_save_project(
    path: &Path,
    project: &ProjectData,
    project_meta: &ProjectMeta,
    code_buffers: &[CodeBuffer],
) -> Result<(), SaveError> {
    let program_res = save_programs(code_buffers);
    let proj_res = save_project(path, project, project_meta);
    program_res.map_err(SaveError::IoError).and(proj_res)
}

/// Save all opened programs to their respective file paths.
fn save_programs(code_buffers: &[CodeBuffer]) -> std::io::Result<()> {
    for code_buffer in code_buffers {
        code_buffer.save_to_file()?;
    }
    Ok(())
}

pub(super) fn run_write_wav(
    path: &Path,
    samples: &[f32],
    export_ctx: &PlaybackContext,
) -> hound::Result<()> {
    let spec = hound::WavSpec {
        channels: export_ctx.channels as u16,
        sample_rate: export_ctx.sample_rate as u32,
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
