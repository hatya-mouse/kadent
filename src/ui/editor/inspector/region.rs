use crate::{
    components::text_input::text_input,
    theme,
    ui::{
        EditorUi,
        editor::inspector::{inspector_item, inspector_section},
    },
};
use eframe::egui;
use kreniq_engine::{mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(super) fn region_inspector(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        region_id: &RegionID,
    ) {
        let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
            return;
        };
        let Some(region_meta) = track_meta.get_region_mut(region_id) else {
            return;
        };

        inspector_section(
            ui,
            ("region_section", track_id, region_id),
            "Region",
            |ui| {
                inspector_item(ui, "Name", |ui| {
                    text_input(ui, &mut region_meta.name);
                });

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
                }
            },
        );
    }
}
