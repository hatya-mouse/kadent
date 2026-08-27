use crate::ui::{
    components::not_available_text::not_available_text,
    editor::views::{CodeEditorPanelState, CodeEditorView, code_editor::CODE_EDITOR_FONT_SIZE},
};
use eframe::egui;
use kasl::core::{ast_nodes::Range, error::ErrorRecord, localization::format_error};
use uuid::Uuid;

const ERROR_MARGIN: egui::Margin = egui::Margin::symmetric(6, 4);

impl CodeEditorView {
    pub(super) fn error_panel(
        &mut self,
        ui: &mut egui::Ui,
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
    ) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        if code_buffer.errors.records.is_empty() {
            not_available_text(ui, "No errors found");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for error in code_buffer.errors.records.iter() {
                for range in error.ranges.iter() {
                    show_error_record(ui, error, range, panel_state);
                    ui.separator();
                }
            }
        });
    }
}

fn show_error_record(
    ui: &mut egui::Ui,
    error: &ErrorRecord,
    range: &Range,
    panel_state: &mut CodeEditorPanelState,
) {
    let message = format_error(error, "en");

    let frame_response = egui::Frame::new()
        .inner_margin(ERROR_MARGIN)
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(message)
                        .strong()
                        .size(CODE_EDITOR_FONT_SIZE),
                )
                .selectable(true),
            );
        })
        .response;

    let response = ui.allocate_rect(frame_response.rect, egui::Sense::click());
    if response.clicked() {
        panel_state.jump_index = Some((range.start, range.end));
    }
}
