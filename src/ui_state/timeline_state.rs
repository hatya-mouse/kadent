pub struct TimelineState {
    /// The height of each track in the timeline.
    pub track_height: f32,

    /// The width of the track list.
    pub track_list_width: f32,

    /// The y scroll amount of the timeline.
    pub timeline_scroll_y: f32,

    /// Pixels per beat in the timeline.
    pub pixels_per_beat: f32,

    /// True while the user is dragging on the beat ruler to seek.
    /// Used to send AudioCommand::Seek only on mouse release, not every frame.
    pub ruler_seeking: bool,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            track_height: 50.0,
            track_list_width: 200.0,
            timeline_scroll_y: 0.0,
            pixels_per_beat: 80.0,
            ruler_seeking: false,
        }
    }
}
