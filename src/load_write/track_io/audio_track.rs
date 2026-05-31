use crate::load_write::{AsBytes, FromBytes};
use kreniq_engine::track::audio_track::AudioTrack;

impl AsBytes for AudioTrack {
    fn as_bytes(&self, _bytes: &mut Vec<u8>) {}
}

impl FromBytes for AudioTrack {
    fn from_bytes(_bytes: &[u8]) -> std::io::Result<Self> {
        let track = AudioTrack::default();
        Ok(track)
    }
}
