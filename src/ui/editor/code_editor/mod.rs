mod file_tree;

use crate::{theme, ui::EditorUi};
use eframe::egui;
use file_tree::collect_kasl_files;
use std::{fs, path::PathBuf};

impl EditorUi {
    pub(in crate::ui) fn code_editor(&mut self, ui: &mut egui::Ui) {
        let project_dir = self.project_dir.clone();
        let dark_mode = ui.visuals().dark_mode;

        if ui
            .ctx()
            .input(|i| i.key_pressed(egui::Key::S) && i.modifiers.command)
        {
            self.save_kasl_file();
        }

        if self.ui_state.code_editor_state.file_list.is_none() {
            self.ui_state.code_editor_state.file_list = Some(collect_kasl_files(&project_dir));
        }

        let mut file_to_open: Option<PathBuf> = None;

        egui::Panel::left("code_editor_files")
            .resizable(true)
            .default_size(200.0)
            .frame(egui::Frame::new().fill(theme::tertiary_bg(dark_mode)))
            .show_inside(ui, |ui| {
                let state = &mut self.ui_state.code_editor_state;

                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Files").strong());
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("⟳").clicked() {
                                        state.file_list = Some(collect_kasl_files(&project_dir));
                                    }
                                },
                            );
                        });
                    });

                let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                let sep_y = ui.available_rect_before_wrap().top();
                let sep_rect = ui.available_rect_before_wrap();
                ui.painter().line_segment(
                    [
                        egui::pos2(sep_rect.left(), sep_y),
                        egui::pos2(sep_rect.right(), sep_y),
                    ],
                    stroke,
                );

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;

                    let files = state.file_list.as_deref().unwrap_or(&[]);
                    let open = state.open_file.clone();

                    if files.is_empty() {
                        egui::Frame::new()
                            .inner_margin(egui::Margin::symmetric(8, 8))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("No .kasl files found").weak());
                            });
                    } else {
                        for path in files {
                            let rel = path.strip_prefix(&project_dir).unwrap_or(path);
                            let selected = open.as_deref() == Some(path.as_path());
                            let resp = egui::Frame::new()
                                .inner_margin(egui::Margin::symmetric(8, 3))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.selectable_label(selected, rel.to_string_lossy().as_ref())
                                });
                            if resp.inner.clicked() {
                                file_to_open = Some(path.clone());
                            }
                        }
                    }
                });
            });

        if let Some(path) = file_to_open {
            self.open_kasl_file(path);
        }

        let state = &mut self.ui_state.code_editor_state;
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::primary_bg(dark_mode)))
            .show_inside(ui, |ui| {
                if state.open_file.is_some() {
                    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                        let resp = ui.add(
                            egui::TextEdit::multiline(&mut state.content)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace),
                        );
                        if resp.changed() {
                            state.dirty = true;
                        }
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(egui::RichText::new("Select a file to edit").weak());
                    });
                }
            });
    }

    fn save_kasl_file(&mut self) {
        let state = &mut self.ui_state.code_editor_state;
        if !state.dirty {
            return;
        }
        if let Some(path) = state.open_file.clone() {
            let _ = fs::write(&path, &state.content);
            state.dirty = false;
        }
    }

    fn open_kasl_file(&mut self, path: PathBuf) {
        let state = &mut self.ui_state.code_editor_state;
        if state.open_file.as_deref() == Some(path.as_path()) {
            return;
        }
        if state.dirty
            && let Some(current) = state.open_file.clone()
        {
            let _ = fs::write(&current, &state.content);
        }
        state.content = fs::read_to_string(&path).unwrap_or_default();
        state.open_file = Some(path);
        state.dirty = false;
    }
}
