use crate::ui::{
    EditorState,
    editor::{
        DialogType, UiCommand,
        actions::{EditorAction, FileNode, FileNodeKind},
        state::ActionDispatcher,
        views::code_editor::{CodeBuffer, CodeEditorView, tree_item::FileTreeItem},
    },
    theme,
};
use eframe::egui;
use std::path::PathBuf;
use uuid::Uuid;

const FILE_TREE_ITEM_HEIGHT: f32 = 25.0;

impl CodeEditorView {
    pub(super) fn file_browser(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_id: Uuid,
        file_list_width: f32,
    ) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        let selected_file = ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                dir_children(
                    ui,
                    &mut state.actions,
                    &self.project_dir_cache,
                    code_buffer,
                    panel_id,
                    file_list_width,
                    0,
                )
            })
            .inner;

        // Read the content at the path to the buffer if a file is selected
        if let Some(path) = selected_file {
            if let Some(existing_buffer) = self.code_buffers.get_mut(&panel_id)
                && existing_buffer.is_modified
            {
                state.ui_commands.push_command(UiCommand::ShowDialog(
                    DialogType::ChangeCodeBuffer { panel_id, path },
                ));
            } else {
                self.set_code_buffer(panel_id, path, state);
            }
        }
    }

    pub(crate) fn set_code_buffer(
        &mut self,
        panel_id: Uuid,
        path: PathBuf,
        state: &mut EditorState,
    ) {
        if let Ok(content) = std::fs::read_to_string(&path) {
            self.code_buffers.insert(
                panel_id,
                CodeBuffer {
                    path: Some(path),
                    content,
                    is_modified: false,
                },
            );
        } else {
            state.ui_commands.push_command(UiCommand::ShowTempStatus(
                "Could not open the file".to_string(),
                theme::error_fg(),
            ));
        }
    }
}

fn dir_children(
    ui: &mut egui::Ui,
    actions: &mut ActionDispatcher,
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
                let dir_res = dir_expand_button(
                    ui,
                    actions,
                    child,
                    code_buffer,
                    panel_id,
                    file_list_width,
                    indent,
                );
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                // Skip hidden files
                if child.name.starts_with('.') {
                    continue;
                }

                let is_opened = code_buffer.path.as_ref() == Some(&child.path);
                let file_item_res = ui.add_sized(
                    [file_list_width, FILE_TREE_ITEM_HEIGHT],
                    FileTreeItem::new(&child.name, indent).highlighted(is_opened),
                );

                file_item_res.context_menu(|ui| {
                    *ui.style_mut() = theme::menu_style(ui);

                    if ui.selectable_label(false, "Trash").clicked() {
                        actions.push_action(EditorAction::MoveFileToTrash(child.path.clone()));
                        actions.push_action(EditorAction::UpdateDirCache);
                    }
                });

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
    actions: &mut ActionDispatcher,
    node: &FileNode,
    code_buffer: &CodeBuffer,
    panel_id: Uuid,
    file_list_width: f32,
    indent: i32,
) -> Option<PathBuf> {
    // Skip hidden files
    if node.name.starts_with('.') {
        return None;
    }

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
        FileTreeItem::new(&node.name, indent).collapsible(!state.is_open()),
    );
    if parent_dir_res.clicked() {
        state.toggle(ui);
    }

    // Add context menu for adding new files or directories
    parent_dir_res.context_menu(|ui| {
        *ui.style_mut() = theme::menu_style(ui);

        let full_path = node.path.with_file_name(&node.name);
        if ui.selectable_label(false, "New File").clicked() {
            actions.push_action(EditorAction::CreateFile(
                full_path.with_file_name("untitled.kasl"),
            ));
            actions.push_action(EditorAction::UpdateDirCache);
        }
        if ui.selectable_label(false, "New Folder").clicked() {
            actions.push_action(EditorAction::CreateDirectory(
                full_path.with_file_name("Untitled Folder"),
            ));
            actions.push_action(EditorAction::UpdateDirCache);
        }
        if ui.selectable_label(false, "Trash").clicked() {
            actions.push_action(EditorAction::MoveFileToTrash(full_path.clone()));
            actions.push_action(EditorAction::UpdateDirCache);
        }
    });

    // Show the child components
    state
        .show_body_unindented(ui, |ui| {
            dir_children(
                ui,
                actions,
                children,
                code_buffer,
                panel_id,
                file_list_width,
                indent + 1,
            )
        })
        .and_then(|res| res.inner)
}
