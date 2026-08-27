use crate::{
    fonts::RichTextExt,
    ui::{
        EditorState,
        components::toolbar_button::{
            SMALL_TOOLBAR_ICON_SIZE, small_icon_button, small_toolbar_button,
        },
        editor::{
            CodeBuffer, DialogType, UiCommand,
            actions::EditorAction,
            views::{CodeEditorPanelState, CodeEditorView},
        },
    },
};
use eframe::egui::{self, include_image};
use kasl::core::error::Severity;
use uuid::Uuid;

impl CodeEditorView {
    pub(crate) fn header(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut EditorState,
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
    ) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        // Show filename and close button in the header
        if small_icon_button(
            ui,
            egui::Image::new(include_image!("../../../../../assets/icons/reload.svg")),
            true,
        )
        .clicked()
        {
            state.actions.push_action(EditorAction::UpdateDirCache);
        }

        let Some(path) = code_buffer.path.as_ref() else {
            return;
        };
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if code_buffer.is_modified {
            ui.label(egui::RichText::new(&file_name).strong().bold());
        } else {
            ui.label(egui::RichText::new(&file_name).strong());
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if small_icon_button(
                ui,
                egui::Image::new(include_image!("../../../../../assets/icons/x.svg")),
                true,
            )
            .clicked()
            {
                if code_buffer.is_modified {
                    state.ui_commands.push_command(UiCommand::ShowDialog(
                        DialogType::CloseCodeBuffer { panel_id },
                    ));
                } else {
                    code_buffer.path = None;
                }
            }

            if small_icon_button(
                ui,
                egui::Image::new(include_image!("../../../../../assets/icons/save.svg")),
                true,
            )
            .clicked()
            {
                state
                    .actions
                    .push_action(EditorAction::SaveCodeBuffer(panel_id));
            }

            // Show the number of errors
            if error_button(ui, code_buffer).clicked() {
                panel_state.is_error_panel_open = !panel_state.is_error_panel_open;
            }
        });
    }
}

fn error_button(ui: &mut egui::Ui, code_buffer: &CodeBuffer) -> egui::Response {
    let records = &code_buffer.errors.records;
    let compiler_bug_count = records
        .iter()
        .filter(|e| matches!(e.severity, Severity::CompilerBug))
        .count();
    let error_count = records
        .iter()
        .filter(|e| matches!(e.severity, Severity::Error))
        .count();
    let warning_count = records
        .iter()
        .filter(|e| matches!(e.severity, Severity::Warning))
        .count();
    let has_any_error = compiler_bug_count != 0 || error_count != 0 || warning_count != 0;

    if has_any_error {
        small_toolbar_button(ui, |ui, fg_color| {
            let compiler_bug_icon = include_image!("../../../../../assets/icons/compiler_bug.svg");
            let error_icon = include_image!("../../../../../assets/icons/error.svg");
            let warning_icon = include_image!("../../../../../assets/icons/warning.svg");

            if warning_count > 0 {
                icon_and_count(ui, warning_icon, warning_count, fg_color);
            }
            if error_count > 0 {
                icon_and_count(ui, error_icon, error_count, fg_color);
            }
            if compiler_bug_count > 0 {
                icon_and_count(ui, compiler_bug_icon, compiler_bug_count, fg_color);
            }
        })
        .response
    } else {
        let no_error_icon = include_image!("../../../../../assets/icons/no_error.svg");
        small_icon_button(ui, egui::Image::new(no_error_icon), false)
    }
}

fn icon_and_count(
    ui: &mut egui::Ui,
    icon: egui::ImageSource,
    count: usize,
    fg_color: egui::Color32,
) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;

        ui.add_space(2.0);
        ui.label(
            egui::RichText::new(count.to_string())
                .bold()
                .color(fg_color),
        );
        ui.add(egui::Image::new(icon).fit_to_exact_size(SMALL_TOOLBAR_ICON_SIZE));
    });
}
