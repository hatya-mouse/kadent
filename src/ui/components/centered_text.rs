use eframe::egui;

pub(crate) fn centered_text(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>) {
    ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| ui.label(text),
    );
}
