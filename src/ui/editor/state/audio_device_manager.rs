#[derive(Default)]
pub(crate) struct AudioDeviceManager {
    /// The CPAL host, used for fetching audio devices.
    pub host: cpal::Host,
    /// The name of the currently selected CPAL output device.
    pub selected_output: Option<cpal::DeviceId>,
    // The default output device.
    pub default_output: Option<cpal::Device>,
    // The fetched audio output devices.
    pub outputs: Vec<cpal::Device>,
}
