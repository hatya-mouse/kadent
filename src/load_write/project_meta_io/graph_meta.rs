use crate::{
    load_write::{AsBytes, FromBytes, project_meta_io::StoredNodeMeta},
    metadata::GraphMeta,
};
use kreniq_engine::graph::node_id::NodeID;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

#[derive(Default)]
pub struct StoredGraphMeta {
    node_metas: HashMap<NodeID, StoredNodeMeta>,
}

impl StoredGraphMeta {
    pub fn from_graph_meta(graph_meta: &GraphMeta) -> Self {
        Self {
            node_metas: graph_meta
                .nodes
                .iter()
                .map(|(node_id, node)| (*node_id, StoredNodeMeta::from_node_meta(node)))
                .collect(),
        }
    }

    pub fn to_graph_meta(&self) -> GraphMeta {
        let mut graph_meta = GraphMeta::default();
        for (node_id, node) in &self.node_metas {
            graph_meta.set_node_meta(*node_id, node.to_node_meta());
        }
        graph_meta
    }
}

impl AsBytes for StoredGraphMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        let mut nodes_bytes = Vec::new();
        for (node_id, node) in &self.node_metas {
            let mut node_bytes = Vec::new();
            node.as_bytes(&mut node_bytes);

            // Write the ID of the node
            nodes_bytes.extend((node_id.0 as u64).to_le_bytes());
            // Write the length of the node bytes
            nodes_bytes.extend((node_bytes.len() as u64).to_le_bytes());
            // Write the node bytes
            nodes_bytes.extend(node_bytes);
        }

        // Write the length of nodes
        bytes.extend((nodes_bytes.len() as u64).to_le_bytes());
        // Write the nodes bytes
        bytes.extend(nodes_bytes);
    }
}

impl FromBytes for StoredGraphMeta {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the length of the nodes bytes
        let mut nodes_bytes_length_bytes = [0u8; 8];
        cursor.read_exact(&mut nodes_bytes_length_bytes)?;
        let nodes_bytes_length = u64::from_le_bytes(nodes_bytes_length_bytes) as usize;

        // Read the nodes bytes
        let mut nodes_bytes = vec![0u8; nodes_bytes_length];
        cursor.read_exact(&mut nodes_bytes)?;

        // Read each node
        let mut nodes_cursor = Cursor::new(nodes_bytes);
        let mut node_metas = HashMap::new();
        while (nodes_cursor.position() as usize) < nodes_bytes_length {
            // Read the node ID
            let mut node_id_bytes = [0u8; 8];
            nodes_cursor.read_exact(&mut node_id_bytes)?;
            let node_id = NodeID(u64::from_le_bytes(node_id_bytes) as usize);

            // Read the length of the node bytes
            let mut node_bytes_length_bytes = [0u8; 8];
            nodes_cursor.read_exact(&mut node_bytes_length_bytes)?;
            let node_bytes_length = u64::from_le_bytes(node_bytes_length_bytes) as usize;

            // Read the node bytes
            let mut node_bytes = vec![0u8; node_bytes_length];
            nodes_cursor.read_exact(&mut node_bytes)?;

            // Create the StoredNodeMeta from the bytes and insert it into the map
            let node_meta = StoredNodeMeta::from_bytes(&node_bytes)?;
            node_metas.insert(node_id, node_meta);
        }

        // Construct the new StoredGraphMeta
        let stored_graph_meta = StoredGraphMeta { node_metas };

        Ok(stored_graph_meta)
    }
}
