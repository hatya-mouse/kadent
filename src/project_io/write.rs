use knodiq_engine::mixer::Project;
use std::{fs::File, io::Write, path::Path};

pub(crate) fn write_project(path: &Path, project: &Project) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the project data to the file
    // First write "KNODIQ" to check if the file is a Knodiq Project file
    file.write_all("KNODIQ".as_bytes())?;

    // Then write the version of Knodiq
    let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    file.write_all(&major_ver.to_le_bytes())?;
    file.write_all(&minor_ver.to_le_bytes())?;
    file.write_all(&patch_ver.to_le_bytes())?;

    // Write the audio configuration to the file
    file.write_all(&(project.audio_ctx.channels as u64).to_le_bytes());
    file.write_all(&(project.audio_ctx.sample_rate as u64).to_le_bytes());
    file.write_all(&(project.audio_ctx.buffer_size as u64).to_le_bytes());
    file.write_all(&(project.audio_ctx.max_voices as u64).to_le_bytes());

    Ok(())
}
