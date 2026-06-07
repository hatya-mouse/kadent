use eframe::egui::Color32;

// --- FOREGROUND ---

pub(crate) fn primary_fg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(230, 230, 230)
    } else {
        Color32::from_rgb(40, 40, 40)
    }
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

// --- BUTTON ---

pub(crate) fn button_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(70, 70, 70)
    } else {
        Color32::from_rgb(216, 218, 220)
    }
}

// Visuals overrides for icon-style toolbar buttons (no visible background at rest).
pub(crate) fn icon_button_hovered() -> Color32 {
    Color32::from_rgba_unmultiplied(150, 150, 150, 50)
}

pub(crate) fn icon_button_active() -> Color32 {
    Color32::from_rgba_unmultiplied(150, 150, 150, 100)
}

// --- BORDER ---

/// Soft border used on regions, notes, node bodies, and grid lines.
pub(crate) fn border(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgba_unmultiplied(255, 255, 255, 30)
    } else {
        Color32::from_rgba_unmultiplied(0, 0, 0, 30)
    }
}

// --- SEPARATOR ---

/// Solid divider between panel sections (ruler border, panel splitters).
pub(crate) fn separator() -> Color32 {
    Color32::DARK_GRAY
}

/// Separator color when the divider is hovered or dragged.
pub(crate) fn separator_hovered() -> Color32 {
    Color32::from_gray(140)
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

// --- RULER ---

pub(crate) fn ruler_tick(dark_mode: bool) -> Color32 {
    primary_fg(dark_mode).gamma_multiply(0.45)
}

pub(crate) fn ruler_label(dark_mode: bool) -> Color32 {
    primary_fg(dark_mode).gamma_multiply(0.75)
}

// --- NODE GRAPH ---

pub(crate) fn node_port_input() -> Color32 {
    Color32::from_rgb(100, 160, 255)
}

pub(crate) fn node_port_output() -> Color32 {
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

pub(crate) fn region_text() -> Color32 {
    Color32::WHITE
}

/// Default color assigned to a newly created track.
pub(crate) fn default_track_color() -> Color32 {
    Color32::from_rgb(100, 150, 220)
}

// --- PEAK HOLD ---

pub(crate) fn peak_hold(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(255, 30, 0)
    } else {
        Color32::from_rgb(200, 15, 0)
    }
}
