use crate::{
    theme,
    ui::{
        EditorUi,
        editor::inspector::{inspector_item, inspector_section},
    },
};
use eframe::egui;
use kreniq_engine::{
    mixer::TrackID,
    track::{RegionID, note_track::NoteID},
};

impl EditorUi {
    pub(super) fn note_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        region_id: &RegionID,
        note_id: &NoteID,
    ) {
        // let Some(track) = self
        //     .project
        //     .get_track_mut(track_id)
        //     .and_then(|track| track.as_any_mut().downcast_mut::<NoteTrack>())
        // else {
        //     return;
        // };
        // let Some(region) = track.get_region_mut(region_id) else {
        //     return;
        // };
        // let Some(note) = region.get_note_mut(note_id) else {
        //     return;
        // };

        inspector_section(
            ui,
            ("note_section", track_id, region_id, note_id),
            "Note",
            |ui| {
                if self.debug_mode {
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
}
