pub struct TimelineState {
    /// The latest playhead samples received from the audio thread.
    pub last_playhead: usize,

    /// The last playhead x position.
    pub last_playhead_x: f32,

    /// The height of each track in the timeline.
    pub track_height: f32,

    /// The width of the track list.
    pub track_list_width: f32,

    /// The y scroll amount of the timeline.
    pub timeline_scroll_y: f32,

    /// Pixels per beat in the timeline.
    pub pixels_per_beat: f32,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            last_playhead: 0,
            last_playhead_x: 0.0,
            track_height: 50.0,
            track_list_width: 200.0,
            timeline_scroll_y: 0.0,
            pixels_per_beat: 80.0,
        }
    }
}
