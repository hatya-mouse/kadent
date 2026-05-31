mod kasl_node;

use crate::{
    kasl_node::KaslNode,
    load_write::{AsBytes, FromBytes, safe_read},
};
use kreniq_engine::node::{
    Node,
    builtin::{AudioInputNode, AudioOutputNode, NoteInputNode},
};
use std::io::{Cursor, Read};

#[repr(u8)]
enum NodeKind {
    NoteInput = 0,
    AudioInput = 1,
    AudioOutput = 2,
    Kasl = 3,
}

impl AsBytes for dyn Node {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write depending on the type of the node
        if self.as_any().is::<NoteInputNode>() {
            // Note Input Node
            bytes.push(NodeKind::NoteInput as u8);
        } else if self.as_any().is::<AudioInputNode>() {
            // Audio Input Node
            bytes.push(NodeKind::AudioInput as u8);
        } else if self.as_any().is::<AudioOutputNode>() {
            // Audio Output Node
            bytes.push(NodeKind::AudioOutput as u8);
        } else if let Some(kasl_node) = self.as_any().downcast_ref::<KaslNode>() {
            // KASL Node
            bytes.push(NodeKind::Kasl as u8);
            // Convert the KaslNode into bytes
            let mut node_bytes = Vec::new();
            kasl_node.as_bytes(&mut node_bytes);
            // Write the size of the node
            bytes.extend((node_bytes.len() as u64).to_le_bytes());
            // Write the KaslNode data
            bytes.extend(node_bytes);
        }
    }
}

impl FromBytes for Box<dyn Node> {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Get the first one byte and get the type of the node
        let mut type_byte = [0u8; 1];
        cursor.read_exact(&mut type_byte)?;

        match type_byte[0] {
            0 => Ok(Box::new(NoteInputNode::default())),
            1 => Ok(Box::new(AudioInputNode::default())),
            2 => Ok(Box::new(AudioOutputNode::default())),
            3 => {
                // Get the size of the KASL Node data
                let mut buf = [0u8; 8];
                cursor.read_exact(&mut buf)?;
                let node_length = u64::from_le_bytes(buf) as usize;
                // Get the KASL Node data
                let node_bytes = safe_read(&mut cursor, node_length)?;
                // Create a new node and set the code
                Ok(Box::new(KaslNode::from_bytes(&node_bytes)?))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid node kind",
            )),
        }
    }
}
