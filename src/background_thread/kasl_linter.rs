use crate::consts::{DEFAULT_BUFFER_SIZE, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE};
use kasl::core::{KaslCompiler, error::ErrorRecord};
use std::path::{Path, PathBuf};

pub(super) fn lint_kasl(
    code: &str,
    mut search_paths: Vec<PathBuf>,
    file_path: &Path,
) -> Vec<ErrorRecord> {
    // Add the path to the parent directory of the file being linted to the search paths
    if let Some(local_path) = file_path.parent() {
        search_paths.push(local_path.to_path_buf());
    }

    let mut compiler = KaslCompiler::default();
    compiler.set_search_paths(search_paths.iter().map(PathBuf::from).collect());

    // Add the audio module to the compiler's virtual files
    let audio_module = format!(
        include_str!("../../kasl_module/audio.kasl"),
        DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE, DEFAULT_BUFFER_SIZE,
    );
    compiler.add_virtual_file(PathBuf::from("audio"), audio_module);

    // Parse and lint the code
    match compiler.parse(code) {
        Ok(()) => compiler.lint(),
        Err(e) => {
            vec![*e]
        }
    }
}

/// Calculates the byte offsets of each character in the source string.
pub(super) fn char_to_byte_offsets(source: &str) -> Vec<usize> {
    let mut offsets: Vec<_> = source.char_indices().map(|(byte, _)| byte).collect();
    offsets.push(source.len());
    offsets
}
