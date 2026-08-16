use crate::{
    background_thread::{BackgroundTaskStatus, BackgroundThreadCommand},
    core::project_ctx::EditorContext,
    ui::{EditorUi, theme},
};
use kadent_engine::{data_types::Ticks, thread::AudioCommand, timing::TimeBounds};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum FileNodeKind {
    File,
    Dir { children: Vec<FileNode> },
}

#[derive(Debug)]
pub(crate) struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileNodeKind,
}

impl EditorUi {
    pub(super) fn save_all(&mut self) {
        self.ui_state.status_bar_state.current_task = Some(BackgroundTaskStatus::Save);
        self.push_background_job(BackgroundThreadCommand::SaveProject {
            path: self.proj_ctx.project_path.to_path_buf(),
            project: Box::new(self.proj_ctx.project.clone()),
            project_meta: Box::new(self.proj_ctx.project_meta.clone()),
            code_buffers: self
                .ui_state
                .code_editor_state
                .code_buffers
                .values()
                .flatten()
                .cloned()
                .collect(),
        });
    }

    pub(super) fn open_project(&mut self, proj_path: PathBuf) {
        self.ui_state.status_bar_state.current_task = Some(BackgroundTaskStatus::Open);
        self.push_background_job(BackgroundThreadCommand::OpenProject { path: proj_path });
    }

    pub(super) fn export_project(&mut self, path: &Path) {
        // If the project is already being exported, show a message and return early
        if self.ui_state.pending_export_path.is_some() {
            self.show_temp_status("Export already in progress", theme::error_fg());
            return;
        }
        self.ui_state.status_bar_state.current_task = Some(BackgroundTaskStatus::Export);
        self.ui_state.pending_export_path = Some(path.to_path_buf());
        // Request generation the f32 samples for the entire project
        let project = self.proj_ctx.project.clone();
        let export_ctx = self.proj_ctx.project_meta.export_ctx.clone();
        self.thread_handle
            .audio_command_tx
            .send(AudioCommand::ExportAudio(Box::new(project), export_ctx))
            .unwrap();
    }

    pub(super) fn import_audio_file(&mut self, path: &Path, start: Ticks) {
        self.ui_state.status_bar_state.current_task = Some(BackgroundTaskStatus::Import);
        self.push_background_job(BackgroundThreadCommand::ImportAudio {
            file_name: path
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string()),
            start,
            path: path.to_path_buf(),
        });
    }

    pub(super) fn update_dir_cache(&mut self) {
        if let Some(project_dir_path) = self.proj_ctx.project_path.parent()
            && project_dir_path.is_dir()
        {
            self.ui_state.project_dir_cache = recursively_create_graph(project_dir_path);
        }
    }

    /// Set the editor context and update the project context accordingly.
    pub(crate) fn set_editor_ctx(&mut self, editor_ctx: EditorContext) {
        self.proj_ctx = editor_ctx.proj_ctx;

        // Seek to the start of the project after loading
        self.seek(self.proj_ctx.project.export_range.start_time());

        // Notify the audio thread of the project change
        self.modified_project();

        // Generate waveforms for all audio regions in the project
        self.generate_waveforms();
    }

    pub(super) fn set_project_range(&mut self, bounds: TimeBounds) {
        self.proj_ctx.project.export_range = bounds;
        self.modified_project();
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
