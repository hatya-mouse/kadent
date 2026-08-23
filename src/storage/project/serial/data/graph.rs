use crate::core::audio_engine::graph::{Graph, InputKey, InputSource};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- Graph ---

impl Encode for Graph {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, self.get_node_map())?;
        e.field(1, self.get_input_sources())?;
        e.field(2, &self.get_input_id())?;
        e.field(3, &self.get_output_id())?;
        Ok(())
    }
}

impl Decode for Graph {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(Graph::with_nodes(
            d.field(0)?.ok_or(DecodeError::InvalidData)?,
            d.field(1)?.ok_or(DecodeError::InvalidData)?,
            d.field(2)?.ok_or(DecodeError::InvalidData)?,
            d.field(3)?.ok_or(DecodeError::InvalidData)?,
        ))
    }
}

// --- InputKey ---

impl Encode for InputKey {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        let index_u64: u64 = self.1.try_into().map_err(|_| EncodeError::InvalidLength)?;
        e.field(0, &self.0)?;
        e.field(1, &index_u64)?;
        Ok(())
    }
}

impl Decode for InputKey {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let node_id = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        let index_u64: u64 = d.field(1)?.ok_or(DecodeError::InvalidData)?;
        let index_usize: usize = index_u64
            .try_into()
            .map_err(|_| DecodeError::InvalidLength)?;
        Ok(InputKey(node_id, index_usize))
    }
}

// --- InputSource ---

impl Encode for InputSource {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        match self {
            InputSource::Edge(node_id, output_index) => {
                let index_u64: u64 = (*output_index)
                    .try_into()
                    .map_err(|_| EncodeError::InvalidLength)?;
                e.field(0, &0u32)?;
                e.field(1, node_id)?;
                e.field(2, &index_u64)?;
            }
            InputSource::Keyframe => {
                e.field(0, &1u32)?;
            }
            InputSource::Zero => {
                e.field(0, &2u32)?;
            }
        }
        Ok(())
    }
}

impl Decode for InputSource {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant: u32 = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            0 => {
                let node_id = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                let index_u64: u64 = d.field(2)?.ok_or(DecodeError::InvalidData)?;
                let index_usize: usize = index_u64
                    .try_into()
                    .map_err(|_| DecodeError::InvalidLength)?;
                Ok(InputSource::Edge(node_id, index_usize))
            }
            1 => Ok(InputSource::Keyframe),
            2 => Ok(InputSource::Zero),
            _ => Err(DecodeError::InvalidData),
        }
    }
}
