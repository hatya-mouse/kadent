use std::io::{Cursor, Read};

use eframe::egui;

use crate::{
    load_write::{AsBytes, FromBytes},
    metadata::{NodeMeta, NodeType},
};

pub struct StoredNodeMeta {
    node_type: NodeType,
    display_name: String,
    pos: (f32, f32),
}

impl StoredNodeMeta {
    pub fn from_node_meta(node_meta: &NodeMeta) -> Self {
        Self {
            node_type: node_meta.node_type.clone(),
            display_name: node_meta.display_name.clone(),
            pos: (node_meta.pos.x, node_meta.pos.y),
        }
    }

    pub fn to_node_meta(&self) -> NodeMeta {
        NodeMeta {
            node_type: self.node_type.clone(),
            display_name: self.display_name.clone(),
            pos: egui::pos2(self.pos.0, self.pos.1),
        }
    }
}

impl AsBytes for StoredNodeMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the node type as a single byte
        bytes.push(self.node_type.clone() as u8);

        // Write the display name length and bytes
        let name_bytes = self.display_name.as_bytes();
        bytes.extend((name_bytes.len() as u64).to_le_bytes());
        bytes.extend(name_bytes);

        // Write the position to the bytes
        bytes.extend(self.pos.0.to_le_bytes());
        bytes.extend(self.pos.1.to_le_bytes());
    }
}

impl FromBytes for StoredNodeMeta {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the node type from the first byte
        let mut node_type_byte = [0u8; 1];
        cursor.read_exact(&mut node_type_byte)?;
        let node_type: NodeType = node_type_byte[0].into();

        // Read the display name length and bytesa
        let mut name_len_bytes = [0u8; 8];
        cursor.read_exact(&mut name_len_bytes)?;
        let name_len = u64::from_le_bytes(name_len_bytes) as usize;

        let mut name_bytes = vec![0u8; name_len];
        cursor.read_exact(&mut name_bytes)?;
        let display_name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Read the position from the bytes
        let mut pos_x_bytes = [0u8; 4];
        let mut pos_y_bytes = [0u8; 4];
        cursor.read_exact(&mut pos_x_bytes)?;
        cursor.read_exact(&mut pos_y_bytes)?;

        // Construct the StoredNodeMeta from the read data
        let stored_node_meta = StoredNodeMeta {
            node_type,
            display_name,
            pos: (
                f32::from_le_bytes(pos_x_bytes),
                f32::from_le_bytes(pos_y_bytes),
            ),
        };

        Ok(stored_node_meta)
    }
}
