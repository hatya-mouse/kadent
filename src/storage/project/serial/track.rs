use kadent_engine::track::{Track, audio_track::AudioTrack};
use sode::{Decode, Encode, EncodeError, Encoder};

impl Encode for AudioTrack {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {}
}

impl Decode for Box<dyn Track> {}
