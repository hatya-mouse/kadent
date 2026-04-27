use crate::theme;
use eframe::egui;

pub(crate) fn color_picker(ui: &mut egui::Ui, color: &mut egui::Color32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(28.0, 20.0), egui::Sense::click());

    let dark = ui.visuals().dark_mode;
    ui.painter().rect_filled(rect, 4.0, *color);
    ui.painter().rect_stroke(
        rect,
        4.0,
        egui::Stroke::new(1.0, theme::border(dark)),
        egui::StrokeKind::Outside,
    );

    let frame = egui::Frame::popup(ui.style())
        .shadow(egui::Shadow::NONE)
        .fill(theme::primary_bg(dark))
        .stroke(egui::Stroke::new(1.0, theme::border(dark)));
    egui::Popup::from_toggle_button_response(&response)
        .gap(4.0)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .frame(frame)
        .show(|ui| {
            ui.set_min_width(180.0);
            hsv_sliders(ui, color);
        });

    response
}

fn hsv_sliders(ui: &mut egui::Ui, color: &mut egui::Color32) {
    let [r, g, b, _] = color.to_array();
    let mut hsv = rgb_to_hsv(r, g, b);
    let mut changed = false;

    egui::Grid::new("hsv_grid")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.label("H");
            changed |= ui
                .add(egui::Slider::new(&mut hsv[0], 0.0..=360.0).suffix("°"))
                .changed();
            ui.end_row();

            ui.label("S");
            changed |= ui.add(egui::Slider::new(&mut hsv[1], 0.0..=1.0)).changed();
            ui.end_row();

            ui.label("V");
            changed |= ui.add(egui::Slider::new(&mut hsv[2], 0.0..=1.0)).changed();
            ui.end_row();
        });

    if changed {
        let [r, g, b] = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
        *color = egui::Color32::from_rgb(r, g, b);
    }
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> [f32; 3] {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta < 1e-6 {
        0.0
    } else if (max - r).abs() < 1e-6 {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < 1e-6 {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let s = if max < 1e-6 { 0.0 } else { delta / max };

    [h, s, max]
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    ]
}
