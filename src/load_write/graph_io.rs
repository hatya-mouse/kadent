use crate::load_write::{AsBytes, FromBytes, safe_read};
use kreniq_engine::{
    graph::{Graph, node_id::NodeID},
    node::Node,
};
use std::io::{Cursor, Read};

impl AsBytes for Graph {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write input_id and output_id
        bytes.extend((self.get_input_id().0 as u64).to_le_bytes());
        bytes.extend((self.get_output_id().0 as u64).to_le_bytes());

        // Write each node
        let mut nodes_bytes = Vec::new();
        for (node_id, node) in self.get_node_map() {
            let mut node_bytes = Vec::new();
            node.as_bytes(&mut node_bytes);
            nodes_bytes.extend((node_id.0 as u64).to_le_bytes());
            nodes_bytes.extend((node_bytes.len() as u64).to_le_bytes());
            nodes_bytes.extend(node_bytes);
        }
        bytes.extend((nodes_bytes.len() as u64).to_le_bytes());
        bytes.extend(nodes_bytes);

        // Write edges as (from_id, out_idx, to_id, in_idx)
        let edges = self.get_edges();
        bytes.extend((edges.len() as u64).to_le_bytes());
        for (from_id, out_idx, to_id, in_idx) in edges {
            bytes.extend((from_id.0 as u64).to_le_bytes());
            bytes.extend((*out_idx as u64).to_le_bytes());
            bytes.extend((to_id.0 as u64).to_le_bytes());
            bytes.extend((*in_idx as u64).to_le_bytes());
        }
    }
}

impl FromBytes for Graph {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut graph = Graph::default();
        let mut cursor = Cursor::new(bytes);
        let mut buf = [0u8; 8];

        // Read input_id and output_id
        cursor.read_exact(&mut buf)?;
        let input_id = NodeID(u64::from_le_bytes(buf) as usize);
        cursor.read_exact(&mut buf)?;
        let output_id = NodeID(u64::from_le_bytes(buf) as usize);

        // Read nodes
        cursor.read_exact(&mut buf)?;
        let nodes_length = u64::from_le_bytes(buf) as usize;
        let nodes_bytes = safe_read(&mut cursor, nodes_length)?;
        let mut nodes_cursor = Cursor::new(nodes_bytes.as_slice());

        while nodes_cursor.position() < nodes_length as u64 {
            nodes_cursor.read_exact(&mut buf)?;
            let node_id = NodeID(u64::from_le_bytes(buf) as usize);
            nodes_cursor.read_exact(&mut buf)?;
            let node_length = u64::from_le_bytes(buf) as usize;
            let node_bytes = safe_read(&mut nodes_cursor, node_length)?;
            let node = <Box<dyn Node>>::from_bytes(&node_bytes)?;
            graph.add_node_with_id(node_id, node);
        }

        graph.set_input_id(input_id);
        graph.set_output_id(output_id);

        // Read edges
        cursor.read_exact(&mut buf)?;
        let edge_count = u64::from_le_bytes(buf) as usize;
        for _ in 0..edge_count {
            cursor.read_exact(&mut buf)?;
            let from_id = NodeID(u64::from_le_bytes(buf) as usize);
            cursor.read_exact(&mut buf)?;
            let out_idx = u64::from_le_bytes(buf) as usize;
            cursor.read_exact(&mut buf)?;
            let to_id = NodeID(u64::from_le_bytes(buf) as usize);
            cursor.read_exact(&mut buf)?;
            let in_idx = u64::from_le_bytes(buf) as usize;
            graph.add_edge_unchecked((from_id, out_idx, to_id, in_idx));
        }

        restore_next_node_id(&mut graph);
        Ok(graph)
    }
}

fn restore_next_node_id(graph: &mut Graph) {
    let next_id = graph
        .get_node_map()
        .keys()
        .map(|id| id.0)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    graph.set_next_node_id(next_id);
}
