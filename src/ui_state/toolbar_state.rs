use std::time::Instant;

#[derive(Debug, Clone)]
pub struct PeakHold {
    pub value: f32,
    pub hold_time: Instant,
}

#[derive(Default)]
pub struct ToolbarState {
    /// The last VU meter value received from the audio thread.
    pub last_vu_value: Vec<f32>,
    /// The peak hold for each channel.
    pub peak_holds: Vec<PeakHold>,
}
