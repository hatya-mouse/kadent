use std::path::Path;

use crate::ui::editor::{
    EditorUi,
    actions::{FileNode, FileNodeKind},
};

impl EditorUi {
    pub(super) fn update_dir_cache(&mut self) {
        if let Some(project_dir_path) = self.state.project.path.parent()
            && project_dir_path.is_dir()
        {
            self.views.code_editor.project_dir_cache = recursively_create_graph(project_dir_path);
        }
    }

    pub(super) fn create_file(&mut self, path: &std::path::Path) {
        if let Some(parent_dir) = path.parent()
            && !parent_dir.exists()
        {
            std::fs::create_dir_all(parent_dir).unwrap_or_else(|err| {
                eprintln!("Failed to create parent directories: {}", err);
            });
        }

        if let Err(err) = std::fs::File::create(path) {
            eprintln!("Failed to create file: {}", err);
        }
    }

    pub(super) fn create_dir(&mut self, path: &std::path::Path) {
        if let Err(err) = std::fs::create_dir_all(path) {
            eprintln!("Failed to create directory: {}", err);
        }
    }

    pub(super) fn move_file_to_trash(&mut self, path: &std::path::Path) {
        if let Err(err) = trash::delete(path) {
            eprintln!("Failed to move file to trash: {}", err);
        }
    }
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
