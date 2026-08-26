use crate::ui::{
    EditorState,
    editor::{
        DialogType, UiCommand, UiCommandDispatcher,
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

struct FileTreeContext<'a> {
    actions: &'a mut ActionDispatcher,
    ui_commands: &'a mut UiCommandDispatcher,
    code_buffer: &'a CodeBuffer,
    item_rects: &'a mut Vec<FileTreeDragTarget>,
    dropped_path: Option<PathBuf>,
    panel_id: Uuid,
    file_list_width: f32,
}

struct FileTreeDragTarget {
    rect: egui::Rect,
    path: PathBuf,
}

impl CodeEditorView {
    pub(super) fn file_browser(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_id: Uuid,
        file_list_width: f32,
    ) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();
        let mut item_rects = Vec::new();
        let mut ctx = FileTreeContext {
            actions: &mut state.actions,
            ui_commands: &mut state.ui_commands,
            code_buffer,
            item_rects: &mut item_rects,
            dropped_path: None,
            panel_id,
            file_list_width,
        };

        let selected_file = ui
            .vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                dir_children(ui, &self.project_dir_cache, &mut ctx, 0)
            })
            .inner;

        if let Some(dropped_path) = ctx.dropped_path
            && let Some(file_name) = dropped_path.file_name()
            && let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos())
        {
            let target = item_rects
                .iter()
                .find(|item| item.path != dropped_path && item.rect.contains(pointer_pos));

            if let Some(target_item) = target {
                let target_path = target_item.path.join(file_name);

                // If any code buffer is opening the dropped file, update its path to the new location
                for buffer in self.code_buffers.values_mut() {
                    if let Some(buffer_path) = &buffer.path
                        && *buffer_path == dropped_path
                    {
                        buffer.path = Some(target_path.clone());
                    }
                }

                // Move the file to the new location
                state
                    .actions
                    .push_action(EditorAction::MoveFile(dropped_path, target_path.clone()));
                state.actions.push_action(EditorAction::UpdateDirCache);
            }
        }

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
    children: &[FileNode],
    ctx: &mut FileTreeContext,
    indent: i32,
) -> Option<PathBuf> {
    let mut response = None;

    for child in children {
        match &child.kind {
            FileNodeKind::Dir { .. } => {
                let dir_res = dir_expand_button(ui, child, ctx, indent);
                if dir_res.is_some() {
                    response = dir_res;
                }
            }
            FileNodeKind::File => {
                // Skip hidden files
                if child.name.starts_with('.') {
                    continue;
                }

                let is_opened = ctx.code_buffer.path.as_ref() == Some(&child.path);
                let file_item_res = ui.add_sized(
                    [ctx.file_list_width, FILE_TREE_ITEM_HEIGHT],
                    FileTreeItem::new(&child.name, indent).highlighted(is_opened),
                );
                if let Some(parent_path) = child.path.parent() {
                    ctx.item_rects.push(FileTreeDragTarget {
                        rect: file_item_res.rect,
                        path: parent_path.to_path_buf(),
                    });
                }

                if file_item_res.drag_stopped() {
                    ctx.dropped_path = Some(child.path.clone());
                }

                file_item_res.context_menu(|ui| {
                    file_context_menu(ui, ctx, child);
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
    node: &FileNode,
    ctx: &mut FileTreeContext,
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
    let id = ui.id().with(ctx.panel_id).with(&node.path).with(indent);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);

    // Collapsing directory item
    let parent_dir_res = ui.add_sized(
        [ctx.file_list_width, FILE_TREE_ITEM_HEIGHT],
        FileTreeItem::new(&node.name, indent).collapsible(!state.is_open()),
    );
    ctx.item_rects.push(FileTreeDragTarget {
        rect: parent_dir_res.rect,
        path: node.path.clone(),
    });
    if parent_dir_res.clicked() {
        state.toggle(ui);
    }

    if parent_dir_res.drag_stopped() {
        ctx.dropped_path = Some(node.path.clone());
    }

    // Add context menu for adding new files or directories
    parent_dir_res.context_menu(|ui| {
        dir_context_menu(ui, ctx, node);
    });

    // Show the child components
    state
        .show_body_unindented(ui, |ui| dir_children(ui, children, ctx, indent + 1))
        .and_then(|res| res.inner)
}

fn file_context_menu(ui: &mut egui::Ui, ctx: &mut FileTreeContext, node: &FileNode) {
    *ui.style_mut() = theme::menu_style(ui);

    if ui.selectable_label(false, "Rename").clicked() {
        ctx.ui_commands
            .push_command(UiCommand::ShowDialog(DialogType::RenameFile {
                path: node.path.clone(),
            }));
    }
    if ui.selectable_label(false, "Trash").clicked() {
        ctx.actions
            .push_action(EditorAction::MoveFileToTrash(node.path.clone()));
        ctx.actions.push_action(EditorAction::UpdateDirCache);
    }
}

fn dir_context_menu(ui: &mut egui::Ui, ctx: &mut FileTreeContext, node: &FileNode) {
    *ui.style_mut() = theme::menu_style(ui);

    if ui.selectable_label(false, "Rename").clicked() {
        ctx.ui_commands
            .push_command(UiCommand::ShowDialog(DialogType::RenameFile {
                path: node.path.clone(),
            }));
    }
    if ui.selectable_label(false, "New File").clicked() {
        ctx.actions
            .push_action(EditorAction::CreateFile(node.path.join("untitled.kasl")));
        ctx.actions.push_action(EditorAction::UpdateDirCache);
    }
    if ui.selectable_label(false, "New Folder").clicked() {
        ctx.actions.push_action(EditorAction::CreateDirectory(
            node.path.join("Untitled Folder"),
        ));
        ctx.actions.push_action(EditorAction::UpdateDirCache);
    }
    if ui.selectable_label(false, "Trash").clicked() {
        ctx.actions
            .push_action(EditorAction::MoveFileToTrash(node.path.clone()));
        ctx.actions.push_action(EditorAction::UpdateDirCache);
    }
}
