use crate::core::{
    audio_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack},
    metadata::TrackType,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- Box<dyn Track> ---

impl Encode for Box<dyn Track> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            e.field(0, &TrackType::Audio)?;
            e.field(1, audio_track)?;
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            e.field(0, &TrackType::Note)?;
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
        let track_type: TrackType = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match track_type {
            TrackType::Audio => {
                let audio_track: AudioTrack = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(audio_track))
            }
            TrackType::Note => {
                let note_track: NoteTrack = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(note_track))
            }
        }
    }
}

// --- TrackType ---

impl Encode for TrackType {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u32(*self as u32);
        Ok(())
    }
}

impl Decode for TrackType {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        match d.read_u32()? {
            0 => Ok(TrackType::Audio),
            1 => Ok(TrackType::Note),
            _ => Err(DecodeError::InvalidData),
        }
    }
}
