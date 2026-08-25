use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::ui::{
    editor::{
        EditorUi, UiCommand,
        actions::{FileNode, FileNodeKind},
    },
    theme,
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

    pub(super) fn save_code_buffer(&mut self, panel_id: Uuid) {
        if let Some(code_buffer) = self.views.code_editor.code_buffers.get_mut(&panel_id) {
            if let Err(error) = code_buffer.save_to_file() {
                println!("Failed to save code buffer: {}", error);
                self.state
                    .ui_commands
                    .push_command(UiCommand::ShowTempStatus(
                        "Could not save the file".to_string(),
                        theme::error_fg(),
                    ));
            } else {
                code_buffer.is_modified = false;
            }
        }
    }

    pub(super) fn move_file(&mut self, from: &Path, to: &Path) {
        if let Err(err) = std::fs::rename(from, to) {
            eprintln!("Failed to move file: {}", err);
            self.state
                .ui_commands
                .push_command(UiCommand::ShowTempStatus(
                    "Could not move the file".to_string(),
                    theme::error_fg(),
                ));
        }
    }

    pub(super) fn open_file_in_code_editor(&mut self, panel_id: Uuid, path: PathBuf) {
        self.views
            .code_editor
            .set_code_buffer(panel_id, path, &mut self.state);
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
