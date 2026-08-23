use crate::core::audio_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

impl Encode for Box<dyn Track> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            e.field(0, &0u32)?;
            e.field(1, audio_track)?;
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            e.field(0, &1u32)?;
            e.field(1, note_track)?;
        } else {
            return Err(EncodeError::InvalidData);
        }
        Ok(())
    }
}

impl Decode for Box<dyn Track> {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let track_type: u32 = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match track_type {
            0 => {
                let audio_track: AudioTrack = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(audio_track))
            }
            1 => {
                let note_track: NoteTrack = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(note_track))
            }
            _ => Err(DecodeError::InvalidData),
        }
    }
}

impl Encode for AudioTrack {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, self.get_graph())?;
        e.field(1, self.get_all_regions())?;
        Ok(())
    }
}
