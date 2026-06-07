use crate::{
    core::kasl_node::KaslNode,
    storage::{AsBytes, FromBytes},
};

impl AsBytes for KaslNode {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        let path_str = self.get_file_path().cloned().unwrap_or_default();
        bytes.extend_from_slice(path_str.as_bytes());
    }
}

impl FromBytes for KaslNode {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let path_str = String::from_utf8(bytes.to_vec())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut node = KaslNode::new();
        if !path_str.is_empty() {
            node.set_file_path(path_str);
        }
        Ok(node)
    }
}
