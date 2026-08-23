use crate::core::{
    audio_engine::node::{
        Node,
        builtin::{AudioInputNode, AudioOutputNode, AutomationNode, NoteInputNode},
    },
    kasl_node::KaslNode,
    metadata::NodeType,
};
use sode::{Decode, DecodeError, Encode, EncodeError, Encoder, ValueDecoder};

// --- Box<dyn Node> ---

impl Encode for Box<dyn Node> {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        if let Some(node) = self.as_any().downcast_ref::<NoteInputNode>() {
            e.field(0, &NodeType::NoteInput)?;
        } else if let Some(node) = self.as_any().downcast_ref::<AudioInputNode>() {
            e.field(0, &NodeType::AudioInput)?;
        } else if let Some(node) = self.as_any().downcast_ref::<AudioOutputNode>() {
            e.field(0, &NodeType::AudioOutput)?;
        } else if let Some(node) = self.as_any().downcast_ref::<KaslNode>() {
            e.field(0, &NodeType::Kasl)?;
            e.field(1, node)?;
        } else if let Some(node) = self.as_any().downcast_ref::<AutomationNode>() {
            e.field(0, &NodeType::Automation)?;
            e.field(1, node)?;
        }
        Ok(())
    }
}

impl Decode for Box<dyn Node> {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let variant: NodeType = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        match variant {
            NodeType::NoteInput => Ok(Box::new(NoteInputNode::default())),
            NodeType::AudioInput => Ok(Box::new(AudioInputNode::default())),
            NodeType::AudioOutput => Ok(Box::new(AudioOutputNode::default())),
            NodeType::Kasl => {
                let node: KaslNode = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(node))
            }
            NodeType::Automation => {
                let node: AutomationNode = d.field(1)?.ok_or(DecodeError::InvalidData)?;
                Ok(Box::new(node))
            }
        }
    }
}

// --- NodeType ---

impl Encode for NodeType {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.write_u32(*self as u32);
        Ok(())
    }
}

impl Decode for NodeType {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        match d.read_u32()? {
            0 => Ok(NodeType::NoteInput),
            1 => Ok(NodeType::AudioInput),
            2 => Ok(NodeType::AudioOutput),
            3 => Ok(NodeType::Kasl),
            4 => Ok(NodeType::Automation),
            _ => Err(DecodeError::InvalidData),
        }
    }
}

// --- KaslNode ---

impl Encode for KaslNode {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        let kasl_path = self.get_file_path().ok_or(EncodeError::InvalidData)?;
        e.field(0, kasl_path)?;
        Ok(())
    }
}

impl Decode for KaslNode {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        let kasl_path: String = d.field(0)?.ok_or(DecodeError::InvalidData)?;
        Ok(KaslNode::with_path(kasl_path))
    }
}

// --- AutomationNode ---

impl Encode for AutomationNode {
    fn encode(&self, e: &mut Encoder) -> Result<(), EncodeError> {
        e.field(0, &self.track)?;
        Ok(())
    }
}

impl Decode for AutomationNode {
    fn decode(d: &mut ValueDecoder) -> Result<Self, DecodeError> {
        let d = d.to_field_decoder()?;
        Ok(AutomationNode::new(
            d.field(0)?.ok_or(DecodeError::InvalidData)?,
        ))
    }
}
