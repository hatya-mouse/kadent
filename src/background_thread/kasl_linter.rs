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

    // Parse and lint the code
    match compiler.parse(code) {
        Ok(()) => compiler.lint(),
        Err(e) => vec![*e],
    }
}
