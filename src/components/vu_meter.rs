use crate::theme;
use eframe::egui;

const METER_GAP: f32 = 2.0;

pub(crate) fn vu_meter(ui: &mut egui::Ui, channels: &[f32], size: egui::Vec2, corner_radius: u8) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::empty());
    let row_height = size.y / channels.len() as f32;
    let height_with_gap = row_height - METER_GAP;

    // Draw the background
    ui.painter().rect_filled(
        rect,
        corner_radius,
        theme::button_bg(ui.visuals().dark_mode),
    );

    for (row, value) in channels.iter().enumerate() {
        let row_rect = egui::Rect::from_min_size(
            rect.min + egui::vec2(0.0, row_height * row as f32 + METER_GAP / 2.0),
            // Leave space for the corner radius on the right side
            egui::vec2(
                (size.x - corner_radius as f32) * value.clamp(0.0, 1.0),
                height_with_gap,
            ),
        );
        let fill_color = if *value < 0.6 {
            egui::Color32::GREEN
        } else if *value < 0.85 {
            egui::Color32::YELLOW
        } else {
            egui::Color32::RED
        };

        // Apply corner radius only to the first and last rows
        // If there's only one channel, apply the radius to all corners
        let row_radius = if channels.len() == 1 {
            egui::CornerRadius {
                nw: corner_radius,
                ne: 0,
                sw: corner_radius,
                se: 0,
            }
        } else if row == 0 {
            egui::CornerRadius {
                nw: corner_radius,
                ne: 0,
                sw: 0,
                se: 0,
            }
        } else if row == channels.len() - 1 {
            egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: corner_radius,
                se: 0,
            }
        } else {
            egui::CornerRadius::ZERO
        };
        ui.painter().rect_filled(row_rect, row_radius, fill_color);
    }
}
