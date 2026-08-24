use crate::ui::{
    editor::{
        actions::{FileNode, FileNodeKind},
        views::code_editor::{CodeBuffer, CodeEditorView},
    },
    theme,
};
use eframe::egui;
use std::path::PathBuf;
use uuid::Uuid;

impl CodeEditorView {
    pub(super) fn file_browser(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        let selected_file = ui
            .scope(|ui| {
                *ui.style_mut() = theme::menu_style(ui);
                dir_children(ui, &self.project_dir_cache, code_buffer)
            })
            .inner;

        // Read the content at the path to the buffer
        if let Some(path) = selected_file
            && let Ok(content) = std::fs::read_to_string(&path)
        {
            self.code_buffers.insert(
                panel_id,
                CodeBuffer {
                    path: Some(path),
                    content,
                },
            );
        }
    }
}

fn dir_children(
    ui: &mut egui::Ui,
    children: &[FileNode],
    code_buffer: &CodeBuffer,
) -> Option<PathBuf> {
    let mut response = None;

    for child in children {
        match &child.kind {
            FileNodeKind::Dir { .. } => {
                let dir_res = dir_expand_button(ui, child, code_buffer);
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                let is_opened = code_buffer.path.as_ref() == Some(&child.path);
                let file_item_res = ui.selectable_label(is_opened, &child.name);

                if file_item_res.interact(egui::Sense::click()).clicked() {
                    response = Some(child.path.clone());
                }
            }
        }
    }

    response
}

fn dir_expand_button(
    ui: &mut egui::Ui,
    node: &FileNode,
    code_buffer: &CodeBuffer,
) -> Option<PathBuf> {
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
            dir_children(ui, children, code_buffer)
        })
        .and_then(|res| res.inner)
}
