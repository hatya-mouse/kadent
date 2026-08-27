use crate::{
    fonts::RichTextExt,
    ui::{
        components::not_available_text::not_available_text,
        editor::views::{CodeEditorPanelState, CodeEditorView, code_editor::CODE_EDITOR_FONT_SIZE},
        theme,
    },
};
use eframe::egui::{self, include_image};
use kasl::core::{
    ast_nodes::Range,
    error::{ErrorRecord, Severity},
    localization::format_error,
};
use uuid::Uuid;

const ERROR_MARGIN: egui::Margin = egui::Margin::same(8);
const ERROR_ID_MARGIN: egui::Margin = egui::Margin {
    left: 3,
    right: 6,
    top: 2,
    bottom: 2,
};

impl CodeEditorView {
    pub(super) fn error_panel(
        &mut self,
        ui: &mut egui::Ui,
        panel_id: Uuid,
        panel_state: &mut CodeEditorPanelState,
    ) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();

        if code_buffer.errors.records.is_empty() {
            not_available_text(ui, "No Errors Found");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut alternate = false;
            for error in code_buffer.errors.records.iter() {
                for range in error.ranges.iter() {
                    show_error_record(ui, error, range, panel_state, alternate);
                    alternate = !alternate;
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
    alternate: bool,
) {
    let frame_response = error_record_frame(ui, error, alternate);

    let response = ui.allocate_rect(frame_response.rect, egui::Sense::click());
    if response.clicked() {
        panel_state.jump_index = Some((range.start, range.end));
    }

    // Draw background color based on the interaction state (hovered or pressed)
    if response.is_pointer_button_down_on() {
        ui.painter().rect_filled(
            response.rect,
            0,
            theme::card_button_pressed(ui.visuals().dark_mode),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(
            response.rect,
            0,
            theme::card_button_hovered(ui.visuals().dark_mode),
        );
    }

    // Draw a border below the frame
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.max.y,
        theme::border(ui.visuals().dark_mode),
    );
}

fn error_record_frame(ui: &mut egui::Ui, error: &ErrorRecord, alternate: bool) -> egui::Response {
    let error_id = error.key.kind.to_string();
    let message = format_error(error, "en");
    let icon = match &error.severity {
        Severity::CompilerBug => include_image!("../../../../../assets/icons/compiler_bug.svg"),
        Severity::Error => include_image!("../../../../../assets/icons/error.svg"),
        Severity::Warning => include_image!("../../../../../assets/icons/warning.svg"),
    };

    let available_width = ui.available_width();
    let row_bg = if alternate {
        theme::tertiary_bg(ui.visuals().dark_mode)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::new()
        .inner_margin(ERROR_MARGIN)
        .fill(row_bg)
        .show(ui, |ui| {
            ui.set_width(available_width);

            ui.vertical(|ui| {
                egui::Frame::new()
                    .inner_margin(ERROR_ID_MARGIN)
                    .corner_radius(6.0)
                    .fill(theme::button_bg(ui.visuals().dark_mode))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Image::new(icon).fit_to_exact_size(egui::vec2(20.0, 20.0)),
                            );
                            ui.vertical(|ui| {
                                ui.add_space(1.0);
                                ui.label(egui::RichText::new(error_id).strong().bold_mono());
                            });
                        });
                    });

                ui.add_space(6.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .strong()
                            .size(CODE_EDITOR_FONT_SIZE),
                    )
                    .wrap()
                    .selectable(true),
                );
            });
        })
        .response
}
