#[derive(Default)]
pub(crate) struct AudioDeviceManager {
    /// The CPAL host, used for fetching audio devices.
    pub(crate) host: cpal::Host,
    /// The name of the currently selected CPAL output device.
    pub(crate) selected_output: Option<cpal::DeviceId>,
    // The default output device.
    pub(crate) default_output: Option<cpal::Device>,
    // The fetched audio output devices.
    pub(crate) outputs: Vec<cpal::Device>,
}
