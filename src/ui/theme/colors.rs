use eframe::egui::{Color32, Stroke};
use kadent_engine::node::builtin::CurveType;

// --- FOREGROUND ---

pub(crate) fn primary_fg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(230, 230, 230)
    } else {
        Color32::from_rgb(40, 40, 40)
    }
}

pub(crate) fn secondary_fg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(180, 180, 180)
    } else {
        Color32::from_rgb(120, 120, 120)
    }
}

pub(crate) const fn selected_fg() -> Color32 {
    Color32::from_rgb(240, 240, 240)
}

pub(crate) const fn successful_fg() -> Color32 {
    Color32::from_rgb(0, 180, 0)
}

pub(crate) const fn error_fg() -> Color32 {
    Color32::from_rgb(180, 0, 0)
}

// --- BACKGROUNDS ---

pub(crate) fn primary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(38, 40, 50)
    } else {
        Color32::from_rgb(251, 253, 255)
    }
}

pub(crate) fn secondary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(33, 35, 41)
    } else {
        Color32::from_rgb(240, 245, 248)
    }
}

pub(crate) fn tertiary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(28, 30, 38)
    } else {
        Color32::from_rgb(234, 236, 238)
    }
}

pub(crate) const fn selected_bg() -> Color32 {
    Color32::from_rgb(33, 140, 255)
}

pub(crate) fn scroll_bar_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_white_alpha(100)
    } else {
        Color32::from_black_alpha(60)
    }
}

// --- BUTTON ---

pub(crate) fn button_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(70, 70, 70)
    } else {
        Color32::from_rgb(216, 218, 220)
    }
}

pub(crate) fn card_button_hovered(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_white_alpha(10)
    } else {
        Color32::from_black_alpha(10)
    }
}

pub(crate) fn card_button_pressed(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_white_alpha(20)
    } else {
        Color32::from_black_alpha(20)
    }
}

pub(crate) fn icon_button_hovered() -> Color32 {
    Color32::from_rgba_unmultiplied(150, 150, 150, 50)
}

pub(crate) fn icon_button_active() -> Color32 {
    Color32::from_rgba_unmultiplied(150, 150, 150, 100)
}

/// Green color used for play button.
pub(crate) fn transport_green(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(40, 130, 0)
    } else {
        Color32::from_rgb(40, 170, 0)
    }
}

// --- BORDER ---

/// Soft border used on regions, notes, node bodies, and grid lines.
pub(crate) fn border_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgba_unmultiplied(255, 255, 255, 30)
    } else {
        Color32::from_rgba_unmultiplied(0, 0, 0, 30)
    }
}

/// Soft border stroke used on regions, notes, node bodies, and grid lines.
pub(crate) fn border(dark_mode: bool) -> Stroke {
    Stroke::new(1.0, border_color(dark_mode))
}

// --- SEPARATOR ---

/// Solid divider between panel sections (ruler border, panel splitters).
pub(crate) const fn separator(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_gray(60)
    } else {
        Color32::from_gray(200)
    }
}

/// Separator color when the divider is hovered or dragged.
pub(crate) const fn separator_hovered(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_gray(80)
    } else {
        Color32::from_gray(180)
    }
}

// --- PANEL INTERACTIONS ---

pub(crate) fn panel_drag_highlight() -> Color32 {
    Color32::from_rgba_unmultiplied(100, 150, 255, 60)
}

pub(crate) fn panel_hover_highlight() -> Color32 {
    Color32::from_rgba_unmultiplied(100, 150, 255, 40)
}

pub(crate) fn panel_collapse_overlay() -> Color32 {
    Color32::from_rgba_unmultiplied(200, 60, 60, 80)
}

// --- NODE GRAPH ---

pub(crate) const fn node_port_input() -> Color32 {
    Color32::from_rgb(100, 160, 255)
}

pub(crate) const fn node_port_output() -> Color32 {
    Color32::from_rgb(255, 160, 100)
}

pub(crate) fn node_edge(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(200, 200, 200)
    } else {
        Color32::from_rgb(80, 80, 80)
    }
}

// --- TIMELINE / REGIONS ---

pub(crate) fn region_selected(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::WHITE
    } else {
        Color32::from_rgb(160, 160, 160)
    }
}

pub(crate) const fn region_text() -> Color32 {
    Color32::WHITE
}

/// Default color assigned to a newly created track.
pub(crate) const fn default_track_color() -> Color32 {
    Color32::from_rgb(100, 150, 220)
}

pub(crate) const fn waveform_color() -> Color32 {
    Color32::from_rgb(255, 255, 255)
}

// --- RULER ---

pub(crate) fn ruler_label(dark_mode: bool) -> Color32 {
    primary_fg(dark_mode).gamma_multiply(0.75)
}

pub(crate) fn range_outside_overlay() -> Color32 {
    Color32::from_black_alpha(60)
}

// --- PEAK HOLD ---

pub(crate) fn peak_hold(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(255, 30, 0)
    } else {
        Color32::from_rgb(200, 15, 0)
    }
}

// --- KEYFRAMES ---

pub(crate) fn keyframe(curve: &CurveType) -> Color32 {
    match curve {
        CurveType::Step => Color32::from_rgb(254, 68, 36),
        CurveType::Linear => Color32::from_rgb(18, 255, 81),
        CurveType::Smooth { .. } => Color32::from_rgb(32, 172, 255),
    }
}

pub(crate) fn keyframe_stroke(dark_mode: bool) -> Stroke {
    Stroke::new(
        2.0,
        if dark_mode {
            Color32::WHITE
        } else {
            Color32::from_black_alpha(200)
        },
    )
}

pub(crate) fn selected_keyframe() -> Color32 {
    Color32::from_rgb(255, 220, 25)
}
