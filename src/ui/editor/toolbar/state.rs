use crate::core::frame_process::PeakHold;

#[derive(Default)]
pub(crate) struct ToolbarState {
    /// The last VU meter value received from the audio thread.
    pub(crate) last_vu_value: Vec<f32>,
    /// The peak hold for each channel.
    pub(crate) peak_holds: Vec<PeakHold>,
}
