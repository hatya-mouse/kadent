use crate::core::{audio_engine::mixer::ProjectData, metadata::ProjectMeta};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

mod data;
mod meta;
mod misc;

pub(super) struct EncodableProject<'a> {
    pub(super) data: &'a ProjectData,
    pub(super) meta: &'a ProjectMeta,
}

pub(super) struct DecodableProject {
    pub(super) data: ProjectData,
    pub(super) meta: ProjectMeta,
}

impl Encode for EncodableProject<'_> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, self.data)?;
        e.field(1, self.meta)?;
        Ok(())
    }
}

impl Decode for DecodableProject {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(DecodableProject {
            data: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            meta: d.field(1)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}
