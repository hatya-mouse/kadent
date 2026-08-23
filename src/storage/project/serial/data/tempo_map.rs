use crate::core::audio_engine::timing::{TempoEvent, TempoMap};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- TempoMap ---

impl Encode for TempoMap {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.resolution)?;
        e.field(1, &self.initial_bpm)?;
        e.field(2, &self.events)?;
        Ok(())
    }
}

impl Decode for TempoMap {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(TempoMap::with_events(
            d.field(0)?.ok_or(DecodeError::InvalidData)?,
            d.field(1)?.ok_or(DecodeError::InvalidData)?,
            d.field(2)?.ok_or(DecodeError::InvalidData)?,
        ))
    }
}

// --- TempoEvent ---

impl Encode for TempoEvent {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.tick)?;
        e.field(0, &self.bpm)?;
        Ok(())
    }
}

impl Decode for TempoEvent {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(TempoEvent::new(
            d.field(0)?.ok_or(DecodeError::InvalidData)?,
            d.field(1)?.ok_or(DecodeError::InvalidData)?,
        ))
    }
}
