use crate::ui::{
    components::not_available_text::not_available_text, editor::views::code_editor::CodeEditorView,
    theme,
};
use eframe::egui::{self, TextBuffer};
use egui_extras::syntax_highlighting::{CodeTheme, highlight_with};
use kasl::core::error::ErrorRecord;
use std::time::Instant;
use uuid::Uuid;

const CODE_EDITOR_MARGIN: egui::Margin = egui::Margin::symmetric(6, 4);

impl CodeEditorView {
    pub(super) fn kasl_editor(&mut self, ui: &mut egui::Ui, panel_id: Uuid) {
        let code_buffer = self.code_buffers.entry(panel_id).or_default();
        if code_buffer.path.is_none() {
            not_available_text(ui, "Select a File to Edit");
            return;
        }

        // Get the theme and syntect settings for syntax highlighting
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
        let Some(syntect_settings) = self.syntect_settings.clone() else {
            return;
        };

        // Create a layouter closure that highlights the code using KASL syntax set.
        // wrap_width is forced to infinity to disable line wrapping (horizontal scroll instead).
        let mut layouter = |ui: &egui::Ui, buffer: &dyn TextBuffer, _wrap_width: f32| {
            let mut layout_job = highlight_with(
                ui.ctx(),
                ui.style(),
                &theme,
                buffer.as_str(),
                "kasl",
                &syntect_settings,
            );

            // Highlight the errors
            highlight_errors(
                &mut layout_job,
                &code_buffer.errors,
                code_buffer.has_modified_since_last_lint,
            );

            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts_mut(|fonts| fonts.layout_job(layout_job))
        };

        // Compute the minimum rows needed to fill the available vertical space
        let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
        let min_rows = (ui.available_height() / row_height).ceil() as usize;
        let desired_rows = code_buffer.content.lines().count().max(min_rows);

        let text_edit_response = egui::ScrollArea::both()
            .id_salt("kasl_editor")
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    egui::Frame::new()
                        .inner_margin(CODE_EDITOR_MARGIN)
                        .fill(theme::primary_bg(ui.visuals().dark_mode))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                for line_number in 1..=desired_rows {
                                    ui.label(
                                        egui::RichText::new(line_number.to_string()).monospace(),
                                    );
                                }
                            });
                        });

                    ui.add(
                        egui::TextEdit::multiline(&mut code_buffer.content)
                            .frame(
                                egui::Frame::new()
                                    .inner_margin(CODE_EDITOR_MARGIN)
                                    .fill(theme::primary_bg(ui.visuals().dark_mode)),
                            )
                            .id_salt(&code_buffer.path)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(desired_rows)
                            .lock_focus(true)
                            .layouter(&mut layouter),
                    )
                })
            })
            .inner
            .inner;

        if text_edit_response.changed() {
            code_buffer.is_modified = true;
            code_buffer.has_modified_since_last_lint = true;
            code_buffer.last_edit_time = Some(Instant::now())
        }
    }
}

fn highlight_errors(
    layout_job: &mut egui::text::LayoutJob,
    errors: &[ErrorRecord],
    has_modified_since_last_lint: bool,
) {
    let error_color = if has_modified_since_last_lint {
        egui::Color32::GRAY
    } else {
        theme::error_fg()
    };

    for error in errors.iter() {
        for range in &error.ranges {
            let start = range.start;
            let end = range.end;

            for section in &mut layout_job.sections {
                let section_start = section.byte_range.start.0;
                let section_end = section.byte_range.end.0;

                if start < section_end && end > section_start {
                    section.format.underline = egui::Stroke::new(1.5, error_color);
                }
            }
        }
    }
}
