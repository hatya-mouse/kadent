use crate::core::metadata::RegionMeta;
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

impl Encode for RegionMeta {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.name)?;
        e.field(1, &self.bounds)?;
        Ok(())
    }
}

impl Decode for RegionMeta {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(RegionMeta {
            name: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            bounds: d.field(1)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}
