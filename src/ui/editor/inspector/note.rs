use crate::ui::{
    EditorState,
    editor::inspector::{inspector_item, inspector_section},
    theme,
};
use eframe::egui;
use kadent_engine::{
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};

pub(super) fn note_inspector(
    ui: &mut egui::Ui,
    state: &EditorState,
    track_id: &TrackID,
    region_id: &RegionID,
    note_id: &NoteID,
) {
    inspector_section(
        ui,
        ("note_section", track_id, region_id, note_id),
        "Note",
        |ui| {
            if state.debug_mode {
                ui.separator();
                inspector_item(ui, "Track ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", track_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
                inspector_item(ui, "Region ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", region_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
                inspector_item(ui, "Note ID", |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", note_id.0))
                            .size(theme::normal_font_size()),
                    );
                });
            }
        },
    );
}
