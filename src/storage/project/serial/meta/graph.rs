use crate::{
    core::metadata::{GraphMeta, NodeMeta},
    storage::project::serial::meta::StoredPos2,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- GraphMeta ---

impl Encode for GraphMeta {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.nodes)?;
        Ok(())
    }
}

impl Decode for GraphMeta {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(GraphMeta {
            nodes: d.field(0)?.ok_or(DecodeError::InvalidData)?,
        })
    }
}

// --- NodeMeta ---

impl Encode for NodeMeta {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.node_type)?;
        e.field(1, &self.display_name)?;
        e.field(2, &StoredPos2::from_pos2(&self.pos))?;
        Ok(())
    }
}

impl Decode for NodeMeta {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(NodeMeta {
            node_type: d.field(0)?.ok_or(DecodeError::InvalidData)?,
            display_name: d.field(1)?.ok_or(DecodeError::InvalidData)?,
            pos: d
                .field::<StoredPos2>(2)?
                .ok_or(DecodeError::InvalidData)?
                .to_pos2(),
        })
    }
}
