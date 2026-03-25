use eframe::egui;

pub struct TrackMeta {
    pub name: String,
    pub color: egui::Color32,
}

impl TrackMeta {
    pub fn new(name: String, color: egui::Color32) -> Self {
        Self { name, color }
    }
}
