use crate::{
    ui::editor::actions::{FileNode, FileNodeKind},
    ui::{EditorState, theme},
};
use eframe::egui;
use std::path::{Path, PathBuf};

impl EditorState {
    pub(super) fn file_browser(&mut self, ui: &mut egui::Ui, panel_id: egui::Id) {
        // Get the currently opened path for highlighting (empty path = nothing selected)
        let opened_path: PathBuf = self
            .views
            .code_editor
            .code_buffers
            .get(&panel_id)
            .and_then(|b| b.as_ref())
            .map(|(path, _)| path.clone())
            .unwrap_or_default();

        let selected_file = ui
            .scope(|ui| {
                *ui.style_mut() = theme::menu_style(ui);
                dir_children(ui, &self.views.code_editor.project_dir_cache, &opened_path)
            })
            .inner;

        // Read the content at the path to the buffer
        if let Some(path) = selected_file
            && let Ok(content) = std::fs::read_to_string(&path)
            && let Some(buffer) = self.views.code_editor.code_buffers.get_mut(&panel_id)
        {
            *buffer = Some((path, content));
        }
    }
}

fn dir_children(
    ui: &mut egui::Ui,
    children: &[FileNode],
    opened_program: &Path,
) -> Option<PathBuf> {
    let mut response = None;

    for child in children {
        match &child.kind {
            FileNodeKind::Dir { .. } => {
                let dir_res = dir_expand_button(ui, child, opened_program);
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                let is_opened = opened_program == child.path;
                let file_item_res = ui.selectable_label(is_opened, &child.name);

                if file_item_res.interact(egui::Sense::click()).clicked() {
                    response = Some(child.path.clone());
                }
            }
        }
    }

    response
}

fn dir_expand_button(ui: &mut egui::Ui, node: &FileNode, opened_program: &Path) -> Option<PathBuf> {
    // Ensure that the node is of Dir type before proceeding
    let FileNodeKind::Dir { children } = &node.kind else {
        return None;
    };

    // Manage expand state using persistent ID based on the node's path
    let id = ui.id().with(&node.path);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);

    // Collapsing directory item
    let collapse_icon = if state.is_open() {
        egui::include_image!("../../../../../assets/icons/tri_down.svg")
    } else {
        egui::include_image!("../../../../../assets/icons/tri_right.svg")
    };
    let parent_dir_res = ui.add(egui::Button::image_and_text(collapse_icon, &node.name));

    if parent_dir_res.clicked() {
        state.toggle(ui);
    }

    // Show the child components
    state
        .show_body_indented(&parent_dir_res, ui, |ui| {
            dir_children(ui, children, opened_program)
        })
        .and_then(|res| res.inner)
}
