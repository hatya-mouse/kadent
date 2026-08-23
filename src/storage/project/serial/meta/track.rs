use crate::{core::metadata::TrackMeta, storage::project::serial::meta::StoredColor};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

impl Encode for TrackMeta {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.name)?;
        e.field(1, &StoredColor::from_color32(&self.color))?;
        e.field(2, &self.track_type)?;
        e.field(3, &self.regions)?;
        e.field(4, &self.graph)?;
        Ok(())
    }
}

impl Decode for TrackMeta {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(TrackMeta {
            name: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            color: d
                .field::<StoredColor>(1)?
                .ok_or(DecodeError::InvalidData)?
                .to_color32(),
            track_type: d.field(2)?.ok_or(DecodeError::InvalidData)?,
            regions: d.field(3)?.ok_or(DecodeError::InvalidData)?,
            graph: d.field(4)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}
