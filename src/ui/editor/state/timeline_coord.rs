use eframe::egui;

#[derive(Clone, Debug)]
pub(crate) struct TimelineCoord {
    pub(crate) ppb: f32,
    pub(crate) y_scale: f32,
    pub(crate) scroll: egui::Vec2,
}

impl TimelineCoord {
    pub(crate) fn new(ppb: f32, y_scale: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb,
            y_scale,
            scroll,
        }
    }

    /// Calculates pixels per tick based on the current ppb and resolution.
    pub(crate) fn ppt(&self, resolution: u64) -> f32 {
        self.ppb / resolution as f32
    }

    /// Calculates ticks per pixel based on the current ppb and resolution.
    pub(crate) fn tpp(&self, resolution: u64) -> f32 {
        resolution as f32 / self.ppb
    }

    pub(crate) fn with_zoom_and_scroll(&self, y_scale: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb: self.ppb,
            y_scale,
            scroll,
        }
    }

    pub(crate) fn with_ppb_and_scroll(&self, ppb: f32, scroll: egui::Vec2) -> Self {
        Self {
            ppb,
            y_scale: self.y_scale,
            scroll,
        }
    }

    /// Applies the scroll offset in the following priority order:
    /// Scroll Bar > Zoom > Scroll Area
    pub(crate) fn apply_scroll(
        &mut self,
        bar_scroll_x: Option<f32>,
        zoom_output: Option<TimelineCoord>,
        scroll_area_output: egui::Vec2,
    ) {
        match bar_scroll_x {
            Some(bar_scroll_x) => {
                self.scroll.x = bar_scroll_x;
            }
            None => {
                if let Some(zoom_output) = zoom_output {
                    *self = zoom_output;
                } else {
                    self.scroll = scroll_area_output;
                }
            }
        }
    }
}
