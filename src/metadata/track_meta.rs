use eframe::egui;

pub struct TrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub is_muted: bool,
    pub is_solo: bool,
}
