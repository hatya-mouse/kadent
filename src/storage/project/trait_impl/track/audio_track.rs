use crate::storage::project::{AsBytes, FromBytes, error::LoadError};
use kadent_engine::track::audio_track::AudioTrack;

impl AsBytes for AudioTrack {
    fn as_bytes(&self, _bytes: &mut Vec<u8>) {}
}

impl FromBytes for AudioTrack {
    fn from_bytes(_bytes: &[u8]) -> Result<Self, LoadError> {
        let track = AudioTrack::default();
        Ok(track)
    }
}
