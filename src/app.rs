use eframe::{App, egui::CentralPanel};

#[derive(Default)]
pub struct KnodiqApp {}

impl App for KnodiqApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Knodiq");
        });
    }
}
