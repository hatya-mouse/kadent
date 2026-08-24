use crate::ui::editor::{
    actions::{FileNode, FileNodeKind},
    views::code_editor::{CodeBuffer, CodeEditorView, tree_item::FileTreeItem},
};
use eframe::egui;
use std::path::PathBuf;
use uuid::Uuid;

const FILE_TREE_ITEM_HEIGHT: f32 = 25.0;

impl CodeEditorView {
    pub(super) fn file_browser(&mut self, ui: &mut egui::Ui, panel_id: Uuid, file_list_width: f32) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
        let selected_file = dir_children(
            ui,
            &self.project_dir_cache,
            code_buffer,
            panel_id,
            file_list_width,
            0,
        );

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
    panel_id: Uuid,
    file_list_width: f32,
    indent: i32,
) -> Option<PathBuf> {
    let mut response = None;

    for child in children {
        match &child.kind {
            FileNodeKind::Dir { .. } => {
                let dir_res =
                    dir_expand_button(ui, child, code_buffer, panel_id, file_list_width, indent);
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                let is_opened = code_buffer.path.as_ref() == Some(&child.path);
                let file_item_res = ui.add_sized(
                    [file_list_width, FILE_TREE_ITEM_HEIGHT],
                    FileTreeItem::new(&child.name, indent).highlighted(is_opened),
                );

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
    panel_id: Uuid,
    file_list_width: f32,
    indent: i32,
) -> Option<PathBuf> {
    // Ensure that the node is of Dir type before proceeding
    let FileNodeKind::Dir { children } = &node.kind else {
        return None;
    };

    // Manage expand state using persistent ID based on the node's path
    let id = ui.id().with(panel_id).with(&node.path).with(indent);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);

    // Collapsing directory item
    let parent_dir_res = ui.add_sized(
        [file_list_width, FILE_TREE_ITEM_HEIGHT],
        FileTreeItem::new(&node.name, 0).collapsible(!state.is_open()),
    );
    if parent_dir_res.clicked() {
        state.toggle(ui);
    }

    // Show the child components
    state
        .show_body_unindented(ui, |ui| {
            dir_children(
                ui,
                children,
                code_buffer,
                panel_id,
                file_list_width,
                indent + 1,
            )
        })
        .and_then(|res| res.inner)
}
