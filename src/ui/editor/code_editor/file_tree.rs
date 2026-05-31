use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn collect_kasl_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_recursive(dir, &mut files);
    files.sort();
    files
}

fn collect_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_recursive(&path, files);
        } else if path.extension().and_then(|e| e.to_str()) == Some("kasl") {
            files.push(path);
        }
    }
}
