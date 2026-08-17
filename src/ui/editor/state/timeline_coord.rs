use eframe::egui;

#[derive(Clone)]
pub(crate) struct TimelineCoord {
    pub ppb: f32,
    pub y_zoom: f32,
    pub scroll: egui::Vec2,
}

impl TimelineCoord {
    pub fn new(ppb: f32, y_zoom: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb,
            y_zoom,
            scroll,
        }
    }

    /// Calculates pixels per tick based on the current ppb and resolution.
    pub fn ppt(&self, resolution: u64) -> f32 {
        self.ppb / resolution as f32
    }

    /// Calculates ticks per pixel based on the current ppb and resolution.
    pub fn tpp(&self, resolution: u64) -> f32 {
        resolution as f32 / self.ppb
    }

    pub fn with_zoom_and_scroll(&self, y_zoom: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb: self.ppb,
            y_zoom,
            scroll,
        }
    }

    pub fn with_ppb_and_scroll(&self, ppb: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb,
            y_zoom: self.y_zoom,
            scroll,
        }
    }

    pub fn with_scroll(&self, scroll: egui::Vec2) -> Self {
        Self {
            ppb: self.ppb,
            y_zoom: self.y_zoom,
            scroll,
        }
    }
}
