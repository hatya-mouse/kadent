use crate::{
    components::{color_picker::color_picker, text_input::text_input},
    theme,
    ui::{
        EditorUi,
        editor::inspector::{inspector_item, inspector_section},
    },
};
use eframe::egui;
use kreniq_engine::mixer::TrackID;

impl EditorUi {
    pub(super) fn track_inspector(&mut self, ui: &mut egui::Ui, track_id: &TrackID) {
        inspector_section(ui, ("track_section", track_id), "Track", |ui| {
            let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
                return;
            };

            inspector_item(ui, "Name", |ui| {
                text_input(ui, &mut track_meta.name);
            });

            inspector_item(ui, "Color", |ui| {
                color_picker(ui, &mut track_meta.color);
            });

            inspector_item(ui, "Delete", |ui| {
                if ui.button("Delete Track").clicked() {
                    self.remove_track(track_id);
                }
            });

            if self.debug_mode {
                ui.separator();
                inspector_item(ui, "Track ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", track_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
            }
        });
    }
}
