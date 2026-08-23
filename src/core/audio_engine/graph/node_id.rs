#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Debug)]
pub(crate) struct NodeID(pub(crate) u64);

impl From<NodeID> for u64 {
    fn from(value: NodeID) -> u64 {
        value.0
    }
}

impl From<u64> for NodeID {
    fn from(value: u64) -> Self {
        NodeID(value)
    }
}
