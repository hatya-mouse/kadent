use crate::{
    commands::{FileNode, FileNodeKind},
    ui::workspaces::EditorUi,
};
use eframe::egui;
use std::path::PathBuf;

impl EditorUi {
    pub(super) fn file_browser(&mut self, ui: &mut egui::Ui) {
        dir_children(
            ui,
            &self.ui_state.project_dir_cache,
            &self.ui_state.code_editor_state.opened_programs,
        );
    }
}

fn dir_children(
    ui: &mut egui::Ui,
    children: &[FileNode],
    opened_programs: &[PathBuf],
) -> Option<PathBuf> {
    let mut response = None;

    for child in children {
        match &child.kind {
            FileNodeKind::Dir { .. } => {
                let dir_res = dir_expand_button(ui, child, opened_programs);
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                let is_opened = opened_programs.contains(&child.path);
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
    opened_programs: &[PathBuf],
) -> Option<PathBuf> {
    // Ensure that the node is of Dir type before proceeding
    let FileNodeKind::Dir { children } = &node.kind else {
        return None;
    };

    // Manage expand state using persistent ID based on the node's path
    let id = ui.make_persistent_id(&node.path);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);

    // Collapsing directory item
    let collapse_icon = if state.is_open() {
        egui::include_image!("../../../../../assets/icons/tri_down.svg")
    } else {
        egui::include_image!("../../../../../assets/icons/tri_right.svg")
    };
    let parent_dir_item = ui.menu_image_text_button(collapse_icon, &node.name, |_| {});
    let parent_dir_res = parent_dir_item.response;

    if parent_dir_res.interact(egui::Sense::click()).clicked() {
        state.toggle(ui);
    }

    // Show the child components
    state
        .show_body_indented(&parent_dir_res, ui, |ui| {
            dir_children(ui, children, opened_programs)
        })
        .and_then(|res| res.inner)
}
